
use asr::settings::gui::{Gui, Widget};

pub trait StoreGui: Gui {
    fn insert_into(&self, settings_map: &asr::settings::Map) -> bool;

    fn post_update(&mut self) {}

    fn load_update_store_if_unchanged(&mut self) -> bool {
        let settings_map = asr::settings::Map::load();
        let old = settings_map.clone();
        self.update_from(&settings_map);
        self.post_update();
        if self.insert_into(&settings_map) {
            settings_map.store_if_unchanged(&old)
        } else {
            true
        }
    }

    fn loop_load_update_store(&mut self) {
        loop {
            if self.load_update_store_if_unchanged() {
                break;
            }
        }
    }
}

pub trait StoreWidget: Widget {
    fn insert_into(&self, settings_map: &asr::settings::Map, key: &str) -> bool;
}

impl StoreWidget for bool {
    fn insert_into(&self, settings_map: &asr::settings::Map, key: &str) -> bool {
        if settings_map.get(key).is_some_and(|old_v| old_v.get_bool().is_some_and(|old_b| old_b == *self)) {
            return false;
        }
        settings_map.insert(key, *self);
        true
    }
}
