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
