mod impls;

use enum_map_derive::Enum;
use enum_primitive_derive::Primitive;
use std::collections::HashMap;

use super::*;
use crate::game::object;
use crate::sequence::chain::Chain;

pub struct State {
    pub sequences: HashMap<object::Handle, Chain>,

    /// Keeps the `script_overrides` flag state.
    /// It is cleared on each invocation of the program initialization code or a procedure.
    pub script_overrides: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            sequences: HashMap::new(),
            script_overrides: false,
        }
    }
}

pub struct Context<'a, 'b> {
    pub prg: &'a mut ProgramState,
    pub ext: &'a mut super::Context<'b>,
}

#[derive(Clone, Copy, Debug, Enum, Hash, Eq, PartialEq, Primitive)]
#[repr(u16)]
pub enum Opcode {
    Noop8000                    = 0x8000,
    ConstShort                  = 0x8001,
    CriticalStart               = 0x8002,
    CriticalDone                = 0x8003,
    Jmp                         = 0x8004,
    Call                        = 0x8005,
    CallAt                      = 0x8006,
    CallCondition               = 0x8007,
    Callstart                   = 0x8008,
    Exec                        = 0x8009,
    Spawn                       = 0x800a,
    Fork                        = 0x800b,
    AToD                        = 0x800c,
    DToA                        = 0x800d,
    Exit                        = 0x800e,
    Detach                      = 0x800f,
    ExitProg                    = 0x8010,
    StopProg                    = 0x8011,
    FetchGlobal                 = 0x8012,
    StoreGlobal                 = 0x8013,
    FetchExternal               = 0x8014,
    StoreExternal               = 0x8015,
    ExportVar                   = 0x8016,
    ExportProc                  = 0x8017,
    Swap                        = 0x8018,
    Swapa                       = 0x8019,
    Pop                         = 0x801a,
    Dup                         = 0x801b,
    PopReturn                   = 0x801c,
    PopExit                     = 0x801d,
    PopAddress                  = 0x801e,
    PopFlags                    = 0x801f,
    PopFlagsReturn              = 0x8020,
    PopFlagsExit                = 0x8021,
    PopFlagsReturnExtern        = 0x8022,
    PopFlagsExitExtern          = 0x8023,
    PopFlagsReturnValExtern     = 0x8024,
    PopFlagsReturnValExit       = 0x8025,
    PopFlagsReturnValExitExtern = 0x8026,
    CheckArgCount               = 0x8027,
    LookupStringProc            = 0x8028,
    PopBase                     = 0x8029,
    PopToBase                   = 0x802a,
    PushBase                    = 0x802b,
    SetGlobal                   = 0x802c,
    FetchProcAddress            = 0x802d,
    Dump                        = 0x802e,
    If                          = 0x802f,
    While                       = 0x8030,
    Store                       = 0x8031,
    Fetch                       = 0x8032,
    Equal                       = 0x8033,
    NotEqual                    = 0x8034,
    LessEqual                   = 0x8035,
    GreaterEqual                = 0x8036,
    Less                        = 0x8037,
    Greater                     = 0x8038,
    Add                         = 0x8039,
    Sub                         = 0x803a,
    Mul                         = 0x803b,
    Div                         = 0x803c,
    Mod                         = 0x803d,
    And                         = 0x803e,
    Or                          = 0x803f,
    Bwand                       = 0x8040,
    Bwor                        = 0x8041,
    Bwxor                       = 0x8042,
    Bwnot                       = 0x8043,
    Floor                       = 0x8044,
    Not                         = 0x8045,
    Negate                      = 0x8046,
    Wait                        = 0x8047,
    Cancel                      = 0x8048,
    Cancelall                   = 0x8049,
    CriticalStart804a           = 0x804a,
    CriticalDone804b            = 0x804b,
    Sayquit                     = 0x804c,
    Sayend                      = 0x804d,
    Saystart                    = 0x804e,
    Saystartpos                 = 0x804f,
    Sayreplytitle               = 0x8050,
    Saygotoreply                = 0x8051,
    Sayreply                    = 0x8052,
    Sayoption                   = 0x8053,
    Saymessage                  = 0x8054,
    Sayreplywindow              = 0x8055,
    Sayoptionwindow             = 0x8056,
    Sayborder                   = 0x8057,
    Sayscrollup                 = 0x8058,
    Sayscrolldown               = 0x8059,
    Saysetspacing               = 0x805a,
    Sayoptioncolor              = 0x805b,
    Sayreplycolor               = 0x805c,
    Sayrestart                  = 0x805d,
    Saygetlastpos               = 0x805e,
    Sayreplyflags               = 0x805f,
    Sayoptionflags              = 0x8060,
    Saymessagetimeout           = 0x8061,
    Createwin                   = 0x8062,
    Deletewin                   = 0x8063,
    Selectwin                   = 0x8064,
    Resizewin                   = 0x8065,
    Scalewin                    = 0x8066,
    Showwin                     = 0x8067,
    Fillwin                     = 0x8068,
    Fillrect                    = 0x8069,
    Fillwin3X3                  = 0x806a,
    Display                     = 0x806b,
    Displaygfx                  = 0x806c,
    Displayraw                  = 0x806d,
    Loadpalettetable            = 0x806e,
    Fadein                      = 0x806f,
    Fadeout                     = 0x8070,
    Gotoxy                      = 0x8071,
    Print                       = 0x8072,
    Format                      = 0x8073,
    Printrect                   = 0x8074,
    Setfont                     = 0x8075,
    Settextflags                = 0x8076,
    Settextcolor                = 0x8077,
    Sethighlightcolor           = 0x8078,
    Stopmovie                   = 0x8079,
    Playmovie                   = 0x807a,
    Movieflags                  = 0x807b,
    Playmovierect               = 0x807c,
    Addregion                   = 0x807f,
    Addregionflag               = 0x8080,
    Addregionproc               = 0x8081,
    Addregionrightproc          = 0x8082,
    Deleteregion                = 0x8083,
    Activateregion              = 0x8084,
    Checkregion                 = 0x8085,
    Addbutton                   = 0x8086,
    Addbuttontext               = 0x8087,
    Addbuttonflag               = 0x8088,
    Addbuttongfx                = 0x8089,
    Addbuttonproc               = 0x808a,
    Addbuttonrightproc          = 0x808b,
    Deletebutton                = 0x808c,
    Hidemouse                   = 0x808d,
    Showmouse                   = 0x808e,
    Mouseshape                  = 0x808f,
    Refreshmouse                = 0x8090,
    Setglobalmousefunc          = 0x8091,
    Addnamedevent               = 0x8092,
    Addnamedhandler             = 0x8093,
    Clearnamed                  = 0x8094,
    Signalnamed                 = 0x8095,
    Addkey                      = 0x8096,
    Deletekey                   = 0x8097,
    Soundplay                   = 0x8098,
    Soundpause                  = 0x8099,
    Soundresume                 = 0x809a,
    Soundstop                   = 0x809b,
    Soundrewind                 = 0x809c,
    Sounddelete                 = 0x809d,
    Setoneoptpause              = 0x809e,
    Selectfilelist              = 0x809f,
    Tokenize                    = 0x80a0,
    GiveExpPoints               = 0x80a1,
    ScrReturn                   = 0x80a2,
    PlaySfx                     = 0x80a3,
    ObjName                     = 0x80a4,
    SfxBuildOpenName            = 0x80a5,
    GetPcStat                   = 0x80a6,
    TileContainsPidObj          = 0x80a7,
    SetMapStart                 = 0x80a8,
    OverrideMapStart            = 0x80a9,
    HasSkill                    = 0x80aa,
    UsingSkill                  = 0x80ab,
    RollVsSkill                 = 0x80ac,
    SkillContest                = 0x80ad,
    DoCheck                     = 0x80ae,
    IsSuccess                   = 0x80af,
    IsCritical                  = 0x80b0,
    HowMuch                     = 0x80b1,
    MarkAreaKnown               = 0x80b2,
    ReactionInfluence           = 0x80b3,
    Random                      = 0x80b4,
    RollDice                    = 0x80b5,
    MoveTo                      = 0x80b6,
    CreateObjectSid             = 0x80b7,
    DisplayMsg                  = 0x80b8,
    ScriptOverrides             = 0x80b9,
    ObjIsCarryingObjPid         = 0x80ba,
    TileContainsObjPid          = 0x80bb,
    SelfObj                     = 0x80bc,
    SourceObj                   = 0x80bd,
    TargetObj                   = 0x80be,
    DudeObj                     = 0x80bf,
    ObjBeingUsedWith            = 0x80c0,
    LocalVar                    = 0x80c1,
    SetLocalVar                 = 0x80c2,
    MapVar                      = 0x80c3,
    SetMapVar                   = 0x80c4,
    GlobalVar                   = 0x80c5,
    SetGlobalVar                = 0x80c6,
    ScriptAction                = 0x80c7,
    ObjType                     = 0x80c8,
    ObjItemSubtype              = 0x80c9,
    GetCritterStat              = 0x80ca,
    SetCritterStat              = 0x80cb,
    AnimateStandObj             = 0x80cc,
    AnimateStandReverseObj      = 0x80cd,
    AnimateMoveObjToTile        = 0x80ce,
    TileInTileRect              = 0x80cf,
    Attack                      = 0x80d0,
    Noop80d1                    = 0x80d1,
    TileDistance                = 0x80d2,
    TileDistanceObjs            = 0x80d3,
    TileNum                     = 0x80d4,
    TileNumInDirection          = 0x80d5,
    PickupObj                   = 0x80d6,
    DropObj                     = 0x80d7,
    AddObjToInven               = 0x80d8,
    RmObjFromInven              = 0x80d9,
    WieldObjCritter             = 0x80da,
    UseObj                      = 0x80db,
    ObjCanSeeObj                = 0x80dc,
    Attack80dd                  = 0x80dd,
    StartGdialog                = 0x80de,
    EndDialogue                 = 0x80df,
    DialogueReaction            = 0x80e0,
    Metarule3                   = 0x80e1,
    SetMapMusic                 = 0x80e2,
    SetObjVisibility            = 0x80e3,
    LoadMap                     = 0x80e4,
    WmAreaSetPos                = 0x80e5,
    SetExitGrids                = 0x80e6,
    AnimBusy                    = 0x80e7,
    CritterHeal                 = 0x80e8,
    SetLightLevel               = 0x80e9,
    GameTime                    = 0x80ea,
    GameTimeInSeconds           = 0x80eb,
    Elevation                   = 0x80ec,
    KillCritter                 = 0x80ed,
    KillCritterType             = 0x80ee,
    CritterDamage               = 0x80ef,
    AddTimerEvent               = 0x80f0,
    RmTimerEvent                = 0x80f1,
    GameTicks                   = 0x80f2,
    HasTrait                    = 0x80f3,
    DestroyObject               = 0x80f4,
    ObjCanHearObj               = 0x80f5,
    GameTimeHour                = 0x80f6,
    FixedParam                  = 0x80f7,
    TileIsVisible               = 0x80f8,
    DialogueSystemEnter         = 0x80f9,
    ActionBeingUsed             = 0x80fa,
    CritterState                = 0x80fb,
    GameTimeAdvance             = 0x80fc,
    RadiationInc                = 0x80fd,
    RadiationDec                = 0x80fe,
    CritterAttemptPlacement     = 0x80ff,
    ObjPid                      = 0x8100,
    CurMapIndex                 = 0x8101,
    CritterAddTrait             = 0x8102,
    CritterRmTrait              = 0x8103,
    ProtoData                   = 0x8104,
    MessageStr                  = 0x8105,
    CritterInvenObj             = 0x8106,
    ObjSetLightLevel            = 0x8107,
    WorldMap                    = 0x8108,
    InvenCmds                   = 0x8109,
    FloatMsg                    = 0x810a,
    Metarule                    = 0x810b,
    Anim                        = 0x810c,
    ObjCarryingPidObj           = 0x810d,
    RegAnimFunc                 = 0x810e,
    RegAnimAnimate              = 0x810f,
    RegAnimAnimateReverse       = 0x8110,
    RegAnimObjMoveToObj         = 0x8111,
    RegAnimObjRunToObj          = 0x8112,
    RegAnimObjMoveToTile        = 0x8113,
    RegAnimObjRunToTile         = 0x8114,
    PlayGmovie                  = 0x8115,
    AddMultObjsToInven          = 0x8116,
    RmMultObjsFromInven         = 0x8117,
    GetMonth                    = 0x8118,
    GetDay                      = 0x8119,
    Explosion                   = 0x811a,
    DaysSinceVisited            = 0x811b,
    GsayStart                   = 0x811c,
    GsayEnd                     = 0x811d,
    GsayReply                   = 0x811e,
    GsayOption                  = 0x811f,
    GsayMessage                 = 0x8120,
    GiqOption                   = 0x8121,
    Poison                      = 0x8122,
    GetPoison                   = 0x8123,
    PartyAdd                    = 0x8124,
    PartyRemove                 = 0x8125,
    RegAnimAnimateForever       = 0x8126,
    CritterInjure               = 0x8127,
    CombatIsInitialized         = 0x8128,
    GdialogBarter               = 0x8129,
    DifficultyLevel             = 0x812a,
    RunningBurningGuy           = 0x812b,
    InvenUnwield                = 0x812c,
    ObjIsLocked                 = 0x812d,
    ObjLock                     = 0x812e,
    ObjUnlock                   = 0x812f,
    ObjIsOpen                   = 0x8130,
    ObjOpen                     = 0x8131,
    ObjClose                    = 0x8132,
    GameUiDisable               = 0x8133,
    GameUiEnable                = 0x8134,
    GameUiIsDisabled            = 0x8135,
    GfadeOut                    = 0x8136,
    GfadeIn                     = 0x8137,
    ItemCapsTotal               = 0x8138,
    ItemCapsAdjust              = 0x8139,
    AnimActionFrame             = 0x813a,
    RegAnimPlaySfx              = 0x813b,
    CritterModSkill             = 0x813c,
    SfxBuildCharName            = 0x813d,
    SfxBuildAmbientName         = 0x813e,
    SfxBuildInterfaceName       = 0x813f,
    SfxBuildItemName            = 0x8140,
    SfxBuildWeaponName          = 0x8141,
    SfxBuildSceneryName         = 0x8142,
    AttackSetup                 = 0x8143,
    DestroyMultObjs             = 0x8144,
    UseObjOnObj                 = 0x8145,
    EndgameSlideshow            = 0x8146,
    MoveObjInvenToObj           = 0x8147,
    EndgameMovie                = 0x8148,
    ObjArtFid                   = 0x8149,
    ArtAnim                     = 0x814a,
    PartyMemberObj              = 0x814b,
    RotationToTile              = 0x814c,
    JamLock                     = 0x814d,
    GdialogSetBarterMod         = 0x814e,
    CombatDifficulty            = 0x814f,
    ObjOnScreen                 = 0x8150,
    CritterIsFleeing            = 0x8151,
    CritterSetFleeState         = 0x8152,
    TerminateCombat             = 0x8153,
    DebugMsg                    = 0x8154,
    CritterStopAttacking        = 0x8155,
    ConstString                 = 0x9001,
    ConstFloat                  = 0xa001,
    ConstLong                   = 0xc001,
}

