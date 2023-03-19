use std::mem::size_of;
use serde::{Serialize, Deserialize};
use bincode;

pub use super::Msg;

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Msg for Vec3 {
    fn dtype(&self) -> String {"Vec3".to_string()}
    fn len(&self) -> usize {size_of::<f32>()}
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(&mut self, buf: &[u8]) {
        let v: Vec3 = bincode::deserialize(&buf[..]).unwrap();
        *self = v;
    }
}