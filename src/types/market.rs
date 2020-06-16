use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Market {
    pub _0: i32,
}

impl BinaryData for Market {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        40
    }
}

impl Decode for Market {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let _0 = state.decode()?;
        Ok(Market{ _0 })
    }
}

