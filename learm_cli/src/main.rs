use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Error, UsbContext};
use std::str::FromStr;
use std::time::Duration;

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

#[derive(Debug)]
struct DeviceInfo {
    vendor_id: u16,
    product_id: u16,
    interface_num: u8,
    handle: DeviceHandle<rusb::Context>,
}

// Find USB HID device based on Vendor ID and Product ID
// // Returns a Result containing the DeviceInfo struct on success, or an error on failure.

fn find_hid_device(
    //<T: UsbContext>(
    context: &Context,
    vendor_id: u16,
    product_id: u16,
    interface_number: u8, // add interface number
) -> Result<DeviceInfo, Error> {
    // Iterate over all devices connected to the system
    for device in context.devices()?.iter() {
        let device_descriptor = device.device_descriptor()?;
        // Check if the device matches the Vendor ID and Product ID
        if device_descriptor.vendor_id() == vendor_id
            && device_descriptor.product_id() == product_id
        {
            // Attempt to open the device
            let mut handle = match device.open() {
                Ok(handle) => handle,
                Err(err) => {
                    eprintln!("Failed to open device: {}", err);
                    continue; // Go to the next device
                    // TODO: Ensure this does not cause an infinite loop.
                }
            };

            // Claim the interface. This is crucial for being able to send data.
            if let Err(err) = handle.claim_interface(interface_number) {
                eprintln!("Failed to claim interface {}: {}", interface_number, err);
                // Dont return here, try to continue. Device might be in use, or have other issues.
                // TODO: Ensure this does not cause an infinite loop.
            } else {
                println!("Interface claimed successfully!");
            }
            return Ok(DeviceInfo {
                vendor_id: vendor_id,
                product_id: product_id,
                interface_num: interface_number,
                handle,
            });
        }
    }
    // If no matching device is found, return an error
    Err(Error::NotFound)
}
// Function to send a hexadecimal data report to the HID device
// Takes a mutable DeviceHandle and the hexadecimal data string as input.
// Returns a Result indicating success or failure.

fn send_hex_report(
    device_info: &mut DeviceInfo, // Ensure device_info is mutable
    hex_data: &str,
    report_id: u8, // Added report ID
) -> Result<(), Error> {
    // Convert the hexadecimal string to a vector of bytes.
    let bytes = hex_to_bytes(hex_data)?;
    let handle = &mut device_info.handle;

    // Prepend the report ID to the data. HID reports often start with an ID.
    let mut data_with_id = vec![report_id];
    data_with_id.extend_from_slice(&bytes);
    let data_to_send = &data_with_id;

    // Send the data as a control transfer. HID devices often use control transfers.
    // This is the most common way to send data. Alternative is Interrupt Transfer.
    let timeout = Duration::from_millis(1000); // 1 second timeout

    // Print the data being sent
    println!("Sending data {:x?}", data_to_send);
    let result = handle.write_interrupt(device_info.interface_num, data_to_send, timeout);

    match result {
        Ok(bytes_written) => {
            println!("Sent {} bytes", bytes_written);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error sending data: {}", err);
            Err(err)
        }
    }
}

// Helper function to convert a hexadecimal string to a vector of bytes.
// Handles potential errors in the hex string format.

fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, Error> {
    // Remove any spaces or non-hex characters from the string.
    let hex_string = hex_string.to_lowercase();
    let hex_string = hex_string.replace(" ", "");

    //Check if the string has an even number of characters.
    if hex_string.len() % 2 != 0 {
        return Err(Error::InvalidParam); // Hex string must have an even number of digits.
    } else if hex_string.len() % 2 == 0 {
    } else {
        println!(
            "Something very weird happened and the hex string length was neither odd nor even."
        );
        return Err(Error::Other);
    }
    // Convert the string to a vector of bytes.
    let mut bytes = Vec::new();
    for i in (0..hex_string.len()).step_by(2) {
        let byte_str = &hex_string[i..i + 2];
        // use u8::from_str_radix to convert the hex string to a byte.
        let byte = u8::from_str_radix(byte_str, 16).map_err(|_| Error::InvalidParam)?;
        bytes.push(byte);
    }
    Ok(bytes)
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

    // initialize the rusb context.
    let context = rusb::Context::new().expect("Couldn't open rusb context.");

    // Specify the Vendor ID and Product ID of your HID device.
    let vendor_id: u16 = 0x0483; // STMicroelectronics
    let product_id: u16 = 0x5750; // LED badge -- mini LED display -- 11x44
    // NOTE: It is a little odd that the LeArm's USB controller shows up as an LED display.
    let interface_number: u8 = 0; // Most HID devices use interface 0. Check with lsusb -v

    // Find the HID device.
    let mut device_info = find_hid_device(&context, vendor_id, product_id, interface_number)
        .expect("Could not find HID device!");
    println!("Device found and opened successfully!");

    // Example hexadecimal data to send (replace with your desired data).
    let hex_data = "0102030405060708";
    let report_id: u8 = 0x00; // Set the Report ID. 0 is common, check your device.

    // Send the hexadecimal data to the device.
    send_hex_report(&mut device_info, hex_data, report_id).expect("Could not send data to device!");
    println!("Data sent successfully!");

    // No need to explicitly release the handle; it's dropped automatically
    // when the DeviceInfo struct goes out of scope. However, you *can*
    // add an explicit handle.release_interface(interface_number) if you want.
}
