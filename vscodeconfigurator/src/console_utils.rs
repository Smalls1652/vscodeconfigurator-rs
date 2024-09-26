use std::{io::{Result, Stdout, Write}, process};

use crossterm::{
    cursor::{RestorePosition, SavePosition}, event:: {
        read, Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags
    }, execute, queue, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};

/// Utility for writing to the console.
pub struct ConsoleUtils {
    pub stdout: Stdout
}

#[allow(dead_code)]
impl ConsoleUtils {
    /// Creates a new instance of `ConsoleUtils`.
    /// 
    /// ## Arguments
    /// 
    /// * `stdout` - The standard output stream. If `None`, the default standard output stream is used.
    pub fn new(stdout: Option<Stdout>) -> Self {
        let stdout_item = match stdout.is_some() {
            true => stdout.unwrap(),
            false => std::io::stdout()
        };

        Self {
            stdout: stdout_item
        }
    }

    /// Write an informational message to the console.
    pub fn write_info(&mut self, message: String) -> Result<()> {
        execute!(
            self.stdout,
            SetForegroundColor(Color::Cyan),
            Print(message),
            ResetColor
        )
    }

    /// Write a success message to the console.
    pub fn write_success(&mut self, message: String) -> Result<()> {
        execute!(
            self.stdout,
            SetForegroundColor(Color::Green),
            Print(message),
            ResetColor
        )
    }

    /// Write a warning message to the console.
    pub fn write_warning(&mut self, message: String) -> Result<()> {
        execute!(
            self.stdout,
            SetForegroundColor(Color::Yellow),
            Print(message),
            ResetColor
        )
    }

    /// Write an error message to the console.
    pub fn write_error(&mut self, message: String) -> Result<()> {
        execute!(
            self.stdout,
            SetForegroundColor(Color::Red),
            Print(message),
            ResetColor
        )
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

        execute!(
            self.stdout,
            SavePosition,
            SetForegroundColor(Color::Yellow),
            Print("\nOverwrite existing file? ([y]es/[n]o/[q]uit) "),
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
                                SavePosition,
                                SetForegroundColor(Color::Red),
                                Print("\n\n🛑 Quitting...\n"),
                                ResetColor
                            )?;
                            process::exit(1);
                        },
                        _ => {
                            disable_raw_mode()?;
                            self.restore_cursor_position_and_clear_below()?;
                            execute!(
                                self.stdout,
                                SavePosition,
                                SetForegroundColor(Color::Red),
                                Print("\nInvalid input. "),
                                SetForegroundColor(Color::Yellow),
                                Print("Overwrite existing file? ([y]es/[n]o/[q]uit) "),
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

        Ok(result)
    }

    /// Flush and release the standard output stream.
    pub fn release(&mut self) {
        self.stdout.flush().unwrap();
    }
}
