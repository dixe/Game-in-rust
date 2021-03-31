use nalgebra as na;
use sdl2;
use crate::render_gl;


pub struct Controls {
    pub quit: bool,
    event_pump: sdl2::EventPump,
    pub a: bool,
    pub d: bool,
    pub w: bool,
    pub s: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub q: bool,
    pub e: bool,
    pub movement_dir: na::Vector3::<f32>,
    pub right_stick: Option<na::Vector3::<f32>>,
    pub right_shoulder: bool,
    pub cam_mode: CameraMode,
    pub reset: bool,
    pub keys: std::collections::HashMap<sdl2::keyboard::Keycode, bool>,

    pub mouse_move: na::Vector2::<f32>,



}


#[derive(PartialEq)]
pub enum CameraMode {
    Follow,
    TopDown,
    Free
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
            q: false,
            e: false,
            up: false,
            down: false,
            left: false,
            right: false,
            right_shoulder: false,
            cam_mode: CameraMode::Follow,
            reset: false,
            keys: std::collections::HashMap::new(),

            mouse_move: na::Vector2::new(0.0, 0.0),
        }
    }

    pub fn handle_inputs(&mut self,  ctx: &mut render_gl::context::Context) -> Action  {

        self.reset = false;
        let mut action = Action::NoAction;
        let mut right_stick = match self.right_stick {
            Some(dir) => dir,
            None => na::Vector3::<f32>::new(0.0, 0.0, 0.0)
        };


        self.mouse_move = na::Vector2::new(0.0, 0.0);
        if self.a || self.w || self.d || self.s  {
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
                        }
                        Some(sdl2::keyboard::Keycode::E) =>  {
                            self.e = false;
                        },
                        Some(sdl2::keyboard::Keycode::Q) =>  {
                            self.q = false;
                        },

                        Some(sdl2::keyboard::Keycode::Left) =>  {
                            self.left = false;
                        },
                        Some(sdl2::keyboard::Keycode::Right) =>  {
                            self.right = false;
                        },
                        Some(sdl2::keyboard::Keycode::Down) =>  {
                            self.down = false;
                        },
                        Some(sdl2::keyboard::Keycode::Up) =>  {
                            self.up = false;
                        },
                        Some(sdl2::keyboard::Keycode::A) =>  {
                            self.s = false;
                        }

                        Some(key) => {
                            self.keys.insert(key, false);
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
                        Some(sdl2::keyboard::Keycode::E) =>  {
                            self.e = true;
                        },
                        Some(sdl2::keyboard::Keycode::Q) =>  {
                            self.q = true;
                        },

                        Some(sdl2::keyboard::Keycode::Left) =>  {
                            self.left = true;
                        },
                        Some(sdl2::keyboard::Keycode::Right) =>  {
                            self.right = true;
                        },
                        Some(sdl2::keyboard::Keycode::Down) =>  {
                            self.down = true;
                        },
                        Some(sdl2::keyboard::Keycode::Up) =>  {
                            self.up = true;
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
                                _ => {}
                            }

                        },

                        // ACTION KEYS
                        Some(sdl2::keyboard::Keycode::N) =>  {
                            action = Action::AddEnemy
                        },

                        Some(key) => {
                            self.keys.insert(key, true);
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
                        sdl2::controller::Button::Y => self.reset = true,
                        _ => {}

                    }
                },

                Event::ControllerDeviceAdded {which,..} => {
                    ctx.set_controller(which);
                },
                Event::MouseMotion {xrel, yrel, x, y, ..} => {
                    println!("x: {}\ny: {}",xrel,yrel);
                    self.mouse_move.x = xrel as f32;
                    self.mouse_move.y = yrel as f32;
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
