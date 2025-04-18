// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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

//#[tauri::command]
//fn move_servo(servo_num: &u8, pos_cmd: &u16) -> Result<String, std::error> {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
