use bstring::{bstr, BString};
use log::*;
use measure_time::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use std::time::{Instant, Duration};

use crate::asset::{self, EntityKind, CritterAnim};
use crate::asset::frame::{FrameDb, FrameId};
use crate::asset::map::{MapReader, ELEVATION_COUNT};
use crate::asset::message::{BULLET, Messages};
use crate::asset::proto::ProtoDb;
use crate::asset::script::db::ScriptDb;
use crate::fs::FileSystem;
use crate::game::dialog::Dialog;
use crate::game::fidget::Fidget;
use crate::game::object::{self, LightEmitter, Object};
use crate::game::sequence::move_seq::Move;
use crate::game::sequence::stand::Stand;
use crate::game::script::{self, Scripts, ScriptKind};
use crate::game::ui::action_menu::{self, Action};
use crate::game::ui::hud;
use crate::game::ui::playfield::{HexCursorStyle, Playfield};
use crate::game::world::World;
use crate::graphics::Rect;
use crate::graphics::font::Fonts;
use crate::graphics::geometry::hex::{self, Direction};
use crate::sequence::{self, *};
use crate::state::AppState;
use crate::ui::{self, Ui};
use crate::ui::command::{UiCommand, UiCommandData, ObjectPickKind};
use crate::ui::message_panel::MessagePanel;
use crate::util::EnumExt;
use crate::vm::{Vm, PredefinedProc, Suspend};

const SCROLL_STEP: i32 = 10;

pub struct GameState {
    time: PausableTime,
    fs: Rc<FileSystem>,
    proto_db: Rc<ProtoDb>,
    frm_db: Rc<FrameDb>,
    world: Rc<RefCell<World>>,
    scripts: Scripts,
    sequencer: Sequencer,
    fidget: Fidget,
    message_panel: ui::Handle,
    playfield: ui::Handle,
    dialog: Option<Dialog>,
    shift_key_down: bool,
    last_picked_obj: Option<object::Handle>,
    object_action: Option<ObjectAction>,
    user_paused: bool,
    map_id: Option<i32>,
}

impl GameState {
    pub fn new(
        fs: Rc<FileSystem>,
        language: &str,
        proto_db: Rc<ProtoDb>,
        frm_db: Rc<FrameDb>,
        fonts: Rc<Fonts>,
        now: Instant,
        ui: &mut Ui,
    ) -> Self {
        let time = PausableTime::new(now);

        let viewport = Rect::with_size(0, 0, 640, 380);
        let hex_grid = hex::TileGrid::default();

        let critter_names = Messages::read_file(&fs, language, "game/scrname.msg").unwrap();

        let scripts = Scripts::new(
            proto_db.clone(),
            ScriptDb::new(fs.clone(), language).unwrap(),
            Vm::default());
        let world = World::new(
            proto_db.clone(),
            frm_db.clone(),
            critter_names,
            hex_grid.clone(),
            viewport,
            now,
            fonts.clone());
        let world = Rc::new(RefCell::new(world));
        let sequencer = Sequencer::new(now);
        let fidget = Fidget::new(now);

        let playfield = {
            let rect = Rect::with_size(0, 0, 640, 379);
            let win = ui.new_window(rect.clone(), None);
            ui.new_widget(win, rect, None, None, Playfield::new(world.clone()))
        };
        let message_panel = hud::create(ui);

        Self {
            time,
            fs,
            frm_db,
            proto_db,
            world,
            scripts,
            sequencer,
            fidget,
            message_panel,
            playfield,
            dialog: None,
            shift_key_down: false,
            last_picked_obj: None,
            object_action: None,
            user_paused: false,
            map_id: None,
        }
    }

