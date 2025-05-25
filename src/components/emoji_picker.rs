use dioxus::prelude::*;

#[rustfmt::skip]
const EMOJIS: &[&str] = &[
    // Fitness & Wellness
    "💪", "🏃", "🧘", "🚴", "🏋️", "🛌", "💤", "🚿", "🛁", "🧴",
    // Food & Health
    "🍎", "🥦", "🥗", "🍞", "🍳", "🧃", "💧", "🍌", "🥕", "🍇", 
    // Productivity & Study
    "📚", "📝", "📅", "📈", "🧠", "💡", "💻", "⏰", "📖", "📦", 
    // Housekeeping
    "🧹", "🧺", "🧽", "🧼", "🗑️", "🧯", "🪣", "🧰", "🛒", "🧻", 
    // Fun & Social
    "🎮", "🎨", "🎵", "🎲", "🎤", "🎧", "🎉", "📸", "🎬", "🍿",
];

#[derive(PartialEq, Props, Clone)]
pub struct EmojiPickerProps {
    on_select: Callback<String>,
}

#[component]
pub fn EmojiPicker(props: EmojiPickerProps) -> Element {
    let emojis = EMOJIS.iter().map(|&emoji| {
        rsx! {
                button {
                    class: "emoji",
                    onclick: move |_| props.on_select.call(emoji.to_string()),
                    "{emoji}"
                }
        }
    });
    rsx! {
       div {
            class: "emoji-picker",
            {
                emojis
            }
        }
    }
}
