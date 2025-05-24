use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    class: Option<String>,
    on_click: Option<fn()>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: props.class,
            onclick: move |_| {
                if let Some(on_click) = props.on_click {
                    on_click();
                }
            },
            {props.children}
        }
    }
}
