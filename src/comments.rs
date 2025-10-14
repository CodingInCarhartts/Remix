/// Removes comments from source code based on file extension
pub fn remove_comments(content: &str, file_extension: &str) -> String {
    match file_extension {
        "rs" => remove_rust_comments(content),
        "js" | "ts" | "jsx" | "tsx" => remove_js_comments(content),
        "py" => remove_python_comments(content),
        "c" | "cpp" | "h" | "hpp" | "cs" | "java" | "go" | "swift" | "kt" => {
            remove_c_style_comments(content)
        }
        "rb" => remove_ruby_comments(content),
        "php" => remove_php_comments(content),
        "html" | "xml" | "svg" => remove_html_comments(content),
        "css" | "scss" | "sass" | "less" => remove_css_comments(content),
        "sh" | "bash" => remove_shell_comments(content),
        "yaml" | "yml" => remove_yaml_comments(content),
        _ => content.to_string(),
    }
}

/// Determines if comment removal is supported for this file type
pub fn is_comment_removal_supported(extension: &str) -> bool {
    matches!(
        extension,
        "rs" | "js"
            | "ts"
            | "jsx"
            | "tsx"
            | "py"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "cs"
            | "java"
            | "go"
            | "swift"
            | "kt"
            | "rb"
            | "php"
            | "html"
            | "xml"
            | "svg"
            | "css"
            | "scss"
            | "sass"
            | "less"
            | "sh"
            | "bash"
            | "yaml"
            | "yml"
    )
}

/// Removes comments from Rust code
fn remove_rust_comments(content: &str) -> String {
    remove_c_style_comments(content) // Rust uses the same comment style as C
}

/// Removes comments from JavaScript/TypeScript code
fn remove_js_comments(content: &str) -> String {
    remove_c_style_comments(content) // JS/TS use the same comment style as C
}

