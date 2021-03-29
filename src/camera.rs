use nalgebra as na;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub cam_pos: na::Vector3::<f32>,
    pub cam_target: na::Vector3::<f32>,
    pub up: na::Vector3::<f32>,
    pub projection: na::Matrix4::<f32>,


    pub follow_distance: f32,
    pub follow_dir: na::Vector3::<f32>,
    pub follow_yaw: f32,
    pub follow_pitch: f32
}



impl Camera {

    pub fn new(width: u32, height: u32) -> Camera {

        let cam_pos = na::Vector3::new(0.0, -10.5, 20.0);

        let up = na::Vector3::new(0.0, 0.0, 1.0);
        //let yaw: f32 = -90.0_f32.to_radians();
        //let pitch: f32 = 20.0_f32.to_radians();

        let yaw: f32 = -1.70;
        let pitch: f32 = 0.54;
        let front = na::Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos()
        );

        let cam_target = cam_pos + front;

        let fov: f32 = 60.0;

        // perspective 3d with depth
        let projection = na::Matrix4::new_perspective(width as f32 / height as f32, fov.to_radians(), 0.1, 100.0);

        //orthonoal, more topdown 2d like
        //let zoom = 0.27;
        //let projection = na::Matrix4::new_orthographic(-100.0 *zoom, 100.0*zoom, -100.0* zoom, 100.0* zoom, -1.0, 30.0);

        Camera {
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
