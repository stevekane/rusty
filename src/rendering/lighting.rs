extern crate cgmath;

use cgmath::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Light {
    pub position: Vector3<f32>,
}
