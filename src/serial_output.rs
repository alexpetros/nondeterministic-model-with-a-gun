use serialport::SerialPort;
use std::thread::sleep;
use core::time::Duration;
use crate::llm_interpreter::Instruction;

pub fn open_connection (port: &str) -> Box<dyn SerialPort> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    // TODO get port dynamically
    let connection = serialport::new(port, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    println!("Successfully opened connection, waiting until serial ready");
    sleep(Duration::from_secs(3));
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
