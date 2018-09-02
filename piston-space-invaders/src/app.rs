/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{ Instant, Duration };

use piston::input::*;
use piston_window::texture::Filter;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::Texture;
use piston_window::texture::TextureSettings;

use space_invaders_core::{ SpaceInvaderDataBus, SpaceInvaderMachine, INVADERS_ROM };
use intel_8080_emu::proc_state::Proc8080;

use super::{PIXEL_WIDTH, PIXEL_HEIGHT};

pub struct App {
    gl: GlGraphics,
    screen: Texture,
    cpu: Rc<RefCell<Proc8080<SpaceInvaderDataBus>>>,
    machine: Rc<RefCell<SpaceInvaderMachine>>,
    last_vbl_interrupt: Instant,
    last_half_interrupt: Instant,
    last_cpu_run: Instant,
}

// scancodes, not sure it is os dependent but i did not find a way to access the original enum from
// the SDL in there.
mod scancodes {
    pub const LEFT: i32 = 105;
    pub const DOWN: i32 = 108;
    pub const RIGHT: i32 = 106;
    pub const CTRL_R: i32 = 97;
    pub const SYMBOLIC_A: i32 = 30;
    pub const SYMBOLIC_S: i32 = 31;
    pub const SYMBOLIC_D: i32 = 32;
    pub const ENTER: i32 = 28;
    pub const INSERT: i32 = 110;
}



impl App {

    pub fn new(opengl: OpenGL) -> App {

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
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let screen = Texture::from_memory_alpha(
            &[128u8; (PIXEL_WIDTH * PIXEL_HEIGHT) as usize],
            PIXEL_WIDTH, 
            PIXEL_HEIGHT, 
            &texture_settings).unwrap();
        App {
            gl: GlGraphics::new(opengl),
            screen,
            cpu: proc8080,
            machine,
            last_vbl_interrupt: now,
            last_half_interrupt: now,
            last_cpu_run: now,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let screen = &self.screen;
        self.gl.draw(args.viewport(), |c, gl| {

            let scale_width = args.draw_width as f64 / PIXEL_WIDTH as f64;
            let scale_height = args.draw_height as f64 / PIXEL_HEIGHT as f64;
            let actual_scale = scale_width.min(scale_height);
            let actual_width = PIXEL_WIDTH as f64 * actual_scale;
            let transform = c.transform
                .trans((args.draw_width as f64 - actual_width) / 2.0, 0.0)
                .scale(actual_scale, actual_scale);
            image(screen, transform, gl);
        });
    }

    const FRAME_DURATION: Duration = Duration::from_nanos(16666667);

    pub fn run_processor(&mut self) {
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

    pub fn handle_input(&mut self, args: ButtonArgs) {
        let pressed = args.state == ButtonState::Press;
        match args.scancode {
            Some(scancodes::INSERT) => self.machine.borrow_mut().insert_coin(pressed),
            Some(scancodes::ENTER) => self.machine.borrow_mut().p1_start_button(pressed),
            Some(scancodes::CTRL_R) => self.machine.borrow_mut().p2_start_button(pressed),
            Some(scancodes::DOWN) => self.machine.borrow_mut().p1_fire_button(pressed),
            Some(scancodes::LEFT) => self.machine.borrow_mut().p1_left_button(pressed),
            Some(scancodes::RIGHT) => self.machine.borrow_mut().p1_right_button(pressed),
            Some(scancodes::SYMBOLIC_A) => self.machine.borrow_mut().p2_left_button(pressed),
            Some(scancodes::SYMBOLIC_S) => self.machine.borrow_mut().p2_fire_button(pressed),
            Some(scancodes::SYMBOLIC_D) => self.machine.borrow_mut().p2_right_button(pressed),
            _ => (),
        }
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
