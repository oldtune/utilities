pub fn generate_string(config: &StringGeneratorConfig<'_>) -> String {
    let initial = match config.initial {
        Some(val) => val,
        None => "",
    };

    let ending = match config.ending {
        Some(val) => val,
        None => "",
    };

    return format!(
        "{}{}{}",
        initial,
        get_pattern_repeat(config.pattern, config.repeat, config.separator),
        ending
    );
}

fn get_pattern_repeat(pattern: &'_ str, repeat: u32, separator: Option<char>) -> String {
    let mut result = String::new();

    for i in 0..repeat {
        result.push_str(pattern);
        if i < repeat - 1 && separator.is_some() {
            result.push(separator.unwrap());
        }
    }

    result
}

pub struct StringGeneratorConfig<'a> {
    pub initial: Option<&'a str>,
    pub pattern: &'a str,
    pub ending: Option<&'a str>,
    pub repeat: u32,
    pub separator: Option<char>,
}

mod test {
    use super::{generate_string, get_pattern_repeat, StringGeneratorConfig};

    #[test]
    pub fn string_generator_works() {
        let config = StringGeneratorConfig {
            ending: None,
            initial: None,
            pattern: "%x",
            repeat: 4,
            separator: None,
        };

        let result = generate_string(&config);
        assert_eq!("%x%x%x%x", result);
    }

    #[test]
    pub fn string_generator_complex_workks() {
        let config = StringGeneratorConfig {
            ending: Some("%x"),
            initial: Some("%x-"),
            pattern: "%x",
            repeat: 40,
            separator: None,
        };

        let result = generate_string(&config);

        assert_eq!(
            "%x-%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x%x",
            result
        );
    }

    #[test]
    pub fn get_pattern_repeat_works() {
        let result = get_pattern_repeat("%x", 4, None);
        assert_eq!(result, "%x%x%x%x");
    }
}
