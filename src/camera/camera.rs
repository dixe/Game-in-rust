use nalgebra as na;



#[derive(Debug, PartialEq)]
pub enum CameraMode {
    Follow,
    Free
}



pub trait Camera {

    fn mode(&self) -> CameraMode;

    fn update_movement(&mut self, x_diff: f32, y_change: f32);

    fn update_camera_vectors(&mut self);

    fn projection(&self ) -> na::Matrix4::<f32> ;

    fn z_rotation(&self ) -> f32 {
        let follow_dir = -self.front();
        f32::atan2(follow_dir.y, follow_dir.x)
    }

    fn update_target(&mut self, target: na::Vector3::<f32>);

    fn pos(&self) -> na::Vector3::<f32>;

    fn front(&self) -> na::Vector3::<f32>;

    fn up(&self) -> na::Vector3::<f32>;

    fn move_camera(&mut self, dir: na::Vector3::<f32>, delta: f32);

    fn view(&self) -> na::Matrix4::<f32> {
        let target_vec = self.pos() + self.front();

        let target = na::Point3::new(target_vec.x, target_vec.y, target_vec.z);

        let pos = self.pos();
        let point_pos = na::Point3::new(pos.x, pos.y, pos.z);

        na::Matrix::look_at_rh(&point_pos, &target, &self.up())
    }
}
