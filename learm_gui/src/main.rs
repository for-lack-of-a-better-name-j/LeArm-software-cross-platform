use dioxus::{logger::tracing, prelude::*};
use futures_util::StreamExt;

use crate::robot::RobotCommand;
use crate::robot::ServoMoveCommand;
mod robot;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

enum ProfileUpdate {
    SetUsername(String),
    SetAge(i32),
}
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let msg_text = use_signal(|| String::from("sup bro"));
    let mut slider_value1 = use_signal(|| 1750);
    let mut slider_value2 = use_signal(|| 1500);
    let mut slider_value3 = use_signal(|| 500);
    let mut slider_value4 = use_signal(|| 2200);
    let mut slider_value5 = use_signal(|| 2000);
    let mut slider_value6 = use_signal(|| 2500);
    let robot_service = use_coroutine(robot::robot_service);
    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            div {
            id: "app_container",
            /*
            p {
                "What you want to do is create a little coroutine that handles messages that can own the Rusb Context and device handle."
            }
            button {
                onclick: move |_| profile.send(ProfileUpdate::SetUsername("Bob".to_string())),
                "click this shiz.",

            }
            p {
                "{msg_text.read()}"
            }
            */

            div {
                class: "servo",
            p { "Servo 1:" }
            input {
                id: "servo 1",
                r#type: "range",
                min: "1500",
                max: "2500",
                value: "{slider_value1}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value1.set(pos_num.clone());
                    let cmd = ServoMoveCommand {
                        servo_id: 1,
                        servo_position: pos_num.clone(),
                    };
                    robot_service.send(RobotCommand::MoveServo(cmd));
                }

            }
            p { "Selected value: {slider_value1}"}
            }
            div {
                class: "servo",
            p { "Servo 2:" }
            input {
                id: "servo 2",
                r#type: "range",
                min: "500",
                max: "2500",
                value: "{slider_value2}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value2.set(pos_num);
                    let cmd = ServoMoveCommand{
                        servo_id: 2,
                        servo_position: pos_num.clone(),
                    };
                robot_service.send(RobotCommand::MoveServo(cmd));

                }

            }
            p { "Selected value: {slider_value2}"}
            }
            div {
                class: "servo",
            p { "Servo 3:" }
            input {
                id: "servo 3",
                r#type: "range",
                min: "500",
                max: "2500",
                value: "{slider_value3}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value3.set(pos_num.clone());
                    // add in the move handler coroutine here
                    let cmd = ServoMoveCommand {
                        servo_id: 3,
                        servo_position: pos_num.clone(),
                    };
                    robot_service.send(RobotCommand::MoveServo(cmd));

                }

            }
            p { "Selected value: {slider_value3}"}
        }
            div {
                class: "servo",
            p { "Servo 4:" }
            input {
                id: "servo 4",
                r#type: "range",
                min: "500",
                max: "2500",
                value: "{slider_value4}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value4.set(pos_num);
                    let cmd = ServoMoveCommand {
                        servo_id: 4,
                        servo_position: pos_num.clone(),
                    };
                    robot_service.send(RobotCommand::MoveServo(cmd));

                }

            }
            p { "Selected value: {slider_value4}"}
            }

            div {
                class: "servo",
            p { "Servo 5:" }
            input {
                id: "servo 5",
                r#type: "range",
                min: "500",
                max: "2500",
                value: "{slider_value5}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value5.set(pos_num);
                    let cmd = ServoMoveCommand {
                        servo_id: 5,
                        servo_position: pos_num.clone(),
                    };
                    robot_service.send(RobotCommand::MoveServo(cmd));

                }

            }
            p { "Selected value: {slider_value5}"}
        }

            div {
                class: "servo",
            p { "Servo 6:" }
            input {
                id: "servo 6",
                r#type: "range",
                min: "500",
                max: "2500",
                value: "{slider_value6}",
                oninput: move |evt| {
                    let pos_num = evt.value().parse().unwrap_or(1500);
                    slider_value6.set(pos_num);
                    let cmd = ServoMoveCommand {
                        servo_id: 6,
                        servo_position: pos_num.clone(),
                    };
                    robot_service.send(RobotCommand::MoveServo(cmd));
                }

            }
            p { "Selected value: {slider_value6}"}
            }

        }
    }
}
