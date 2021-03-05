use nalgebra as na;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pos: na::Vector3::<f32>,
    front: na::Vector3::<f32>,
    up: na::Vector3::<f32>,

    projection: na::Matrix4::<f32>
}



impl Camera {

    pub fn new() -> Camera {

        let pos = na::Vector3::new(0.0, -3.5, 20.0);
        let up = na::Vector3::new(0.0, 1.0, 0.0);
        let yawn: f32 = - 90.0_f32.to_radians();
        let pitch: f32 = 10.0_f32.to_radians();
        let front = na::Vector3::new(
            yawn.cos() * pitch.cos(),
            pitch.sin(),
            yawn.sin() * pitch.cos()
        );

        let fov: f32 = 60.0;

        // perspective 3d with depth
        let projection = na::Matrix4::new_perspective(900.0 / 700.0, fov.to_radians(), 0.1, 100.0);

        // orthonoal, more topdown 2d like
        let zoom = 0.27;
        //let projection = na::Matrix4::new_orthographic(-100.0 *zoom, 100.0*zoom, -100.0* zoom, 100.0* zoom, -1.0, 30.0);

        Camera {
            pos,
            front: front.normalize(),
            up,
            projection,
        }
    }

    pub fn move_distance(&mut self, dir: na::Vector3::<f32>) {

        if dir.x == 0.0 && dir.y == 0.0 && dir.z == 0.0 {
            return;
        }
        // use camera speed instead of 0.1
        let unit_dir = dir.normalize() * 0.5;

        self.pos = self.pos + unit_dir;

    }

    pub fn projection(self) -> na::Matrix4::<f32> {

        self.projection
    }

    pub fn view(self) -> na::Matrix4::<f32> {

        let t = self.pos + self.front;
        let target = na::Point3::new(t.x, t.y, t.z);
        let p = na::Point3::new(self.pos.x, self.pos.y, self.pos.z);

        na::Matrix::look_at_rh(&p, &target, &self.up)
    }


}