impl Opcode {
    pub const SIZE: usize = 2;
}

macro_rules! is {
    ($opcode:expr, $handler:expr) => {
        Instruction {
            opcode: $opcode,
            handler: $handler
        }
    };
}

macro_rules! i {
    ($opcode:expr, $handler:expr) => {
        is!($opcode, |ctx| { $handler(ctx).map(|_| None) })
    };
}

pub(in super) mod instructions {
    use super::*;
    use self::Opcode::*;
    use self::impls::*;

    pub static INSTRUCTIONS: [Instruction; enum_len!(Opcode)] = [
        i!(ActionBeingUsed,             unimplemented),
        i!(Activateregion,              unimplemented),
        i!(Add,                         add),
        i!(Addbutton,                   unimplemented),
        i!(Addbuttonflag,               unimplemented),
        i!(Addbuttongfx,                unimplemented),
        i!(Addbuttonproc,               unimplemented),
        i!(Addbuttonrightproc,          unimplemented),
        i!(Addbuttontext,               unimplemented),
        i!(Addkey,                      unimplemented),
        i!(AddMultObjsToInven,          add_mult_objs_to_inven),
        i!(Addnamedevent,               unimplemented),
        i!(Addnamedhandler,             unimplemented),
        i!(AddObjToInven,               add_obj_to_inven),
        i!(Addregion,                   unimplemented),
        i!(Addregionflag,               unimplemented),
        i!(Addregionproc,               unimplemented),
        i!(Addregionrightproc,          unimplemented),
        i!(AddTimerEvent,               add_timer_event),
        i!(And,                         and),
        i!(Anim,                        anim),
        i!(AnimActionFrame,             unimplemented),
        i!(AnimateMoveObjToTile,        unimplemented),
        i!(AnimateStandObj,             unimplemented),
        i!(AnimateStandReverseObj,      unimplemented),
        i!(AnimBusy,                    unimplemented),
        i!(ArtAnim,                     unimplemented),
        i!(AToD,                        atod),
        i!(Attack,                      unimplemented),
        i!(Attack80dd,                  unimplemented),
        i!(AttackSetup,                 unimplemented),
        i!(Bwand,                       bwand),
        i!(Bwnot,                       bwnot),
        i!(Bwor,                        bwor),
        i!(Bwxor,                       bwxor),
        i!(Call,                        call),
        i!(CallAt,                      unimplemented),
        i!(CallCondition,               unimplemented),
        i!(Callstart,                   unimplemented),
        i!(Cancel,                      unimplemented),
        i!(Cancelall,                   unimplemented),
        i!(CheckArgCount,               unimplemented),
        i!(Checkregion,                 unimplemented),
        i!(Clearnamed,                  unimplemented),
        i!(CombatDifficulty,            unimplemented),
        i!(CombatIsInitialized,         combat_is_initialized),
        i!(ConstFloat,                  const_float),
        i!(ConstLong,                   const_int),
        i!(ConstShort,                  const_int),
        i!(ConstString,                 const_string),
        i!(CreateObjectSid,             create_object_sid),
        i!(Createwin,                   unimplemented),
        i!(CriticalDone,                noop),
        i!(CriticalDone804b,            noop),
        i!(CriticalStart,               noop),
        i!(CriticalStart804a,           noop),
        i!(CritterAddTrait,             critter_add_trait),
        i!(CritterAttemptPlacement,     critter_attempt_placement),
        i!(CritterDamage,               unimplemented),
        i!(CritterHeal,                 unimplemented),
        i!(CritterInjure,               unimplemented),
        i!(CritterInvenObj,             critter_inven_obj),
        i!(CritterIsFleeing,            unimplemented),
        i!(CritterModSkill,             unimplemented),
        i!(CritterRmTrait,              unimplemented),
        i!(CritterSetFleeState,         unimplemented),
        i!(CritterState,                unimplemented),
        i!(CritterStopAttacking,        unimplemented),
        i!(CurMapIndex,                 cur_map_index),
        i!(DaysSinceVisited,            unimplemented),
        i!(DebugMsg,                    debug_msg),
        i!(Deletebutton,                unimplemented),
        i!(Deletekey,                   unimplemented),
        i!(Deleteregion,                unimplemented),
        i!(Deletewin,                   unimplemented),
        i!(DestroyMultObjs,             unimplemented),
        i!(DestroyObject,               destroy_object),
        i!(Detach,                      unimplemented),
        i!(DialogueReaction,            unimplemented),
        i!(DialogueSystemEnter,         unimplemented),
        i!(DifficultyLevel,             unimplemented),
        i!(Display,                     unimplemented),
        i!(Displaygfx,                  unimplemented),
        i!(DisplayMsg,                  display_msg),
        i!(Displayraw,                  unimplemented),
        i!(Div,                         div),
        i!(DoCheck,                     do_check),
        i!(DropObj,                     unimplemented),
        i!(DToA,                        dtoa),
        i!(DudeObj,                     dude_obj),
        i!(Dump,                        unimplemented),
        i!(Dup,                         dup),
        i!(Elevation,                   elevation),
        i!(EndDialogue,                 end_dialogue),
        i!(EndgameMovie,                unimplemented),
        i!(EndgameSlideshow,            unimplemented),
        i!(Equal,                       equal),
        i!(Exec,                        unimplemented),
        i!(Exit,                        unimplemented),
        i!(ExitProg,                    exit_prog),
        i!(Explosion,                   unimplemented),
        i!(ExportProc,                  unimplemented),
        i!(ExportVar,                   export_var),
        i!(Fadein,                      unimplemented),
        i!(Fadeout,                     unimplemented),
        i!(Fetch,                       fetch),
        i!(FetchExternal,               fetch_external),
        i!(FetchGlobal,                 fetch_global),
        i!(FetchProcAddress,            unimplemented),
        i!(Fillrect,                    unimplemented),
        i!(Fillwin,                     unimplemented),
        i!(Fillwin3X3,                  unimplemented),
        i!(FixedParam,                  unimplemented),
        i!(FloatMsg,                    float_msg),
        i!(Floor,                       unimplemented),
        i!(Fork,                        unimplemented),
        i!(Format,                      unimplemented),
        i!(GameTicks,                   game_ticks),
        i!(GameTime,                    game_time),
        i!(GameTimeAdvance,             unimplemented),
        i!(GameTimeHour,                game_time_hour),
        i!(GameTimeInSeconds,           game_time_in_seconds),
        i!(GameUiDisable,               unimplemented),
        i!(GameUiEnable,                unimplemented),
        i!(GameUiIsDisabled,            unimplemented),
        i!(GdialogBarter,               gdialog_barter),
        i!(GdialogSetBarterMod,         gdialog_set_barter_mod),
        i!(GetCritterStat,              get_critter_stat),
        i!(GetDay,                      get_day),
        i!(GetMonth,                    get_month),
        i!(GetPcStat,                   unimplemented),
        i!(GetPoison,                   unimplemented),
        i!(GfadeIn,                     unimplemented),
        i!(GfadeOut,                    unimplemented),
        i!(GiqOption,                   giq_option),
        i!(GiveExpPoints,               unimplemented),
        i!(GlobalVar,                   global_var),
        i!(Gotoxy,                      unimplemented),
        i!(Greater,                     greater),
        i!(GreaterEqual,                greater_equal),
        is!(GsayEnd,                    gsay_end),
        i!(GsayMessage,                 gsay_message),
        i!(GsayOption,                  unimplemented),
        i!(GsayReply,                   gsay_reply),
        i!(GsayStart,                   gsay_start),
        i!(HasSkill,                    has_skill),
        i!(HasTrait,                    has_trait),
        i!(Hidemouse,                   unimplemented),
        i!(HowMuch,                     unimplemented),
        i!(If,                          if_),
        i!(InvenCmds,                   unimplemented),
        i!(InvenUnwield,                unimplemented),
        i!(IsCritical,                  unimplemented),
        i!(IsSuccess,                   unimplemented),
        i!(ItemCapsAdjust,              unimplemented),
        i!(ItemCapsTotal,               item_caps_total),
        i!(JamLock,                     jam_lock),
        i!(Jmp,                         jmp),
        i!(KillCritter,                 unimplemented),
        i!(KillCritterType,             unimplemented),
        i!(Less,                        less),
        i!(LessEqual,                   less_equal),
        i!(LoadMap,                     unimplemented),
        i!(Loadpalettetable,            unimplemented),
        i!(LocalVar,                    local_var),
        i!(LookupStringProc,            unimplemented),
        i!(MapVar,                      map_var),
        i!(MarkAreaKnown,               unimplemented),
        i!(MessageStr,                  message_str),
        i!(Metarule,                    metarule),
        i!(Metarule3,                   metarule3),
        i!(Mod,                         mod_),
        i!(Mouseshape,                  unimplemented),
        i!(MoveObjInvenToObj,           move_obj_inven_to_obj),
        i!(MoveTo,                      move_to),
        i!(Movieflags,                  unimplemented),
        i!(Mul,                         mul),
        i!(Negate,                      negate),
        i!(Noop80d1,                    noop),
        i!(Noop8000,                    noop),
        i!(Not,                         not),
        i!(NotEqual,                    not_equal),
        i!(ObjArtFid,                   obj_art_fid),
        i!(ObjBeingUsedWith,            unimplemented),
        i!(ObjCanHearObj,               unimplemented),
        i!(ObjCanSeeObj,                obj_can_see_obj),
        i!(ObjCarryingPidObj,           unimplemented),
        i!(ObjClose,                    unimplemented),
        i!(ObjIsCarryingObjPid,         obj_is_carrying_obj_pid),
        i!(ObjIsLocked,                 obj_is_locked),
        i!(ObjIsOpen,                   obj_is_open),
        i!(ObjItemSubtype,              unimplemented),
        i!(ObjLock,                     obj_lock),
        i!(ObjName,                     obj_name),
        i!(ObjOnScreen,                 obj_on_screen),
        i!(ObjOpen,                     unimplemented),
        i!(ObjPid,                      obj_pid),
        i!(ObjSetLightLevel,            unimplemented),
        i!(ObjType,                     unimplemented),
        i!(ObjUnlock,                   obj_unlock),
        i!(Or,                          or),
        i!(OverrideMapStart,            override_map_start),
        i!(PartyAdd,                    unimplemented),
        i!(PartyMemberObj,              party_member_obj),
        i!(PartyRemove,                 unimplemented),
        i!(PickupObj,                   unimplemented),
        i!(PlayGmovie,                  unimplemented),
        i!(Playmovie,                   unimplemented),
        i!(Playmovierect,               unimplemented),
        i!(PlaySfx,                     unimplemented),
        i!(Poison,                      unimplemented),
        i!(Pop,                         pop),
        i!(PopAddress,                  unimplemented),
        i!(PopBase,                     pop_base),
        i!(PopExit,                     unimplemented),
        i!(PopFlags,                    pop_flags),
        i!(PopFlagsExit,                pop_flags_exit),
        i!(PopFlagsExitExtern,          unimplemented),
        i!(PopFlagsReturn,              pop_flags_return),
        i!(PopFlagsReturnExtern,        unimplemented),
        i!(PopFlagsReturnValExit,       unimplemented),
        i!(PopFlagsReturnValExitExtern, unimplemented),
        i!(PopFlagsReturnValExtern,     unimplemented),
        i!(PopReturn,                   pop_return),
        i!(PopToBase,                   pop_to_base),
        i!(Print,                       unimplemented),
        i!(Printrect,                   unimplemented),
        i!(ProtoData,                   unimplemented),
        i!(PushBase,                    push_base),
        i!(RadiationDec,                unimplemented),
        i!(RadiationInc,                unimplemented),
        i!(Random,                      random),
        i!(ReactionInfluence,           unimplemented),
        i!(Refreshmouse,                unimplemented),
        i!(RegAnimAnimate,              unimplemented),
        i!(RegAnimAnimateForever,       reg_anim_animate_forever),
        i!(RegAnimAnimateReverse,       unimplemented),
        i!(RegAnimFunc,                 reg_anim_func),
        i!(RegAnimObjMoveToObj,         unimplemented),
        i!(RegAnimObjMoveToTile,        unimplemented),
        i!(RegAnimObjRunToObj,          unimplemented),
        i!(RegAnimObjRunToTile,         unimplemented),
        i!(RegAnimPlaySfx,              unimplemented),
        i!(Resizewin,                   unimplemented),
        i!(RmMultObjsFromInven,         unimplemented),
        i!(RmObjFromInven,              unimplemented),
        i!(RmTimerEvent,                rm_timer_event),
        i!(RollDice,                    unimplemented),
        i!(RollVsSkill,                 roll_vs_skill),
        i!(RotationToTile,              rotation_to_tile),
        i!(RunningBurningGuy,           unimplemented),
        i!(Sayborder,                   unimplemented),
        i!(Sayend,                      unimplemented),
        i!(Saygetlastpos,               unimplemented),
        i!(Saygotoreply,                unimplemented),
        i!(Saymessage,                  unimplemented),
        i!(Saymessagetimeout,           unimplemented),
        i!(Sayoption,                   unimplemented),
        i!(Sayoptioncolor,              unimplemented),
        i!(Sayoptionflags,              unimplemented),
        i!(Sayoptionwindow,             unimplemented),
        i!(Sayquit,                     unimplemented),
        i!(Sayreply,                    unimplemented),
        i!(Sayreplycolor,               unimplemented),
        i!(Sayreplyflags,               unimplemented),
        i!(Sayreplytitle,               unimplemented),
        i!(Sayreplywindow,              unimplemented),
        i!(Sayrestart,                  unimplemented),
        i!(Sayscrolldown,               unimplemented),
        i!(Sayscrollup,                 unimplemented),
        i!(Saysetspacing,               unimplemented),
        i!(Saystart,                    unimplemented),
        i!(Saystartpos,                 unimplemented),
        i!(Scalewin,                    unimplemented),
        i!(ScriptAction,                unimplemented),
        i!(ScriptOverrides,             script_overrides),
        i!(ScrReturn,                   unimplemented),
        i!(Selectfilelist,              unimplemented),
        i!(Selectwin,                   unimplemented),
        i!(SelfObj,                     self_obj),
        i!(SetCritterStat,              unimplemented),
        i!(SetExitGrids,                unimplemented),
        i!(Setfont,                     unimplemented),
        i!(SetGlobal,                   set_global),
        i!(Setglobalmousefunc,          unimplemented),
        i!(SetGlobalVar,                set_global_var),
        i!(Sethighlightcolor,           unimplemented),
        i!(SetLightLevel,               set_light_level),
        i!(SetLocalVar,                 set_local_var),
        i!(SetMapMusic,                 unimplemented),
        i!(SetMapStart,                 unimplemented),
        i!(SetMapVar,                   set_map_var),
        i!(SetObjVisibility,            set_obj_visibility),
        i!(Setoneoptpause,              unimplemented),
        i!(Settextcolor,                unimplemented),
        i!(Settextflags,                unimplemented),
        i!(SfxBuildAmbientName,         unimplemented),
        i!(SfxBuildCharName,            unimplemented),
        i!(SfxBuildInterfaceName,       unimplemented),
        i!(SfxBuildItemName,            unimplemented),
        i!(SfxBuildOpenName,            unimplemented),
        i!(SfxBuildSceneryName,         unimplemented),
        i!(SfxBuildWeaponName,          unimplemented),
        i!(Showmouse,                   unimplemented),
        i!(Showwin,                     unimplemented),
        i!(Signalnamed,                 unimplemented),
        i!(SkillContest,                unimplemented),
        i!(Sounddelete,                 unimplemented),
        i!(Soundpause,                  unimplemented),
        i!(Soundplay,                   unimplemented),
        i!(Soundresume,                 unimplemented),
        i!(Soundrewind,                 unimplemented),
        i!(Soundstop,                   unimplemented),
        i!(SourceObj,                   source_obj),
        i!(Spawn,                       unimplemented),
        i!(StartGdialog,                start_gdialog),
        i!(Stopmovie,                   unimplemented),
        i!(StopProg,                    unimplemented),
        i!(Store,                       store),
        i!(StoreExternal,               store_external),
        i!(StoreGlobal,                 store_global),
        i!(Sub,                         sub),
        i!(Swap,                        swap),
        i!(Swapa,                       swapa),
        i!(TargetObj,                   target_obj),
        i!(TerminateCombat,             unimplemented),
        i!(TileContainsObjPid,          tile_contains_pid_obj),
        i!(TileContainsPidObj,          tile_contains_pid_obj),
        i!(TileDistance,                tile_distance),
        i!(TileDistanceObjs,            tile_distance_objs),
        i!(TileInTileRect,              tile_in_tile_rect),
        i!(TileIsVisible,               unimplemented),
        i!(TileNum,                     tile_num),
        i!(TileNumInDirection,          tile_num_in_direction),
        i!(Tokenize,                    unimplemented),
        i!(UseObj,                      unimplemented),
        i!(UseObjOnObj,                 unimplemented),
        i!(UsingSkill,                  unimplemented),
        i!(Wait,                        unimplemented),
        i!(While,                       while_),
        i!(WieldObjCritter,             unimplemented),
        i!(WmAreaSetPos,                unimplemented),
        i!(WorldMap,                    unimplemented),
    ];
}

pub fn instruction_map() -> HashMap<u16, Instruction> {
    let mut map = HashMap::new();
    for &instr in &instructions::INSTRUCTIONS[..] {
        map.insert(instr.opcode() as u16, instr);
    }
    map
}

pub type Handler = fn(Context) -> Result<Option<Suspend>>;

#[derive(Clone, Copy)]
pub struct Instruction {
    opcode: Opcode,
    handler: Handler,
}

impl Instruction {
    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn execute(&self, ctx: Context) -> Result<Option<Suspend>> {
        (self.handler)(ctx)
    }
}