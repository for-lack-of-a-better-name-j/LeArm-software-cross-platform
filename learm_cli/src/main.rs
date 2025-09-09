use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Error, UsbContext};
use std::str::FromStr;
use std::thread;
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
    endpoint_in: u8,
    endpoint_out: u8,
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
    endpoint_out: u8,     // the bEndpointAddress of the endpoint to write to
    endpoint_in: u8,
) -> Result<DeviceInfo, Error> {
    // Iterate over all devices connected to the system
    for device in context.devices()?.iter() {
        let device_descriptor = device.device_descriptor()?;
        // Check if the device matches the Vendor ID and Product ID
        if device_descriptor.vendor_id() == vendor_id
            && device_descriptor.product_id() == product_id
        {
            // print some information about the device
            /*let device_desc = device
                .active_config_descriptor()
                .expect("couldn't open active config descriptor");
            println!("{:?}", device_desc);
            println!("{:?}", device_descriptor);*/

            // Attempt to open the device
            let handle = match device.open() {
                Ok(handle) => handle,
                Err(err) => {
                    eprintln!("Failed to open device: {}", err);
                    continue; // Go to the next device
                    // TODO: Ensure this does not cause an infinite loop.
                }
            };
            let active_config = handle
                .active_configuration()
                .expect("Couldn't get active configuration.");
            println!("Active configuration: {:?}", active_config);

            // Tell the interface to auto detach kernel driver
            match handle.set_auto_detach_kernel_driver(true) {
                Ok(()) => {
                    println!("Successfully set automatic kernel driver detachment!");
                }
                Err(err) => {
                    println!(
                        "Failed to set automatic kernel detachment with error {}. Your platform may not be supported.",
                        err
                    )
                }
            };
            /*handle
            .set_active_configuration(1)
            .expect("Failed to set active configuration!");*/

            // Claim the interface. This is crucial for being able to send data.
            //handle.detach_kernel_driver(0).expect("uh-oh");
            if let Err(err) = handle.claim_interface(interface_number) {
                eprintln!("Failed to claim interface {}: {}", interface_number, err);
                // Dont return here, try to continue. Device might be in use, or have other
                // issues.
                // TODO: Ensure this does not cause an infinite loop.
            } else {
                println!("Interface claimed successfully!");

                thread::sleep(Duration::from_millis(1000));
                let reqtype: u8 = 0x21;
                let req: u8 = 0x0a;
                let w_value: u16 = 0x0000;
                let w_index: u16 = 0;
                let timeout = Duration::from_millis(1000); // 1 second timeout
                handle
                    .write_control(reqtype, req, w_value, w_index, &mut [], timeout)
                    .expect("failed to set idle");
            }
            return Ok(DeviceInfo {
                vendor_id: vendor_id,
                product_id: product_id,
                interface_num: interface_number,
                endpoint_in: endpoint_in,
                endpoint_out: endpoint_out,
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

    // NOTE: Using interrupt transfer.
    let timeout = Duration::from_millis(1000); // 1 second timeout

    // Print the data being sent
    println!("Sending data (as hex format) {:x?}", bytes);
    println!("Sending data (as decimal format) {:?}", bytes);
    let other_bytes = (2228 as u32).to_be_bytes();
    // should be 08b4 or b4 08
    println!("and 2228 as data is {:?}", other_bytes);
    println!("and 2228 as hex data is {:x?}", other_bytes);
    let result = handle.write_interrupt(device_info.endpoint_out, &bytes, timeout);
    let result2 = handle.read_interrupt(device_info.endpoint_in, &mut [], timeout);

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
    println!("{}", hex_string);

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
    println!("bytes are: {bytes:?}");
    Ok(bytes)
}

fn main() {
    /*let mut le_arm = LeArm {
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
    };*/

    // initialize the rusb context.
    let context = rusb::Context::new().expect("Couldn't open rusb context.");

    // Specify the Vendor ID and Product ID of your HID device.
    let vendor_id: u16 = 0x0483; // STMicroelectronics 
    let product_id: u16 = 0x5750; // LED badge -- mini LED display -- 11x44
    // NOTE: It is a little odd that the LeArm's USB controller shows up as an LED display.
    let interface_number: u8 = 0; // Most HID devices use interface 0. Check with lsusb -v
    let endpoint_out: u8 = 0x01; // specify the endpoint out address
    let endpoint_in: u8 = 0x81;

    // Find the HID device.
    let mut device_info = find_hid_device(
        &context,
        vendor_id,
        product_id,
        interface_number,
        endpoint_out,
        endpoint_in,
    )
    .expect("Could not find HID device!");
    println!("Device found and opened successfully!");

    // Now I'll try it without the trailing zeros:
    //let hex_data = "55 55 08 03 01 00 00 06 ab 05";
    /*let hex_data = "55  start byte
     *                 55  start byte
     *                 08  Length of data - there is a formula for this in each command
     *                 03  command value
     *                 01  Number of controlled servos
     *                 00  lower time value of 8 bits
     *                 00  higher time value of 8 bits
     *                 06  Servo ID number
     *                 b4 lower the angle position value of 8 bits
     *                  08 higher the angle position value of 8 bits
     *                  ";*/
    //let hex_data = "55 55 08 03 01 00 00 06 b4 08 00 00 00 00 00 00 00 00 00 00 0000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 0000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 0000 00 00 00 00 00 00 00 00 00 00";
    // I'll try it again without the trailing zeros:
    //let hex_data = "55 55 08 03 01 00 00 05 dc 05";
    //let hex_data = "55 55 08 03 01 00 00 06 c4 09";
    let hex_data = "55 55 08 03 01 00 00 06 b4 08";
    let report_id: u8 = 0x00; // Set the Report ID. 0 is common, check your device.

    // Send the hexadecimal data to the device.
    send_hex_report(&mut device_info, hex_data, report_id).expect("Could not send data to device!");
    println!("Data sent successfully!");

    // No need to explicitly release the handle; it's dropped automatically
    // when the DeviceInfo struct goes out of scope. However, you *can*
    // add an explicit handle.release_interface(interface_number) if you want.
}
