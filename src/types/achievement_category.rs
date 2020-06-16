use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AchievementCategory {
    pub id: i32,
    pub parent_id: i32,
}

impl BinaryData for AchievementCategory {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        3
    }
}

impl Decode for AchievementCategory {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let parent_id = state.decode()?;
        Ok(AchievementCategory{ id, parent_id })
    }
}

