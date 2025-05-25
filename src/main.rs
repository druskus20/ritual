use chrono::Datelike;
use components::{button::Button, icon::Icon};
use db::{Db, State};
use dioxus::{desktop::LogicalSize, prelude::*};
use dioxus_free_icons::icons::io_icons::{IoAddOutline, IoCheckmarkOutline, IoCloseOutline};
use futures_util::StreamExt;
use itertools::Itertools;
use prelude::*;
use types::DateTime;
use utils::{NonEmpty, Validate};
use uuid::Uuid;

mod cli;
mod components;
mod db;
mod types;
mod utils;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub type Result<T> = color_eyre::Result<T>;
    pub use tracing::{error, info, trace, warn};
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const NORMALIZE_CSS: Asset = asset!("/assets/normalize.css");
const REMOVE_DEFAULT_STYLES_CSS: Asset = asset!("/assets/remove_default_styles.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let args = cli::ParsedArgs::parse_raw();

    dioxus::logger::init(args.log_level).expect("failed to init logger");
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::default().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("ritual")
                    .with_inner_size(LogicalSize::new(1000, 720))
                    .with_decorations(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut db_state = use_context_provider(|| Signal::new(State::default()));

    use_coroutine::<RitualCmd, _, _>(move |mut rx| async move {
        let db: Db = Db::open_or_new("/tmp/db.json".into()).expect("Failed to open db");
        // load db contents into db_state
        db_state.with_mut(|state| {
            *state = db.load().unwrap_or_else(|err| {
                error!("Failed to load db: {}", err);
                std::process::exit(1);
            });
        });

        while let Some(msg) = rx.next().await {
            match msg {
                RitualCmd::NewDay => {
                    let date: DateTime = chrono::Utc::now();
                    info!("Adding new day for date: {}", date);
                    db_state.with_mut(|state| {
                        state.add_day(date).unwrap_or_else(|err| {
                            error!("Failed to add new day: {}", err);
                        });
                    });
                }
                RitualCmd::AddHabitToDay { title, day_id } => {
                    info!("Adding habit to day: {}", day_id);
                    let title = NonEmpty::new_validated(title);
                    let title = match title {
                        Ok(title) => title,
                        Err(e) => {
                            error!("Invalid habit title {e}");
                            continue;
                        }
                    };

                    db_state.with_mut(|state| {
                        state.add_habit_to_day(title, day_id).unwrap_or_else(|err| {
                            error!("Failed to add habit to day {}: {}", day_id, err);
                        });
                    });
                }
                RitualCmd::HabitSetDone {
                    day_id,
                    habit_id,
                    done,
                } => {
                    info!(
                        "Setting habit {} for day {} to done: {}",
                        habit_id, day_id, done
                    );
                    db_state.with_mut(|state| {
                        state
                            .set_habit_done(day_id, habit_id, done)
                            .unwrap_or_else(|err| {
                                error!(
                                    "Failed to set habit {} for day {} to done: {}",
                                    habit_id, day_id, err
                                );
                            });
                    });
                }
                RitualCmd::Save => db.save(&db_state.read()).unwrap_or_else(|err| {
                    error!("Failed to save db: {}", err);
                }),
            }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: NORMALIZE_CSS }
        document::Link { rel: "stylesheet", href: REMOVE_DEFAULT_STYLES_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let cmd = use_coroutine_handle::<RitualCmd>();
    let state = use_context::<Signal<State>>();
    rsx! {
        div {
            class: "main",
            h1 { "Ritual" },
            div {
                class: "days",
                for day in state.read().days.values().sorted_by_key(|d| d.date) {
                    Day { day: day.clone() }
                }
                Button {
                    onclick: move |_| {
                        cmd.send(RitualCmd::NewDay);
                },
                    "Add Day"
                }
            }
            Button {
                onclick: move |_| {
                    cmd.send(RitualCmd::Save);
                },
                "Save"
            }
            components::emoji_picker::EmojiPicker {
                on_select: move |_| {
                        todo!()
                }
            }
        }
    }
}

use chrono::Utc;
fn fmt_nice_date(date: DateTime) -> String {
    fn get_day_suffix(day: u32) -> &'static str {
        match day {
            11 | 12 | 13 => "th",
            _ => match day % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            },
        }
    }
    fn get_month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        }
    }
    let today = Utc::now().date_naive();
    let input_date = date.date_naive();
    let duration = today.signed_duration_since(input_date);

    if duration.num_days() == 0 {
        "Today".to_string()
    } else if duration.num_days() == 1 {
        "Yesterday".to_string()
    } else if duration.num_days() < 7 && date.weekday() == today.weekday().pred() {
        format!("{}", date.weekday())
    } else if date.month() == today.month() && date.year() == today.year() {
        format!("{}{}", date.day(), get_day_suffix(date.day()))
    } else {
        format!(
            "{} of {}, {}",
            date.day(),
            get_month_name(date.month()),
            date.year()
        )
    }
}

#[component]
fn Day(day: types::Day) -> Element {
    rsx! {
        div {
            class: "day",
            h3 {
                class: "date",
                "{fmt_nice_date(day.date)}"
            }
            div {
                class: "habits",
                for habit in day.habits.values() {
                    Habit { day_id: day.id, habit: habit.clone() }
                }
            }
            NewHabitForm { day_id: day.id }
        }
    }
}

#[component]
fn NewHabitForm(day_id: Uuid) -> Element {
    let cmd = use_coroutine_handle::<RitualCmd>();
    let mut title = use_signal(String::new);

    let mut show_form = use_signal(|| false);

    let new_day_button = rsx! {
        Button {
            class: "new-habit",
            onclick: move |_| { show_form.set(true) },
            Icon {
                icon: IoAddOutline,
            }
        }
    };

    let new_day_form = rsx! {
        form {
            class: "new-habit-form",
            onsubmit: move |e| {
                e.prevent_default();
                cmd.send(RitualCmd::AddHabitToDay {
                    title: title.read().clone(),
                    day_id,
                });
                title.set(String::new());
                show_form.set(false);
            },
            input {
                r#type: "text",
                placeholder: "New Habit",
                value: "{title}",
                oninput: move |e| title.set(e.data.value())
            }
            Button { class: "submit", r#type: "submit",
                Icon {
                    icon: IoCheckmarkOutline,
                }
            }
        }
        Button {
            class: "cancel",
            onclick: move |_| { show_form.set(false) },
            Icon  {
                icon: IoCloseOutline,
            }
        }
    };

    if show_form() {
        new_day_form
    } else {
        new_day_button
    }
}

#[component]
fn Habit(day_id: Uuid, habit: types::HabitRef) -> Element {
    let cmd = use_coroutine_handle::<RitualCmd>();
    rsx! {
        div {
            class: "habit",
            div { span {
                class: "name",
                "{habit.name}" }
            }
            input {
                r#type: "checkbox",
                checked: habit.done,
                onchange: move |e| {
                    cmd.send(RitualCmd::HabitSetDone {
                        day_id,
                        habit_id: habit.id,
                        done: e.data.value() == "true",
                    });
                }
            }
        }
    }
}

pub enum RitualCmd {
    NewDay,
    AddHabitToDay {
        title: String,
        day_id: Uuid,
    },
    HabitSetDone {
        day_id: Uuid,
        habit_id: Uuid,
        done: bool,
    },
    Save,
}
