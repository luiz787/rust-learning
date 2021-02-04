use minigrep;

#[test]
fn filename_is_correct() {
    let args = create_args_array("pattern", "file.txt");

    let config = minigrep::Config::new(&args).unwrap();

    assert_eq!(config.filename, "file.txt");
}

#[test]
fn query_is_correct() {
    let args = create_args_array("pattern", "file.txt");

    let config = minigrep::Config::new(&args).unwrap();

    assert_eq!(config.query, "pattern");
}

#[test]
fn throws_if_less_than_three_args() {
    let args: Vec<String> = Vec::new();

    if let Err(e) = minigrep::Config::new(&args) {
        assert_eq!(e, "not enough arguments");
    }
}

fn create_args_array(query: &'static str, file: &'static str) -> Vec<String> {
    vec![
        String::from("program"),
        String::from(query),
        String::from(file),
    ]
}
