extern crate image;

use std::io::Read;
use std::fs::File;
use std::path::Path;
use image::DynamicImage;

pub enum ResourceLoadError {
    FileReadFailure,
    FileNotFound,
    ParsingError,
}

pub fn load_shader 
(vertex_path: &Path, fragment_path: &Path) ->
Result<(String, String), ResourceLoadError> {
    match (read_file(&vertex_path), read_file(&fragment_path)) {
        (Ok(vsrc), Ok(fsrc)) => Ok((vsrc, fsrc)),
        _                    => Err(ResourceLoadError::ParsingError)
    }
}

pub fn load_image
(path: &Path) ->
Result<DynamicImage, ResourceLoadError> {
    match image::open(&path) {
        Ok(data) => Ok(data),
        Err(_)   => Err(ResourceLoadError::ParsingError)
    }
}

fn read_file 
(path: &Path) -> 
Result<String, ResourceLoadError> {
    match File::open(&path) {
        Ok(mut file) => {
            let mut s = String::new();

            match file.read_to_string(&mut s) {
                Ok(_)    => Ok(s),
                Err(err) => Err(ResourceLoadError::FileReadFailure),
            }
        } 
        Err(_) => Err(ResourceLoadError::FileNotFound)
    }
}
