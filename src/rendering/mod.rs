extern crate glium;

pub mod lighting;
pub mod geometry;
pub mod camera;
pub mod shapes;

use self::lighting::Light;
use self::geometry::Mesh;
use self::camera::Camera;

pub struct Renderer <'a> {
    pub camera: Camera,
    pub display: glium::backend::glutin_backend::GlutinFacade,
    pub draw_params: glium::DrawParameters <'a>,
    pub light: Light,
}
