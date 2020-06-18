extern crate inputbot;
extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use touchpage::control_nexus::{ControlNexus, ControlUpdateProcessor};
use touchpage::control_updates as cu;
use touchpage::json as J;
use touchpage::webserver;
use touchpage::websocketserver;
// use serde_lexpr::{to_string_pretty, from_str}

#[cfg(target_os = "linux")]
use inputbot::{MouseButton, MouseCursor};
#[cfg(target_os = "windows")]
use inputbot::{MouseButton, MouseCursor, MouseWheel};

mod buildlisp;

use buildlisp::{
  Control::{Key, Label, MouseXy, ScrollButton, Sizer},
   Prefs,
};

use buildlisp as BL;

extern crate serde;
extern crate serde_json;
extern crate serde_lexpr;

fn default_prefs() -> BL::Settings {
  BL::Settings {
    prefs: BL::Prefs {
      xmult: 1000.0,
      ymult: 1000.0,
      max_tap_duration: 100,
      show_press_duration: false,
      scroll_threshold: 10,
      html_port: 8000,
      websocket_port: 9000,
    },
    gui: BL::Gui {
      title: "example".to_string(),
      control: Sizer {
        orientation: BL::Orientation::Vertical,
        controls: vec![
          Sizer {
            orientation: BL::Orientation::Horizontal,
            controls: vec![
              Label {
                label: "label1".to_string(),
                proportion: None,
              },
              Label {
                label: "label2".to_string(),
                proportion: None,
              },
            ],
            proportion: Some(0.1),
          },
          Sizer {
            orientation: BL::Orientation::Horizontal,
            proportion: Some(0.1),
            controls: vec![
              BL::Control::MouseButton {
                label: Some("L".to_string()),
                button: BL::MouseButton::LeftButton,
                proportion: None,
              },
              ScrollButton {
                label: Some("Scroll".to_string()),
                proportion: None,
              },
              BL::Control::MouseButton {
                label: Some("R".to_string()),
                button: BL::MouseButton::RightButton,
                proportion: None,
              },
            ],
          },
          MouseXy {
            label: None,
            proportion: Some(0.5),
          },
          Sizer {
            orientation: BL::Orientation::Horizontal,
            proportion: None,
            controls: vec![Key {
              label: Some("Enter Key".to_string()),
              keys: vec![BL::KeybdKey::EnterKey],
              proportion: Some(0.1),
            }],
          },
        ],
        proportion: None,
      },
    },
    colors: Some(vec![BL::SetColor {
      color: BL::Color::Text,
      hexstring: "FF0000".to_string()}]),
  }
}

fn main() {
  // read in the settings json.
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  let mut prefs_filename = None;
  match iter.next() {
    Some(s1) => match s1.as_str() {
      "--help" => {
        println!("usage:");
        println!("mousepage");
        println!("mousepage --help");
        println!("mousepage <prefs filename>");
        println!("mousepage --writeprefs <filename>");
        return;
      }
      "--writeprefs" => match iter.next() {
        Some(filename) => {
          let p = default_prefs();
          match serde_lexpr::to_string(&p) {
            Err(e) => {
              println!("error converting prefs to s-expression: {:?}", e);
              return;
            }
            Ok(s) => {
              match write_string(s.as_str(), filename.as_str()) {
                Err(e) => println!("error writing prefs file: {:?}", e),
                _ => println!("wrote default prefs to {}", filename),
              }
              return;
            }
          }
        }
        None => {
          println!("no filename supplied for --writeprefs option");
          return;
        }
      },
      pf => {
        prefs_filename = Some(pf.to_string());
      }
    },
    _ => (),
  }

  let settings = match prefs_filename {
    Some(pf) => match load_string(pf.as_str()) {
      Ok(s) => match serde_lexpr::from_str(s.as_str()) {
        Ok(p) => p,
        Err(e) => {
          println!("error loading prefs: {}", e);
          default_prefs()
        }
      },
      Err(e) => {
        println!("prefs file \"{}\" not loaded, using defaults", pf);
        println!("error {}", e);
        default_prefs()
      }
    },
    None => {
      println!("no prefs file specified, using defaults");
      default_prefs()
    }
  };

  let p = settings.prefs;

  print!(
    "current prefs: {}\n",
    serde_json::to_string_pretty(&p).unwrap_or("error serializing prefs".to_string())
  );

  let rootv: Result<String, FError> = BL::build_gui(settings.gui, settings.colors.unwrap_or(vec![]))
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

  let hp = p.html_port.to_string();
  let wp = p.websocket_port.to_string();

  // the 'ControlUpdateProcessor' does something when an update message comes in.
  let cup = MouseUpdate {
    last_loc: None,
    press_start: None,
    scroll_mode: false,
    prefs: p,
  };

  // start the websocket server.  mandatory for receiving control messages.
  match websocketserver::start(
    guijson.as_str(),
    Box::new(cup),
    "0.0.0.0",
    wp.as_str(),
    false,
  ) {
    Ok(_) => (),
    Err(e) => println!("error starting websocket server: {},", e),
  }

  // start the webserver.  not necessary if you want to serve up the html with your
  // own server.
  webserver::start("0.0.0.0", hp.as_str(), wp.as_str(), None, true);
}