    pub fn new_game(&mut self, map_name: &str, dude_name: &bstr, ui: &mut Ui) {
        self.world.borrow_mut().clear();
        // Reinsert the hex cursor. Needs `world` to be not borrowed.
        ui.widget_mut::<Playfield>(self.playfield).ensure_hex_cursor();

        let world = &mut self.world.borrow_mut();

        let map = MapReader {
            reader: &mut self.fs.reader(&format!("maps/{}.map", map_name)).unwrap(),
            objects: world.objects_mut(),
            proto_db: &self.proto_db,
            frm_db: &self.frm_db,
            scripts: &mut self.scripts,
        }.read().unwrap();

        self.map_id = Some(map.id);

        for elev in &map.sqr_tiles {
            if let Some(ref elev) = elev {
                for &(floor, roof) in elev.as_slice() {
                    self.frm_db.get(FrameId::new_generic(EntityKind::SqrTile, floor).unwrap()).unwrap();
                    self.frm_db.get(FrameId::new_generic(EntityKind::SqrTile, roof).unwrap()).unwrap();
                }
            } else {}
        }

        fn for_each_direction(fid: FrameId, mut f: impl FnMut(FrameId)) {
            for direction in Direction::iter() {
                if let Some(fid) = fid.with_direction(Some(direction)) {
                    f(fid);
                }
            }
        }
        {
            debug_time!("preloading object FIDs");
            for obj in world.objects().iter() {
                for_each_direction(world.objects().get(obj).borrow().fid, |fid| {
                    if let Err(e) = self.frm_db.get(fid) {
                        warn!("error preloading {:?}: {:?}", fid, e);
                    }
                });
            }
        }
        self.frm_db.get(FrameId::EGG).unwrap();

        world.set_sqr_tiles(map.sqr_tiles);
        world.rebuild_light_grid();

        let dude_fid = FrameId::from_packed(0x100003E).unwrap();
        //    let dude_fid = FrameId::from_packed(0x101600A).unwrap();
        let mut dude_obj = Object::new(dude_fid, None, Some(map.entrance));
        dude_obj.direction = Direction::NE;
        dude_obj.light_emitter = LightEmitter {
            intensity: 0x10000,
            radius: 4,
        };
        let dude_objh = world.insert_object(dude_obj);
        debug!("dude obj: {:?}", dude_objh);
        world.set_dude_obj(dude_objh);
        world.dude_name = dude_name.into();

        world.make_object_standing(dude_objh);

        world.camera_mut().look_at(map.entrance.point);

        self.scripts.vars.global_vars = if map.savegame {
            unimplemented!("read save.dat")
        } else {
            asset::read_game_global_vars(&mut self.fs.reader("data/vault13.gam").unwrap()).unwrap().into()
        };
        self.scripts.vars.map_vars = if map.savegame {
            map.map_vars.clone()
        } else {
            let path = format!("maps/{}.gam", map_name);
            if self.fs.exists(&path) {
                asset::read_map_global_vars(&mut self.fs.reader(&path).unwrap()).unwrap().into()
            } else {
                Vec::new().into()
            }
        };

        // Init scripts.
        {
            let ctx = &mut script::Context {
                world,
                sequencer: &mut self.sequencer,
                dialog: &mut self.dialog,
                message_panel: self.message_panel,
                ui,
                map_id: map.id,
            };

            // PredefinedProc::Start for map script is never called.
            // MapEnter in map script is called before anything else.
            if let Some(sid) = self.scripts.map_sid() {
                self.scripts.execute_predefined_proc(sid, PredefinedProc::MapEnter, ctx)
                    .suspend.map(|_| panic!("can't suspend in MapEnter"));
            }

            self.scripts.execute_procs(PredefinedProc::Start, ctx, |sid| sid.kind() != ScriptKind::System);
            self.scripts.execute_map_procs(PredefinedProc::MapEnter, ctx);
        }
    }

