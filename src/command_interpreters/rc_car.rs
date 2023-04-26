use crate::command_interpreters::Instruction;
use regex::Regex;

pub fn filter_instructions (text: &str) -> (String, Vec<Instruction>) {
    // TODO move this to static?
    let re = Regex::new(r"\[([^\]]*)\]").unwrap();

    let instructions = re.captures_iter(text)
        .map(|capture| capture.get(1).unwrap().as_str())
        .map(|str| str.to_lowercase())
        .filter_map(|raw_command| get_command(&raw_command))
        .collect();
    let spoken_text = re.replace_all(text, "").to_string();

    (spoken_text, instructions)
}

fn get_command (text: &str) -> Option<Instruction> {
    // TODO more elegant construction here
    if text == "left" { return Some( Instruction { command: b'l', duration: 1 } ) };
    if text == "right" { return Some( Instruction { command: b'r', duration: 1 } ) };
    if text == "straight" { return Some( Instruction { command: b's', duration: 1 } ) };

    let (command_verb, duration) = text.split_once('_')?;
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
        let (spoken_text, instructions) = filter_instructions("There are no commands in this string.");
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn interprets_one_command() {
        let (spoken_text, instructions) = filter_instructions("[forward_2]");
        assert_eq!(instructions[0].command, b'f');
        assert_eq!(instructions[0].duration, b'2');
    }

    #[test]
    fn interprets_string_of_commands() {
        let (spoken_text, instructions) = filter_instructions("[forward_2] [backward_1]");
        assert_eq!(instructions[0].command, b'f');
        assert_eq!(instructions[0].duration, b'2');
        assert_eq!(instructions[1].command, b'b');
        assert_eq!(instructions[1].duration, b'1');
    }

    #[test]
    fn interprets_forward_command() {
        let instruction = get_command("forward_7").unwrap();
        assert_eq!(instruction.command, b'f');
        assert_eq!(instruction.duration, b'7');
    }

    #[test]
    fn interprets_backward_command() {
        let instruction = get_command("backward_2").unwrap();
        assert_eq!(instruction.command, b'b');
        assert_eq!(instruction.duration, b'2');
    }

    #[test]
    fn interprets_straight_command() {
        let instruction = get_command("straight_0").unwrap();
        assert_eq!(instruction.command, b's');
        assert_eq!(instruction.duration, b'0');
    }
}

