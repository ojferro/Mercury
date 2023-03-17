use std::mem::size_of;

trait Msg {
    fn len(&self);
    fn serialize(&self);
}

pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Msg for Vec3<T> {
    fn len(&self) {size_of<T>()}
    fn serialize(&self) {
        [x.to_be_bytes(), y.to_be_bytes(), z.to_be_bytes()].concat()
    }
}