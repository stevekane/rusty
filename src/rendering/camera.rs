extern crate cgmath;

use cgmath::{Vector3, Matrix4};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub up: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub position: Vector3<f32>,
}

impl Camera {
    pub fn new_default () -> Camera {
        Camera {
            up: Vector3::new(0.0, 1.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
            position: Vector3::new(0.0, 0.0, -1.0),
        } 
    }

    pub fn to_view_mat (self) -> Matrix4<f32> {
        view(&self.position, &self.direction, &self.up)
    }
}

pub fn perspective (width: f32, height: f32, 
                    z_near: f32, z_far: f32) -> Matrix4<f32> {
    let aspect_ratio = height / width;
    let fov: f32 = 3.14592 / 3.0;
    let f: f32 = 1.0 / (fov / 2.0).tan();
    let m11 = f * aspect_ratio;
    let m22 = f;
    let m33 = (z_far + z_near) / (z_far - z_near);
    let m34 = (-2.0 * z_far * z_near) / (z_far - z_near);

    Matrix4::new(
        m11, 0.0, 0.0, 0.0,
        0.0, m22, 0.0, 0.0,
        0.0, 0.0, m33, 1.0,
        0.0, 0.0, m34, 0.0,
    )
}

pub fn view(position: &Vector3<f32>, 
            direction: &Vector3<f32>, 
            up: &Vector3<f32>) -> Matrix4<f32> {
    let f = {
        let d = direction;
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();

        [d[0] / len, d[1] / len, d[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]
    ];

    let s_norm = {
        let len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();

        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
    ];

    Matrix4::new(
        s[0], u[0], f[0], 0.0,
        s[1], u[1], f[1], 0.0,
        s[2], u[2], f[2], 0.0,
        p[0], p[1], p[2], 1.0,
    )
}
