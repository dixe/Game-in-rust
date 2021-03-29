use crate::resources::Resources;
use crate::entity;
use crate::render_gl::{self, Model};

pub struct Animation {
    pub frames: usize,
    pub frame_models: Vec<Model>,
    pub frame_anchors: Vec<entity::AnchorPoint>

}


impl Animation {

    pub fn load_from_path(gl: &gl::Gl, clr: na::Vector3::<f32>, path: &str, res: &Resources) -> Result<Animation, failure::Error> {

        println!("{:#?}", path);
        let files = res.list_files(path)?;

        let obj_files = files.iter().filter(|f| f.ends_with(".obj"));

        let mut frame_models = Vec::new();
        let mut frame_anchors = Vec::new();
        let mut frames = 0;
        for obj_path in obj_files {
            println!("Loading from : '{}'", obj_path);
            let (model, anchor) = Model::load_from_path_tobj(gl, clr, obj_path, res)?;
            frame_models.push(model);
            anchor.map(|a| {frame_anchors.push(a);});

            frames +=1;
        }

        Ok(Animation {
            frames,
            frame_models,
            frame_anchors,
        })
    }


    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model_mat: na::Matrix4<f32>, percent: f32) {

        let current_frame = (percent * self.frames as f32) as usize;

        let model = &self.frame_models[current_frame];
        model.render(gl, shader, model_mat);

    }
}
