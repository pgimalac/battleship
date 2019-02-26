// multiplayer connection panel
use crate::view::buttons::Button;
use crate::view::panel::Panel;

pub struct ConnectPanel {
    buttons: Vec<Button>,
    _panel: *mut Option<Box<Panel>>,
}

impl ConnectPanel {
    pub fn new(panel: *mut Option<Box<Panel>>) -> Self {
        ConnectPanel {
            buttons: vec![],
            _panel: panel,
        }
    }
}

impl Panel for ConnectPanel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    fn button_vec(&self) -> &Vec<Button> {
        &self.buttons
    }
}
