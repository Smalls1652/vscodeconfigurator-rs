use std::{borrow::Cow, io::{Result, Stderr, Stdout, Write}, process};

use crossterm::{
    cursor::{
        RestorePosition,
        SavePosition
    },
    event:: {
        read,
        Event,
        KeyCode,
        KeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags
    },
    execute,
    queue,
    style::{
        Attribute,
        Color,
        Print,
        ResetColor,
        SetAttribute,
        SetBackgroundColor,
        SetForegroundColor
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        Clear,
        ClearType
    },
    tty::IsTty
};
use regex_lite::Regex;

/// Utility for writing to the console.
pub struct ConsoleUtils {
    pub stdout: Stdout,

    pub stderr: Stderr,

    unicode_remove_regex: Regex
}

#[allow(dead_code)]
impl ConsoleUtils {
    /// Creates a new instance of `ConsoleUtils`.
    /// 
    /// ## Arguments
    /// 
    /// * `stdout` - The standard output stream. If `None`, the default standard output stream is used.
    pub fn new(stdout: Option<Stdout>, stderr: Option<Stderr>) -> Self {
        let stdout_item = match stdout.is_some() {
            true => stdout.unwrap(),
            false => std::io::stdout()
        };

        let stderr_item = match stderr.is_some() {
            true => stderr.unwrap(),
            false => std::io::stderr()
        };

        Self {
            stdout: stdout_item,
            stderr: stderr_item,
            unicode_remove_regex: Regex::new(concat!(
                "[",
                "\u{1F680}", // 🚀
                "\u{1F4C4}", // 📄
                "\u{2705}", // ✅
                "\u{1F7E0}", // 🟠
                "\u{1F4C1}", // 📁
                "\u{1F4E6}", // 📦
                "\u{1F973}", // 🥳
                "\u{270B}", // ✋
                "\u{1F6D1}", // 🛑
                "\u{1F6A8}", // 🚨
                "]"
            )).unwrap()
        }
    }

    /// Write an informational message to the console.
    pub fn write_info(&mut self, message: String) -> Result<()> {
        if !self.stdout.is_tty() {
            let message_noemojis = &self.remove_emojis(&message);
            let message_noemojis = message_noemojis.trim_start();

            execute!(
                self.stdout,
                Print(format!("[Info] - {}", &message_noemojis))
            )?;

            return Ok(());
        }

        execute!(
            self.stdout,
            SetForegroundColor(Color::Cyan),
            Print(message),
            ResetColor
        )
    }

    /// Write a success message to the console.
    pub fn write_success(&mut self, message: String) -> Result<()> {
        if !self.stdout.is_tty() {
            let message_noemojis = self.remove_emojis(&message);
            let message_noemojis = message_noemojis.trim_start();

            execute!(
                self.stdout,
                Print(format!("{}", &message_noemojis))
            )?;

            return Ok(());
        }

        execute!(
            self.stdout,
            SetForegroundColor(Color::Green),
            Print(message),
            ResetColor
        )
    }

    /// Write a warning message to the console.
    pub fn write_warning(&mut self, message: String) -> Result<()> {
        if !self.stdout.is_tty() {
            let message_noemojis = self.remove_emojis(&message);
            let message_noemojis = message_noemojis.trim_start();

            execute!(
                self.stdout,
                Print(format!("[Warning] - {}", &message_noemojis))
            )?;

            return Ok(());
        }

        execute!(
            self.stdout,
            SetForegroundColor(Color::Yellow),
            Print(message),
            ResetColor
        )
    }

    /// Write an error message to the console.
    pub fn write_error(&mut self, message: String) -> Result<()> {
        if !self.stdout.is_tty() {
            let message_noemojis = self.remove_emojis(&message);
            let message_noemojis = message_noemojis.trim_start();

            execute!(
                self.stdout,
                Print(format!("[Error] - {}", &message_noemojis))
            )?;

            return Ok(());
        }

        execute!(
            self.stdout,
            SetForegroundColor(Color::Red),
            Print(message),
            ResetColor
        )
    }

