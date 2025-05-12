use dioxus::prelude::*;
use std::any::type_name;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //Hero {}
        ServoSlider {
            servo_num : 1,
            min: 1500,
            max: 2500,
            default: 1750
                     }
        ServoSlider {
            servo_num: 2,
            min: 500,
            max: 2500,
            default: 1400
        }
        ServoSlider {
            servo_num: 3,
            min: 500,
            max: 2500,
            default: 750
        }
        ServoSlider {servo_num: 4,
            min: 500,
            max: 2500,
            default: 1750
         }
        ServoSlider {
         servo_num: 5,
         min: 500,
         max: 2500,
         default: 1750}
        ServoSlider {servo_num: 6,
        min: 500,
        max: 2500,
        default: 1750}
    }
}
/*
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
*/

#[component]
fn ServoSlider(servo_num: u8, min: u16, max: u16, default: u16) -> Element {
    let mut slider_value = use_signal(|| 50); // default value here
    rsx! {

        p {"Servo {servo_num}:"}
        input{
            id: "servo {servo_num}",
            r#type: "range",
            min: "{min}",
            max: "{max}",
            value: "{slider_value}",
            oninput: move |evt|{
                slider_value.set(evt.value().parse().unwrap_or(0));
                move_servo(servo_num, slider_value());
                //type_checker(&slider_value());
            },
        }
        p { "Selected value: {slider_value}"}
    }
}

fn move_servo(servo_num: u8, pos: u32) {
    let hex = format!("{:X}", pos);
    type_checker(&hex, &String::from("hex"));
    let chars: Vec<char> = hex.chars().collect();
    let len = chars.len();
    assert!(len == 3, "Hex is not three characters, got: {}", hex);
    let hex_flipped = format!("0{} {}{}", chars[0], chars[1], chars[2]);
    type_checker(&hex_flipped, &String::from("hex_flipped"));
    println!(
        "Moving servo {} to {}, which is {} in flipped hexadecimal",
        servo_num, pos, hex_flipped
    );
}

fn type_checker<T>(_: &T, name: &String) {
    println!("The type of {} is {:?}", name, std::any::type_name::<T>());
}
