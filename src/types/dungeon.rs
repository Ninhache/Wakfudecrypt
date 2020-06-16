use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Dungeon {
    pub id: i32,
    pub min_level: i16,
    pub instance_id: i16,
    pub _3: i64,
    pub _4: Vec<i64>,
    pub _5: bool,
    pub _6: bool,
    pub _7: bool,
    pub tps: Vec<i32>,
}

impl BinaryData for Dungeon {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        119
    }
}

impl Decode for Dungeon {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let min_level = state.decode()?;
        let instance_id = state.decode()?;
        let _3 = state.decode()?;
        let _4 = state.decode()?;
        let _5 = state.decode()?;
        let _6 = state.decode()?;
        let _7 = state.decode()?;
        let tps = state.decode()?;
        Ok(Dungeon{ id, min_level, instance_id, _3, _4, _5, _6, _7, tps })
    }
}

