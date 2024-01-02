use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use asr::future::retry;
use std::{path::Path, fs::File, io::{self, Read}};
use xmltree::{Element, XMLNode};

pub trait Settings: Sized {
    fn as_string(&self) -> Option<String>;
    fn as_bool(&self) -> Option<bool>;
    fn as_list(&self) -> Option<Vec<Self>>;
    fn as_dict(&self) -> Option<BTreeMap<String, Self>>;
    fn dict_get(&self, key: &str) -> Option<Self>;
}

pub enum SettingsObject {
    Map(asr::settings::Map),
    Value(asr::settings::Value),
}

impl SettingsObject {
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
    pub fn load_merge_store<S: Settings>(new: &S) -> Option<SettingsObject> {
        let old = asr::settings::Map::load();
        let merged = maybe_asr_settings_map_merge(Some(old.clone()), new);
        if merged.store_if_unchanged(&old) {
            Some(SettingsObject::Map(merged))
        } else {
            None
        }
    }
    pub async fn wait_load_merge_store<S: Settings>(new: &S) -> SettingsObject {
        retry(|| SettingsObject::load_merge_store(new)).await
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

    fn as_dict(&self) -> Option<BTreeMap<String, Self>> {
        Some(self.as_map()?.iter().map(|(k, v)| (k, SettingsObject::Value(v))).collect())
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
    pub fn from_file<P: AsRef<Path>>(path: P, list_items: &[(&str, &str)]) -> Option<Self> {
        let list_items = list_items.into_iter().map(|(l, i)| (l.to_string(), i.to_string())).collect();
        let bs = file_read_all_bytes(path).ok()?;
        let es = Element::parse_all(bs.as_slice()).ok()?;
        let auto_splitter_settings = es.iter().find_map(xml_find_auto_splitter_settings)?;
        Some(XMLSettings { name: None, children: auto_splitter_settings, list_items })
    }
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

    fn as_dict(&self) -> Option<BTreeMap<String, Self>> {
        Some(self.children.iter().filter_map(|c| -> Option<(String, Self)> {
            let e = c.as_element()?;
            Some((e.name.clone(), XMLSettings {
                name: Some(e.name.clone()),
                children: e.children.clone(),
                list_items: self.list_items.clone(),
            }))
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

fn xml_find_auto_splitter_settings(xml: &XMLNode) -> Option<Vec<XMLNode>> {
    let e = xml.as_element()?;
    match e.name.as_str() {
        "AutoSplitterSettings" => Some(e.children.clone()),
        "Run" => Some(e.get_child("AutoSplitterSettings")?.children.clone()),
        "Layout" => e.get_child("Components")?.children.iter().find_map(xml_find_auto_splitter_settings),
        "Component" if component_is_asr(e) => Some(e.get_child("Settings")?.children.clone()),
        _ => None,
    }
}

fn component_is_asr(e: &Element) -> bool {
    let Some(p) = e.get_child("Path") else { return false; };
    let [c] = &p.children[..] else { return false; };
    let Some(s) = c.as_text() else { return false; };
    s.contains("LiveSplit.AutoSplittingRuntime")
}

// --------------------------------------------------------

fn maybe_asr_settings_map_merge<S: Settings>(old: Option<asr::settings::Map>, new: &S) -> asr::settings::Map {
    let om = if let Some(om) = old { om } else { asr::settings::Map::new() };
    let mut keys: BTreeSet<String> = om.keys().collect();
    if let Some(nm) = new.as_dict() {
        keys.extend(nm.into_keys());
    }
    for key in keys {
        if let Some(new_v) = new.dict_get(&key) {
            om.insert(&key, &maybe_asr_settings_value_merge(om.get(&key), &new_v));
        }
    }
    om
}

fn maybe_asr_settings_value_merge<S: Settings>(old: Option<asr::settings::Value>, new: &S) -> asr::settings::Value {
    if let Some(b) = new.as_bool() {
        asr::settings::Value::from(b)
    } else if let Some(s) = new.as_string() {
        asr::settings::Value::from(s.as_str())
    } else if let Some(l) = new.as_list() {
        asr::settings::Value::from(&maybe_asr_settings_list_merge(old.and_then(|o| o.get_list()), l))
    } else {
        asr::settings::Value::from(&maybe_asr_settings_map_merge(old.and_then(|o| o.get_map()), new))
    }
}

fn is_asr_settings_list_length(l: &asr::settings::List, n: usize) -> bool {
    l.len() == n as u64
}

fn maybe_asr_settings_list_merge<S: Settings>(old: Option<asr::settings::List>, new: Vec<S>) -> asr::settings::List {
    let ol = if let Some(ol) = old { ol } else { asr::settings::List::new() };
    let nn = new.len();
    if is_asr_settings_list_length(&ol, nn) {
        // same length, merge elements
        let ml = asr::settings::List::new();
        for (i, ne) in new.into_iter().enumerate() {
            ml.push(&maybe_asr_settings_value_merge(ol.get(i as u64), &ne));
        }
        ml
    } else {
        // different length, replace the whole thing
        let ml = asr::settings::List::new();
        for ne in new.into_iter() {
            ml.push(&maybe_asr_settings_value_merge(None, &ne));
        }
        ml
    }
}

// --------------------------------------------------------

fn file_read_all_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}
