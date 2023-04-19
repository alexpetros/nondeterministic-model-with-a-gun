use serialport::SerialPort;
use std::thread::sleep;
use core::time::Duration;
use crate::llm_interpreter::Instruction;

pub fn get_usb_connection () -> Box<dyn SerialPort> {
    let ports = serialport::available_ports().expect("Failed to find open port");
    let usb_port = ports.iter().find(|port| {
        port.port_name.starts_with("/dev/tty.usb")
    }).unwrap_or_else(|| {
        eprintln!("Available ports: {:?}", &ports);
        panic!("Failed to connect to USB port");
    });

    let mut connection = open_connection(&usb_port.port_name);
    run_connection_test(&mut connection);
    connection
}

fn open_connection (port: &str) -> Box<dyn SerialPort> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let connection = serialport::new(port, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    // TODO actually add a call and response
    println!("Successfully opened connection, waiting until serial ready");
    sleep(Duration::from_secs(2));
    println!("Serial is ready (probably)");
    connection
}

pub fn send_instructions(connection: &mut Box<dyn SerialPort>, instructions: Vec<Instruction>) {
    println!("{}", instructions[0]);
    println!("Received instructions {}", instructions[0]);
    for instruction in &instructions {
        connection.write(&[instruction.command, instruction.duration, b'\n']).expect("Failed to write!");
    }
}

fn run_connection_test(connection: &mut Box<dyn SerialPort>) {
    send_instructions(connection, vec![Instruction {command: b'l', duration: 1}]);
    sleep(Duration::from_secs(1));
    send_instructions(connection, vec![Instruction {command: b'r', duration: 1}]);
    sleep(Duration::from_secs(1));
    send_instructions(connection, vec![Instruction {command: b's', duration: 1}]);
}
