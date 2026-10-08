use is_terminal::IsTerminal;
use std::io::{self, Write};

/// Determines whether to emit a trailing newline based on terminal detection and explicit flags.
pub fn should_append_newline(no_newline: bool, force_newline: bool) -> bool {
    if force_newline {
        return true;
    }
    if no_newline {
        return false;
    }
    // Default: append newline if output is connected to a terminal (TTY),
    // omit newline if called inline (piped, subshell, redirected, etc.)
    io::stdout().is_terminal()
}

/// Emits the text to stdout respecting terminal newline rules.
pub fn print_output(text: &str, no_newline: bool, force_newline: bool) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    write!(stdout, "{}", text)?;
    if should_append_newline(no_newline, force_newline) {
        writeln!(stdout)?;
    }
    stdout.flush()
}

/// Emits raw binary bytes to stdout.
pub fn print_raw_bytes(bytes: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(bytes)?;
    stdout.flush()
}
