// #![no_std]

extern crate alloc;

mod auto_splitter_settings;
mod hollow_knight_memory;
mod settings_gui;
mod splits;

use asr::{future::next_tick, Process};
use asr::time::Duration;
use asr::timer::TimerState;
use settings_gui::SettingsGui;
use hollow_knight_memory::*;
use ugly_widget::store::StoreGui;

asr::async_main!(stable);
// asr::panic_handler!();

const TICKS_PER_GUI: usize = 0x100;

async fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        asr::print_message(&panic_info.to_string());
    }));

    // TODO: Set up some general state and settings.

    asr::print_message("Hello, World!");

    let mut gui = SettingsGui::wait_load_merge_register().await;

    let mut ticks_since_gui = 0;
    let mut splits = gui.get_splits();
    asr::print_message(&format!("splits: {:?}", splits));

    let mut auto_reset = splits::auto_reset_safe(&splits);

    loop {
        let process = wait_attach_hollow_knight(&mut gui).await;
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut scene_store = SceneStore::new();
                let mut load_remover = HitCounter::new();

                next_tick().await;
                let game_manager_finder = GameManagerFinder::wait_attach(&process).await;
                let mut player_data_store = PlayerDataStore::new();

                #[cfg(debug_assertions)]
                asr::print_message(&format!("geo: {:?}", game_manager_finder.get_geo(&process)));

                let gui_splits = gui.get_splits();
                if gui_splits != splits {
                    splits = gui_splits;
                    asr::print_message(&format!("splits: {:?}", splits));
                    auto_reset = splits::auto_reset_safe(&splits);
                }

                let mut i = 0;
                let mut n = splits.len();
                loop {
                    let current_split = &splits[i];
                    let trans_now = scene_store.transition_now(&process, &game_manager_finder);
                    if splits::splits(current_split, &process, &game_manager_finder, trans_now, &mut scene_store, &mut player_data_store) {
                        split_index(&mut i, n);
                        next_tick().await;
                    } else if auto_reset && splits::splits(&splits[0], &process, &game_manager_finder, trans_now, &mut scene_store, &mut player_data_store) {
                        i = 0;
                        load_remover.load_removal(&process, &game_manager_finder, i);
                        split_index(&mut i, n);
                    }

                    if trans_now && scene_store.pair().old == MENU_TITLE {
                        player_data_store.reset();
                    }

                    // detect manual resets
                    if 0 < i && asr::timer::state() == TimerState::NotRunning {
                        i = 0;
                    }

                    load_remover.load_removal(&process, &game_manager_finder, i);

                    ticks_since_gui += 1;
                    if TICKS_PER_GUI <= ticks_since_gui && gui.load_update_store_if_unchanged() {
                        let gui_splits = gui.get_splits();
                        if gui_splits != splits {
                            splits = gui_splits;
                            asr::print_message(&format!("splits: {:?}", splits));
                            auto_reset = splits::auto_reset_safe(&splits);
                            n = splits.len();
                        }
                        ticks_since_gui = 0;
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

fn split_index(i: &mut usize, n: usize) {
    if *i == 0 {
        asr::timer::reset();
        asr::timer::start();
    } else {
        asr::timer::split();
    }
    *i += 1;
    if n <= *i {
        *i = 0;
    }
}

struct LoadRemover {
    look_for_teleporting: bool,
    last_game_state: i32,
    #[cfg(debug_assertions)]
    last_paused: bool,
}

#[allow(unused)]
impl LoadRemover {
    fn new() -> LoadRemover {
        LoadRemover { 
            look_for_teleporting: false,
            last_game_state: GAME_STATE_INACTIVE,
            #[cfg(debug_assertions)]
            last_paused: false,
        }
    }

    fn load_removal(&mut self, process: &Process, game_manager_finder: &GameManagerFinder, _i: usize) -> Option<()> {

        // only remove loads if timer is running
        if asr::timer::state() != TimerState::Running {
            asr::timer::pause_game_time();
            return Some(());
        }

        let maybe_ui_state = game_manager_finder.get_ui_state(process);
        let ui_state = maybe_ui_state.unwrap_or_default();

        let maybe_scene_name =  game_manager_finder.get_scene_name(process);
        let scene_name = maybe_scene_name.clone().unwrap_or_default();
        let maybe_next_scene = game_manager_finder.get_next_scene_name(process);
        fn is_none_or_empty(ms: Option<&str>) -> bool {
            match ms {  None | Some("") => true, Some(_) => false }
        }
        let loading_menu = (scene_name != "Menu_Title" && is_none_or_empty(maybe_next_scene.as_deref()))
            || (scene_name != "Menu_Title" && maybe_next_scene.as_deref() == Some("Menu_Title")
                || (scene_name == "Quit_To_Menu"));

        let maybe_teleporting = game_manager_finder.camera_teleporting(process);
        let teleporting = maybe_teleporting.unwrap_or_default();

        let maybe_game_state = game_manager_finder.get_game_state(process);
        let game_state = maybe_game_state.unwrap_or_default();
        if game_state == GAME_STATE_PLAYING && self.last_game_state == GAME_STATE_MAIN_MENU {
            self.look_for_teleporting = true;
        }
        if self.look_for_teleporting && (teleporting || (game_state != GAME_STATE_PLAYING && game_state != GAME_STATE_ENTERING_LEVEL)) {
            self.look_for_teleporting = false;
        }

        // TODO: look into Current Patch quitout issues. // might have been fixed? cerpin you broke them in a way that made them work, right?
        let maybe_hazard_respawning = game_manager_finder.hazard_respawning(process);
        let hazard_respawning = maybe_hazard_respawning.unwrap_or_default();
        let maybe_accepting_input = game_manager_finder.accepting_input(process);
        let accepting_input = maybe_accepting_input.unwrap_or_default();
        let maybe_hero_transition_state = game_manager_finder.hero_transition_state(process);
        let hero_transition_state = maybe_hero_transition_state.unwrap_or_default();
        let maybe_tile_map_dirty = game_manager_finder.tile_map_dirty(process);
        let tile_map_dirty = maybe_tile_map_dirty.unwrap_or_default();
        let uses_scene_transition_routine = game_manager_finder.uses_scene_transition_routine(process).unwrap_or_default();
        let is_game_time_paused =
            (game_state == GAME_STATE_PLAYING && teleporting && !hazard_respawning)
            || (self.look_for_teleporting)
            || ((game_state == GAME_STATE_PLAYING || game_state == GAME_STATE_ENTERING_LEVEL) && ui_state != UI_STATE_PLAYING)
            || (game_state != GAME_STATE_PLAYING && !accepting_input)
            || (game_state == GAME_STATE_EXITING_LEVEL || game_state == GAME_STATE_LOADING)
            || (hero_transition_state == HERO_TRANSITION_STATE_WAITING_TO_ENTER_LEVEL)
            || (ui_state != UI_STATE_PLAYING
                && (loading_menu || (ui_state != UI_STATE_PAUSED && (!is_none_or_empty(maybe_next_scene.as_deref()) || scene_name == "_test_charms")))
                && maybe_next_scene != Some(scene_name))
            || (tile_map_dirty && !uses_scene_transition_routine);
        if is_game_time_paused {
            asr::timer::pause_game_time();
        } else {
            asr::timer::resume_game_time();
        }

        self.last_game_state = game_state;
        #[cfg(debug_assertions)]
        {
            if is_game_time_paused != self.last_paused {
                asr::print_message(&format!("is_game_time_paused: {}", is_game_time_paused));
            }
            self.last_paused = is_game_time_paused;
        }
        Some(())
    }
}

struct HitCounter {
    hits: u64,
    last_recoiling: bool,
    last_hazard: bool,
    last_dead_or_0: bool,
    last_exiting_level: Option<String>,
    last_index: usize,
}

#[allow(unused)]
impl HitCounter {
    fn new() -> HitCounter {
        asr::timer::set_variable_int("hits", 0);
        HitCounter {
            hits: 0,
            last_recoiling: false,
            last_hazard: false,
            last_dead_or_0: false,
            last_exiting_level: None,
            last_index: 0,
        }
    }

    fn load_removal(&mut self, process: &Process, game_manager_finder: &GameManagerFinder, i: usize) -> Option<()> {

        asr::timer::pause_game_time();

        // detect resets
        if i == 0 && 0 < self.last_index {
            self.hits = 0;
            asr::timer::set_game_time(Duration::seconds(0));
            asr::timer::set_variable_int("hits", 0);
        }
        self.last_index = i;

        // only count hits if timer is running
        if asr::timer::state() != TimerState::Running { return Some(()); }

        // new state
        let maybe_recoiling = game_manager_finder.hero_recoiling(process);
        let maybe_hazard = game_manager_finder.hazard_death(process);
        let maybe_dead = game_manager_finder.hero_dead(process);
        let maybe_health = game_manager_finder.get_health(process);
        let maybe_scene_name = game_manager_finder.get_scene_name(process);
        let maybe_game_state = game_manager_finder.get_game_state(process);

        if let Some(r) = maybe_recoiling {
            if !self.last_recoiling && r {
                self.hits += 1;
                asr::timer::set_game_time(Duration::seconds(self.hits as i64));
                asr::timer::set_variable_int("hits", self.hits);
                asr::print_message(&format!("hit: {}, from recoiling", self.hits));
            }
            self.last_recoiling = r;
        }

        if let Some(h) = maybe_hazard {
            if !self.last_hazard && h {
                self.hits += 1;
                asr::timer::set_game_time(Duration::seconds(self.hits as i64));
                asr::timer::set_variable_int("hits", self.hits);
                asr::print_message(&format!("hit: {}, from hazard", self.hits));
            }
            self.last_hazard = h;
        }


        {
            let d = maybe_dead == Some(true) || (maybe_health == Some(0) && maybe_game_state == Some(GAME_STATE_PLAYING));
            if !self.last_dead_or_0 && d {
                self.hits += 1;
                asr::timer::set_game_time(Duration::seconds(self.hits as i64));
                asr::timer::set_variable_int("hits", self.hits);
                asr::print_message(&format!("hit: {}, from dead", self.hits));
            }
            self.last_dead_or_0 = d;
        }

        // TODO: make a togglable setting for whether dream falling counts as a hit or not
        if let Some(s) = maybe_scene_name {
            if maybe_game_state == Some(GAME_STATE_ENTERING_LEVEL) && self.last_exiting_level.as_deref() == Some(&s) && s.starts_with("Dream_") {
                self.hits += 1;
                asr::timer::set_game_time(Duration::seconds(self.hits as i64));
                asr::timer::set_variable_int("hits", self.hits);
                asr::print_message(&format!("hit: {}, from dream falling", self.hits));
            }
            if maybe_game_state == Some(GAME_STATE_EXITING_LEVEL) {
                if self.last_exiting_level.is_none() {
                    self.last_exiting_level = Some(s);
                }
            } else {
                self.last_exiting_level = None;
            }
        }

        Some(())
    }
}
