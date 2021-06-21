use crate::resources::{self, Resources} ;
use std::fmt;


#[derive(Debug)]
pub struct Level {

    pub width: i32,
    pub height: i32,
    pub level_data: Vec<i32>,

}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, inner: resources::Error },
    #[fail(display = "Can not parse the level: {}", name)]
    LevelParseFailed { name: String },
}



impl Level {

    pub fn load( res: &Resources, name: &str) -> Result<Level, Error> {

        let level_data_str = res.load_string(name)
            .map_err(|e| Error::ResourceLoad {
                name: name.into(),
                inner: e
            })?;



        let mut level_data = Vec::<i32>::new();
        let lines = level_data_str.lines();

        let mut height: i32 = 0;
        let mut width: i32 = 0;
        for (i, line) in lines.enumerate() {
            width = 0;
            for (j,c) in line.trim().chars().enumerate() {
                let p = c.to_string().parse::<i32>().unwrap();

                level_data.push(p);
                width = j as i32
            }

            height = i  as i32;
        }



        Ok(Level {
            width: width + 1,
            height: height + 1,
            level_data
        })
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "w={}, h={}", self.width, self.height)?;
        Ok(())

    }
}
