use std::{self, thread, time};
use std::io::stdout;

use serde_json;

use termion;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use core::Core;
use cursor::Cursor;
use errors::*;

pub struct Screen {
    pub stdout: MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>,
    pub size: (u16, u16),
}

impl Screen {
    pub fn new() -> Result<Screen> {
        let stdout = MouseTerminal::from(AlternateScreen::from(stdout().into_raw_mode()?));
        Ok(Screen {
               size: termion::terminal_size()?,
               stdout: stdout,
           })
    }

    pub fn update(&mut self, core: &mut Core) -> Result<()> {
        // TODO(#27): check if terminal size changed. If so, send a `render_line` command to the
        // backend, and a `scroll` command for future updates.
        if let Ok(msg) = core.update_rx.try_recv() {
            let msg_list = msg.as_array().unwrap();
            let (method, params) = (msg_list[0].as_str().unwrap(),
                                    msg_list[1].as_object().unwrap());
            match method {
                "update" => {
                    let update = serde_json::from_value(params.get("update").unwrap().clone())?;
                    core.update(&update)?;
                    core.get_view_mut()
                        .ok_or_else(|| {
                            error!("No view found");
                            ErrorKind::DisplayError
                        })?
                        .render(&mut self.stdout, self.size.1)?
                }
                "scroll_to" => {
                    // Deserialize the cursor position, and let the core update the view.
                    let coord = (params.get("line").unwrap().as_u64().unwrap(),
                                 params.get("col").unwrap().as_u64().unwrap());
                    core.scroll_to(&Cursor::from(coord))?;
                    core.get_view_mut()
                        .ok_or_else(|| {
                            error!("No view found");
                            ErrorKind::DisplayError
                        })?
                        .render(&mut self.stdout, self.size.1)?
                }
                "set_style" => {
                    // TODO:(#26): ???
                }
                _ => {
                    info!("Unknown request from backend {:?}", method);
                }
            }
        } else {
            thread::sleep(time::Duration::from_millis(10));
        }
        Ok(())
    }
}
