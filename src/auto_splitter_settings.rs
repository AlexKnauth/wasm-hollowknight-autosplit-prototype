use alloc::vec::Vec;
use asr::future::retry;
use xmltree::{Element, XMLNode};

pub trait Settings: Sized {
    fn as_string(&self) -> Option<String>;
    fn as_bool(&self) -> Option<bool>;
    fn as_list(&self) -> Option<Vec<Self>>;
    fn dict_get(&self, key: &str) -> Option<Self>;
}

pub enum SettingsObject {
    Map(asr::settings::Map),
    Value(asr::settings::Value),
}

impl SettingsObject {
    pub fn load() -> Self {
        SettingsObject::Map(asr::settings::Map::load())
    }
    fn as_value(&self) -> Option<&asr::settings::Value> {
        match self {
            SettingsObject::Value(v) => Some(v),
            _ => None,
        }
    }
    fn as_asr_list(&self) -> Option<asr::settings::List> {
        self.as_value()?.get_list()
    }
    fn as_map(&self) -> Option<asr::settings::Map> {
        Some(match self {
            SettingsObject::Map(m) => m.clone(),
            SettingsObject::Value(v) => v.get_map()?,
        })
    }
    pub fn load_merge_store<S: Settings>(new: &S, keys: &[&str], elems: &[&str]) -> Option<SettingsObject> {
        let old = asr::settings::Map::load();
        let merged = maybe_asr_settings_map_merge(Some(old.clone()), new, keys, elems);
        if merged.store_if_unchanged(&old) {
            Some(SettingsObject::Map(merged))
        } else {
            None
        }
    }
    pub async fn wait_load_merge_store<S: Settings>(new: &S, keys: &[&str], elems: &[&str]) -> SettingsObject {
        retry(|| SettingsObject::load_merge_store(new, keys, elems)).await
    }
}

impl Settings for SettingsObject {
    fn as_string(&self) -> Option<String> {
        self.as_value()?.get_string()
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_value()?.get_bool()
    }

    fn as_list(&self) -> Option<Vec<Self>> {
        Some(self.as_asr_list()?.iter().map(SettingsObject::Value).collect())
    }

    fn dict_get(&self, key: &str) -> Option<Self> {
        Some(SettingsObject::Value(self.as_map()?.get(key)?))
    }
}

#[derive(Clone, Debug)]
pub struct XMLSettings {
    name: Option<String>,
    children: Vec<XMLNode>,
    list_items: Vec<(String, String)>,
}

impl Default for XMLSettings {
    fn default() -> Self { XMLSettings { name: None, children: vec![], list_items: Vec::new() } }
}

impl XMLSettings {
    pub fn from_xml_string(s: &str, list_items: &[(&str, &str)]) -> Result<Self, xmltree::ParseError> {
        let list_items = list_items.into_iter().map(|(l, i)| (l.to_string(), i.to_string())).collect();
        Ok(XMLSettings { name: None, children: Element::parse_all(s.as_bytes())?, list_items })
    }
    fn is_list_get_item_name(&self) -> Option<&str> {
        let n = self.name.as_deref()?;
        for (l, i) in self.list_items.iter() {
            if n == l {
                return Some(i);
            }
        }
        None
    }
}

impl Settings for XMLSettings {
    fn as_string(&self) -> Option<String> {
        match &self.children[..] {
            [] => Some("".to_string()),
            [XMLNode::Text(s)] => Some(s.to_string()),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self.as_string()?.trim() {
            "True" => Some(true),
            "False" => Some(false),
            _ => None,
        }
    }

    fn as_list(&self) -> Option<Vec<Self>> {
        let i = self.is_list_get_item_name()?;
        Some(self.children.iter().filter_map(|c| {
            match c.as_element() {
                Some(e) if e.name == i => {
                    Some(XMLSettings {
                        name: Some(e.name.clone()),
                        children: e.children.clone(),
                        list_items: self.list_items.clone(),
                    })
                },
                _ => None,
            }
        }).collect())
    }

    fn dict_get(&self, key: &str) -> Option<Self> {
        for c in self.children.iter() {
            match c.as_element() {
                Some(e) if e.name == key => {
                    return Some(XMLSettings {
                        name: Some(e.name.clone()),
                        children: e.children.clone(),
                        list_items: self.list_items.clone(),
                    });
                },
                _ => (),
            }
        }
        None
    }
}

// --------------------------------------------------------

fn maybe_asr_settings_map_merge<S: Settings>(old: Option<asr::settings::Map>, new: &S, keys: &[&str], elems: &[&str]) -> asr::settings::Map {
    let om = if let Some(om) = old { om } else { asr::settings::Map::new() };
    for key in keys {
        if let Some(new_v) = new.dict_get(key) {
            om.insert(key, &maybe_asr_settings_value_merge(om.get(key), &new_v, keys, elems));
        }
    }
    om
}

fn maybe_asr_settings_value_merge<S: Settings>(old: Option<asr::settings::Value>, new: &S, keys: &[&str], elems: &[&str]) -> asr::settings::Value {
    if let Some(b) = new.as_bool() {
        asr::settings::Value::from(b)
    } else if let Some(s) = new.as_string() {
        asr::settings::Value::from(s.as_str())
    } else {
        match new.as_list() {
            Some(l) if new.dict_get("0").is_some()
                    || elems.iter().any(|e| new.dict_get(e).is_some())
                    || keys.iter().all(|k| new.dict_get(k).is_none()) => {
                let l2 = l.into_iter().enumerate().map(|(i, v)| {
                    if let Some(v2) = v.dict_get(&i.to_string()) {
                        v2
                    } else if let Some (v2) = elems.iter().find_map(|e| v.dict_get(e)) {
                        v2
                    } else {
                        v
                    }
                }).collect();
                asr::settings::Value::from(&maybe_asr_settings_list_merge(old.and_then(|o| o.get_list()), l2, keys, elems))
            },
            _ => {
                asr::settings::Value::from(&maybe_asr_settings_map_merge(old.and_then(|o| o.get_map()), new, keys, elems))
            },
        }
    }
}

fn is_asr_settings_list_length(l: &asr::settings::List, n: usize) -> bool {
    l.len() == n as u64
}

fn maybe_asr_settings_list_merge<S: Settings>(old: Option<asr::settings::List>, new: Vec<S>, keys: &[&str], elems: &[&str]) -> asr::settings::List {
    let ol = if let Some(ol) = old { ol } else { asr::settings::List::new() };
    let nn = new.len();
    if is_asr_settings_list_length(&ol, nn) {
        // same length, merge elements
        let ml = asr::settings::List::new();
        for (i, ne) in new.into_iter().enumerate() {
            ml.push(&maybe_asr_settings_value_merge(ol.get(i as u64), &ne, keys, elems));
        }
        ml
    } else {
        // different length, replace the whole thing
        let ml = asr::settings::List::new();
        for ne in new.into_iter() {
            ml.push(&maybe_asr_settings_value_merge(None, &ne, keys, elems));
        }
        ml
    }
}
