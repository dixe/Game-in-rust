use nalgebra as na;



pub trait Camera {

    fn update_movement(&mut self, x_diff: f32, y_change: f32);

    fn update_camera_vectors(&mut self);

    fn projection(&self ) -> na::Matrix4::<f32> ;

    fn view(&self ) -> na::Matrix4::<f32> ;

    fn z_rotation(&self ) -> f32;

    fn y_rotation(&self ) -> f32;

    fn update_target(&mut self );

    fn pos(&self) -> na::Vector3::<f32>;

    fn move_camera(&mut self, dir: na::Vector3::<f32>, delta: f32);
}



#[derive(Copy, Clone, Debug)]
pub struct FixedCamera {
    pub cam_pos: na::Vector3::<f32>,
    pub cam_target: na::Vector3::<f32>,
    pub up: na::Vector3::<f32>,
    pub projection: na::Matrix4::<f32>,


    pub follow_distance: f32,
    pub follow_dir: na::Vector3::<f32>,
    pub follow_yaw: f32,
    pub follow_pitch: f32,
}


impl FixedCamera {

    pub fn new(width: u32, height: u32) -> FixedCamera {

        let cam_pos = na::Vector3::new(0.0, -10.5, 20.0);

        let up = na::Vector3::new(0.0, 0.0, 1.0);
        //let yaw: f32 = -90.0_f32.to_radians();
        //let pitch: f32 = 20.0_f32.to_radians();

        let yaw: f32 = -3.70;
        let pitch: f32 = -2.54;
        let front = na::Vector3::new(0.0, -1.0, 0.0);

        let cam_target = cam_pos + front;

        let fov: f32 = 60.0;

        // perspective 3d with depth
        let projection = na::Matrix4::new_perspective(width as f32 / height as f32, fov.to_radians(), 0.1, 100.0);

        //orthonoal, more topdown 2d like
        //let zoom = 0.27;
        //let projection = na::Matrix4::new_orthographic(-100.0 *zoom, 100.0*zoom, -100.0* zoom, 100.0* zoom, -1.0, 30.0);

        FixedCamera {
            cam_pos,
            cam_target,
            up,
            projection,
            follow_distance : 12.0,
            follow_pitch: 0.4,
            follow_yaw: std::f32::consts::PI,
            follow_dir: na::Vector3::new(0.0, 0.0, 1.0)
        }
    }


    pub fn set_target(&mut self, target: na::Vector3::<f32>) {

        self.cam_target = target;
        self.cam_pos = target + (self.follow_dir * self.follow_distance);
    }


    pub fn follow_dir_xy(&self) -> na::Vector3::<f32> {
        na::Vector3::new(self.follow_dir.x, self.follow_dir.y,  1.0).normalize()
    }

    pub fn z_rotation(&self) -> f32 {
        f32::atan2(self.follow_dir.y, self.follow_dir.x)
    }

    pub fn y_rotation(&self) -> f32 {

        f32::atan2(self.follow_dir.y, self.follow_dir.z)
    }

    pub fn change_follow_dir(&mut self, change: na::Vector3::<f32>) {

        // TODO also take Delta to make 0.1 depend on delta
        self.follow_yaw -= change.x * 0.05;
        self.follow_pitch += change.y * 0.05;
        self.follow_pitch = f32::max(self.follow_pitch, 0.0);
        self.follow_pitch = f32::min(std::f32::consts::PI/2.0, self.follow_pitch);

        if self.follow_yaw > std::f32::consts::PI * 2.0 {
            self.follow_yaw -= std::f32::consts::PI * 2.0;
        }

        if self.follow_yaw < 0.0 {
            self.follow_yaw += std::f32::consts::PI * 2.0;
        }


        self.follow_dir.x = self.follow_yaw.cos() * f32::max(self.follow_pitch.cos(), 0.01);
        self.follow_dir.y = self.follow_yaw.sin() * f32::max(self.follow_pitch.cos(), 0.01);
        self.follow_dir.z = self.follow_pitch.sin();


        self.follow_dir.normalize();

        // println!("{} {}", self.follow_yaw, self.follow_pitch);
        // println!("{:#?}", self.follow_dir);

    }

    pub fn set_top_down_cam(&mut self) {

        let cam_pos = na::Vector3::new(0.0, -10.5, 20.0);

        let up = na::Vector3::new(0.0, 0.0, 1.0);

        let yaw: f32 = -90.0_f32.to_radians();
        let pitch: f32 = 20.0_f32.to_radians();

        let front = na::Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        );

        self.cam_target = cam_pos + front;

    }

    pub fn update_target(&mut self) {
        let front = na::Vector3::new(
            self.follow_yaw.cos() * self.follow_pitch.cos(),
            self.follow_pitch.sin(),
            self.follow_yaw.sin() * self.follow_pitch.cos()
        );

        self.cam_target = self.cam_pos + front;

    }

    pub fn projection(self) -> na::Matrix4::<f32> {

        self.projection
    }

    pub fn view(self) -> na::Matrix4::<f32> {


        let t = self.cam_target;

        let target = na::Point3::new(t.x, t.y, t.z);

        let p = na::Point3::new(self.cam_pos.x, self.cam_pos.y, self.cam_pos.z);

        na::Matrix::look_at_rh(&p, &target, &self.up)
    }
}

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

pub enum Direction {
    Forward,
    Left,
    Right,
    Backward
}

impl FreeCamera {

    pub fn new() -> FreeCamera {

        let pos = na::Vector3::new(0.0, 3.0, 3.0);
        let front = na::Vector3::new(0.0, -1.0, 0.0);
        let up = na::Vector3::new(0.0, 0.0, 1.0);
        let right = na::Vector3::new(1.0, 0.0, 0.0);

        FreeCamera {
            pos,
            front,
            up,
            world_up: na::Vector3::new(0.0, 0.0, 1.0),
            right,
            yaw: -90_f32.to_radians(),
            pitch: 0.0,
            width: 900.0,
            height: 700.0,
            fov: 60.0

        }
    }
}


impl Camera for FreeCamera {

    fn move_camera(&mut self, dir: na::Vector3::<f32>, delta: f32) {

        let speed = 4.0;


        self.pos += self.front * dir.y * delta * speed;

        self.pos += self.right * dir.x * delta * speed;

        self.pos += self.up * dir.z * delta * speed;
    }

    fn update_movement(&mut self, x_change: f32, y_change: f32) {

        // println!("YAW {}", self.yaw);
        // println!("pitch {}", self.pitch);
        let sens = 0.02;

        self.yaw -= x_change * sens;
        self.pitch -= y_change * sens;



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

        );

        self.front.normalize();

        self.right = self.front.cross(&self.world_up).normalize();

        self.up = self.right.cross(&self.front).normalize();
    }


    fn projection(&self ) -> na::Matrix4::<f32> {

        na::Matrix4::new_perspective(self.width as f32 / self.height as f32, self.fov.to_radians(), 0.1, 100.0)

    }

    fn z_rotation(&self ) -> f32 {
        0.0
    }

    fn y_rotation(&self ) -> f32 {
        0.0
    }

    fn update_target(&mut self )  {

    }

    fn pos(&self) -> na::Vector3::<f32> {
        self.pos
    }

    fn view(&self) -> na::Matrix4::<f32> {
        let target_vec = self.pos + self.front;

        let target = na::Point3::new(target_vec.x, target_vec.y, target_vec.z);

        let point_pos = na::Point3::new(self.pos.x, self.pos.y, self.pos.z);

        na::Matrix::look_at_rh(&point_pos, &target, &self.up)
    }


}
