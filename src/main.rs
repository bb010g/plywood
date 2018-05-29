// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error::Error;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate xcb;

mod state;

fn main() -> Result<(), Box<Error>> {
    let _ = simplelog::TermLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default());

    let comp = state::Compositor::new()?;
    comp.event_loop();

    Ok(())
}
