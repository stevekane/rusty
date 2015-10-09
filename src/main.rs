#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;
extern crate time;

mod loading;
mod rendering;

use std::io::Cursor;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::collections::{HashMap};
use glium::{DisplayBuild, Surface};
use glium::texture::{SrgbTexture2d, Texture2d};
use cgmath::{Matrix4, Vector2, Vector3, Vector4};

use self::rendering::Renderer;
use self::rendering::lighting::*;
use self::rendering::geometry::*;
use self::rendering::camera::*;
use self::rendering::shapes::*;
use self::loading::*;

struct ResourceCache <'a> {
    images: HashMap<&'a str, image::DynamicImage>,
}

#[derive(Copy, Clone, Debug)]
struct Clock {
    pub elapsed: f32,
}

struct App <'a> {
    resource_cache: ResourceCache<'a>,
    renderer: Renderer<'a>,
    clock: Clock,
}

const TIME_PER_FRAME: u64 = 16666666;
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1024.0;
const DEFAULT_DEPTH: f32 = 1.0;
const DEFAULT_COLOR: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
const DEFAULT_WINDOW_DIMENSIONS: Vector2<u32> = Vector2 {
    x: 1024, 
    y: 768,
};

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(DEFAULT_WINDOW_DIMENSIONS.x, 
                         DEFAULT_WINDOW_DIMENSIONS.y)
        .with_title(format!("Rusty"))
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
           test: glium::draw_parameters::DepthTest::IfLess,
           write: true,
           .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
        .. Default::default()
    };

    let mut app = Box::new(App {
        clock: Clock { 
            elapsed: 0.0 
        },
        resource_cache: ResourceCache { 
            images: HashMap::new(),
        },
        renderer: Renderer {
            camera: Camera::new_default(),
            draw_params: draw_params,
            display: display,
            light: Light { 
                position: Vector3::new(0.0, 0.0, 1.5f32) 
            }
        }
    });

    //move camera back on the z axis a bit
    //app.renderer.camera.position.x = 2.0;
    app.renderer.camera.position.z = 2.0;

    let mesh: Option<Mesh> = {
        let vertex_path = Path::new("src/shaders/vertex.glsl");
        let fragment_path = Path::new("src/shaders/fragment.glsl");
        let plane_vertices = new_plane();
        let shader = load_shader(vertex_path, fragment_path);
        let program = match shader {
            Ok((vsrc, fsrc)) => glium::Program::from_source(&app.renderer.display, &vsrc, &fsrc, None),
            _                => panic!("We failed some how to find either the vertex or fragment source")
        };
        let tex_diffuse = {
            match load_image(Path::new("assets/wall-diffuse.jpg")) {
                Ok(img) => SrgbTexture2d::new(&app.renderer.display, img).ok(),
                Err(e)  => None
            }
        };
        let tex_normal = {
            match load_image(Path::new("assets/wall-normal.png")) {
                Ok(img) => Texture2d::new(&app.renderer.display, img).ok(),
                Err(e)  => None
            }
        };
        let vertex_buffer = glium::VertexBuffer::new(&app.renderer.display, &plane_vertices);

        if let (Ok(program), 
                Some(tex_diffuse), 
                Some(tex_normal), 
                Ok(vertex_buffer)) = (program, tex_diffuse, tex_normal, vertex_buffer) {
             
            Some(Mesh {
                vertices: plane_vertices,
                vertex_buffer: vertex_buffer,
                tex_diffuse: tex_diffuse,
                tex_normal: tex_normal,
                program: program,
            })
        } else {
            None 
        }
    };

    let model_matrix = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,     
        0.0, 1.0, 0.0, 0.0,     
        0.0, 0.0, 1.0, 0.0,     
        0.0, 0.0, 0.0, 1.0,     
    );

    loop {
        let cur_time = time::precise_time_ns();
        let mut target = app.renderer.display.draw();
        let (width, height) = target.get_dimensions();

        //UPDATING
        let light_radius = 10.0;
        let cam_radius = 3.0;
        let orbit_delay = 100.0;

        app.renderer.light.position.x = light_radius * (app.clock.elapsed / orbit_delay).sin();
        app.renderer.light.position.y = light_radius * (app.clock.elapsed / orbit_delay).cos();
        app.renderer.camera.position.x = cam_radius * (app.clock.elapsed / orbit_delay).sin();
        app.renderer.camera.position.z = cam_radius * (app.clock.elapsed / orbit_delay).cos();
        app.renderer.camera.direction.x = -app.renderer.camera.position.x;
        app.renderer.camera.direction.y = -app.renderer.camera.position.y;
        app.renderer.camera.direction.z = -app.renderer.camera.position.z;

        //RENDERING
        target.clear_color_and_depth(DEFAULT_COLOR, DEFAULT_DEPTH);

        if let Some(ref mesh) = mesh {
            let uniforms = uniform! {
                t: app.clock.elapsed,
                tex_diffuse: &mesh.tex_diffuse,
                tex_normal: &mesh.tex_normal,
                u_light: app.renderer.light.position,
                model_mat: model_matrix,
                view_mat: app.renderer.camera.to_view_mat(),
                perspective_mat: perspective(width as f32, height as f32, Z_NEAR, Z_FAR)
            };
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
            let draw_task = target.draw(&mesh.vertex_buffer, &indices, &mesh.program, &uniforms, &app.renderer.draw_params);
        }

        let render_task = target.finish();

        for ev in app.renderer.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _                            => ()
            } 
        }

        let elapsed = time::precise_time_ns() - cur_time;

        if elapsed < TIME_PER_FRAME {
            thread::sleep_ms(((TIME_PER_FRAME - elapsed) / 1000000) as u32);
        }
        app.clock.elapsed += 1.0;
    }
}
