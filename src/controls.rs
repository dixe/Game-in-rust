use nalgebra as na;
use sdl2;
use crate::render_gl;


pub struct Controls {
    pub quit: bool,
    event_pump: sdl2::EventPump,
    a: bool,
    d: bool,
    w: bool,
    s: bool,
    pub movement_dir: na::Vector3::<f32>,
    pub right_stick: Option<na::Vector3::<f32>>,
    pub right_shoulder: bool,
    pub cam_mode: CameraMode,
}

#[derive(PartialEq)]
pub enum CameraMode {
    Follow,
    TopDown
}


#[derive(Debug)]
pub enum Action {
    AddEnemy,
    NoAction
}


impl Controls {

    pub fn new(event_pump: sdl2::EventPump) -> Self {
        let movement_dir = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

        Controls {
            quit: false,
            event_pump: event_pump,
            movement_dir,
            right_stick: None,
            a: false,
            w: false,
            s: false,
            d: false,
            right_shoulder: false,
            cam_mode: CameraMode::Follow
        }
    }

    pub fn handle_inputs(&mut self,  ctx: &mut render_gl::context::Context) -> Action  {

        let mut action = Action::NoAction;
        let mut right_stick = match self.right_stick {
            Some(dir) => dir,
            None => na::Vector3::<f32>::new(0.0, 0.0, 0.0)
        };


        if self.a || self.w || self.d || self.s {
            self.movement_dir.x = 0.0;
            self.movement_dir.y = 0.0;
            self.movement_dir.z = 0.0;
        }

        self.right_shoulder = false;

        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => self.quit = true,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w,h),
                    ..
                } => {
                    ctx.viewport.update_size(w,h);
                    ctx.viewport.set_used(&ctx.gl);
                },
                Event::KeyUp {keycode, ..} =>  {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::A) =>  {
                            self.a = false;
                        },
                        Some(sdl2::keyboard::Keycode::D) =>  {
                            self.d = false;
                        },
                        Some(sdl2::keyboard::Keycode::W) =>  {
                            self.w = false;
                        },
                        Some(sdl2::keyboard::Keycode::S) =>  {
                            self.s = false;
                        },
                        _ => {}
                    }
                },
                Event::KeyDown {keycode, ..} =>  {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::A) =>  {
                            self.a = true;
                        },
                        Some(sdl2::keyboard::Keycode::D) =>  {
                            self.d = true;
                        },
                        Some(sdl2::keyboard::Keycode::W) =>  {
                            self.w = true;
                        },
                        Some(sdl2::keyboard::Keycode::S) =>  {
                            self.s = true;
                        },
                        Some(sdl2::keyboard::Keycode::Escape) =>  {
                            self.quit = true;
                        },
                        Some(sdl2::keyboard::Keycode::R) =>  {
                            println!("Switch Render Mode");
                            ctx.switch_mode();
                        },
                        Some(sdl2::keyboard::Keycode::C) =>  {
                            println!("Switch Cam mdoe");
                            match &self.cam_mode {
                                CameraMode::TopDown => { self.cam_mode = CameraMode::Follow;
                                },
                                CameraMode::Follow => { self.cam_mode = CameraMode::TopDown;
                                },
                            }

                        },

                        // ACTION KEYS
                        Some(sdl2::keyboard::Keycode::N) =>  {
                            action = Action::AddEnemy
                        },
                        _ => {}
                    }
                },
                Event::ControllerAxisMotion {axis, value,..} => {

                    let f_value = (value as f32) / 32768.0;

                    match axis {
                        sdl2::controller::Axis::LeftX => {
                            self.movement_dir.x = f_value;
                        },

                        sdl2::controller::Axis::LeftY => {
                            self.movement_dir.y = -f_value;
                        },

                        sdl2::controller::Axis::RightX => {
                            right_stick.x = f_value;

                        },

                        sdl2::controller::Axis::RightY => {
                            right_stick.y = -f_value;

                        },
                        _ => {}

                    }
                },



                // TRIGGER BUTTON
                Event::ControllerButtonDown {button,..} => {
                    //println!("Pressed button : {:#?} ", button);
                    match button {
                        sdl2::controller::Button::RightShoulder => self.right_shoulder = true,
                        _ => {}

                    }
                },

                Event::ControllerDeviceAdded {which,..} => {
                    ctx.set_controller(which);
                },
                _ => {
                    //println!("Pressed : {:#?} ", a);
                }
            }

        }

        if self.a {
            self.movement_dir.x -= 1.0;
        }


        if self.d {
            self.movement_dir.x -= - 1.0;
        }

        if self.w {
            self.movement_dir.y += 1.0;
        }

        if self.s {
            self.movement_dir.y += - 1.0;
        }

        //CONTROLLER DEADZONE HANDLING
        if self.movement_dir.magnitude() < 0.3 {
            self.movement_dir.x = 0.0;
            self.movement_dir.y = 0.0;
        }

        if right_stick.magnitude() > 0.6 {
            //println!("{} - {}", right_stick, right_stick.magnitude());
            self.right_stick = Some(right_stick.normalize());
        }
        else{
            self.right_stick = None;
        }

        action
    }
}
