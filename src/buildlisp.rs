// use touchpage::controls::Orientation::{Horizontal, Vertical};
// use touchpage::controls::Orientation;
use failure::Error as FError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use touchpage::controls as C;
use touchpage::guibuilder as G;

// pub struct LispState {
//   prefs: Prefs,
//   gui: Option<G::Gui>,
// }

// pub trait LispCmd {
//   fn execute(state: LispState) -> Result<String, LispState>;
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
  pub prefs: Prefs,
  pub gui: Gui,
  pub colors: Option<Vec<SetColor>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Prefs {
  pub xmult: f32,
  pub ymult: f32,
  pub max_tap_duration: u32,
  pub show_press_duration: bool,
  pub scroll_threshold: i32,
  pub html_port: i32,
  pub websocket_port: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Orientation {
  Horizontal,
  Vertical,
}

fn convert_orientation(o: &Orientation) -> C::Orientation {
  match o {
    Orientation::Horizontal => C::Orientation::Horizontal,
    Orientation::Vertical => C::Orientation::Vertical,
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Color {
  Controls,
  Labels,
  Text,
  Pressed,
  Unpressed,
  Background,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeybdKey {
  BackspaceKey,
  TabKey,
  EnterKey,
  EscapeKey,
  SpaceKey,
  HomeKey,
  LeftKey,
  UpKey,
  RightKey,
  DownKey,
  InsertKey,
  DeleteKey,
  Numrow0Key,
  Numrow1Key,
  Numrow2Key,
  Numrow3Key,
  Numrow4Key,
  Numrow5Key,
  Numrow6Key,
  Numrow7Key,
  Numrow8Key,
  Numrow9Key,
  AKey,
  BKey,
  CKey,
  DKey,
  EKey,
  FKey,
  GKey,
  HKey,
  IKey,
  JKey,
  KKey,
  LKey,
  MKey,
  NKey,
  OKey,
  PKey,
  QKey,
  RKey,
  SKey,
  TKey,
  UKey,
  VKey,
  WKey,
  XKey,
  YKey,
  ZKey,
  Numpad0Key,
  Numpad1Key,
  Numpad2Key,
  Numpad3Key,
  Numpad4Key,
  Numpad5Key,
  Numpad6Key,
  Numpad7Key,
  Numpad8Key,
  Numpad9Key,
  F1Key,
  F2Key,
  F3Key,
  F4Key,
  F5Key,
  F6Key,
  F7Key,
  F8Key,
  F9Key,
  F10Key,
  F11Key,
  F12Key,
  NumLockKey,
  ScrollLockKey,
  CapsLockKey,
  LShiftKey,
  RShiftKey,
  LControlKey,
  RControlKey,
  OtherKey(u64),
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MouseButton {
  LeftButton,
  MiddleButton,
  RightButton,
  X1Button,
  X2Button,
  OtherButton(u32),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Gui {
  pub title: String,
  pub control: Control,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetColor {
  color: Color,
  hexstring: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Control {
  MouseButton {
    label: Option<String>,
    button: MouseButton,
    proportion: Option<f32>,
  },
  MouseXy {
    label: Option<String>,
    proportion: Option<f32>,
  },
  ScrollButton {
    label: Option<String>,
    proportion: Option<f32>,
  },
  Key {
    label: Option<String>,
    keys: Vec<KeybdKey>,
    proportion: Option<f32>,
  },
  Label {
    label: String,
    proportion: Option<f32>,
  },
  Sizer {
    orientation: Orientation,
    controls: Vec<Control>,
    proportion: Option<f32>,
  },
}

// mousepage UI
fn build_gui(gui: Gui) -> Result<G::Gui, FError> {
  let mut mpgui = G::Gui::new_gui(gui.title);
  add_control(&mut mpgui, &gui.control)?;
  Ok(mpgui)
}

pub fn add_control<'a>(gui: &'a mut G::Gui, control: &Control) -> Result<&'a mut G::Gui, FError> {
  match control {
    Control::MouseButton {
      label,
      button,
      proportion,
    } => gui.add_button(serde_lexpr::to_string(button)?, label.as_ref().cloned()),
    Control::MouseXy { label, proportion } => gui.add_xy("xy".to_string(), label.as_ref().cloned()),
    Control::ScrollButton { label, proportion } => {
      gui.add_button("S".to_string(), label.as_ref().cloned())
    }
    Control::Key {
      label,
      keys,
      proportion,
    } => gui.add_button(serde_lexpr::to_string(keys)?, label.as_ref().cloned()),
    Control::Label { label, proportion } => gui.add_label("".to_string(), label.clone()),
    Control::Sizer {
      orientation,
      controls,
      proportion,
    } => {
      let mut g = gui.add_sizer(convert_orientation(orientation), None)?;
      for c in controls {
        g = add_control(g, c)?;
      }
      // also get the proportion array.
      g.end_sizer()
    }
  }
}

/*#[derive(Deserialize, Serialize, Debug)]
pub enum ControlCmd {
  AddButton {
    name: String,
    label: Option<String>,
  },
  AddSlider {
    name: String,
    label: Option<String>,
    orientation: Orientation,
  },
  AddXy {
    name: String,
    label: Option<String>,
  },
  AddLabel {
    name: String,
    label: String,
  },
  AddSizer {
    orientation: Orientation,
    // proportions: Option<Vec<f32>>,
    cmds: Vec<ControlCmd>,
  },
}

*/// pub fn end_sizer(&mut self) -> Result<&mut Gui, FError> {
// pub fn to_root(self) -> Result<Root, FError> {
// pub fn next_id(&self) -> Result<Vec<i32>, FError> {
// pub fn add_control(&mut self, control: Box<dyn Control>) -> Result<&mut Gui, FError> {
