use crate::command_interpreters::Instruction;
use regex::Regex;

pub fn filter_instructions (text: &str) -> (String, Vec<Instruction>) {
    // TODO move this to static?
    let re = Regex::new(r"\[([^\]]*)\]").unwrap();

    let instructions = re.captures_iter(text)
        .map(|capture| capture.get(1).unwrap().as_str())
        .filter_map(|raw_command| get_command(&raw_command))
        .collect();
    let spoken_text = re.replace_all(text, "").to_string();

    (spoken_text, instructions)
}

fn get_command (text: &str) -> Option<Instruction> {
    match text {
        "ACTION: KILL" => Some(Instruction { command: b's', duration: b'7' }),
        "ACTION: RAISE GUN" => Some(Instruction { command: b'u', duration: b'2' }),
        "LEFT" => Some(Instruction { command: b'l', duration: b'4' }),
        "RIGHT" => Some(Instruction { command: b'r', duration: b'4' }),
        _ => return None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interprets_string_with_no_commands() {
        let text = "There are no commands in this string.";
        let (spoken_text, instructions) = filter_instructions(text);
        assert_eq!(instructions.len(), 0);
        assert_eq!(spoken_text, text);
    }

    #[test]
    fn interprets_one_command() {
        let (spoken_text, instructions) = filter_instructions("[ACTION: KILL]");
        assert_eq!(instructions[0].command, b's');
        assert_eq!(instructions[0].duration, b'4');
        assert_eq!(spoken_text, "");
    }

    #[test]
    fn interprets_string_of_commands() {
        let (spoken_text, instructions) = filter_instructions("[ACTION: KILL] [ACTION: KILL]");
        assert_eq!(instructions[0].command, b's');
        assert_eq!(instructions[0].duration, b'4');
        assert_eq!(instructions[1].command, b's');
        assert_eq!(instructions[1].duration, b'4');
        assert_eq!(spoken_text, " ");
    }

    #[test]
    fn interprets_forward_command() {
        let (spoken_text, instructions) = filter_instructions("Exterminate! [ACTION: KILL]");
        assert_eq!(instructions[0].command, b's');
        assert_eq!(instructions[0].duration, b'4');
        assert_eq!(spoken_text, "Exterminate! ");
    }
}

