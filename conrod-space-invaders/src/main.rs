/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;

extern crate intel_8080_emu;
extern crate space_invaders_core;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{ Instant, Duration };

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::Texture;
use piston_window::texture::TextureSettings;
use piston::input::{ Event, Loop };

use space_invaders_core::{ SpaceInvaderDataBus, SpaceInvaderMachine, INVADERS_ROM };
use intel_8080_emu::proc_state::Proc8080;

const PIXEL_WIDTH: u32 = 224;
const PIXEL_HEIGHT: u32 = 256;

pub struct App {
    gl: GlGraphics,
    screen: Texture,
    cpu: Rc<RefCell<Proc8080<SpaceInvaderDataBus>>>,
    last_vbl_interrupt: Instant,
    last_half_interrupt: Instant,
    last_cpu_run: Instant,
}

impl App {

    fn new(opengl: OpenGL) -> App {

        let machine = Rc::new(RefCell::new(SpaceInvaderMachine::new()));
        let data_bus = SpaceInvaderDataBus::new(machine.clone());
        
        let mut memory = Box::new([0x00; 0xffff]);
        memory[0..INVADERS_ROM.len()].copy_from_slice(INVADERS_ROM);
        
        let proc8080 = Rc::new(RefCell::new(Proc8080::new(
            memory, 
            Box::new(|| { panic!("halted")}), 
            data_bus,
        )));

        let now = Instant::now();
        let screen = Texture::from_memory_alpha(
            &[128u8; (PIXEL_WIDTH * PIXEL_HEIGHT) as usize],
            PIXEL_WIDTH, 
            PIXEL_HEIGHT, 
            &TextureSettings::new()).unwrap();
        App {
            gl: GlGraphics::new(opengl),
            screen,
            cpu: proc8080,
            last_vbl_interrupt: now,
            last_half_interrupt: now,
            last_cpu_run: now,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let screen = &self.screen;
        self.gl.draw(args.viewport(), |c, gl| {
            image(screen, c.transform, gl);
        });
    }

    const FRAME_DURATION: Duration = Duration::from_nanos(16666667);

    fn run_processor(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_vbl_interrupt) > Self::FRAME_DURATION {
            self.cpu.borrow_mut().interrupt(1);
            self.last_vbl_interrupt = now;
        } else if now.duration_since(self.last_half_interrupt) > Self::FRAME_DURATION {
            self.cpu.borrow_mut().interrupt(2);
            self.last_half_interrupt = now;
        }

        let cycles = now.duration_since(self.last_cpu_run).subsec_nanos() as u64 / 500 ;
        let current_cycles = self.cpu.borrow().cycles();
        while self.cpu.borrow().cycles() < (current_cycles + cycles) {
            self.cpu.borrow_mut().emulate().unwrap();
        }

        self.last_cpu_run = now;

        self.copy_screen();
    }

    fn copy_screen(&mut self) {
        use image::{ ImageBuffer, Rgba };

        let borrowed = self.cpu.borrow();
        let pixels: &[u8] = &borrowed.memory()[0x2400..0x4000];

        let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(PIXEL_WIDTH, PIXEL_HEIGHT);

        // transform the pixels (reverse + rotation)
        for (i, px_byte) in pixels.iter().enumerate() {
            let (x_source, y_source) = memory_buffer_index_to_coordinates(i);

            let x_target = y_source;
            
            for bit in 0..8 {
                let y_target = PIXEL_HEIGHT - 1 - x_source - bit;

                let px = px_byte & (1 << bit) == 0 ;
                
                const WHITE: [u8; 4] = [ 255, 255, 255 , 255];
                const BLACK: [u8; 4] = [ 0, 0, 0 , 255];

                buffer[(x_target, y_target)] = 
                if px { 
                    Rgba { data: BLACK }
                } else { 
                    Rgba { data: WHITE } 
                } ;
            }
        }

        self.screen.update(&buffer)
    }
}

fn memory_buffer_index_to_coordinates(index: usize) -> (u32, u32) {
    let x = (index as u32 * 8) % PIXEL_HEIGHT;
    let y = (index as u32 * 8) / PIXEL_HEIGHT;
    (x, y)
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [PIXEL_WIDTH, PIXEL_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Loop(Loop::Render(args)) => app.render(&args),
            Event::Loop(Loop::Idle(_)) => app.run_processor(),
            _ => (),
        }
    }
}
