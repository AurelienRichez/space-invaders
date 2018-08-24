/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate gtk;
extern crate gdk;
extern crate cairo;
extern crate intel_8080_emu;

mod machine;

use gtk::prelude::*;
use gtk::{DrawingArea, Window, WindowType};
use cairo::ImageSurface;
use intel_8080_emu::proc_state::Proc8080;
use machine::{ SpaceInvaderDataBus, SpaceInvaderMachine };
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{ Instant, Duration };
use gdk::EventType;
use gdk::enums::key;

fn main() {
    run_space_invader()
}

const PIXEL_WIDTH: i32 = 224;
const PIXEL_HEIGHT: i32 = 256;

fn run_space_invader() {

    let invader_rom = include_bytes!(env!("ROM_PATH"));

    let machine = Rc::new(RefCell::new(SpaceInvaderMachine::new()));
    let data_bus = SpaceInvaderDataBus::new(machine.clone());

    let mut memory = Box::new([0x00; 0xffff]);
    memory[0..invader_rom.len()].copy_from_slice(invader_rom);

    let proc8080 = Rc::new(RefCell::new(Proc8080::new(
            memory, 
            Box::new(|| { panic!("halted")}), 
            data_bus,
        )));

    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Space invader");

    let drawing_area = Rc::new(set_up_drawing_area(proc8080.clone()));
    drawing_area.set_size_request(PIXEL_WIDTH, PIXEL_HEIGHT);

    set_proc_timeout(proc8080.clone(), drawing_area.clone());
    window.add(drawing_area.as_ref());

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_event(move |_w, event| {
        match event.get_event_type() {
            EventType::KeyPress => {
                let event_key = event.clone().downcast::<gdk::EventKey>().unwrap();
                handle_key(&machine, event_key, true);
            },
            EventType::KeyRelease => {
                let event_key = event.clone().downcast::<gdk::EventKey>().unwrap();
                handle_key(&machine, event_key, false);
            }, 
            _ => (),
        }
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn handle_key(machine: &Rc<RefCell<SpaceInvaderMachine>>, event: gdk::EventKey, pressed: bool) {
    match event.get_keyval() {
        key::Return => machine.borrow_mut().insert_coin(pressed),
        key::s => machine.borrow_mut().start_button(pressed),
        key::space => machine.borrow_mut().fire_button(pressed),
        key::Left => machine.borrow_mut().left_button(pressed),
        key::Right => machine.borrow_mut().right_button(pressed),
        _ => (),
    }
}


fn set_up_drawing_area(proc8080: Rc<RefCell<Proc8080<SpaceInvaderDataBus>>>) -> DrawingArea {
    

    let drawing_area = DrawingArea::new();
    drawing_area.connect_draw(move |canvas, cr| {
        
        let mut surface =ImageSurface::create(cairo::Format::A8, PIXEL_WIDTH, PIXEL_HEIGHT)
            .expect("Could not create image surface");

        surface.get_data().as_mut().map(|data| {
            
            let borrowed = proc8080.borrow();
            let pixels: &[u8] = &borrowed.memory()[0x2400..0x4000];

            // transform the pixels (reverse + rotation)
            for (i, px_byte) in pixels.iter().enumerate() {
                let (x_source, y_source) = memory_buffer_index_to_coordinates(i);

                let x_target = y_source;
                
                for bit in 0..8 {
                    let y_target = PIXEL_HEIGHT - 1 - x_source - bit;

                    let px = px_byte & (1 << bit) == 0 ;
                    let index = (y_target * PIXEL_WIDTH) + x_target;

                    data[index as usize] = if px { 0xff } else { 0 } ;
                }
            }
        }).unwrap();
        let drawing_width = canvas.get_allocated_width();
        let drawing_height = canvas.get_allocated_height();
        cr.scale(drawing_width as f64 / PIXEL_WIDTH as f64, drawing_height as f64 / PIXEL_HEIGHT as f64);
        cr.set_source_surface(&surface, 0.0, 0.0);
        cr.paint();
        Inhibit(false)
    });
    drawing_area
}

fn memory_buffer_index_to_coordinates(index: usize) -> (i32, i32) {
    let x = (index as i32 * 8) % PIXEL_HEIGHT;
    let y = (index as i32 * 8) / PIXEL_HEIGHT;
    (x, y)
}

fn set_proc_timeout(proc8080: Rc<RefCell<Proc8080<SpaceInvaderDataBus>>>, drawing_area: Rc<DrawingArea>) {

    let frame_duration = Duration::from_nanos(16666667);
    let mut last_run = Instant::now();
    let mut last_vbl_interrupt = Instant::now();
    let mut last_half_interrupt = Instant::now() - (frame_duration / 2);

    timeout_add(4, move || {
        let now = Instant::now();

        if now.duration_since(last_vbl_interrupt) > frame_duration {
            proc8080.borrow_mut().interrupt(1);
            last_vbl_interrupt = now;
            drawing_area.queue_draw();
        } else if now.duration_since(last_half_interrupt) > frame_duration {
            proc8080.borrow_mut().interrupt(2);
            last_half_interrupt = now;
        }

        let cycles = now.duration_since(last_run).subsec_nanos() as u64 / 500 ;
        let current_cycles = proc8080.borrow().cycles();
        while proc8080.borrow().cycles() < (current_cycles + cycles) {
            proc8080.borrow_mut().emulate().unwrap();
        }

        last_run = now;

        gtk::Continue(true)
    });
}