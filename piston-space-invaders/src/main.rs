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

mod app;

use piston::window::WindowSettings;
use piston_window::AdvancedWindow;
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL };
use piston::input::{ Event, Loop, Input };

use app::App;

const PIXEL_WIDTH: u32 = 224;
const PIXEL_HEIGHT: u32 = 256;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "space-invaders",
            [PIXEL_WIDTH*2, PIXEL_HEIGHT*2]
        )
        .opengl(opengl)
        .samples(0)
        .fullscreen(true)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Loop(Loop::Render(args)) => app.render(&args),
            Event::Loop(Loop::Idle(_)) => app.run_processor(),
            Event::Input(Input::Button(args)) => app.handle_input(args),
            _ => (),
        }
    }
}
