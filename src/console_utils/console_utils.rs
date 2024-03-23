use termion::{color, style, terminal_size};
use crate::console_utils::{MessageLevel};
use unicode_width::UnicodeWidthStr;

pub fn message(message: &str) {
    println!("{}", message);
}

pub fn message_color<C: color::Color>(message: &str, color: C) {
    println!("{}", wrap_color(message, color));
}

pub fn message_level(message: &str, level: MessageLevel) {
    let (width, _) = terminal_size().unwrap_or((80, 0));

    let status_text = match level {
        MessageLevel::Success => (wrap_color("SUCCESS", color::Green), "SUCCESS"),
        MessageLevel::Error => (wrap_color("ERROR", color::Red), "ERROR"),
        MessageLevel::Warning => (wrap_color("WARNING", color::Yellow), "WARNING"),
    };

    let formatted_status = format!("[{: ^9}]", status_text.1);
    let status = bold(&formatted_status);

    // 유니코드 문자열의 "시각적" 너비를 계산
    let message_len = UnicodeWidthStr::width(message);

    let space_length = if width > message_len as u16 + formatted_status.len() as u16 {
        width - message_len as u16 - formatted_status.len() as u16
    } else {
        0
    };

    // 모든 부분을 하나의 문자열로 결합
    let full_message = format!(
        "{}{:width$}{}",
        message,
        "",
        status.replace(&status_text.1, &status_text.0),
        width = space_length as usize
    );

    println!("{}", full_message);
}

fn wrap_color<C: color::Color>(message: &str, color: C) -> String {
    format!("{}{}{}", color::Fg(color), message, color::Fg(color::Reset))
}

fn bold(message: &str) -> String {
    format!("{}{}{}", style::Bold, message, style::Reset)
}

fn faint(message: &str) -> String {
    format!("{}{}{}", style::Faint, message, style::Reset)
}

fn underline(message: &str) -> String {
    format!("{}{}{}", style::Underline, message, style::Reset)
}