/// Removes comments from Python code
fn remove_python_comments(content: &str) -> String {
    let mut result = String::new();
    let mut in_multiline_string = false;
    let mut multiline_quotes = "";

    let lines = content.lines();
    for line in lines {
        let line_trim = line.trim();

        // Handle multi-line strings (which could contain # that aren't comments)
        if in_multiline_string {
            result.push_str(line);
            result.push('\n');

            if line.contains(multiline_quotes) {
                // Count the quotes at the end to see if we're closing the multi-line string
                let parts: Vec<&str> = line.rsplitn(2, multiline_quotes).collect();
                if parts.len() > 1 && !parts[0].ends_with('\\') {
                    in_multiline_string = false;
                }
            }
            continue;
        }

        // Check for multi-line string start
        if (line_trim.contains("'''") || line_trim.contains("\"\"\""))
            && !(line_trim.starts_with("#"))
        {
            if line_trim.contains("'''") {
                multiline_quotes = "'''";
            } else {
                multiline_quotes = "\"\"\"";
            }

            // Check if it opens and closes on the same line
            let count = line_trim.matches(multiline_quotes).count();
            if count % 2 == 1 {
                in_multiline_string = true;
            }

            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Regular comment handling
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            // Check if the # is in a string
            let in_string = preceding.chars().filter(|&c| c == '"' || c == '\'').count() % 2 == 1;

            if in_string {
                result.push_str(line);
            } else {
                result.push_str(preceding);
            }
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Removes comments from C-style languages (C, C++, Java, C#, etc.)
fn remove_c_style_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut in_char = false;
    let mut in_single_line_comment = false;
    let mut in_multi_line_comment = false;
    let mut string_quote = '"';

    while let Some(c) = chars.next() {
        match c {
            // String handling
            '"' | '\'' => {
                if !in_single_line_comment && !in_multi_line_comment {
                    if c == '"' && !in_char {
                        if !in_string {
                            string_quote = '"';
                            in_string = true;
                        } else if string_quote == '"' {
                            in_string = false;
                        }
                        result.push(c);
                    } else if c == '\'' && !in_string {
                        in_char = !in_char;
                        result.push(c);
                    } else {
                        result.push(c);
                    }
                }
            }
            // Escape sequence handling
            '\\' => {
                if !in_single_line_comment && !in_multi_line_comment && (in_string || in_char) {
                    result.push(c);
                    if let Some(next) = chars.next() {
                        result.push(next);
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            // Comment handling
            '/' => {
                if !in_string && !in_char && !in_single_line_comment && !in_multi_line_comment {
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            in_single_line_comment = true;
                            chars.next(); // consume the second '/'
                        } else if next == '*' {
                            in_multi_line_comment = true;
                            chars.next(); // consume the '*'
                        } else {
                            result.push(c);
                        }
                    } else {
                        result.push(c);
                    }
                } else if in_multi_line_comment {
                    if let Some(prev) = result.chars().last() {
                        if prev == '*' {
                            result.pop(); // remove the '*'
                            in_multi_line_comment = false;
                        }
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            // New line handling
            '\n' => {
                if in_single_line_comment {
                    in_single_line_comment = false;
                }
                result.push(c);
            }
            // Star handling for multi-line comments
            '*' => {
                if !in_string && !in_char && !in_single_line_comment && in_multi_line_comment {
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            in_multi_line_comment = false;
                            chars.next(); // consume the '/'
                        }
                    }
                } else if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
            // Regular character handling
            _ => {
                if !in_single_line_comment && !in_multi_line_comment {
                    result.push(c);
                }
            }
        }
    }

    result
}

/// Removes comments from Ruby code
fn remove_ruby_comments(content: &str) -> String {
    // Ruby uses # for single line comments similar to Python
    remove_python_comments(content)
}

/// Removes comments from PHP code
fn remove_php_comments(content: &str) -> String {
    // PHP supports both C-style comments and # comments
    let content = remove_c_style_comments(content);

    // Handle # comments
    let mut result = String::new();
    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            result.push_str(preceding);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Removes comments from HTML/XML code
fn remove_html_comments(content: &str) -> String {
    // Use a simple state machine approach
    let mut result = String::with_capacity(content.len());
    let mut state = 0; // 0: normal, 1: saw '<', 2: saw '<!', 3: saw '<!-', 4: in comment

    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        match state {
            0 => {
                if c == '<' {
                    state = 1;
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
            1 => {
                if c == '!' {
                    state = 2;
                    result.push(c);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            2 => {
                if c == '-' {
                    state = 3;
                    result.push(c);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            3 => {
                if c == '-' {
                    // Start of comment detected
                    state = 4;
                    // Remove "<!--" that we've accumulated
                    result.truncate(result.len() - 3);
                } else {
                    state = 0;
                    result.push(c);
                }
            }
            4 => {
                // In comment, look for "-->"
                if c == '-' {
                    // Check for end of comment
                    if chars.peek() == Some(&'-') {
                        chars.next(); // consume second '-'
                        if chars.peek() == Some(&'>') {
                            chars.next(); // consume '>'
                            state = 0; // Back to normal
                        }
                    }
                }
                // Don't add anything to result while in comment
            }
            _ => unreachable!(),
        }
    }

    result
}

/// Removes comments from CSS code
fn remove_css_comments(content: &str) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < content.len() {
        if i + 1 < content.len() && &content[i..i + 2] == "/*" {
            let comment_end = content[i + 2..].find("*/");
            if let Some(end) = comment_end {
                i = i + 2 + end + 2;
            } else {
                result.push_str(&content[i..]);
                break;
            }
        } else {
            result.push(content.chars().nth(i).unwrap());
            i += 1;
        }
    }

    result
}

/// Removes comments from shell scripts
fn remove_shell_comments(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            // Check if the # is part of a command, not a comment
            let preceding = &line[0..comment_pos];
            if preceding.contains("echo") || preceding.contains("printf") {
                result.push_str(line);
            } else {
                result.push_str(preceding);
            }
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Removes comments from YAML files
fn remove_yaml_comments(content: &str) -> String {
    let mut result = String::new();

    for line in content.lines() {
        if let Some(comment_pos) = line.find('#') {
            let preceding = &line[0..comment_pos];
            result.push_str(preceding);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}
