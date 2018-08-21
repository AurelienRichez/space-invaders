/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use intel_8080_emu::proc_state::DataBus;
use std::rc::Rc;
use std::cell::RefCell;

/*
 From : http://computerarcheology.com/Arcade/SpaceInvaders/Hardware.html
    Port 0
    bit 0 DIP4 (Seems to be self-test-request read at power up)
    bit 1 Always 1
    bit 2 Always 1
    bit 3 Always 1
    bit 4 Fire
    bit 5 Left
    bit 6 Right
    bit 7 ? tied to demux port 7 ?

    Port 1
    bit 0 = CREDIT (1 if deposit)
    bit 1 = 2P start (1 if pressed)
    bit 2 = 1P start (1 if pressed)
    bit 3 = Always 1
    bit 4 = 1P shot (1 if pressed)
    bit 5 = 1P left (1 if pressed)
    bit 6 = 1P right (1 if pressed)
    bit 7 = Not connected

    Port 2
    bit 0 = DIP3 00 = 3 ships  10 = 5 ships
    bit 1 = DIP5 01 = 4 ships  11 = 6 ships
    bit 2 = Tilt
    bit 3 = DIP6 0 = extra ship at 1500, 1 = extra ship at 1000
    bit 4 = P2 shot (1 if pressed)
    bit 5 = P2 left (1 if pressed)
    bit 6 = P2 right (1 if pressed)
    bit 7 = DIP7 Coin info displayed in demo screen 0=ON

    Port 3
    bit 0-7 Shift register data

*/ 

pub struct SpaceInvaderMachine {
    port_0: u8,
    port_1: u8,
    port_2: u8,
    shift_value: u16,
    shift_offset: u8,
}

const START_P1_OFFSET: u8 = 2;
const FIRE_BTN_OFFSET: u8 = 4;
const LEFT_BTN_OFFSET: u8 = 5;
const RIGHT_BTN_OFFSET: u8 = 6;

impl SpaceInvaderMachine {
    pub fn new() -> SpaceInvaderMachine {
        SpaceInvaderMachine {
            shift_value: 0,
            shift_offset: 0,
            port_0: 0b00001111,
            port_1: 0b00001000,
            port_2: 0b00000000,
        }
        }

    pub fn insert_coin(&mut self, pressed: bool) {
        self.port_1 = (self.port_1 & !1) | pressed as u8;
    }

    pub fn left_button(&mut self, pressed: bool) {
        self.port_1 = (self.port_1 & !(1 << LEFT_BTN_OFFSET)) | ((pressed as u8) << LEFT_BTN_OFFSET);
    }

    pub fn right_button(&mut self, pressed: bool) {
        self.port_1 = (self.port_1 & !(1 << RIGHT_BTN_OFFSET)) | ((pressed as u8) << RIGHT_BTN_OFFSET);
    }

    pub fn fire_button(&mut self, pressed: bool) {
        self.port_1 = (self.port_1 & !(1 << FIRE_BTN_OFFSET)) | ((pressed as u8) << FIRE_BTN_OFFSET);
    }

    pub fn start_button(&mut self, pressed:bool) {
        self.port_1 = (self.port_1 & !(1 << START_P1_OFFSET)) | ((pressed as u8) << START_P1_OFFSET);
    }

    fn read_port(&self, port: u8) -> u8 {
        match port {
            0 => self.port_0,
            1 => self.port_1,
            2 => self.port_2,
            3 => ((self.shift_value << (self.shift_offset as u16)) >> 8) as u8,
            _ => panic!("unknown in port"),
        }
    }

    fn write_port(&mut self, port: u8, value: u8) {
         match port {
            2 => self.shift_offset = value & 0x07,
            3 => (/* TODO play sound*/),
            4 => {
                self.shift_value >>= 8;
                self.shift_value |= (value as u16) << 8;
            },
            5 => (/* TODO play sound*/),
            6 => (/* TODO should reset*/),
            _ => panic!("unknown out port"),
        }
    }
}

pub struct SpaceInvaderDataBus {
    machine_ref: Rc<RefCell<SpaceInvaderMachine>>,
}

impl SpaceInvaderDataBus {
    pub fn new(machine_ref: Rc<RefCell<SpaceInvaderMachine>>) -> SpaceInvaderDataBus {
        SpaceInvaderDataBus { machine_ref }
    }
}

impl DataBus for SpaceInvaderDataBus {

    fn read_port(&self, port: u8) -> u8 {
        self.machine_ref.borrow_mut().read_port(port)
    }

    fn write_port(&mut self, port: u8, value:u8) {
         self.machine_ref.borrow_mut().write_port(port, value)
    }
}

#[cfg(test)]
mod tests {

    use machine::SpaceInvaderDataBus;
    use machine::SpaceInvaderMachine;
    use intel_8080_emu::proc_state::DataBus;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test] 
    fn shift(){
        let machine = SpaceInvaderMachine::new();
        let mut shifter = SpaceInvaderDataBus::new(Rc::new(RefCell::new(machine)));
        shifter.write_port(4, 0xee);
        shifter.write_port(4, 0xff);
        assert_eq!(0xff, shifter.read_port(3));
        shifter.write_port(2, 4);
        assert_eq!(0xfe, shifter.read_port(3));
    }

}