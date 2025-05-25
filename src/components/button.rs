use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    class: Option<String>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Element,
    r#type: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = format!("{} default-button", props.class.unwrap_or_default());
    rsx! {
        button {
            class: class,
            onclick: props.onclick.unwrap_or_default(),
            r#type: props.r#type.unwrap_or_else(|| "button".to_string()),
            {props.children},
        }
    }
}
