use std::collections::HashMap;

pub fn get_command_map(arguments: &Vec<String>) -> HashMap<String, String> {
    let mut output: HashMap<String, String> = HashMap::new();
    let mut previous_argument: String = "".to_string();
    let mut x = 0;

    for argument in arguments.iter().skip(1) {
        x+=1;
        if argument.starts_with("-") {
            previous_argument = argument.clone();
            output.insert(argument.clone(), "".to_string());
        } else if previous_argument != "" {
            output.insert(previous_argument.clone(), argument.clone());
            previous_argument = "".to_string();
        } else {
            output.insert(x.to_string(), argument.clone());
        }
    }

    output
}

#[cfg(test)]
mod cli_test {
    use std::collections::HashMap;

    use super::get_command_map;

    #[test]
    pub fn get_command_map_will_correctly_parse_arguments() {
        //given we have a set of command line arguments
        let arguments: Vec<String> = vec!["executable name".to_string(), "-t".to_string(), "test".to_string(), "bad1".to_string(), "bad2".to_string(), "-p".to_string(), "pickle".to_string(), "bad3".to_string()];

        //when we parse the argument list
        let actual = get_command_map(&arguments);

        //then we get back a map of the arguments
        assert_eq!(actual, HashMap::from([
            ("-t".to_string(), "test".to_string()),
            ("3".to_string(), "bad1".to_string()),
            ("4".to_string(), "bad2".to_string()),
            ("7".to_string(), "bad3".to_string()), 
            ("-p".to_string(), "pickle".to_string())
        ]))
    }
}