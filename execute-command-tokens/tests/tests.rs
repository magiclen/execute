use execute_command_tokens::command_tokens;

#[test]
fn v1() {
    assert_eq!(vec!["program"], command_tokens("program"));
    assert_eq!(vec!["program"], command_tokens("'program'"));
    assert_eq!(vec!["program"], command_tokens("\"program\""));
}

#[test]
fn v2() {
    assert_eq!(vec!["program", "arg1"], command_tokens("program arg1"));
    assert_eq!(vec!["program", "arg1"], command_tokens("program 'arg1'"));
    assert_eq!(vec!["program", "arg1"], command_tokens("program \"arg1\""));

    assert_eq!(vec!["program", "arg1"], command_tokens("'program' arg1"));
    assert_eq!(vec!["program", "arg1"], command_tokens("'program' 'arg1'"));
    assert_eq!(vec!["program", "arg1"], command_tokens("'program' \"arg1\""));

    assert_eq!(vec!["program", "arg1"], command_tokens("\"program\" arg1"));
    assert_eq!(vec!["program", "arg1"], command_tokens("\"program\" 'arg1'"));
    assert_eq!(vec!["program", "arg1"], command_tokens("\"program\" \"arg1\""));
}

#[test]
fn v3() {
    assert_eq!(vec!["program", "arg1", "arg2"], command_tokens("program arg1 arg2"));
    assert_eq!(vec!["program", "arg1 arg2"], command_tokens("program 'arg1 arg2'"));
    assert_eq!(vec!["program", "arg1 arg2"], command_tokens("program \"arg1 arg2\""));
}

#[test]
fn v4() {
    assert_eq!(vec!["program", "arg1 arg2"], command_tokens(r"program arg1\ arg2"));
}

#[test]
fn v5() {
    assert_eq!(vec!["program", "arg1", "arg2", " "], command_tokens("program arg1 arg2 ' '"));
    assert_eq!(vec!["program", "arg1", "arg2", "  "], command_tokens("program arg1 arg2 '  '"));
    assert_eq!(vec!["program", "arg1", " ", "arg2"], command_tokens("program arg1 ' ' arg2"));
    assert_eq!(vec!["program", "arg1", "  ", "arg2"], command_tokens("program arg1 '  ' arg2"));
    assert_eq!(vec!["program", "arg1", "arg2", " "], command_tokens("program arg1 arg2 \" \""));
    assert_eq!(vec!["program", "arg1", "arg2", "  "], command_tokens("program arg1 arg2 \"  \""));
    assert_eq!(vec!["program", "arg1", " ", "arg2"], command_tokens("program arg1 \" \" arg2"));
    assert_eq!(vec!["program", "arg1", "  ", "arg2"], command_tokens("program arg1 \"  \" arg2"));
    assert_eq!(vec!["program", "arg1", "arg2", " "], command_tokens(r"program arg1 arg2 \ "));
    assert_eq!(vec!["program", "arg1", "arg2", "  "], command_tokens(r"program arg1 arg2 \ \ "));
    assert_eq!(vec!["program", "arg1", " ", "arg2"], command_tokens(r"program arg1 \  arg2"));
    assert_eq!(vec!["program", "arg1", "  ", "arg2"], command_tokens(r"program arg1 \ \  arg2"));
}

#[test]
fn v6() {
    assert_eq!(vec!["program", "123 \"456\""], command_tokens("program '123 \"456\"'"));
    assert_eq!(vec!["program", "123 '456'"], command_tokens("program \"123 '456'\""));
}

#[test]
fn v7() {
    assert_eq!(vec!["program", "123456"], command_tokens("program 123'456'"));
    assert_eq!(vec!["program", "123456"], command_tokens("program 123\"456\""));

    assert_eq!(vec!["program", "123456"], command_tokens("program '123'456"));
    assert_eq!(vec!["program", "123456"], command_tokens("program \"123\"456"));

    assert_eq!(vec!["program", "123456"], command_tokens("program '123''456'"));
    assert_eq!(vec!["program", "123456"], command_tokens("program \"123\"\"456\""));

    assert_eq!(vec!["program", "123456"], command_tokens("program '123'\"456\""));
    assert_eq!(vec!["program", "123456"], command_tokens("program \"123\"'456'"));
}

#[test]
fn tolerance_v1() {
    assert_eq!(vec!["program", "1234  "], command_tokens("program '1234  "));
    assert_eq!(vec!["program", "1234  "], command_tokens("program \"1234  "));

    assert_eq!(vec!["program", "1234\"  "], command_tokens("program '1234\"  "));
    assert_eq!(vec!["program", "1234'  "], command_tokens("program \"1234'  "));
}
