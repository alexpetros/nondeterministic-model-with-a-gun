use core::time::Duration;
use crate::llm_interpreter::Instruction;

pub fn send_instructions(port: &str, instructions: Vec<Instruction>) {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    // TODO don't open a new port each time
    let mut connection = serialport::new(port, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    println!("{}", instructions[0]);
    for instruction in &instructions {
        connection.write(&[instruction.command, instruction.duration, b'\n'])
            .expect("Failed to write!");
    }
}
