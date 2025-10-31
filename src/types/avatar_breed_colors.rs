use crate::decode::{Decode, DecodeState};
use crate::BinaryData;
use std::collections::HashMap;
use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AvatarBreedColors {
    pub id: i32,
    pub values: Vec<Data>,
    pub edr: Vec<Data>,
    pub eds: aKX,
    pub edt: aKX,
    pub edu: HashMap<i16, i8>,
    pub edv: HashMap<i16, i8>,
}

impl BinaryData for AvatarBreedColors {
    fn id(_phantom: PhantomData<Self>) -> i32 {
        116
    }
}

impl Decode for AvatarBreedColors {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let id = state.decode()?;
        let values = state.decode()?;
        let edr = state.decode()?;
        let eds = state.decode()?;
        let edt = state.decode()?;
        let edu = state.decode()?;
        let edv = state.decode()?;
        
        Ok(AvatarBreedColors{ id, values, edr, eds, edt, edu, edv})
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    pub sex: i8,
    pub defaultSkinIndex: i8,
    pub defaultSkinFactor: i8,
    pub defaultHairIndex: i8,
    pub defaultHairFactor: i8,
    pub defaultPupilIndex: i8,
    pub skinColors: Vec<Color>,
    pub hairColors: Vec<Color>,
    pub pupilColors: Vec<Color>,
}

impl Decode for Data {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let sex = state.decode()?;
        let defaultSkinIndex = state.decode()?;
        let defaultSkinFactor = state.decode()?;
        let defaultHairIndex = state.decode()?;
        let defaultHairFactor = state.decode()?;
        let defaultPupilIndex = state.decode()?;
        let skinColors = state.decode()?;
        let hairColors = state.decode()?;
        let pupilColors = state.decode()?;

        Ok(Data{sex, defaultSkinIndex, defaultSkinFactor, defaultHairIndex, defaultHairFactor, defaultPupilIndex, skinColors, hairColors, pupilColors})
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Decode for Color {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let red = state.decode()?;
        let green = state.decode()?;
        let blue = state.decode()?;
        let alpha = state.decode()?;

        Ok(Color{red, green, blue, alpha})
    }
}

#[derive(Debug, Clone)]
pub struct aKX {
    pub edz: i8,
    pub edA: i8,
}

impl Decode for aKX {
    fn decode<R: io::Read>(state: &mut DecodeState<R>) -> io::Result<Self> {
        let edz = state.decode()?;
        let edA = state.decode()?;
        
        Ok(aKX { edz, edA })
    }
}