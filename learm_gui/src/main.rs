use dioxus::logger::tracing;
use dioxus::prelude::*;
use rusb::{Context, DeviceHandle, Error, UsbContext};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[derive(Debug)]
enum UsbCommand {
    InitDevice, // Command to perform the initial x-byte write to IN endpoint #FIXME: Don't
    // remember how many bytes
    SendCommand(Vec<u8>), // Command to send the main data to OUT endpoint #FIXME: not sure that it
                          // works this way
}

struct DiscoveredDeviceInfo {
    vendor_id: u16,
    product_id: u16,
    interface_number: u8,
    endpoint_out: u8, // Endpoint address for OUT interrupt transfers
    endpoint_in: u8,  // Endpoint address for IN interrupt transfors (for init packet) #FIXME:
                      // verify this information
}
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // the AI seems to be pulling these from Yew.
    // I'll have to find their equivalents and translate them.
    let context = rusb::Context::new().expect("Couldn't open rusb context.");
    let vendor_id: u16 = 0x0483;
    let product_id: u16 = 0x5750;
    let interface_number: u8 = 0;
    let endpoint_out: u8 = 0x01;
    let endpoint_in: u8 = 0x81;
    //let mut signal = use_signal(|| 0);

    let device_info = find_hid_device(
        &context,
        vendor_id,
        product_id,
        interface_number,
        endpoint_out,
        endpoint_in,
    )
    .expect("Could not find HID device!");

    let (usb_sender, mut usb_receiver) = mpsc::channel(4);

    let discovered_device_info = DiscoveredDeviceInfo {
        vendor_id,
        product_id,
        interface_number,
        endpoint_out,
        endpoint_in,
    };
    let status_receiver: Arc<Mutex<Option<String>>> =
        Arc::new(Mutex::new(Some(String::from("stuff"))));
    //let move_robot = usb_communication_task(
    //    usb_receiver,
    //    device_info.handle,
    //    discovered_device_info,
    //    status_receiver,
    //);
    let hello_world_op = hello_world(usb_receiver);
    let hello_sig = use_signal(|| hello_world_op);

    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            p{
            button { onclick: move |evt| {
                let usb_sender_clone = usb_sender.clone();
                spawn(async move {
                    let res = usb_sender_clone.send(String::from("HELLO WORLD!!")).await;
                    match res {
                        Ok(res_) => tracing::debug!("message sent successfully! {:?}", res_),
                        Err(err) => tracing::debug!("Error sending message {:?}", err)
                    }
                    });
                },
            "CLICK THIS"
            }
            }
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
                slider_value.set(evt.value().parse().unwrap_or(1500));
                move_servo(servo_num, slider_value());
                //type_checker(&slider_value());
            },
        }
        p { "Selected value: {slider_value}"}
    }
}

fn move_servo(servo_num: u8, pos: u32) {
    let hex = format!("{:x}", pos);
    type_checker(&hex, &String::from("hex"));
    let chars: Vec<char> = hex.chars().collect();
    let len = chars.len();
    assert!(len == 3, "Hex is not three characters, got: {}", hex);
    let hex_flipped = format!(
        "55 55 08 03 01 00 00 0{} 0{} {}{}",
        servo_num, chars[0], chars[1], chars[2]
    );
    type_checker(&hex_flipped, &String::from("hex_flipped"));
    println!(
        "Moving servo {} to {}, which will send {}",
        servo_num, pos, hex_flipped
    );
}

fn type_checker<T>(_: &T, name: &String) {
    println!("The type of {} is {:?}", name, std::any::type_name::<T>());
}

#[derive(Debug)]
struct DeviceInfo {
    endpoint_in: u8,
    endpoint_out: u8,
    handle: DeviceHandle<rusb::Context>,
}

