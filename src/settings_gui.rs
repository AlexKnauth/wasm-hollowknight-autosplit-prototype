use asr::{settings::gui::{FileSelect, Gui, Title, Widget}, watcher::Pair};

use ugly_widget::{
    ugly_list::{UglyList, UglyListArgs},
    store::{StoreWidget, StoreGui},
    args::SetHeadingLevel,
};

use crate::{splits::{Split, self}, auto_splitter_settings::{Settings, SettingsObject, XMLSettings}};

#[derive(Gui)]
pub struct SettingsGui {
    /// Import Splits
    #[filter((_, "*.lss *.lsl"))]
    import: Pair<FileSelect>,
    /// General Settings
    _general_settings: Title,
    /// Splits
    #[heading_level = 1]
    splits: UglyList<Split>,
}

impl StoreGui for SettingsGui {
    fn post_update(&mut self) {
        if self.import.changed() {
            asr::print_message(&format!("import {}", self.import.current.path));
            if let Some(xml_settings) = XMLSettings::from_file(&self.import.current.path, &[("Splits", "Split")]) {
                let new_splits = splits_from_settings(&xml_settings);
                // new empty map, which will only include the new splits
                let settings_map = asr::settings::Map::new();
                let l = asr::settings::List::new();
                for split in new_splits.iter() {
                    l.push(Split::to_string(split).as_str());
                }
                settings_map.insert("splits", l);
                let mut splits_args = UglyListArgs::default();
                splits_args.set_heading_level(1);
                self.splits.update_from(&settings_map, "splits", splits_args);
                let new_splits2 = self.splits.get_list();
                if new_splits.iter().collect::<Vec<&Split>>() != new_splits2 {
                    asr::print_message("BAD");
                } else {
                    asr::print_message(&format!("splits: {:?}", new_splits));
                }
            }
        }
    }

    fn insert_into(&self, settings_map: &asr::settings::Map) -> bool {
        self.splits.insert_into(settings_map, "splits")
    }
}


impl SettingsGui {
    pub fn get_splits(&self) -> Vec<Split> {
        self.splits.get_list().into_iter().map(|rb| rb.clone()).collect()
    }

    pub async fn wait_load_merge_register() -> SettingsGui {
        let settings1 = asr::settings::Map::load();
        let auto_splitter_settings = include_str!("AutoSplitterSettings.txt");
        let settings2 = XMLSettings::from_xml_string(auto_splitter_settings, &[("Splits", "Split")]).unwrap_or_default();
        let splits2 = splits_from_settings(&settings2);
        if settings1.get("splits").is_some_and(|v| v.get_list().is_some_and(|l| !l.is_empty())) {
            asr::print_message("settings1: from asr::settings::Map::load");
        } else {
            asr::print_message("settings2: from AutoSplitterSettings.txt");
            let settings3 = asr::settings::Map::new();
            let l = asr::settings::List::new();
            for split in splits2.iter() {
                l.push(split.to_string().as_str());
            }
            settings3.insert("splits", &l);
            SettingsObject::wait_load_merge_store(&SettingsObject::Map(settings3)).await;
        }
        let mut gui = SettingsGui::register();
        gui.loop_load_update_store();
        let splits1 = gui.get_splits();
        if splits2 != splits1 {
            asr::print_message("WARNING: splits from asr::settings::Map::load differ from AutoSplitterSettings.txt");
                asr::print_message("assuming AutoSplitterSettings.txt is out of date, using asr::settings::Map::load");
        }
        gui
    }
}


pub fn splits_from_settings<S: Settings>(s: &S) -> Vec<Split> {
    let maybe_ordered = s.dict_get("Ordered");
    let maybe_start = s.dict_get("AutosplitStartRuns");
    let maybe_end = s.dict_get("AutosplitEndRuns");
    let maybe_splits = s.dict_get("Splits");
    if maybe_ordered.is_some() || maybe_start.is_some() || maybe_end.is_some() {
        // Splits files from up through version 3 of ShootMe/LiveSplit.HollowKnight
        let start = maybe_start.and_then(Split::from_settings_str).unwrap_or(Split::StartNewGame);
        let end = maybe_end.and_then(|s| s.as_bool()).unwrap_or_default();
        let mut result = vec![start];
        if let Some(splits) = maybe_splits {
            result.append(&mut splits_from_settings_split_list(&splits));
        }
        if !end {
            result.push(Split::EndingSplit);
        }
        result
    } else if let Some(splits) = maybe_splits {
        // Splits files from after version 4 of mayonnaisical/LiveSplit.HollowKnight
        splits_from_settings_split_list(&splits)
    } else {
        splits::default_splits()
    }
}

fn splits_from_settings_split_list<S: Settings>(s: &S) -> Vec<Split> {
    s.as_list().unwrap_or_default().into_iter().filter_map(Split::from_settings_split).collect()
}
