use super::*;

pub struct Then<U, V> {
    first: Option<U>,
    second: V,
}

impl<U: Sequence, V: Sequence> Then<U, V> {
    pub(in super::super) fn new(first: U, second: V) -> Self {
        Self {
            first: Some(first),
            second,
        }
    }
}

impl<U: Sequence, V: Sequence> Sequence for Then<U, V> {
    fn update(&mut self, ctx: &mut Update) -> Result {
        loop {
            break if self.first.is_some() {
                let r = self.first.as_mut().unwrap().update(ctx);
                match r {
                    Result::Running(_) => r,
                    Result::Done => {
                        self.first = None;
                        continue;
                    }
                }
            } else {
                self.second.update(ctx)
            };
        }
    }
}