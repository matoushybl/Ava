use crate::utils::stateful_list::StatefulList;
use huelib::{Group, Scene};
use tui::widgets::Text;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum SelectedList {
    Lights,
    Scenes,
}

pub struct AppState {
    pub groups: StatefulList<Group>,
    pub scenes: StatefulList<Scene>,
    pub selected_list: SelectedList,
}

impl AppState {
    pub fn new(lights: Vec<Group>, scenes: Vec<Scene>) -> AppState {
        let mut state = AppState {
            groups: StatefulList::with_items(lights),
            scenes: StatefulList::with_items(scenes),
            selected_list: SelectedList::Lights,
        };

        state.groups.state.select(Some(0usize));
        state.scenes.state.select(Some(0usize));

        return state;
    }
}
