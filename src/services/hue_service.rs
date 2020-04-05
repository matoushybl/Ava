use std::net::IpAddr;

use huelib::scene::LightStateModifier;
use huelib::Alert::Select;
use huelib::{light::StateModifier, Group, Light, Modifier as HueModifier, Scene};

pub struct HueService {
    bridge: huelib::bridge::Bridge,
}

impl HueService {
    pub fn new(address: IpAddr, user: &str) -> Self {
        HueService {
            bridge: huelib::bridge::Bridge::new(address, user),
        }
    }

    pub fn get_all_groups(&self) -> Vec<Group> {
        let mut groups = self
            .bridge
            .get_all_groups()
            .expect("Failed to access the bridge. #get_all_groups");
        groups.sort_by(|a, b| b.name.cmp(&a.name));
        return groups;
    }

    pub fn get_all_scenes(&self) -> Vec<Scene> {
        let mut scenes = self
            .bridge
            .get_all_scenes()
            .expect("Failed to access the bridge. #get_all_scenes");
        scenes.sort_by(|a, b| b.name.cmp(&a.name));
        return scenes;
    }

    pub fn get_all_lights(&self) -> Vec<Light> {
        let mut lights = self
            .bridge
            .get_all_lights()
            .expect("Failed to access the bridge. #get_all_lights");
        lights.sort_by(|a, b| b.name.cmp(&a.name));
        return lights;
    }

    pub fn toggle_group(&self, group: &Group) {
        let modifier =
            huelib::group::StateModifier::new().on(!group.state.map(|s| s.any_on).unwrap_or(false));
        self.bridge.set_group_state(group.id.as_str(), &modifier);
    }

    pub fn set_scene_to_group(&self, group: &Group, scene: &Scene) {
        let modifier = huelib::group::StateModifier::new()
            .scene(scene.id.as_str())
            .on(true);
        self.bridge
            .set_group_state(group.id.as_str(), &modifier)
            .unwrap();
    }

    // FIXME add method for registration of user
}
