use stringreader::StringReader;
use std::io::BufReader;
use image::io::Reader as ImageReader;
use image;
use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use bvh_anim;
use walkdir::WalkDir;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(io::Error),
    #[fail(display = "WalkDir error")]
    Walkdir(walkdir::Error),
    #[fail(display = "Image error")]
    Image(image::ImageError),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FailedToGetExePath,
    #[fail(display = "Failed to get executable path")]
    FileContainsNil,
    #[fail(display = "Was None")]
    NoneE,
    #[fail(display = "Error loading Bvh")]
    Bvh

}



impl From<bvh_anim::errors::LoadError> for Error {
    fn from(other: bvh_anim::errors::LoadError) -> Self {
        Error::Bvh
    }
}

impl From<walkdir::Error> for Error {
    fn from(other: walkdir::Error) -> Self {
        Error::Walkdir(other)
    }
}


impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}


impl From<image::ImageError> for Error {

    fn from(other: image::ImageError) -> Self {
        Error::Image(other)
    }
}


pub struct Resources {
    root_path: PathBuf
}


impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;

        let exe_path = exe_file_name.parent()
            .ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn list_files(&self, path: &str) -> Result<Vec<String>, Error> {
        let root_path = &self.root_path.to_str().ok_or(Error::NoneE)?;

        let f_p = resource_name_to_path(&self.root_path, path);
        let full_path: &str = f_p.to_str().ok_or(Error::NoneE)?;

        let mut res = Vec::new();

        for entry in WalkDir::new(full_path) {
            let full_p = format!("{}", entry?.path().display());

            let split = full_p.replace(root_path, "")[1..].to_string();
            res.push(split);

        }

        Ok(res)
    }


    pub fn load_bvh(&self, name: &str) -> Result<bvh_anim::Bvh, Error> {

        let data = self.load_string(name)?;

        let str_reader = StringReader::new(&data);
        let mut buf = BufReader::new(str_reader);
        let bvh = bvh_anim::from_reader(buf)?;

        Ok(bvh)
    }


    pub fn load_image_rgb8(&self, resource_name: &str) -> Result<image::RgbImage, Error> {

        let path = resource_name_to_path(&self.root_path, resource_name);

        let image = ImageReader::open(path)?.decode()?.into_rgb8();

        Ok(image)


    }


    pub fn load_string(&self, resource_name: &str) -> Result<String, Error> {

        let content = std::fs::read_to_string(resource_name_to_path(&self.root_path, resource_name))?;
        Ok(content)
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;

        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as  usize + 1
        );

        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part)
    }

    path
}
