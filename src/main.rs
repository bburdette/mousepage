extern crate inputbot;
extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use std::time::SystemTime;
use touchpage::control_nexus::{ControlNexus, ControlUpdateProcessor};
use touchpage::control_updates as cu;
use touchpage::controls::Orientation::{Horizontal, Vertical};
use touchpage::guibuilder as G;
use touchpage::json as J;
use touchpage::webserver;
use touchpage::websocketserver;

#[cfg(target_os = "linux")]
use inputbot::{MouseButton, MouseCursor};
#[cfg(target_os = "windows")]
use inputbot::{MouseButton, MouseCursor, MouseWheel};

fn main() {
  let mbhtml = None;

  let rootv: Result<String, FError> = build_gui()
    .and_then(|gui| gui.to_root())
    .map(|root| J::serialize_root(&root))
    .and_then(|rootv| serde_json::to_string_pretty(&rootv).map_err(|_| err_msg("uh oh")));

  let guijson = match rootv {
    Ok(s) => s,
    Err(e) => {
      println!("error loading controls! {}", e);
      ERRORUI.to_string()
    }
  };

  // the 'ControlUpdateProcessor' does something when an update message comes in.
  let cup = MouseUpdate {
    last_loc: None,
    press_start: None,
    scroll_mode: false,
  };

  // start the websocket server.  mandatory for receiving control messages.
  match websocketserver::start(guijson.as_str(), Box::new(cup), "0.0.0.0", "9001", false) {
    Ok(_) => (),
    Err(e) => println!("error starting websocket server: {},", e),
  }

  // start the webserver.  not necessary if you want to serve up the html with your
  // own server.
  webserver::start("0.0.0.0", "8000", "9001", mbhtml, true);
}

pub struct MouseUpdate {
  last_loc: Option<(f32, f32)>,
  press_start: Option<SystemTime>,
  scroll_mode: bool,
}

impl ControlUpdateProcessor for MouseUpdate {
  fn on_update_received(&mut self, update: &cu::UpdateMsg, cn: &mut ControlNexus) -> () {
    let mousemult = 1200.0;
    let click_duration = 100;
    // println!("control update: {:?}", update);
    match update {
      cu::UpdateMsg::XY {
        control_id: _,
        state,
        location,
        label: _,
      } => {
        match location {
          Some((x, y)) => match self.last_loc {
            Some((lx, ly)) => {
              let nx = (mousemult * (x - lx)).round() as i32;
              let ny = (mousemult * (y - ly)).round() as i32;
              if self.scroll_mode {
                let scrollthres = 10;
                #[cfg(target_os = "linux")]
                {
                  if i32::abs(ny) > scrollthres {
                    if ny < 0 {
                      MouseButton::OtherButton(4).press();
                      MouseButton::OtherButton(4).release();
                    } else {
                      MouseButton::OtherButton(5).press();
                      MouseButton::OtherButton(5).release();
                    }
                  }
                  if i32::abs(nx) > scrollthres {
                    if nx < 0 {
                      MouseButton::OtherButton(6).press();
                      MouseButton::OtherButton(6).release();
                    } else {
                      MouseButton::OtherButton(7).press();
                      MouseButton::OtherButton(7).release();
                    }
                  }
                }

                #[cfg(target_os = "windows")]
                {
                  MouseWheel.scroll_hor(nx);
                  MouseWheel.scroll_ver(ny);
                }
              } else {
                MouseCursor.move_rel(nx, ny);
              };
              self.last_loc = Some((*x, *y));
            }
            None => {
              self.last_loc = Some((*x, *y));
            }
          },
          None => (),
        };
        match state {
          Some(cu::PressState::Pressed) => match self.press_start {
            None => {
              self.press_start = Some(SystemTime::now());
            }
            _ => (),
          },

          None => match self.press_start {
            None => {
              self.press_start = Some(SystemTime::now());
            }
            _ => (),
          },

          Some(cu::PressState::Unpressed) => {
            // reset last location, we'll start that again next press.
            self.last_loc = None;

            // check the press duration.  if its short enough we'll do a
            // button press.
            match self.press_start {
              Some(lu) => {
                let now = SystemTime::now();
                match now.duration_since(lu) {
                  Ok(duration) => {
                    // println!("press duration: {}", duration.as_millis());
                    if duration.as_millis() < click_duration {
                      MouseButton::LeftButton.press();
                      MouseButton::LeftButton.release();
                    }
                  }
                  Err(_) => (),
                }
              }
              _ => (),
            }
            self.press_start = None;
          }
        };
      }
      cu::UpdateMsg::Button {
        control_id, state, ..
      } => {
        let pr = match state {
          Some(cu::PressState::Pressed) => true,
          Some(cu::PressState::Unpressed) => false,
          _ => false,
        };
        cn.get_name(control_id).map(|name| {
          if name == "LB" {
            // left mouse.
            if pr {
              MouseButton::LeftButton.press()
            } else {
              MouseButton::LeftButton.release()
            };
          } else if name == "RB" {
            // right mouse.
            if pr {
              MouseButton::RightButton.press()
            } else {
              MouseButton::RightButton.release()
            };
          } else if name == "S" {
            // scroll.
            if pr {
              self.scroll_mode = true;
              self.press_start = None;
            } else {
              self.scroll_mode = false;
            };
          };
        });
        ()
      }
      _ => (),
    };
  }
}

// build the UI with a series of rust function calls.
fn build_gui() -> Result<G::Gui, FError> {
  let mut gui = G::Gui::new_gui("test".to_string());
  gui
    .add_sizer(Vertical, Some(vec![0.1, 0.5]))?
    .add_sizer(Horizontal, None)?
    .add_button("LB".to_string(), Some("Left".to_string()))?
    .add_button("S".to_string(), Some("Scroll".to_string()))?
    .add_button("RB".to_string(), Some("Right".to_string()))?
    .end_sizer()?
    .add_xy("xy".to_string(), Some("xy".to_string()))?
    .end_sizer()?
    .set_color(G::Color::Controls, "001F00")
    .set_color(G::Color::Text, "1F0000");
  Ok(gui)
}

// you can also specify the controls in json, like so.
const ERRORUI: &'static str = r##"
{
  "title": "test",
  "rootControl": 
    { "type": "label"
       , "name": "lb3"
       , "label": "error loading controls!"
    }
}"##;
