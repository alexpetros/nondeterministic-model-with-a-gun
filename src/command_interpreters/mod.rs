use core::fmt;

pub mod rc_car;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub command: u8,
    pub duration: u8
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = self.command as char;
        let duration = self.duration as char;
        write!(f, "Instruction {{ command: {}, duration: {} }}", command, duration)
    }
}
