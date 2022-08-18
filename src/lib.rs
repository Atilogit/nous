mod body;
mod shapes;
mod sim;
mod traits;

pub type Num = f32;
pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Vec4 = glam::Vec4;

pub use body::RigidBody;
pub use shapes::Shape;
pub use sim::Simulation;
