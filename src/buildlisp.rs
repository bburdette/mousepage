// use touchpage::controls::Orientation::{Horizontal, Vertical};
// use touchpage::controls::Orientation;
use failure::Error as FError;
use inputbot as I;
use serde::{Deserialize, Serialize};
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

pub fn convert_keybdkey(k: &KeybdKey) -> I::KeybdKey {
  match k {
    KeybdKey::BackspaceKey => I::KeybdKey::BackspaceKey,
    KeybdKey::TabKey => I::KeybdKey::TabKey,
    KeybdKey::EnterKey => I::KeybdKey::EnterKey,
    KeybdKey::EscapeKey => I::KeybdKey::EscapeKey,
    KeybdKey::SpaceKey => I::KeybdKey::SpaceKey,
    KeybdKey::HomeKey => I::KeybdKey::HomeKey,
    KeybdKey::LeftKey => I::KeybdKey::LeftKey,
    KeybdKey::UpKey => I::KeybdKey::UpKey,
    KeybdKey::RightKey => I::KeybdKey::RightKey,
    KeybdKey::DownKey => I::KeybdKey::DownKey,
    KeybdKey::InsertKey => I::KeybdKey::InsertKey,
    KeybdKey::DeleteKey => I::KeybdKey::DeleteKey,
    KeybdKey::Numrow0Key => I::KeybdKey::Numrow0Key,
    KeybdKey::Numrow1Key => I::KeybdKey::Numrow1Key,
    KeybdKey::Numrow2Key => I::KeybdKey::Numrow2Key,
    KeybdKey::Numrow3Key => I::KeybdKey::Numrow3Key,
    KeybdKey::Numrow4Key => I::KeybdKey::Numrow4Key,
    KeybdKey::Numrow5Key => I::KeybdKey::Numrow5Key,
    KeybdKey::Numrow6Key => I::KeybdKey::Numrow6Key,
    KeybdKey::Numrow7Key => I::KeybdKey::Numrow7Key,
    KeybdKey::Numrow8Key => I::KeybdKey::Numrow8Key,
    KeybdKey::Numrow9Key => I::KeybdKey::Numrow9Key,
    KeybdKey::AKey => I::KeybdKey::AKey,
    KeybdKey::BKey => I::KeybdKey::BKey,
    KeybdKey::CKey => I::KeybdKey::CKey,
    KeybdKey::DKey => I::KeybdKey::DKey,
    KeybdKey::EKey => I::KeybdKey::EKey,
    KeybdKey::FKey => I::KeybdKey::FKey,
    KeybdKey::GKey => I::KeybdKey::GKey,
    KeybdKey::HKey => I::KeybdKey::HKey,
    KeybdKey::IKey => I::KeybdKey::IKey,
    KeybdKey::JKey => I::KeybdKey::JKey,
    KeybdKey::KKey => I::KeybdKey::KKey,
    KeybdKey::LKey => I::KeybdKey::LKey,
    KeybdKey::MKey => I::KeybdKey::MKey,
    KeybdKey::NKey => I::KeybdKey::NKey,
    KeybdKey::OKey => I::KeybdKey::OKey,
    KeybdKey::PKey => I::KeybdKey::PKey,
    KeybdKey::QKey => I::KeybdKey::QKey,
    KeybdKey::RKey => I::KeybdKey::RKey,
    KeybdKey::SKey => I::KeybdKey::SKey,
    KeybdKey::TKey => I::KeybdKey::TKey,
    KeybdKey::UKey => I::KeybdKey::UKey,
    KeybdKey::VKey => I::KeybdKey::VKey,
    KeybdKey::WKey => I::KeybdKey::WKey,
    KeybdKey::XKey => I::KeybdKey::XKey,
    KeybdKey::YKey => I::KeybdKey::YKey,
    KeybdKey::ZKey => I::KeybdKey::ZKey,
    KeybdKey::Numpad0Key => I::KeybdKey::Numpad0Key,
    KeybdKey::Numpad1Key => I::KeybdKey::Numpad1Key,
    KeybdKey::Numpad2Key => I::KeybdKey::Numpad2Key,
    KeybdKey::Numpad3Key => I::KeybdKey::Numpad3Key,
    KeybdKey::Numpad4Key => I::KeybdKey::Numpad4Key,
    KeybdKey::Numpad5Key => I::KeybdKey::Numpad5Key,
    KeybdKey::Numpad6Key => I::KeybdKey::Numpad6Key,
    KeybdKey::Numpad7Key => I::KeybdKey::Numpad7Key,
    KeybdKey::Numpad8Key => I::KeybdKey::Numpad8Key,
    KeybdKey::Numpad9Key => I::KeybdKey::Numpad9Key,
    KeybdKey::F1Key => I::KeybdKey::F1Key,
    KeybdKey::F2Key => I::KeybdKey::F2Key,
    KeybdKey::F3Key => I::KeybdKey::F3Key,
    KeybdKey::F4Key => I::KeybdKey::F4Key,
    KeybdKey::F5Key => I::KeybdKey::F5Key,
    KeybdKey::F6Key => I::KeybdKey::F6Key,
    KeybdKey::F7Key => I::KeybdKey::F7Key,
    KeybdKey::F8Key => I::KeybdKey::F8Key,
    KeybdKey::F9Key => I::KeybdKey::F9Key,
    KeybdKey::F10Key => I::KeybdKey::F10Key,
    KeybdKey::F11Key => I::KeybdKey::F11Key,
    KeybdKey::F12Key => I::KeybdKey::F12Key,
    KeybdKey::NumLockKey => I::KeybdKey::NumLockKey,
    KeybdKey::ScrollLockKey => I::KeybdKey::ScrollLockKey,
    KeybdKey::CapsLockKey => I::KeybdKey::CapsLockKey,
    KeybdKey::LShiftKey => I::KeybdKey::LShiftKey,
    KeybdKey::RShiftKey => I::KeybdKey::RShiftKey,
    KeybdKey::LControlKey => I::KeybdKey::LControlKey,
    KeybdKey::RControlKey => I::KeybdKey::RControlKey,
    KeybdKey::OtherKey(v) => I::KeybdKey::OtherKey(*v),
  }
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

pub fn convert_mousebutton(mb: &MouseButton) -> I::MouseButton {
  match mb {
    MouseButton::LeftButton => I::MouseButton::LeftButton,
    MouseButton::MiddleButton => I::MouseButton::MiddleButton,
    MouseButton::RightButton => I::MouseButton::RightButton,
    MouseButton::X1Button => I::MouseButton::X1Button,
    MouseButton::X2Button => I::MouseButton::X2Button,
    MouseButton::OtherButton(v) => I::MouseButton::OtherButton(*v),
  }
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

fn get_proportion(c: &Control) -> Option<f32> {
  match c {
    Control::MouseButton {
      label: _,
      button: _,
      proportion,
    } => *proportion,
    Control::MouseXy { label: _, proportion } => *proportion,
    Control::ScrollButton { label: _, proportion } => *proportion,
    Control::Key {
      label: _,
      keys: _,
      proportion,
    } => *proportion,
    Control::Label { label: _, proportion } => *proportion,
    Control::Sizer {
      orientation: _,
      controls: _,
      proportion,
    } => *proportion,
  }
}
// mousepage UI
pub fn build_gui(gui: Gui) -> Result<G::Gui, FError> {
  let mut mpgui = G::Gui::new_gui(gui.title);
  add_control(&mut mpgui, &gui.control)?;
  Ok(mpgui)
}

pub fn add_control<'a>(gui: &'a mut G::Gui, control: &Control) -> Result<&'a mut G::Gui, FError> {
  match control {
    Control::MouseButton {
      label,
      button,
      proportion: _,
    } => gui.add_button(serde_lexpr::to_string(button)?, label.as_ref().cloned()),
    Control::MouseXy { label, proportion: _ } => gui.add_xy("xy".to_string(), label.as_ref().cloned()),
    Control::ScrollButton { label, proportion: _ } => {
      gui.add_button("S".to_string(), label.as_ref().cloned())
    }
    Control::Key {
      label,
      keys,
        proportion: _,
    } => gui.add_button(serde_lexpr::to_string(keys)?, label.as_ref().cloned()),
    Control::Label { label, proportion: _ } => gui.add_label("".to_string(), label.clone()),
    Control::Sizer {
      orientation,
      controls,
      proportion: _,
    } => {
      // make a proportion array.
      let defaultprop: f32 = if controls.len() > 0 {
        (1.0 / f32::from(controls.len() as u16))
      } else {
        0.0
      };
      let mut props = Vec::new();
      for c in controls {
        props.push(get_proportion(c).unwrap_or(defaultprop));
      }
      let mut g = gui.add_sizer(convert_orientation(orientation), Some(props))?;
      for c in controls {
        g = add_control(g, c)?;
      }
      g.end_sizer()
    }
  }
}
