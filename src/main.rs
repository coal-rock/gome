#![allow(non_snake_case, deprecated)]

use dioxus::events::{KeyCode, KeyboardEvent};
use dioxus::prelude::*;
use dioxus_tui::TuiContext;

fn main() {
    dioxus_tui::launch_cfg(
        App,
        dioxus_tui::Config::new()
            // Some older terminals only support 16 colors or ANSI colors
            // If your terminal is one of these, change this to BaseColors or ANSI
            .with_rendering_mode(dioxus_tui::RenderingMode::Rgb),
    );
}

#[derive(PartialEq, Props)]
struct SetupProps<'a> {
    text: &'a str,
    selected: bool,
}

fn SetupEntry<'a>(cx: Scope<'a, SetupProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            height: "3px",
            border_style: if cx.props.selected {"solid"} else {"none"},
            border_width: "thick",
            cx.props.text.clone(),
        }
    })
}

fn Setup(cx: Scope) -> Element {
    let mut selected = use_state(cx, || 0);

    if *selected.get() > 4 {
        selected.set(0);
    } else if *selected.get() < 0 {
        selected.set(4);
    }

    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            justify_content: "center",
            align_items: "center",
            align_content: "center",
            position: "relative",
            display: "flex",
            padding: "2px",
            flex: "1",
            flex_direction: "column",

            onkeydown: move |k: KeyboardEvent| match k.key_code {
                KeyCode::DownArrow => selected += 1,
                KeyCode::UpArrow => selected -= 1,
                _ => {},
            },
            "--- GOME ---",
            "",
            SetupEntry {text: "Review Decks", selected: *selected == 0}
            SetupEntry {text: "Edit Decks", selected: *selected == 1}
            SetupEntry {text: "New Deck", selected: *selected == 2}
            SetupEntry {text: "Options", selected: *selected == 3}
            SetupEntry {text: "Quit", selected: *selected == 4}

        }
    })
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Setup {},
    })
}
