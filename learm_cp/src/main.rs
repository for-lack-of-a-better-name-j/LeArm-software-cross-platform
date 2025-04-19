use dioxus::prelude::*;
use tracing::info;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

static ICON: Asset = asset!("/assets/crappy_learm_standin.png");

fn main() {
    dioxus::launch(App);
}

/*#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}*/

/*#[component]
fn DogApp(props: DogAppProps) -> Element {
    rsx! {
        "Breed: {props.breed}"
    }
}*/

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
              h1 { "HotDog!  🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭 🌭"}}
    }
}

#[component]
fn DogView() -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    rsx! {
        div {
            id: "dogview",
            img {src: "{img_src}"}
        }
        div {id: "buttons",
            button {id: "skip", "skip"}
            button {id: "save", "save!"}

        }
    }
}

#[derive(Clone)]
struct TitleState(String);

fn title() -> Element {
    let title_ = use_context::<TitleState>();
    rsx! {
        h1 { "{title_.0}"}
    }
}

#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>,
}

fn use_music_player_provider() {
    let song = use_signal(|| "Not That".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: MAIN_CSS }
        Title {}
        DogView {}
    }
}

/*#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}*/
