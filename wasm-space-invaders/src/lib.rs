extern crate cfg_if;
extern crate intel_8080_emu;
extern crate space_invaders_core;
extern crate wasm_bindgen;

mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::ops::{ Add, Sub };

use intel_8080_emu::proc_state::Proc8080;
use space_invaders_core::{SpaceInvaderDataBus, SpaceInvaderMachine, INVADERS_ROM};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct App {
  cpu: Rc<RefCell<Proc8080<SpaceInvaderDataBus>>>,
  machine: Rc<RefCell<SpaceInvaderMachine>>,
  timeElapsed: Duration,
  last_vbl_interrupt: Duration,
  half_interrupt_done: bool,
}

const FRAME_DURATION: Duration = Duration::from_nanos(16666667);
const HALF_FRAME_DURATION: Duration = Duration::from_nanos(16666667 / 2);

#[wasm_bindgen]
impl App {
  pub fn new() -> App {
    let machine = Rc::new(RefCell::new(SpaceInvaderMachine::new()));
    let data_bus = SpaceInvaderDataBus::new(machine.clone());
    let mut memory = Box::new([0x00; 0xffff]);
    memory[0..INVADERS_ROM.len()].copy_from_slice(INVADERS_ROM);
    let cpu = Rc::new(RefCell::new(Proc8080::new(memory, data_bus)));

    App { 
      cpu,
      machine, 
      timeElapsed: Duration::from_millis(0),
      last_vbl_interrupt: Duration::from_millis(0),
      half_interrupt_done: false,
    }
  }


  pub fn run(&mut self, duration_micros: u64) {
    let duration = Duration::from_micros(duration_micros);
    let new_duration = self.timeElapsed + duration;
    
    if new_duration.sub(self.last_vbl_interrupt) > FRAME_DURATION {
      self.cpu.borrow_mut().interrupt(1);
      self.last_vbl_interrupt = new_duration;
      self.half_interrupt_done = false;
    } else if new_duration.sub(self.last_vbl_interrupt) > HALF_FRAME_DURATION && !self.half_interrupt_done{
      self.cpu.borrow_mut().interrupt(2);
      self.half_interrupt_done = true;
    }

    let cycles = ((duration.as_secs() * 1000000) + (duration.subsec_nanos() as u64)) / 500;
    let current_cycles = self.cpu.borrow().cycles();
    while self.cpu.borrow().cycles() < (current_cycles + cycles) {
      self.cpu.borrow_mut().emulate();
    }

    self.timeElapsed = new_duration;

  }
}
