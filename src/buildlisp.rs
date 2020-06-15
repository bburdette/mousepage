pub struct LispState
{
  prefs: Prefs,
  gui: Option<Gui>,
}

pub trait LispCmd {
  pub fn Execute(LispState) -> Result<String, LispState>
}

pub struct Prefs {
  prefs: Prefs,
}

pub struct new_gui {
  title: String,
  cmds: Vec<ControlCmd>,
}

pub trait ControlCmd {
  pub fn buildGui() -> Gui;
}

pub struct add_button {
  name: String,
  label: Option<String>,
}
pub struct add_slider {
   name: String,
   label: Option<String>,
   orientation: Orientation,
}
pub struct add_xy {
  name: String,
  label: Option<String>,
}
pub struct add_label {
  name: String,
  label: String,
}
pub struct add_sizer {
  orientation: Orientation,
  proportions: Option<Vec<f32>>,
  cmds: Vec<ControlCmd>,
}
pub struct set_color {
  color: Color,
  hexstring: &str,
}


  // pub fn end_sizer(&mut self) -> Result<&mut Gui, FError> {
  // pub fn to_root(self) -> Result<Root, FError> {
  // pub fn next_id(&self) -> Result<Vec<i32>, FError> {
  // pub fn add_control(&mut self, control: Box<dyn Control>) -> Result<&mut Gui, FError> {