    /// Write an error message to the console by utilizing an error source.
    pub fn write_error_extended(&mut self, source_error: Box<dyn std::error::Error>) -> Result<()> {
        let source_error_type: &str;
        let mut source_error_kind: Option<String> = None;

        if source_error.is::<std::io::Error>() {
            source_error_type = "I/O error";
            source_error_kind = Some(
                source_error.downcast_ref::<std::io::Error>()
                    .unwrap()
                    .kind()
                    .to_string()
                    .clone()
            );
        }
        else if source_error.is::<clap::error::Error>() {
            source_error_type = "CLI Argument Parser error";
        }
        else if source_error.is::<crate::error::CliError>() {
            source_error_type = "Internal error";
            source_error_kind = Some(
                source_error.downcast_ref::<crate::error::CliError>()
                    .unwrap()
                    .kind
                    .to_string()
                    .clone()
            );
        }
        else {
            source_error_type = "Unknown error";
        }

        if !self.stdout.is_tty() {
            let error_message = match source_error_kind.is_some() {
                true => format!(
                    "[Error] - {} (Kind: {}): {}",
                    source_error_type,
                    source_error_kind.unwrap(),
                    source_error
                ),
                false => format!(
                    "[Error] - {}: {}",
                    source_error_type,
                    source_error
                )
            };

            execute!(
                self.stderr,
                Print(error_message)
            )?;

            return Ok(());
        }

        execute!(
            self.stdout,
            SetAttribute(Attribute::Bold),
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print(format!("🚨 Error ")),
            ResetColor,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            SetBackgroundColor(Color::Grey),
            Print(format!(" {} ", source_error_type)),
            ResetColor
        )?;

        if source_error_kind.is_some() {
            execute!(
                self.stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Red),
                SetBackgroundColor(Color::Black),
                Print(format!(" Kind: {} ", source_error_kind.unwrap())),
                ResetColor
            )?;
        }

        execute!(
            self.stdout,
            SetForegroundColor(Color::Red),
            Print(format!("\n\n{}", source_error)),
            ResetColor
        )?;

        Ok(())
    }

    /// Manually save the current cursor position.
    pub fn save_cursor_position(&mut self) -> Result<()> {
        execute!(self.stdout, SavePosition)
    }

    /// Manually restore the cursor position.
    pub fn restore_cursor_position(&mut self) -> Result<()> {
        execute!(self.stdout, RestorePosition)
    }

    /// Manually restore the cursor position and clear the screen below the cursor.
    pub fn restore_cursor_position_and_clear_below(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            RestorePosition,
            Clear(ClearType::FromCursorDown)
        )
    }

    /// Ask the user if they want to overwrite a file.
    pub fn ask_for_overwrite(&mut self) -> Result<bool> {
        if !self.stdout.is_tty() {
            panic!("Cannot ask for overwrite. Terminating...");
        }

        let supports_keyboard_enhancement = matches!(
            crossterm::terminal::supports_keyboard_enhancement(),
            Ok(true)
        );

        if supports_keyboard_enhancement {
            queue!(
                self.stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                )
            )?;
        }

        self.save_cursor_position()?;

        execute!(
            self.stdout,
            SetForegroundColor(Color::Yellow),
            Print("✋ Overwrite? ([y]es/[n]o/[q]uit) "),
            ResetColor
        )?;

        // TODO: Figure out how to handle mofifier keys for closing the program.
        enable_raw_mode()?;

        let result;
        loop {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('y') => {
                            result = true;
                            break;
                        },
                        KeyCode::Char('n') => {
                            result = false;
                            break;
                        },
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(
                                self.stdout,
                                SetForegroundColor(Color::Red),
                                Print("\n\n🛑 Quitting...\n"),
                                ResetColor
                            )?;
                            process::exit(1);
                        },
                        _ => {
                            disable_raw_mode()?;
                            self.restore_cursor_position_and_clear_below()?;
                            self.save_cursor_position()?;
                            execute!(
                                self.stdout,
                                SetForegroundColor(Color::Red),
                                Print("🛑 Invalid input. "),
                                SetForegroundColor(Color::Yellow),
                                Print("✋ Overwrite? ([y]es/[n]o/[q]uit) "),
                                ResetColor
                            )?;
                            enable_raw_mode()?;
                            continue
                        }
                    }
                },
                _ => continue
            }
        }

        disable_raw_mode()?;

        self.restore_cursor_position_and_clear_below()?;

        Ok(result)
    }

    /// Remove emojis from a message.
    fn remove_emojis<'a>(&self, message: &'a str) -> Cow<'a, str> {
        self.unicode_remove_regex
            .replace_all(&message, "")
    }

    /// Flush and release the standard output stream.
    pub fn release(&mut self) {
        self.stdout.flush().unwrap();

        self.stderr.flush().unwrap();
    }
}

/// An emoji to use in the console.
#[allow(dead_code)]
pub enum OutputEmoji {
    /// 🚀
    Rocket,

    /// 📄
    Document,

    /// ✅
    CheckMark,

    /// 🟠
    OrangeCircle,

    /// 📁
    Folder,

    /// 📦
    Package,

    /// 🥳
    Party,

    /// ✋
    Hand,

    /// 🛑
    Stop,

    /// 🚨
    Siren
}

impl std::fmt::Display for OutputEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            OutputEmoji::Rocket => "🚀",
            OutputEmoji::Document => "📄",
            OutputEmoji::CheckMark => "✅",
            OutputEmoji::OrangeCircle => "🟠",
            OutputEmoji::Folder => "📁",
            OutputEmoji::Package => "📦",
            OutputEmoji::Party => "🥳",
            OutputEmoji::Hand => "✋",
            OutputEmoji::Stop => "🛑",
            OutputEmoji::Siren => "🚨"
        };
        write!(f, "{}", emoji)
    }
}
