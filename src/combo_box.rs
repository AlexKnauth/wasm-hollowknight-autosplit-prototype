use alloc::collections::BTreeMap;

use asr::settings::gui::{add_bool, add_title, Gui, Title, Widget, set_tooltip};

fn single_from_bool_map<'a>(bool_map: &BTreeMap<&'a str, bool>) -> Option<&'a str> {
    let trues: Vec<&str> = bool_map.into_iter().filter_map(|(&k, &v)| {
        if v { Some(k) } else { None }
    }).collect();
    match &trues[..] {
        [t] => Some(t),
        _ => None,
    }
}

#[derive(Clone)]
struct RadioButtonOption<'a, T> {
    value: T,
    key: &'a str,
    description: &'a str,
    tooltip: Option<&'a str>,
}

impl<T> RadioButtonOption<'_, T> {
    fn bool_key(&self, key: &str) -> String {
        format!("{}_{}", key, self.key)
    }
}

#[derive(Default)]
#[non_exhaustive]
pub struct RadioButtonArgs<'a> {
    heading_level: u32,
    default: &'a str,
}

impl RadioButtonArgs<'_> {
    fn default_value<T: RadioButtonOptions>(&self) -> T {
        options_value::<T>(self.default)
    }
}

trait RadioButtonOptions: Default + PartialEq {
    fn radio_button_options() -> Vec<RadioButtonOption<'static, Self>>;
}

fn options_str<T: RadioButtonOptions>(v: T) -> &'static str {
    T::radio_button_options().into_iter().find_map(|o| {
        if o.value == v {
            Some(o.key)
        } else {
            None
        }
    }).unwrap_or_default()
}

fn options_value<T: RadioButtonOptions>(s: &str) -> T {
    T::radio_button_options().into_iter().find_map(|o| {
        if o.key == s {
            Some(o.value)
        } else {
            None
        }
    }).unwrap_or_default()
}

struct RadioButton<T>(T);

impl<T: RadioButtonOptions> Widget for RadioButton<T> {
    type Args = RadioButtonArgs<'static>;

    fn register(key: &str, description: &str, args: Self::Args) -> Self {
        add_title(key, description, args.heading_level);
        let default = args.default_value::<T>();
        let default_s = options_str(default);
        let bool_map: BTreeMap<&str, bool> = T::radio_button_options().into_iter().map(|o| {
            let bool_key = o.bool_key(key);
            let b = add_bool(&bool_key, &o.description, o.key == default_s);
            if let Some(t) = o.tooltip {
                set_tooltip(&bool_key, &t);
            }
            (o.key, b)
        }).collect();
        RadioButton(options_value::<T>(single_from_bool_map(&bool_map).unwrap_or(&default_s)))
    }

    fn update_from(&mut self, settings_map: &asr::settings::Map, key: &str, args: Self::Args) {
        let default = args.default_value::<T>();
        let default_s = options_str(default);
        let old = settings_map.get(key).and_then(|v| v.get_string()).unwrap_or(default_s.to_string());
        let new_bools: Vec<(&str, bool)> = T::radio_button_options().iter().filter_map(|o| {
            let bool_key = o.bool_key(key);
            let old_b = old == o.key;
            let map_b = settings_map.get(&bool_key).and_then(|v| v.get_bool()).unwrap_or_default();
            if map_b != old_b {
                Some((o.key, map_b))
            } else {
                None
            }
        }).collect();
        let new = match &new_bools[..] {
            [(v, true)] => *v,
            [(_, false)] => default_s,
            _ => old.as_str(),
        };
        if new != old.as_str() {
            asr::print_message(&new);
        }
        settings_map.insert(key, &new.into());
        for o in T::radio_button_options() {
            let bool_key = o.bool_key(key);
            let new_b = new == o.key;
            settings_map.insert(&bool_key, &new_b.into());
        }
        self.0 = options_value::<T>(new);
        settings_map.store();
    }
}

// #[derive(Gui)]
#[derive(Clone, Default, PartialEq)]
pub enum ListItemAction {
    // None
    #[default]
    None,
    // Remove
    Remove,
    // Move before
    MoveBefore,
    // Move after
    MoveAfter,
    // Insert before
    InsertBefore,
    // Insert after
    InsertAfter,
}

impl RadioButtonOptions for ListItemAction {
    fn radio_button_options() -> Vec<RadioButtonOption<'static, Self>> {
        vec![
            RadioButtonOption { value: ListItemAction::None, key: "None", description: "None", tooltip: None },
            RadioButtonOption { value: ListItemAction::Remove, key: "Remove", description: "Remove", tooltip: None },
            RadioButtonOption { value: ListItemAction::MoveBefore, key: "MoveBefore", description: "Move before", tooltip: None },
            RadioButtonOption { value: ListItemAction::MoveAfter, key: "MoveAfter", description: "Move after", tooltip: None },
            RadioButtonOption { value: ListItemAction::InsertBefore, key: "InsertBefore", description: "Insert before", tooltip: None },
            RadioButtonOption { value: ListItemAction::InsertAfter, key: "InsertAfter", description: "Insert after", tooltip: None },
        ]
    }
}

#[derive(Gui)]
pub struct ListItemActionGui {
    /// General Settings
    _general_settings: Title,
    /// Choose an Action
    /// 
    /// This is a tooltip.
    #[heading_level = 1]
    lia: RadioButton<ListItemAction>,
}
