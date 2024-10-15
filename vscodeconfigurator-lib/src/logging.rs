use std::{
    io::{Result, Stderr, Stdout, Write},
    process
};

use crossterm::{
    cursor::{RestorePosition, SavePosition},
    event::{read, Event, KeyCode, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
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
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    tty::IsTty
};

/// Utility for writing to the console.
pub struct ConsoleLogger {
    /// The standard output stream.
    pub stdout: Stdout,

    /// The standard error stream.
    pub stderr: Stderr
}

#[allow(dead_code)]
impl ConsoleLogger {
    /// Creates a new instance of `ConsoleLogger`.
    ///
    /// # Arguments
    ///
    /// - `stdout` - The standard output stream. If `None`, the default standard
    ///   output stream is used.
    /// - `stderr` - The standard error stream. If `None`, the default standard
    ///   error stream is used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let logger = ConsoleLogger::new(None, None);
    /// ```
    pub fn new(
        stdout: Option<Stdout>,
        stderr: Option<Stderr>
    ) -> Self {
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
            stderr: stderr_item
        }
    }

    /// Write an informational message to the console.
    ///
    /// # Arguments
    ///
    /// - `message` - The message to write.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_info("This is an informational message.".to_string());
    /// ```
    pub fn write_info(
        &mut self,
        message: String
    ) -> Result<()> {
        if !self.stdout.is_tty() {
            execute!(self.stdout, Print(format!("[Info] - {}", message)))?;

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
    ///
    /// # Arguments
    ///
    /// - `message` - The message to write.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_success("This is a success message.".to_string());
    /// ```
    pub fn write_success(
        &mut self,
        message: String
    ) -> Result<()> {
        if !self.stdout.is_tty() {
            execute!(self.stdout, Print(format!("{}", &message)))?;

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
    ///
    /// # Arguments
    ///
    /// - `message` - The message to write.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_warning("This is a warning message.".to_string());
    /// ```
    pub fn write_warning(
        &mut self,
        message: String
    ) -> Result<()> {
        if !self.stdout.is_tty() {
            execute!(self.stdout, Print(format!("[Warning] - {}", &message)))?;

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
    ///
    /// # Arguments
    ///
    /// - `message` - The message to write.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_error("This is an error message.".to_string());
    /// ```
    pub fn write_error(
        &mut self,
        message: String
    ) -> Result<()> {
        if !self.stdout.is_tty() {
            execute!(self.stdout, Print(format!("[Error] - {}", &message)))?;

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
    ///
    /// # Arguments
    ///
    /// - `source_error` - The source error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::{
    ///     error::{CliError, CliErrorKind},
    ///     logging::ConsoleLogger
    /// };
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    /// let error = CliError::new("An error occurred.", CliErrorKind::UnknownError);
    ///
    /// logger.write_error_extended(Box::new(error));
    /// ```
    pub fn write_error_extended(
        &mut self,
        source_error: Box<dyn std::error::Error>
    ) -> Result<()> {
        let source_error_type: &str;
        let mut source_error_kind: Option<String> = None;

        if source_error.is::<std::io::Error>() {
            source_error_type = "I/O error";
            source_error_kind = Some(
                source_error
                    .downcast_ref::<std::io::Error>()
                    .unwrap()
                    .kind()
                    .to_string()
                    .clone()
            );
        } else if source_error.is::<clap::error::Error>() {
            source_error_type = "CLI Argument Parser error";
        } else if source_error.is::<serde_json::Error>() {
            source_error_type = "JSON parsing error";

            let source_error_downcast = source_error
                .downcast_ref::<serde_json::Error>()
                .unwrap();

            source_error_kind = Some(match source_error_downcast.classify() {
                serde_json::error::Category::Io => "I/O error",
                serde_json::error::Category::Syntax => "Syntax error",
                serde_json::error::Category::Data => "Data error",
                serde_json::error::Category::Eof => "End of file error"
            }.to_string());
        } else if source_error.is::<crate::error::CliError>() {
            source_error_type = "Internal error";
            source_error_kind = Some(
                source_error
                    .downcast_ref::<crate::error::CliError>()
                    .unwrap()
                    .kind
                    .to_string()
                    .clone()
            );
        } else {
            source_error_type = "Unknown error";
        }

        if !self.stdout.is_tty() {
            let error_message = match source_error_kind.is_some() {
                true => format!(
                    "\n[Error] - {} (Kind: {}): {}",
                    source_error_type,
                    source_error_kind.unwrap(),
                    source_error
                ),
                false => format!("\n[Error] - {}: {}", source_error_type, source_error)
            };

            execute!(self.stderr, Print(error_message))?;

            return Ok(());
        }

        execute!(
            self.stdout,
            Print(format!("\n\n")),
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

    /// Manually restore the cursor position and clear the screen below the
    /// cursor.
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
                Event::Key(event) => match event.code {
                    KeyCode::Char('y') => {
                        result = true;
                        break;
                    }
                    KeyCode::Char('n') => {
                        result = false;
                        break;
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            self.stdout,
                            SetForegroundColor(Color::Red),
                            Print("\n\n🛑 Quitting...\n"),
                            ResetColor
                        )?;
                        process::exit(1);
                    }
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
                        continue;
                    }
                },
                _ => continue
            }
        }

        disable_raw_mode()?;

        self.restore_cursor_position_and_clear_below()?;

        Ok(result)
    }

    /// Write a newline to the console.
    pub fn write_newline(&mut self) -> Result<()> {
        execute!(self.stdout, Print("\n"))
    }

    /// Writes an operation category header to the console.
    ///
    /// # Arguments
    ///
    /// - `name` - The category name of the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_operation_category("Installing dependencies"); // "🚀 Installing dependencies"
    /// ```
    pub fn write_operation_category(
        &mut self,
        name: &str
    ) -> Result<()> {
        let message = match self.stdout.is_tty() {
            false => format!("{}\n", name),
            true => format!("{} {}\n", OutputEmoji::Rocket, name)
        };

        self.write_info(message)
    }

    /// Writes an operation log to the console.
    ///
    /// # Arguments
    ///
    /// - `message` - The message to write.
    /// - `emoji` - The emoji to use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::{ConsoleLogger, OutputEmoji};
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_operation_log("Adding package to tasks.json...", OutputEmoji::Document);
    /// // "📄 Adding package to tasks.json... "
    /// ```
    pub fn write_operation_log(
        &mut self,
        message: &str,
        emoji: OutputEmoji
    ) -> Result<()> {
        let message = match self.stdout.is_tty() {
            false => format!("- {} ", message),
            true => format!("- {} {} ", emoji, message)
        };

        self.write_info(message)
    }

    /// Writes an operation success log to the console.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_operation_success_log(); // "Done! ✅"
    /// ```
    pub fn write_operation_success_log(&mut self) -> Result<()> {
        let message = match self.stdout.is_tty() {
            false => format!("Done!\n"),
            true => format!("Done! {}\n", OutputEmoji::CheckMark)
        };

        self.write_success(message)
    }

    /// Writes a project initialized log to the console.
    ///
    /// # Example
    ///
    /// ```rust
    /// use vscodeconfigurator_lib::logging::ConsoleLogger;
    ///
    /// let mut logger = ConsoleLogger::new(None, None);
    ///
    /// logger.write_project_initialized_log(); // "🥳 VSCode project initialized!"
    /// ```
    pub fn write_project_initialized_log(&mut self) -> Result<()> {
        let message = match self.stdout.is_tty() {
            false => format!("VSCode project initialized!\n"),
            true => format!("{} VSCode project initialized!\n", OutputEmoji::Party)
        };

        self.write_info(message)
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
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
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
