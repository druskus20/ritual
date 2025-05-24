use db::{Db, State};
use dioxus::{desktop::LogicalSize, prelude::*};
use futures_util::StreamExt;
use itertools::Itertools;
use prelude::*;
use types::DateTime;
use utils::{NonEmpty, Validate};
use uuid::Uuid;

mod cli;
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
                button {
                    onclick: move |_| {
                        cmd.send(RitualCmd::NewDay);
                },
                    "Add Day"
                }
            }
            button {
                onclick: move |_| {
                    cmd.send(RitualCmd::Save);
                },
                "Save"
            }
        }
    }
}

#[component]
fn Day(day: types::Day) -> Element {
    let human_date = |date: DateTime| date.format("%Y %m %d").to_string();
    let cmd = use_coroutine_handle::<RitualCmd>();
    rsx! {
        div {
            class: "day",
            span {
                class: "date",
                "{human_date(day.date)}"
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
    let mut title = use_signal(|| String::new());

    rsx! {
        form {
            class: "new-habit-form",
            onsubmit: move |e| {
                e.prevent_default();
                cmd.send(RitualCmd::AddHabitToDay {
                    title: title.read().clone(),
                    day_id,
                });
                title.set(String::new());
            },
            input {
                r#type: "text",
                placeholder: "New Habit",
                value: "{title}",
                oninput: move |e| title.set(e.data.value())
            }
            button { r#type: "submit", "Add" }
        }
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
