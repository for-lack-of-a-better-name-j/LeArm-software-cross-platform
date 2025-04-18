// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use learm_cpr_lib::LeArm;
use learm_cpr_lib::Servo;

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
    learm_cpr_lib::run()
}
