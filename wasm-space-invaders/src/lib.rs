/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
 
extern crate cfg_if;
extern crate intel_8080_emu;
extern crate space_invaders_core;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use std::cell::RefCell;
use std::ops::Sub;
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use intel_8080_emu::proc_state::Proc8080;
use space_invaders_core::{SpaceInvaderDataBus, SpaceInvaderMachine, INVADERS_ROM};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct App {
  cpu: Proc8080<SpaceInvaderDataBus>,
  machine: Rc<RefCell<SpaceInvaderMachine>>,
  time_elapsed: Duration,
  last_vbl_interrupt: Duration,
  half_interrupt_done: bool,
}

const FRAME_DURATION: Duration = Duration::from_nanos(16666667);
const HALF_FRAME_DURATION: Duration = Duration::from_nanos(16666667 / 2);
const PIXEL_WIDTH: u32 = 224;
const PIXEL_HEIGHT: u32 = 256;

#[wasm_bindgen]
impl App {
  pub fn new() -> App {
    let machine = Rc::new(RefCell::new(SpaceInvaderMachine::new()));
    let data_bus = SpaceInvaderDataBus::new(machine.clone());
    let mut memory = Box::new([0x00; 0xffff]);
    memory[0..INVADERS_ROM.len()].copy_from_slice(INVADERS_ROM);
    let cpu = Proc8080::new(memory, data_bus);

    App {
      cpu,
      machine,
      time_elapsed: Duration::from_millis(0),
      last_vbl_interrupt: Duration::from_millis(0),
      half_interrupt_done: false,
    }
  }

  pub fn run(&mut self, duration_millis: u32) {
    let duration = Duration::from_millis(duration_millis as u64);
    let new_duration = self.time_elapsed + duration;

    if new_duration.sub(self.last_vbl_interrupt) > FRAME_DURATION {
      self.cpu.interrupt(1);
      self.last_vbl_interrupt = new_duration;
      self.half_interrupt_done = false;
    } else if new_duration.sub(self.last_vbl_interrupt) > HALF_FRAME_DURATION
      && !self.half_interrupt_done
    {
      self.cpu.interrupt(2);
      self.half_interrupt_done = true;
    }

    let cycles = ((duration.as_secs() * 1000000) + (duration.subsec_nanos() as u64)) / 500;
    let current_cycles = self.cpu.cycles();
    while self.cpu.cycles() < (current_cycles + cycles) {
      self.cpu.emulate();
    }

    self.time_elapsed = new_duration;
  }

  pub fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
    let pixels: &[u8] = &self.cpu.memory()[0x2400..0x4000];
    let mut buffer = vec![200; (PIXEL_WIDTH * PIXEL_HEIGHT * 4) as usize];

    for (i, px_byte) in pixels.iter().enumerate() {
      let (x_source, y_source) = memory_buffer_index_to_coordinates(i);

      let x_target = y_source;

      for bit in 0..8 {
        let y_target = PIXEL_HEIGHT - 1 - x_source - bit;

        let px = px_byte & (1 << bit) == 0;

        const WHITE: [u8; 4] = [255, 255, 255, 255];
        const BLACK: [u8; 4] = [0, 0, 0, 255];
        let index = (x_target * 4 + (4 * PIXEL_WIDTH * y_target)) as usize;

      
        buffer[index..(index+4)].copy_from_slice(if px { &BLACK } else { &WHITE });
        // console_log!("{:?}", &buffer[index..(index+4)]);
      }
    }

    // console_log!("{}", buffer[110*120*4]);
    ctx.set_image_smoothing_enabled(false);
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), PIXEL_WIDTH, PIXEL_HEIGHT)?;
    
    ctx.put_image_data(&image_data, 0.0, 0.0)
  }

  pub fn handle_key_down(&self, keycode: &str) {
    self.handle_key(keycode, true)
  } 

  pub fn handle_key_up(&self, keycode: &str) {
    self.handle_key(keycode, false)
  } 

  fn handle_key(&self, keycode: &str, down: bool) {
    match keycode {
            "Enter" => self.machine.borrow_mut().insert_coin(down),
            "ControlLeft" => self.machine.borrow_mut().p1_start_button(down),
            "ControlRight" => self.machine.borrow_mut().p2_start_button(down),
            "Space" => {
                self.machine.borrow_mut().p1_fire_button(down);
                self.machine.borrow_mut().p2_fire_button(down);
            },
            "ArrowLeft" => {
                self.machine.borrow_mut().p1_left_button(down);
                self.machine.borrow_mut().p2_left_button(down);
            },
            "ArrowRight" => {
                self.machine.borrow_mut().p1_right_button(down);
                self.machine.borrow_mut().p2_right_button(down);
            },
            _ => (),
    }
  }

}

fn memory_buffer_index_to_coordinates(index: usize) -> (u32, u32) {
    let x = (index as u32 * 8) % PIXEL_HEIGHT;
    let y = (index as u32 * 8) / PIXEL_HEIGHT;
    (x, y)
}