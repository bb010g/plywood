// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{HashMap, VecDeque};
use std::error::Error;

use xcb;

const INITIAL_CAPACITY: usize = 256;

struct Window {
}

pub struct Compositor {
    conn: xcb::Connection,
    root: xcb::Window,
    windows: HashMap<xcb::Window, Window>,
    stack: VecDeque<xcb::Window>,
}

impl Compositor {
    pub fn new() -> Result<Compositor, Box<Error>> {
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        let root = {
            let setup = conn.get_setup();
            let screen = setup
                .roots()
                .nth(screen_num as usize)
                .ok_or(format!("Couldn't find screen {}", screen_num))?;
            screen.root()
        };

        let mut c = Compositor {
            conn: conn,
            root: root,
            windows: HashMap::with_capacity(INITIAL_CAPACITY),
            stack: VecDeque::with_capacity(INITIAL_CAPACITY),
        };

        c.set_event_mask(
            c.root,
            xcb::EVENT_MASK_STRUCTURE_NOTIFY
                | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY
                | xcb::EVENT_MASK_PROPERTY_CHANGE,
        );
        c.conn.flush();

        let tree = xcb::query_tree(&c.conn, c.root).get_reply()?;
        for child in tree.children() {
            c.add_window(*child);
        }

        Ok(c)
    }

    fn set_event_mask(&self, win: xcb::Window, mask: xcb::EventMask) {
        let attrs = [(xcb::CW_EVENT_MASK, mask)];
        xcb::change_window_attributes(&self.conn, win, &attrs);
    }

    fn add_window(&mut self, win: xcb::Window) {
        debug!("Tracking window {}", win);
        if self.windows.contains_key(&win) {
            warn!("Attempted to add known window {}", win);
            return;
        }
        self.windows.insert(win, Window {
            // TODO initialize this
        });
        self.stack.push_back(win);
    }

    fn remove_window(&mut self, win: xcb::Window) {
        debug!("Untracking window {}", win);
        if !self.windows.contains_key(&win) {
            warn!("Attempted to remove unknown window {}", win);
            return;
        }
        self.windows.remove(&win);
        let i = self.stack.iter().rposition(|&w| w == win).unwrap();
        self.stack.remove(i);
    }

    pub fn event_loop(&mut self) {
        loop {
            if let Some(event) = self.conn.wait_for_event() {
                match event.response_type() & !0x80 {
                    // TODO track damage events
                    xcb::CIRCULATE_NOTIFY => {
                        debug!("CIRCULATE_NOTIFY");
                    }
                    xcb::CONFIGURE_NOTIFY => {
                        debug!("CONFIGURE_NOTIFY");
                    }
                    xcb::CREATE_NOTIFY => {
                        let event = unsafe { xcb::cast_event::<xcb::CreateNotifyEvent>(&event) };
                        self.add_window(event.window());
                    }
                    xcb::DESTROY_NOTIFY => {
                        let event = unsafe { xcb::cast_event::<xcb::DestroyNotifyEvent>(&event) };
                        self.remove_window(event.window());
                    }
                    xcb::GRAVITY_NOTIFY => {
                        debug!("GRAVITY_NOTIFY");
                    }
                    xcb::MAP_NOTIFY => {
                        let event = unsafe { xcb::cast_event::<xcb::MapNotifyEvent>(&event) };
                        debug!("Mapped window {}", event.window());
                    }
                    xcb::REPARENT_NOTIFY => {
                        let event = unsafe { xcb::cast_event::<xcb::ReparentNotifyEvent>(&event) };
                        let win = event.window();
                        let parent = event.parent();
                        if parent == self.root {
                            self.add_window(win);
                        } else {
                            self.remove_window(win);
                        }
                    }
                    xcb::UNMAP_NOTIFY => {
                        let event = unsafe { xcb::cast_event::<xcb::UnmapNotifyEvent>(&event) };
                        debug!("Unmapped window {}", event.window());
                    }
                    xcb::PROPERTY_NOTIFY => {
                        debug!("PROPERTY_NOTIFY");
                    }
                    xcb::CLIENT_MESSAGE => {
                        debug!("CLIENT_MESSAGE");
                    }
                    t => {
                        warn!("Unhandled event {}", t);
                    }
                }
            } else {
                break;
            }
        }
    }
}
