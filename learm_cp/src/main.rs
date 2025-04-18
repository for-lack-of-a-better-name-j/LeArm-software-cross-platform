use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[derive(Debug)]
pub struct Servo {
    pub pos_max: u16,
    pub pos_min: u16,
    pub current_pos: u16,
    pub description: String,
}

#[derive(Debug)]
pub struct LeArm {
    pub servo_1: Servo,
    pub servo_2: Servo,
    pub servo_3: Servo,
    pub servo_4: Servo,
    pub servo_5: Servo,
    pub servo_6: Servo,
}
fn main() {
    let mut le_arm = LeArm {
        servo_1: Servo {
            pos_max: 2500,
            pos_min: 1500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Gripper, Joint 6"),
        },
        servo_2: Servo {
            pos_max: 2500,
            pos_min: 500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Joint 5"),
        },
        servo_3: Servo {
            pos_max: 2500,
            pos_min: 500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Joint 4"),
        },
        servo_4: Servo {
            pos_max: 2500,
            pos_min: 500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Joint 3"),
        },
        servo_5: Servo {
            pos_max: 2500,
            pos_min: 500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Joint 2"),
        },
        servo_6: Servo {
            pos_max: 2500,
            pos_min: 500,
            current_pos: 2000, // TODO: Figure out what the servo home positions are
            description: String::from("Base, Joint 1"),
        },
    };

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
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
}