pub struct MouseUpdate {
  last_loc: Option<(f32, f32)>,
  press_start: Option<SystemTime>,
  scroll_mode: bool,
  prefs: Prefs,
}

impl ControlUpdateProcessor for MouseUpdate {
  fn on_update_received(&mut self, update: &cu::UpdateMsg, cn: &mut ControlNexus) -> () {
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
              let nx = (self.prefs.xmult * (x - lx)).round() as i32;
              let ny = (self.prefs.ymult * (y - ly)).round() as i32;
              if self.scroll_mode {
                #[cfg(target_os = "linux")]
                {
                  let mut nlx = lx;
                  let mut nly = ly;
                  if i32::abs(nx) > self.prefs.scroll_threshold {
                    if nx < 0 {
                      MouseButton::OtherButton(6).press();
                      MouseButton::OtherButton(6).release();
                    } else {
                      MouseButton::OtherButton(7).press();
                      MouseButton::OtherButton(7).release();
                    }
                    nlx = *x;
                  }

                  if i32::abs(ny) > self.prefs.scroll_threshold {
                    if ny < 0 {
                      MouseButton::OtherButton(4).press();
                      MouseButton::OtherButton(4).release();
                    } else {
                      MouseButton::OtherButton(5).press();
                      MouseButton::OtherButton(5).release();
                    }

                    nly = *y;
                  }
                  self.last_loc = Some((nlx, nly));
                }

                #[cfg(target_os = "windows")]
                {
                  MouseWheel.scroll_hor(nx);
                  MouseWheel.scroll_ver(ny);
                  self.last_loc = Some((*x, *y));
                }
              } else {
                MouseCursor.move_rel(nx, ny);
                self.last_loc = Some((*x, *y));
              };
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
                    if self.prefs.show_press_duration {
                      println!("press duration: {}", duration.as_millis());
                    }
                    if duration.as_millis() < self.prefs.max_tap_duration.into() {
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
          let mb: Result<BL::MouseButton, serde_lexpr::Error> =
            serde_lexpr::from_str(name.as_str());
          match mb {
            Ok(blmb) => {
              if pr {
                BL::convert_mousebutton(&blmb).press()
              } else {
                BL::convert_mousebutton(&blmb).release()
              }
            }
            Err(_) => {
              if name == "S" {
                // scroll.
                if pr {
                  self.scroll_mode = true;
                  self.press_start = None;
                } else {
                  self.scroll_mode = false;
                };
              } else {
                // assume the name is a KeybdKey vec.
                let keys: Vec<BL::KeybdKey> = match serde_lexpr::from_str(name.as_str()) {
                  Ok(keys) => keys,
                  Err(e) => {
                    println!("key error: {:?}", e);
                    vec![]
                  }
                };

                if pr {
                  for k in &keys {
                    BL::convert_keybdkey(&k).press();
                  }
                } else {
                  for k in keys.iter().rev() {
                    BL::convert_keybdkey(&k).release();
                  }
                }
              }
            }
          }
       });
        ()
      }
      _ => (),
    };
  }
}

const ERRORUI: &'static str = r##"
{
  "title": "test",
  "rootControl": 
    { "type": "label"
       , "name": "lb3"
       , "label": "error loading controls!"
    }
}"##;

fn load_string(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
  let path = &Path::new(&file_name);
  let mut inf = File::open(path)?;
  let mut result = String::new();
  inf.read_to_string(&mut result)?;
  Ok(result)
}

fn write_string(text: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
  let path = &Path::new(&file_name);
  let mut inf = File::create(path)?;
  match inf.write(text.as_bytes()) {
    Ok(_) => Ok(()),
    Err(e) => Err(Box::new(e)),
  }
}