    fn handle_action(
        &mut self,
        ui: &mut Ui,
        obj: object::Handle,
        action: Action,
    ) {
        let mut world = self.world.borrow_mut();
        let world = &mut world;
        match action {
            Action::Rotate => {
                let mut obj = world.objects().get(obj).borrow_mut();
                if let Some(signal) = obj.sequence.take() {
                    signal.cancel();
                }
                obj.direction = obj.direction.rotate_cw();
            }
            Action::Talk => {
                // TODO optimize this.
                for obj in world.objects().iter() {
                    world.objects().get(obj).borrow_mut().cancel_sequence();
                }
                self.sequencer.cleanup(&mut sequence::Cleanup {
                    world,
                });
                let script = world.objects().get(obj).borrow().script;
                if let Some((sid, _)) = script {
                    match self.scripts.execute_predefined_proc(sid, PredefinedProc::Talk,
                        &mut script::Context {
                            world,
                            sequencer: &mut self.sequencer,
                            dialog: &mut self.dialog,
                            ui,
                            message_panel: self.message_panel,
                            map_id: self.map_id.unwrap(),
                        }).suspend
                    {
                        None | Some(Suspend::GsayEnd) => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn world(&self) -> &RefCell<World> {
        &self.world
    }

    pub fn playfield(&self) -> ui::Handle {
        self.playfield
    }

    pub fn time(&self) -> &PausableTime {
        &self.time
    }
}

impl AppState for GameState {
    fn handle_event(&mut self, event: &Event, ui: &mut Ui) -> bool {
        let mut world = self.world.borrow_mut();
        match event {
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                world.camera_mut().origin.x -= SCROLL_STEP;
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                world.camera_mut().origin.x += SCROLL_STEP;
            }
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                world.camera_mut().origin.y += SCROLL_STEP;
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                world.camera_mut().origin.y -= SCROLL_STEP;
            }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                let dude_obj = world.dude_obj().unwrap();
                let new_pos = {
                    let obj = world.objects().get(dude_obj).borrow_mut();
                    let mut new_pos = obj.pos.unwrap();
                    new_pos.elevation += 1;
                    while new_pos.elevation < ELEVATION_COUNT && !world.has_elevation(new_pos.elevation) {
                        new_pos.elevation += 1;
                    }
                    new_pos
                };
                if new_pos.elevation < ELEVATION_COUNT && world.has_elevation(new_pos.elevation) {
                    world.objects_mut().set_pos(dude_obj, new_pos);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                let dude_obj = world.dude_obj().unwrap();
                let new_pos = {
                    let obj = world.objects().get(dude_obj).borrow_mut();
                    let mut new_pos = obj.pos.unwrap();
                    if new_pos.elevation > 0 {
                        new_pos.elevation -= 1;
                        while new_pos.elevation > 0 && !world.has_elevation(new_pos.elevation) {
                            new_pos.elevation -= 1;
                        }
                    }
                    new_pos
                };
                if world.has_elevation(new_pos.elevation) {
                    world.objects_mut().set_pos(dude_obj, new_pos);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::LeftBracket), .. } => {
                world.ambient_light = cmp::max(world.ambient_light as i32 - 1000, 0) as u32;
            }
            Event::KeyDown { keycode: Some(Keycode::RightBracket), .. } => {
                world.ambient_light = cmp::min(world.ambient_light + 1000, 0x10000);
            }
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                let mut pf = ui.widget_mut::<Playfield>(self.playfield);
                pf.roof_visible = pf.roof_visible;
            }
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                self.user_paused = !self.user_paused;
            }

            Event::KeyDown { keycode: Some(Keycode::LShift), .. } |
            Event::KeyDown { keycode: Some(Keycode::RShift), .. } => self.shift_key_down = true,
            Event::KeyUp { keycode: Some(Keycode::LShift), .. } |
            Event::KeyUp { keycode: Some(Keycode::RShift), .. } => self.shift_key_down = false,
            _ => return false,
        }
        true
    }

    fn handle_ui_command(&mut self, command: UiCommand, ui: &mut Ui) {
        match command.data {
            UiCommandData::ObjectPick { kind, obj: objh } => {
                let picked_dude = Some(objh) == self.world.borrow().dude_obj();
                let default_action = if picked_dude {
                    Action::Rotate
                } else {
                    Action::Talk
                };
                match kind {
                    ObjectPickKind::Hover => {
                        ui.widget_mut::<Playfield>(self.playfield).default_action_icon = if self.object_action.is_none() {
                            Some(default_action)
                        }  else {
                            None
                        };

                        if self.last_picked_obj != Some(objh) {
                            self.last_picked_obj = Some(objh);

                            if let Some(name) = self.world.borrow().object_name(objh) {
                                let mut mp = ui.widget_mut::<MessagePanel>(self.message_panel);
                                let mut m = BString::new();
                                m.push(BULLET);
                                m.push_str("You see: ");
                                m.push_str(name);
                                mp.push_message(m);
                            }
                        }
                    }
                    ObjectPickKind::ActionMenu => {
                        ui.widget_mut::<Playfield>(self.playfield).default_action_icon = None;

                        let mut actions = Vec::new();
                        actions.push(default_action);
                        if !actions.contains(&Action::Look) {
                            actions.push(Action::Look);
                        }
                        if !actions.contains(&Action::Talk) {
                            actions.push(Action::Talk);
                        }
                        if !actions.contains(&Action::Cancel) {
                            actions.push(Action::Cancel);
                        }

                        let playfield_win = ui.window_of(self.playfield).unwrap();
                        self.object_action = Some(ObjectAction {
                            menu: action_menu::show(actions, playfield_win, ui),
                            obj: objh,
                        });

                        self.time.set_paused(true);
                    }
                    ObjectPickKind::DefaultAction => self.handle_action(ui, objh, default_action),
                }
            }
            UiCommandData::HexPick { action, pos } => {
                if action {
                    let world = self.world.borrow();
                    let dude_objh = world.dude_obj().unwrap();
                    if let Some(signal) = world.objects().get(dude_objh).borrow_mut().sequence.take() {
                        signal.cancel();
                    }

                    if let Some(path) = world.path_for_object(dude_objh, pos.point, true) {
                        let anim = if self.shift_key_down {
                            CritterAnim::Walk
                        } else {
                            CritterAnim::Running
                        };
                        if !path.is_empty() {
                            let (seq, signal) = Move::new(dude_objh, anim, path).cancellable();
                            world.objects().get(dude_objh).borrow_mut().sequence = Some(signal);
                            self.sequencer.start(seq.then(Stand::new(dude_objh)));
                        }
                    }
                } else {
                    let mut pf = ui.widget_mut::<Playfield>(self.playfield);
                    let dude_obj = self.world.borrow().dude_obj().unwrap();
                    pf.hex_cursor_style = if self.world.borrow().path_for_object(dude_obj, pos.point, true).is_some() {
                        HexCursorStyle::Normal
                    } else {
                        HexCursorStyle::Blocked
                    };
                }
            }
            UiCommandData::Action { action } => {
                let object_action = self.object_action.take().unwrap();
                self.handle_action(ui, object_action.obj, action);
                action_menu::hide(object_action.menu, ui);
                self.time.set_paused(false);
            }
            UiCommandData::Pick { id } => {
                let (sid, proc_id) = {
                    let dialog = self.dialog.as_mut().unwrap();

                    assert!(dialog.is(command.source));
                    let proc_id = dialog.option(id).proc_id;
                    dialog.clear_options(ui);

                    (dialog.sid(), proc_id)
                };
                let finished = if let Some(proc_id) = proc_id {
                    self.scripts.execute_proc(sid, proc_id,
                        &mut script::Context {
                            ui,
                            world: &mut self.world.borrow_mut(),
                            sequencer: &mut self.sequencer,
                            dialog: &mut self.dialog,
                            message_panel: self.message_panel,
                            map_id: self.map_id.unwrap(),
                        }).assert_no_suspend();
                    // No dialog options means the dialog is finished.
                    self.dialog.as_ref().unwrap().is_empty()
                } else {
                    true
                };
                if finished {
                    self.scripts.resume(&mut script::Context {
                        ui,
                        world: &mut self.world.borrow_mut(),
                        sequencer: &mut self.sequencer,
                        dialog: &mut self.dialog,
                        message_panel: self.message_panel,
                        map_id: self.map_id.unwrap(),
                    }).assert_no_suspend();
                    assert!(!self.scripts.can_resume());
                    // TODO call MapUpdate (multiple times?), see gdialogEnter()
                }

            }
            _ => {}
        }
    }

    fn update(&mut self, delta: Duration) {
        self.time.update(delta);

        self.time.set_paused(self.user_paused || self.scripts.can_resume());

        if self.time.is_running() {
            let mut world = self.world.borrow_mut();
            world.update(self.time.time());

            self.sequencer.update(&mut sequence::Update {
                time: self.time.time(),
                world: &mut world
            });

            self.fidget.update(self.time.time(), &mut world, &mut self.sequencer);
        } else {
            self.sequencer.cleanup(&mut sequence::Cleanup {
                world: &mut self.world.borrow_mut(),
            });
        }
    }
}

pub struct PausableTime {
    time: Instant,
    paused: bool,
}

impl PausableTime {
    pub fn new(time: Instant) -> Self {
        Self {
            time,
            paused: false,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_running(&self) -> bool {
        !self.is_paused()
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn toggle(&mut self) {
        self.paused = !self.paused;
    }

    pub fn update(&mut self, delta: Duration) {
        if !self.paused {
            self.time += delta;
        }
    }

    pub fn time(&self) -> Instant {
        self.time
    }
}

struct ObjectAction {
    menu: ui::Handle,
    obj: object::Handle,
}