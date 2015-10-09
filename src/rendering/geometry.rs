extern crate cgmath;
extern crate glium;

use cgmath::{Vector2, Vector3, Matrix4};
use glium::Program;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}
implement_vertex!(Vertex, position, normal, tex_coord);

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub program: glium::program::Program,
    pub tex_diffuse: glium::texture::SrgbTexture2d,
    pub tex_normal: glium::texture::Texture2d,
}

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub matrix: Matrix4<f32>,
}
