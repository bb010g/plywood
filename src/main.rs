// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error::Error;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate xcb;

macro_rules! tuple_lockstep {
    (( ), ($($fun:tt,)*)) => (());

    (($val:expr$(, $vals:expr)*), ($($funs:tt)*)) => {
        tuple_lockstep!(($val, $($vals,)*), ($($funs)*))
    };
    (($($val:expr,)*), ($($funs:tt)*)) => {
        tuple_lockstep!(@name(($($val,)*), ($($funs)*)))
    };

    (@name(($($val:expr,)*), ($($funs:tt)*))) => {
        tuple_lockstep!(@go_name(($($val,)*), (), ($($funs)*)))
    };
    (@go_name(($val:expr, $($vals:expr,)*), ($($acc:tt)*), ($($funs:tt)*))) => {
        tuple_lockstep!(@go_name(
            ($($vals,)*), ($($acc)* val: $val,), ($($funs)*))
        )
    };
    (@go_name((), ($($acc:tt)*), ($($funs:tt)*))) => {
        tuple_lockstep!(@init(($($acc)*), ($($funs)*)))
    };

    (@init(($($name:ident : $val:expr,)*), ($($funs:tt)*))) => ({
        $( let $name = $val; )*
        tuple_lockstep!(@eval(($($name,)*), ($($funs)*)))
    });

    (@eval(($($val:ident,)*), ())) => (($($val,)*));
    (@eval(($($val:ident,)*), (|$pat:pat| $body:expr, $($funs:tt)*))) => ({
        $( let $pat = $val; let $val = $body; )*
        tuple_lockstep!(@eval(($($val,)*), ($($funs)*)))
    });
    (@eval(($($val:ident,)*), ($fun:expr, $($funs:tt)*))) => ({
        $( let $val = $fun($val); )*
        tuple_lockstep!(@eval(($($val,)*), ($($funs)*)))
    });
    (@eval(($($val:ident,)*), ($($fun:tt)+))) => {
        tuple_lockstep!(@eval(($($val,)*), ($($fun)+,)))
    };
}

mod state;

fn main() -> Result<(), Box<Error>> {
    let _ =
        simplelog::TermLogger::init(simplelog::LevelFilter::Trace, simplelog::Config::default());

    let mut comp = state::Compositor::new()?;
    comp.event_loop();

    Ok(())
}
