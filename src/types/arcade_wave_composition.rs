use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ArcadeWaveComposition {
    pub _0: i32,
    pub _1: Vec<i32>,
}

impl BinaryData for ArcadeWaveComposition {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        91
    }
}

impl Decode for ArcadeWaveComposition {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        let _1 = state.decode()?;
        Ok(ArcadeWaveComposition{ _0, _1 })
    }
}