fn find_hid_device(
    context: &Context,
    vendor_id: u16,
    product_id: u16,
    interface_number: u8,
    endpoint_out: u8,
    endpoint_in: u8,
) -> Result<DeviceInfo, Error> {
    // iterate over available usb devices
    for device in context.devices()?.iter() {
        let device_descriptor = device.device_descriptor()?;
        // Check if device matches the Vendor ID and Product ID
        if device_descriptor.vendor_id() == vendor_id
            && device_descriptor.product_id() == product_id
        {
            let handle = match device.open() {
                Ok(handle) => handle,
                Err(err) => {
                    eprintln!("Failed to open device: {}", err);
                    continue; // Go to the next device
                }
            };
            let active_config = handle
                .active_configuration()
                .expect("Couldn't get active configuration from the USB device.");
            println!("Active configuration: {:?}", active_config);

            // Tell the interface to detach the kernel driver
            match handle.set_auto_detach_kernel_driver(true) {
                Ok(()) => {
                    println!("Successfully set automatic kernel driver detachment!");
                }
                Err(err) => {
                    println!(
                    "Failed to set automatic kernel detachment with error {}. Your platform may not be supported by rusb.", err
                )
                }
            };

            if let Err(err) = handle.claim_interface(interface_number) {
                eprintln!(
                    "Failed to claim interface number {} with the error: {}",
                    interface_number, err
                );
                // Don't return here, try to continue, Device might be in use, or have other
                // issues
                // TODO: Ensure this does not cause an infinite loop.
            } else {
                println!("Interface claimed successfully!");

                thread::sleep(Duration::from_millis(1000));
                let reqtype: u8 = 0x21;
                let req: u8 = 0x0a;
                let w_value: u16 = 0x0000;
                let w_index: u16 = 0;
                let timeout = Duration::from_millis(1000);
                handle
                    .write_control(reqtype, req, w_value, w_index, &mut [], timeout)
                    .expect("Failed to set idle, i.e. failed to set up USB device.");
            }
            return Ok(DeviceInfo {
                endpoint_in,
                endpoint_out,
                handle,
            });
        }
    }
    Err(Error::NotFound)
}

fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, Error> {
    // Remove any spaces or non-hex chars from the string
    let hex_string = hex_string.to_lowercase();
    let hex_string = hex_string.replace(" ", "");
    println!("{}", hex_string);

    //Check if the string has an even number of characters
    if hex_string.len() % 2 != 0 {
        return Err(Error::InvalidParam);
    }

    //Convert the string to a vector of bytes.
    let mut bytes = Vec::new();
    for i in (0..hex_string.len()).step_by(2) {
        let byte_str = &hex_string[i..i + 2];
        // use u8::from_str_radix to convert the hex string to a byte.
        let byte = u8::from_str_radix(byte_str, 16).map_err(|_| Error::InvalidParam)?;
        bytes.push(byte);
    }
    Ok(bytes)
}
async fn hello_world(mut receiver: mpsc::Receiver<String>) {
    let timeout = Duration::from_millis(1000);
    loop {
        match receiver.recv().await {
            Some(message) => {
                tracing::debug!("the message was: {}", &message);
            }
            None => {
                tracing::debug!("didn't receive a message.");
            }
        }
    }
}
// This async function will own the DeviceHandle and process USB commands
async fn usb_communication_task(
    mut receiver: mpsc::Receiver<UsbCommand>, // Receives commands from the UI
    mut device_handle: DeviceHandle<Context>, // OWNERSHIP of the DeviceHandle
    discovered_device_info: DiscoveredDeviceInfo,
    ui_status_sender: Arc<Mutex<Option<String>>>, // For sending status updates back to the UI
) {
    let timeout = Duration::from_millis(1000); // USB transfer timeout

    loop {
        match receiver.recv().await {
            Some(command) => {
                let result = match command {
                    UsbCommand::InitDevice => {
                        println!("USB Task: Received InitDevice command.");
                        let init_packet: [u8; 1] = [0x80];

                        println!(
                            "USB Task: Sending 64-byte init packet to IN indpoint {:x}",
                            discovered_device_info.endpoint_in
                        );
                        device_handle
                            .write_interrupt(
                                discovered_device_info.endpoint_in,
                                &init_packet,
                                timeout,
                            )
                            .map(|bytes_written| {
                                format!("Sent {} bytes (Init Packet)", bytes_written)
                            })
                    }
                    UsbCommand::SendCommand(data) => {
                        println!("USB Task: Recieved SendCommand command: {:x?}", data);
                        println!(
                            "USB Task: Sending command to OUT endpoint {:x}",
                            discovered_device_info.endpoint_out
                        );
                        device_handle
                            .write_interrupt(discovered_device_info.endpoint_out, &data, timeout)
                            .map(|bytes_written| format!("Sent {} bytes (Command)", bytes_written))
                    }
                };

                // Update UI status based on the result
                let mut status_guard = ui_status_sender.lock().expect("Lock failed! line 295");
                *status_guard = match result {
                    Ok(msg) => Some(format!("Success: {}", msg)),
                    Err(e) => Some(format!("Error: USB operation failed: {}", e)),
                }
            }
            None => {
                // Sender dropped, means no more commands will be sent
                println!("USB Task: Sender disconnected. Shutting down.");
                break;
            }
        }
    }
}
