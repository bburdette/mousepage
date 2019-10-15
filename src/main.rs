extern crate inputbot;
extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use touchpage::control_nexus::{ControlNexus, ControlUpdateProcessor};
use touchpage::control_updates as cu;
use touchpage::controls::Orientation::{Horizontal, Vertical};
use touchpage::guibuilder as G;
use touchpage::json as J;
use touchpage::webserver;
use touchpage::websocketserver;

use inputbot::{MouseButton, MouseCursor};

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
  let cup = MouseUpdate { last_loc: None };

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
}

impl ControlUpdateProcessor for MouseUpdate {
  fn on_update_received(&mut self, update: &cu::UpdateMsg, cn: &mut ControlNexus) -> () {
    let mousemult = 1200.0;
    // println!("control update: {:?}", update);
    match update {
      cu::UpdateMsg::XY {
        control_id: _,
        state,
        location,
        label: _ ,
      } => {
        match location {
          Some((x, y)) => match self.last_loc {
            Some((lx, ly)) => {
              let nx = x - lx;
              let ny = y - ly;
              MouseCursor.move_rel(
                (nx * mousemult).round() as i32,
                (ny * mousemult).round() as i32,
              );
              self.last_loc = Some((*x, *y));
            }
            None => {
              self.last_loc = Some((*x, *y));
            }
          },
          None => (),
        };
        match state {
          Some(cu::PressState::Unpressed) => self.last_loc = None,
          _ => (),
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
          if name == "b0" {
            // left mouse.
            if pr {
              MouseButton::LeftButton.press()
            } else {
              MouseButton::LeftButton.release()
            };
          } else if name == "b1" {
            // right mouse.
            if pr {
              MouseButton::RightButton.press()
            } else {
              MouseButton::RightButton.release()
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
    .add_button("b0".to_string(), None)?
    .add_button("b1".to_string(), None)?
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


