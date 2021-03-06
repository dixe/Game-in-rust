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
    pub shoot_dir: Option<na::Vector3::<f32>>,

}


impl Controls {

    pub fn new(event_pump: sdl2::EventPump) -> Self {
        let movement_dir = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

        Controls {
            quit: false,
            event_pump: event_pump,
            movement_dir,
            shoot_dir: None,
            a: false,
            w: false,
            s: false,
            d: false,
        }
    }

    pub fn handle_inputs(&mut self,  ctx: &mut render_gl::context::Context)  {

        let mut shoot_dir = match self.shoot_dir {
            Some(dir) => dir,
            None => na::Vector3::<f32>::new(0.0, 0.0, 0.0)
        };

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
                        Some(sdl2::keyboard::Keycode::M) =>  {
                            println!("switch");
                            ctx.switch_mode();
                        },
                        _ => {}
                    }
                },
                Event::ControllerAxisMotion {axis, value,..} => {

                    let mut f_value = (value as f32) / 32768.0;

                    if is_dead_zone(value) {
                        //println!("value : {}", value);
                        f_value = 0.0;

                    }

                    match axis {
                        sdl2::controller::Axis::LeftX => {
                            self.movement_dir.x = f_value;
                        },

                        sdl2::controller::Axis::LeftY => {
                            self.movement_dir.y = -f_value;
                        },

                        sdl2::controller::Axis::RightX => {
                            shoot_dir.x = f_value;

                        },

                        sdl2::controller::Axis::RightY => {
                            shoot_dir.y = -f_value;

                        },
                        _ => {}

                    }
                },

                // TRIGGER BUTTON
                Event::JoyAxisMotion {..} => {
                    //println!("axis");
                },

                Event::ControllerDeviceAdded {which,..} => {
                    ctx.set_controller(which);
                },
                evt => {
                    //println!("EVENT ALL RIGHT {}", evt.is_joy());

                }
            }

        }

        if self.a || self.w || self.d || self.s {
            self.movement_dir.x = 0.0;
            self.movement_dir.y = 0.0;
            self.movement_dir.z = 0.0;
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



        if shoot_dir.magnitude() > 0.5 {
            self.shoot_dir = Some(shoot_dir.normalize());
        }
        else{
            self.shoot_dir = None;
        }
    }
}



fn is_dead_zone(value: i16) -> bool{
    value < 12768 && value > -12768
}
