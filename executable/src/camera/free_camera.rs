use crate::camera::camera::{Camera, CameraMode};


pub struct FreeCamera {
    pub pos: na::Vector3::<f32>,
    pub front: na::Vector3::<f32>,
    pub up: na::Vector3::<f32>,
    pub world_up: na::Vector3::<f32>,
    pub right: na::Vector3::<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub width: f32,
    pub height: f32,
    pub fov: f32
}


impl FreeCamera {

    pub fn new() -> FreeCamera {

        let pos = na::Vector3::new(0.0, 3.8, 2.5);
        let front = na::Vector3::new(1.0, 1.0, 0.0);
        let up = na::Vector3::new(0.0, 1.0, 1.0);
        let right = na::Vector3::new(1.0, 0.0, 0.0);

        FreeCamera {
            pos,
            front,
            up,
            world_up: na::Vector3::new(0.0, 0.0, 1.0),
            right,
            yaw: -1.570,
            pitch: -0.130,
            width: 900.0,
            height: 700.0,
            fov: 60.0

        }
    }
}


impl Camera for FreeCamera {

    fn mode(&self) -> CameraMode {
        CameraMode::Free
    }


    fn move_camera(&mut self, dir: na::Vector3::<f32>, delta: f32) {

        let speed = 5.0;


        self.pos += self.front * dir.y * delta * speed;

        self.pos += self.right * dir.x * delta * speed;

        self.pos += self.up * dir.z * delta * speed;
    }

    fn update_movement(&mut self, x_change: f32, y_change: f32) {

        let sens = 0.01;

        self.yaw -= x_change * sens;
        self.pitch -= y_change * sens;


        //println!("yaw: {:.03} pitch: {:.03} pos: ({:?})", self.yaw, self.pitch, self.pos);

        let max_pitch = 80.0_f32.to_radians();
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        }

        if self.pitch < -max_pitch {
            self.pitch = -max_pitch;
        }

        self.update_camera_vectors();
    }


    fn update_camera_vectors(&mut self) {

        self.front = na::Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        ).normalize();

        self.right = self.front.cross(&self.world_up).normalize();

        self.up = self.right.cross(&self.front).normalize();
    }


    fn projection(&self ) -> na::Matrix4::<f32> {
        na::Matrix4::new_perspective(self.width / self.height, self.fov.to_radians(), 0.1, 100.0)
    }


    fn update_target(&mut self, _target: na::Vector3::<f32>) {

    }

    fn pos(&self) -> na::Vector3::<f32> {
        self.pos
    }


    fn front(&self) -> na::Vector3::<f32> {
        self.front
    }

    fn up(&self) -> na::Vector3::<f32> {
        self.up
    }

    fn set_pos(&mut self, new_pos: na::Vector3::<f32>) {
        self.pos = new_pos;
    }
}
