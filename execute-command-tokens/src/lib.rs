/*!
# Execute Command Tokens

Parse command strings.

See [`execute`](https://crates.io/crates/execute).
*/

/// Parse a command string.
pub fn command_tokens<S: AsRef<str>>(cmd: S) -> Vec<String> {
    let cmd = cmd.as_ref();

    let mut tokens = Vec::with_capacity(1);
    let mut string_buffer = String::new();

    let mut append_mode = false;
    let mut quote_mode = false;
    let mut quote_mode_ending = false; // to deal with '123''456' -> 123456
    let mut quote_char = ' ';
    let mut escaping = false;

    for c in cmd.chars() {
        if escaping {
            append_mode = true;
            escaping = false;

            string_buffer.push(c);
        } else if c.is_whitespace() {
            if append_mode {
                if quote_mode {
                    string_buffer.push(c);
                } else {
                    append_mode = false;

                    tokens.push(string_buffer);
                    string_buffer = String::new();
                }
            } else if quote_mode_ending {
                quote_mode_ending = false;

                tokens.push(string_buffer);
                string_buffer = String::new();
            }
        } else {
            match c {
                '"' | '\'' => {
                    if append_mode {
                        if quote_mode {
                            if quote_char == c {
                                append_mode = false;
                                quote_mode = false;
                                quote_mode_ending = true;
                            } else {
                                string_buffer.push(c);
                            }
                        } else {
                            quote_mode = true;
                            quote_char = c;
                        }
                    } else {
                        append_mode = true;
                        quote_mode = true;
                        quote_char = c;
                    }
                }
                '\\' => {
                    escaping = true;
                }
                _ => {
                    append_mode = true;
                    escaping = false;

                    string_buffer.push(c);
                }
            }
        }
    }

    if append_mode || quote_mode_ending {
        tokens.push(string_buffer);
    }

    tokens
}
