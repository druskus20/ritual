use dioxus::prelude::*;
use dioxus_free_icons::IconShape;
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + PartialEq + Clone + 'static> {
    class: Option<String>,
    #[props(default = 16)]
    width: u32,
    #[props(default = 16)]
    height: u32,
    icon: T,
}

#[component]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let class = format!("{} default-icon", props.class.unwrap_or_default());
    rsx! {
        dioxus_free_icons::Icon {
            class: class,
            width: props.width,
            height: props.height,
            icon: props.icon,
            fill: "currentColor",
        }
    }
}
