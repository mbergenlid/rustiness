
pub struct Command {
    value: Vec<String>
}

impl Command {
    pub fn from(string: String) -> Command {
        Command {
            value: string.split_whitespace().map(|s| s.to_string()).collect()
        }
    }
    pub fn name(&self) -> &str {
        if self.value.len() > 0 { &self.value[0] } else { "" }
    }

    pub fn arg(&self, index: usize) -> Option<&String> {
        self.value.get(index)
    }

    pub fn hex_arg(&self, index: usize) -> Option<u16> {
        self.arg(index).and_then(|s| Command::parse_hex(s))
    }

    pub fn args(&self) -> &[String] {
        return &self.value[1..];
    }

    fn parse_hex(string: &String) -> Option<u16> {
        let mut value: u16 = 0;
        for c in string.chars() {
            let digit = c as u16;
            if digit >= 0x30 && digit <= 0x39 {
                value = value*16 + (digit - 0x30);
            } else if digit >= 0x41 && digit <= 0x46 {
                value = value*16 + (digit - 0x41 + 10);
            } else {
                return None;
            }
        }
        return Some(value);
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_single_argument() {
        let command = super::Command::from(String::from("break me"));
        assert_eq!(command.arg(1), Some(&String::from("me")));
    }
}
