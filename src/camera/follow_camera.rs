use crate::camera::camera::{Camera, CameraMode};

#[derive(Copy, Clone, Debug)]
pub struct FollowCamera {
    pub pos: na::Vector3::<f32>,
    pub target: na::Vector3::<f32>,
    pub look_dir: na::Vector3::<f32>,
    pub up: na::Vector3::<f32>,
    pub world_up: na::Vector3::<f32>,
    pub right: na::Vector3::<f32>,
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub max_dist: f32,
    pub max_pitch: f32,
}



impl FollowCamera {

    pub fn new(_width: u32, _height: u32) -> FollowCamera {


        let pos = na::Vector3::new(-5.0, 0.0, 5.0);
        let target = na::Vector3::new(0.0, 0.0, 2.0);
        let look_dir = na::Vector3::new(0.0, 0.0, 0.0);
        let up = na::Vector3::new(0.0, 0.0, 1.0);
        let right = na::Vector3::new(1.0, 0.0, 0.0);

        FollowCamera {
            pos,
            target,
            look_dir,
            up,
            world_up: na::Vector3::new(0.0, 0.0, 1.0),
            right,
            width: 900.0,
            height: 700.0,
            fov: 60.0,
            max_dist: 7.0,
            max_pitch: 80.0_f32.to_radians(),
        }
    }
}


impl Camera for FollowCamera {


    fn mode(&self) -> CameraMode {
        CameraMode::Follow
    }


    fn move_camera(&mut self, _dir: na::Vector3::<f32>, _delta: f32) {


    }

    fn update_movement(&mut self, x_change: f32, y_change: f32) {


        let speed = 0.4;
        let change_x = self.right * (-x_change * speed);

        let change_y = self.up * (y_change * speed);

        let new_pos = self.pos + change_x + change_y;

        let new_pitch = f32::asin(-(self.target - self.pos).normalize().z);



        if new_pitch < self.max_pitch || y_change < 0.0 {
            self.pos = new_pos;
        }
        else {
            self.pos  += change_x;
            println!("too bic {}", y_change);

        }

        self.update_camera_vectors();
    }


    fn update_camera_vectors(&mut self) {
        self.look_dir = (self.target - self.pos).normalize();
        self.right = self.look_dir.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.look_dir).normalize();
    }


    fn projection(&self ) -> na::Matrix4::<f32> {
        na::Matrix4::new_perspective(self.width / self.height, self.fov.to_radians(), 0.1, 100.0)
    }


    fn update_target(&mut self, target: na::Vector3::<f32>)  {
        self.target = target;
        let new_dist = (self.target - self.pos).magnitude();



        if new_dist < self.max_dist {
            self.look_dir = (self.target - self.pos).normalize();
        }
        else {
            self.pos = self.target - self.look_dir * self.max_dist;
        }


        self.update_camera_vectors();

    }

    fn pos(&self) -> na::Vector3::<f32> {
        self.pos
    }


    fn front(&self) -> na::Vector3::<f32> {
        self.look_dir
    }

    fn up(&self) -> na::Vector3::<f32> {
        self.up
    }

}
