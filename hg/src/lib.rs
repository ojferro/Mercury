use std::mem::size_of;
use serde::{Serialize, Deserialize};
use bincode;


pub trait Msg {
    fn len(&self) -> usize;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, buf: &[u8]);
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Msg for Vec3 {
    fn len(&self) -> usize {size_of::<f32>()}
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(&mut self, buf: &[u8]) {
        let v: Vec3 = bincode::deserialize(&buf[..]).unwrap();
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }
}