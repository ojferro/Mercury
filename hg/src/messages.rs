// TODO: Autogenerate this file from all the message modules in ./messages/

pub mod vec3; // State module hierarchy and makes it accessible publicly
pub use vec3::Vec3; // Shorted module path (otherwise, use hg::messages::vec3::Vec3. Now, use hg::messages::Vec3)
// TODO: Declare all other message types (found under messages) here, publicly.

pub trait Msg {
    fn dtype(&self) -> String;
    fn len(&self) -> usize;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, buf: &[u8]);
}