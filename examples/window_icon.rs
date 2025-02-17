// Copyright 2016 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(not(feature = "image"))]
fn main() {
    eprintln!("run with: --features image")
}

#[cfg(feature = "image")]
use image::{
    imageops::{resize, Nearest},
    open as open_image, DynamicImage,
};

#[cfg(feature = "image")]
use glfw::{Action, Context, Key};

#[cfg(feature = "image")]
fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(600, 600, "Window Icon Testing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    if let DynamicImage::ImageRgba8(icon) = open_image("examples/icon.png").unwrap() {
        //Set the icon to be multiple sizes of the same icon to account for scaling
        window.set_icon(vec![
            resize(&icon, 16, 16, Nearest),
            resize(&icon, 32, 32, Nearest),
            resize(&icon, 48, 48, Nearest),
        ]);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

#[cfg(feature = "image")]
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
