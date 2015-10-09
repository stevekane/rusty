extern crate cgmath;

use cgmath::{Vector2, Vector3};
use super::geometry::Vertex;

pub fn new_plane() -> Vec<Vertex> {
    vec![
        Vertex { 
            position: Vector3::new(-1.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            tex_coord: Vector2::new(0.0, 1.0)
        },     
        Vertex { 
            position: Vector3::new(1.0, 1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            tex_coord: Vector2::new(1.0, 1.0)
        },     
        Vertex { 
            position: Vector3::new(-1.0, -1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            tex_coord: Vector2::new(0.0, 0.0)
        },     
        Vertex { 
            position: Vector3::new(1.0, -1.0, 0.0),
            normal: Vector3::new(0.0, 0.0, -1.0),
            tex_coord: Vector2::new(1.0, 0.0)
        },     
    ]
}
