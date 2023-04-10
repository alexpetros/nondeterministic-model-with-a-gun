use regex::Regex;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Instruction {
    command: u8,
    duration: u8
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = self.command as char;
        let duration = self.duration as char;
        write!(f, "Instruction {{ command: {}, duration: {} }}", command, duration)
    }
}

pub fn get_commands (text: &str) -> Vec<Instruction> {
    // TODO move this to static?
    let re = Regex::new(r"\[([^\]]*)\]").unwrap();

    re.captures_iter(text)
        .map(|capture| capture.get(1).unwrap().as_str())
        .filter_map(|raw_command| get_command(raw_command))
        .collect()
}

fn get_command (text: &str) -> Option<Instruction> {
    let (command_verb, duration) = text.split_once('-')?;
    let command = match command_verb {
        "forward" => b'f',
        "backward" => b'b',
        "left" => b'l',
        "right" => b'r',
        "straight" => b's',
        _ => return None
    };

    let duration = duration.bytes().nth(0)?;

    Some(Instruction { command, duration })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interprets_string_with_no_commands() {
        let instructions = get_commands("There are no commands in this string.");
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn interprets_one_command() {
        let instructions = get_commands("[forward-2]");
        assert_eq!(instructions[0].command, b'f');
        assert_eq!(instructions[0].duration, b'2');
    }

    #[test]
    fn interprets_string_of_commands() {
        let instructions = get_commands("[forward-2] [backward-1]");
        assert_eq!(instructions[0].command, b'f');
        assert_eq!(instructions[0].duration, b'2');
        assert_eq!(instructions[1].command, b'b');
        assert_eq!(instructions[1].duration, b'1');
    }

    #[test]
    fn interprets_forward_command() {
        let instruction = get_command("forward-7").unwrap();
        assert_eq!(instruction.command, b'f');
        assert_eq!(instruction.duration, b'7');
    }

    #[test]
    fn interprets_backward_command() {
        let instruction = get_command("backward-2").unwrap();
        assert_eq!(instruction.command, b'b');
        assert_eq!(instruction.duration, b'2');
    }

    #[test]
    fn interprets_straight_command() {
        let instruction = get_command("straight-0").unwrap();
        assert_eq!(instruction.command, b's');
        assert_eq!(instruction.duration, b'0');
    }
}

