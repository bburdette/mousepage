// use touchpage::controls::Orientation::{Horizontal, Vertical};
// use touchpage::controls::Orientation;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use touchpage::guibuilder as G;

pub struct LispState {
  prefs: Prefs,
  gui: Option<G::Gui>,
}

pub trait LispCmd {
  fn execute(state: LispState) -> Result<String, LispState>;
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

pub struct SetPrefs {
  prefs: Prefs,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Orientation {
  Horizontal,
  Vertical,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct NewGui {
  pub title: String,
  pub cmd: ControlCmd,
}

// SetColor {
//   color: Color,
//   hexstring: String,
// },
#[derive(Deserialize, Serialize, Debug)]
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

// pub fn end_sizer(&mut self) -> Result<&mut Gui, FError> {
// pub fn to_root(self) -> Result<Root, FError> {
// pub fn next_id(&self) -> Result<Vec<i32>, FError> {
// pub fn add_control(&mut self, control: Box<dyn Control>) -> Result<&mut Gui, FError> {
