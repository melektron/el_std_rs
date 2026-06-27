/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
26.06.26, 13:19
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree. 
*/

//! Functionality to setup an async interactive terminal REPL with
//! non-interfering logging and a good starting point for log formatting based 
//! on env_logger.
//! 
//! See [`setup_terminal`] or [`setup_basic_logging`] to get started.

use std::{io::IsTerminal, sync::{Arc}};
use std::io::Write;

use anstyle::{Style};
use clap::{FromArgMatches, Subcommand};
use env_logger::{Env, WriteStyle, fmt::Formatter};
use log::{Level, Record, info, warn};
use rustyline_async::{Readline, ReadlineError, ReadlineEvent};
use tokio_util::{future::FutureExt, sync::CancellationToken};

// some elements are re-exported so the user does not need to depend on
// the additional crates directly
/// Re-exported from [`rustyline_async::SharedWriter`]
/// 
pub use rustyline_async::SharedWriter;
/// Re-exported from [`anstyle::AnsiColor`]
/// 
pub use anstyle::AnsiColor;


/// Log formatter used for the default [`el_std`] logging setups.
/// In most the user doesn't need to use this directly,
/// [`setup_terminal`] and [`setup_basic_logging`] use it internally.
fn format_log(buf: &mut Formatter, record: &Record) -> Result<(), std::io::Error> {
    use std::io::Write;

    const LOC_MAX: usize = 24;

    let level = record.level();
    let color = match level {
        log::Level::Error => "\x1b[31m",
        log::Level::Warn => "\x1b[33m",
        log::Level::Info => "\x1b[32m",
        log::Level::Debug => "\x1b[34m",
        log::Level::Trace => "\x1b[35m",
    };

    // build file:line string
    let file = record.file().unwrap_or("?");
    let line = record.line().unwrap_or(0);
    let mut loc = format!("{file}:{line}");

    // cap length (keep rightmost part, usually the file name)
    if loc.len() > LOC_MAX {
        loc = format!("…{}", &loc[loc.len() - (LOC_MAX - 1)..]);
    }

    // fixed-width padding
    let loc = format!("{loc:>LOC_MAX$}");

    writeln!(
        buf,
        "{}[{}] [{}] {}\x1b[0m",
        color,
        level.as_str().chars().next().unwrap(), // levels are static constants, always longer than 1 char
        loc,
        record.args()
    )
}

/// Tuple of IO primitives needed for a repl.
/// This is typically returned by [`setup_terminal`] and
/// doesn't need to be constructed manually.
pub type ReplIo = (tokio::sync::Mutex<Readline>, SharedWriter);

/// Defines whether the terminal should behave
/// interactively or non-interactively.
#[derive(Default, PartialEq, Eq)]
pub enum TerminalMode {
    /// detect whether we are running in tty or not and configure
    /// interactivity accordingly
    #[default]
    Auto,
    /// force interactive terminal with repl (still falls 
    /// back to NonInteractive if setup fails)
    Interactive,
    /// force non interactive terminal which just logs to stdout without repl
    NonInteractive
}

/// Options for [`setup_terminal`]
pub struct TerminalOpts {
    /// Banner to log at info level immediately after terminal setup
    pub banner: Option<String>,
    /// Mode to operate in (Interactive, NonInteractive or Auto)
    pub mode: TerminalMode,
    /// whether to log the selected terminal mode after the banner
    pub log_mode: bool,
    /// default log filter level if nothing is specified via environment
    pub default_log_level: Level,
    /// Prompt string for repl. Used only in interactive mode.
    pub prompt: String,
}
impl Default for TerminalOpts {
    fn default() -> Self {
        Self { 
            banner: Default::default(), 
            mode: Default::default(), 
            log_mode: true,
            default_log_level: Level::Info,
            prompt: ">> ".to_string(),
        }
    }
}

/// Configures logging with a simple formatter and an (optional)
/// interactive readline editor to support simultaneous user input
/// and logging without interference. Behavior can be customized
/// using `options`.
/// 
/// If running in interactive mode (default if TTY is detected),
/// a [`ReplIo`] is returned, containing the readline editor used
/// to capture user input and output stream to print raw text to console
/// without interfering with the input buffer. This must be polled
/// (usually by a [`ReplLineHandler`] or [`ReplCommandHandler`] implementation), 
/// otherwise output (including logs) will not be forwarded to the console.
/// 
/// If you don't need any interactivity features, prefer using
/// [`setup_basic_logging`] instead.
/// 
pub fn setup_terminal(
    options: TerminalOpts
) -> Option<ReplIo> {

    // check whether we are running interactively, meaning both input and at least stdout
    // are attached to a tty, otherwise we disable the repl.
    let is_terminal = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let mut repl_io: Option<ReplIo> = None;

    if (is_terminal && options.mode == TerminalMode::Auto) || options.mode == TerminalMode::Interactive {
        // initialize interactive readline editor
        let (rl, stdout) = match Readline::new(options.prompt) {
            Err(e) => {
                // if readline init fails, fall back to non interactive setup
                setup_basic_logging(options.default_log_level);
                if let Some(banner) = options.banner {
                    info!("{banner}");
                }
                if options.log_mode {
                    warn!("Failed to set up interactive editor: {e}");
                    warn!("Fallback to non-interactive mode");
                }
                return None;
            }
            Ok(ok) => ok
        };

        // save readline and stdout instance for use by repl later
        repl_io = Some((tokio::sync::Mutex::new(rl), stdout.clone()));

        // setup logging to repl stdout
        env_logger::Builder::from_env(Env::default().default_filter_or(options.default_log_level.as_str()))
            .format(format_log)
            .target(env_logger::Target::Pipe(Box::new(stdout.clone())))
            // force allow ANSI sequences (pipe target normally disables them)
            .write_style(WriteStyle::Always)
            .init();

        if let Some(banner) = options.banner {
            info!("{banner}");
        }
        if options.log_mode {
            info!(
                "Running interactively{}.",
                if options.mode == TerminalMode::Interactive && !is_terminal {
                    " (forced)"
                } else {
                    ""
                }
            );
        }

    } else {
        setup_basic_logging(options.default_log_level);

        if let Some(banner) = options.banner {
            info!("{banner}");
        }
        if options.log_mode {
            info!(
                "Running non-interactively, disabling repl (in: {}, out: {}).",
                std::io::stdin().is_terminal(),
                std::io::stdout().is_terminal()
            );
        }
    };

    repl_io
}

/// Initializes a basic [`env_logger`] setup targeting stdout
/// with a basic log formatter. `default_level` is used as the filter if 
/// no level is configured via environment variables.
/// 
/// # Note
/// 
/// Do not use this in conjunction with [`setup_terminal`], as it already
/// calls [`setup_basic_logging`] internally. Use this only if an interactive
/// terminal is not needed.
pub fn setup_basic_logging(default_level: Level) {
    // setup logging to stdout
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level.as_str()))
        .format(format_log)
        .target(env_logger::Target::Stdout)
        .init();
}

/// Logs IO Errors to stderr. This is mainly used by the
/// [`shellout!`] and [`shelloutln!`] macros to notify the user
/// if something has gone wrong with the non-interfering stdout stream.
/// The user can use it to handle failure of manual writes to [`GetReplIo::get_stdout`].
pub fn handle_write_fail(r: Result<(), std::io::Error>) {
    if let Err(e) = r {
        eprintln!("Failed to write to interactive console: {e:?}")
    }
}

/// Wraps a string in ANSI Escape sequences for the provided color.
/// This is mainly used by the coloring variants of [`shellout!`] 
/// and [`shelloutln!`] but can also be used elsewhere.
pub fn colorize(color: AnsiColor, text: &str) -> String {
    let style = Style::new().fg_color(Some(color.into()));
    format!("{style}{text}{}", anstyle::Reset)
}


/// Writes raw text to the interactive terminal without interfering
/// with the user input buffer using [`writeln!`]. This is intended for use in
/// repl handlers ([`ReplCommandHandler`] or [`ReplLineHandler`])
/// as it relies on [`GetReplIo::get_stdout`] to access the
/// non interfering output stream.
#[macro_export]
macro_rules! shelloutln {
    // colored form: self, color => fmt, args...
    ($self:expr, $color:expr => $fmt:expr $(, $args:expr)* $(,)?) => {
        if let Some(mut stdout) = $self.get_stdout() {
            use ::std::io::Write;
            $crate::terminal::handle_write_fail(
                writeln!(
                    stdout,
                    "{}",
                    $crate::terminal::colorize($color, &format!($fmt $(, $args)*))
                )
            )
        }
    };

    // plain form: self, fmt, args...
    ($self:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        if let Some(mut stdout) = $self.get_stdout() {
            use ::std::io::Write;
            $crate::terminal::handle_write_fail(
                writeln!(
                    stdout,
                    $fmt $(, $args)*
                )
            )
        }
    };
}
pub use shelloutln as shelloutln;

/// Writes raw text to the interactive terminal without interfering
/// with the user input buffer using [`write!`]. This is intended for use in
/// repl handlers ([`ReplCommandHandler`] or [`ReplLineHandler`])
/// as it relies on [`GetReplIo::get_stdout`] to access the
/// non interfering output stream.
#[macro_export]
macro_rules! shellout {
    // colored form: self, color => fmt, args...
    ($self:expr, $color:expr => $fmt:expr $(, $args:expr)* $(,)?) => {

        if let Some(mut stdout) = $self.get_stdout() {
            use ::std::io::Write;
            $crate::terminal::handle_write_fail(
                write!(
                    stdout,
                    "{}",
                    $crate::terminal::colorize($color, &format!($fmt $(, $args)*))
                )
            )
        }
    };

    // plain form: self, fmt, args...
    ($self:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        if let Some(mut stdout) = $self.get_stdout() {
            use ::std::io::Write;
            $crate::terminal::handle_write_fail(
                write!(
                    stdout,
                    $fmt $(, $args)*
                )
            )
        }
    };
}
pub use shellout as shellout;

/// Provides access a ReplIo instance stored
/// in a struct member.
/// 
/// Implementation of this trait is required by
/// and mainly intended for use with [`ReplLineHandler`]
/// and [`ReplCommandHandler`].
#[allow(async_fn_in_trait)]
pub trait GetReplIo {
    /// Should return a reference to the implementing structs
    /// ReplIo instance. Returning None will disable repl functionality.
    fn get_repl_io(&self) -> Option<&ReplIo>;
    
    /// Returns a cloned [`SharedWriter`] to the non-interfering
    /// stdout stream. Can be used to write directly
    fn get_stdout(&self) -> Option<SharedWriter> {
        self.get_repl_io().map(|io| io.1.clone())
    }

    #[doc(hidden)]
    async fn _get_readline(&self) -> Option<tokio::sync::MutexGuard<'_, Readline>> {
        if let Some(io) = self.get_repl_io() {
            Some(io.0.lock().await)
        } else {
            None
        }
    }
    
}

/// Allows processing of [`ReplIo`] provided by [`GetReplIo`]
/// to handle user input and write non-interfering output
/// to a REPL terminal by polling [`ReplLineHandler::run`].
/// 
/// At a minimum [`ReplLineHandler::handle_line`] must be implemented
/// to handle the raw user input when submitted.
/// 
/// If command parsing with [`clap`], is desired, use 
/// [`ReplCommandHandler`] instead.
/// 
/// # Example
/// 
/// ```
/// use std::sync::Arc;
/// use el_std::terminal::{
///     ReplIo,
///     TerminalOpts,
///     setup_terminal,
///     GetReplIo, 
///     shelloutln,
///     ReplLineHandler
/// };
/// use tokio_util::sync::CancellationToken;
/// 
/// struct MyRepl {
///     repl_io: Option<ReplIo>,
///     ct: CancellationToken,
/// }
/// 
/// impl MyRepl {
///     fn new(io: Option<ReplIo>, ct: &CancellationToken) -> Self {
///         Self {
///             repl_io: io,
///             ct: ct.clone(),
///         }
///     }
/// }
/// 
/// impl GetReplIo for MyRepl {
///     fn get_repl_io(&self) -> Option<&ReplIo> {
///         return self.repl_io.as_ref();
///     }
/// }
/// 
/// impl ReplLineHandler for MyRepl {
/// 
///     // optional
///     fn get_cancellation_token(&self) -> Option<CancellationToken> {
///         return Some(self.ct.clone());
///     }
/// 
///     async fn handle_line(&self, line: String) -> anyhow::Result<()> {
///         shelloutln!(self, "You entered: {line}!");
///         Ok(())
///     }
/// }
/// 
/// #[tokio::main]
/// async fn main() {
///     let repl_io = setup_terminal(TerminalOpts::default());
///     let ct = CancellationToken::new();
///     let repl = Arc::new(MyRepl::new(repl_io, &ct));
///     let _ = tokio::spawn(repl.run()).await;
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait ReplLineHandler: GetReplIo {

    #[doc(hidden)]
    async fn _readline(&self, rl: &mut tokio::sync::MutexGuard<'_, Readline>, ct: Option<&CancellationToken>) -> Option<Result<ReadlineEvent, ReadlineError>> {
        if let Some(cancellation_token) = ct {
            rl.readline().with_cancellation_token(&cancellation_token).await
        } else {
            Some(rl.readline().await)
        }
    }

    async fn run(self: Arc<Self>) -> Result<(), ReadlineError> {
        let Some(mut rl) = self._get_readline().await else { return Ok(()); };

        let ct = self.get_cancellation_token();

        while let Some(event) = self._readline(&mut rl, ct.as_ref()).await
        {
            match event {
                Ok(rustyline_async::ReadlineEvent::Line(line)) => {
                    // empty command lines are ignored
                    if line.as_str().trim().is_empty() {
                        continue;
                    }

                    rl.add_history_entry(line.clone());

                    // valid command has been invoked
                    if let Err(e) = self.handle_line(line).await {
                        shelloutln!(self, AnsiColor::Red => "Command failed: {e}");
                    };
                }
                Ok(ReadlineEvent::Interrupted) => {
                    self.on_interrupt();
                }
                Ok(ReadlineEvent::Eof) => {
                    self.on_eof();
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    /// Called when user submits a line for processing by pressing Enter.
    /// The raw text entered by the user is passed as `line`.
    /// If an Error is returned, it is printed to the console in red with
    /// a prefix. Any error can be returned.
    /// 
    /// If you intend to parse the line as a command using [`clap`], you may
    /// prefer using [`ReplCommandHandler`] instead which does the heavy lifting
    /// automatically.
    async fn handle_line(&self, line: String) -> anyhow::Result<()>;

    /// Called when user presses Ctrl+C. Typical implementation
    /// would cancel the main cancellation token to terminate application.
    /// 
    /// Default implementation cancels the cancellation token
    /// provided by `get_cancellation_token()` if any.
    fn on_interrupt(&self) {
        if let Some(ct) = self.get_cancellation_token() {
            ct.cancel();
        }
    }
    
    /// Called when STDIN reaches EOF. May also imply
    /// that the program should be terminated. 
    /// 
    /// Default implementation does nothing.
    fn on_eof(&self) {}

    /// optionally specify a cancellation token to use for REPL loop.
    /// `run()` will exit when this token is canceled.
    /// 
    /// Additionally, by default this token will be cancel when
    /// the user presses Ctrl+C. This behavior can be changed by implementing
    /// `on_interrupt()`.
    fn get_cancellation_token(&self) -> Option<CancellationToken> {
        None
    }
}

/// Allows processing of [`ReplIo`] provided by [`GetReplIo`]
/// to handle user input and write non-interfering output
/// to a REPL terminal by polling [`ReplLineHandler::run`].
/// 
/// Unlike [`ReplLineHandler`], user submitted lines are parsed
/// as [`clap`] subcommands defined by [`ReplCommandHandler::ClapCommandsEnum`].
/// 
/// At a minimum [`ReplCommandHandler::handle_command`] must be 
/// implemented, which is called when the user has submitted a valid
/// command that was be parsed successfully.
/// 
/// # Example
/// 
/// ```
/// use std::sync::Arc;
/// use el_std::terminal::{
///     ReplIo,
///     TerminalOpts,
///     setup_terminal,
///     GetReplIo, 
///     shelloutln,
///     ReplLineHandler,    // required for run() method
///     ReplCommandHandler,
/// };
/// use tokio_util::sync::CancellationToken;
/// use clap::{Subcommand, command};
/// 
/// struct MyRepl {
///     repl_io: Option<ReplIo>,
///     ct: CancellationToken,
/// }
/// 
/// impl MyRepl {
///     fn new(io: Option<ReplIo>, ct: &CancellationToken) -> Self {
///         Self {
///             repl_io: io,
///             ct: ct.clone(),
///         }
///     }
/// }
/// 
/// impl GetReplIo for MyRepl {
///     fn get_repl_io(&self) -> Option<&ReplIo> {
///         return self.repl_io.as_ref();
///     }
/// }
/// 
/// #[derive(Subcommand, Debug)]
/// enum ReplCommands {
///     #[command(
///         visible_alias = "q",
///         visible_alias = "quit",
///         about = "Shuts down cleanly"
///     )]
///     Exit,
/// 
///     #[command(alias = "e", about = "Outputs the input")]
///     Echo { text: String },
/// }
/// 
/// impl ReplCommandHandler for MyRepl {
///     type ClapCommandsEnum = ReplCommands;
/// 
///     fn get_cancellation_token(&self) -> Option<CancellationToken> {
///         return Some(self.ct.clone());
///     }
/// 
///     async fn handle_command(&self, cmd: Self::ClapCommandsEnum) -> anyhow::Result<()> {
///         match cmd {
///             ReplCommands::Exit => {
///                 self.ct.cancel();
///             }
///             ReplCommands::Echo { text } => {
///                 shelloutln!(self, "{text}");
///             }
///         }
/// 
///         Ok(())
///     }
/// }
/// 
/// #[tokio::main]
/// async fn main() {
///     let repl_io = setup_terminal(TerminalOpts::default());
///     let ct = CancellationToken::new();
///     let repl = Arc::new(MyRepl::new(repl_io, &ct));
///     let _ = tokio::spawn(repl.run()).await;
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait ReplCommandHandler: GetReplIo {
    type ClapCommandsEnum: Subcommand + FromArgMatches;

    /// Called when user submits a valid command for processing by pressing Enter.
    /// `cmd` can be any subcommand specified in `ClapCommandsEnum`.
    /// If an Error is returned, it is printed to the console in red with
    /// a prefix. Any error can be returned.
    /// 
    /// If you intend to process raw text the user entered, prefer using 
    /// [`ReplLineHandler`] directly, which gives you access to the raw 
    /// submitted text without any parsing.
    async fn handle_command(&self, cmd: Self::ClapCommandsEnum) -> anyhow::Result<()>;

    /// Called when user presses Ctrl+C. Typical implementation
    /// would cancel the main cancellation token to terminate application.
    /// 
    /// Default implementation cancels the cancellation token
    /// provided by `get_cancellation_token()` if any.
    fn on_interrupt(&self) {
        if let Some(ct) = self.get_cancellation_token() {
            ct.cancel();
        }
    }

    /// Called when STDIN reaches EOF. May also imply
    /// that the program should be terminated. 
    /// 
    /// Default implementation does nothing.
    fn on_eof(&self) {}

    /// optionally specify a cancellation token to use for REPL loop.
    /// `run()` will exit when this token is canceled.
    /// 
    /// Additionally, by default this token will be cancel when
    /// the user presses Ctrl+C. This behavior can be changed by implementing
    /// `on_interrupt()`.
    fn get_cancellation_token(&self) -> Option<CancellationToken> {
        None
    }

    #[doc(hidden)]
    fn _build_command(&self) -> clap::Command {
        let command = clap::Command::new("");
        let command = Self::ClapCommandsEnum::augment_subcommands(command);
        let command = command
            .subcommand_required(true)
            .arg_required_else_help(true)
            .multicall(true);

        command
    }

    #[doc(hidden)]
    fn _try_parse_command(&self, args: Vec<String>) -> Result<Self::ClapCommandsEnum, clap::Error> {

        let command = self._build_command();
        // parse the command from user args. This catches most (if not all)
        // of the incorrect user input errors
        let mut matches = command.try_get_matches_from(args)?;
        // convert matched args into internal rust struct
        let cmd = Self::ClapCommandsEnum::from_arg_matches_mut(&mut matches).map_err(|e| {
            // format the error with context of the command (just like normal Parser 
            // impl does it), which also converts it to a standard clap::Error that
            // will then be shown to the user.
            // In most cases we'll never get here, as try_get_matches_from() catches
            // most of the incorrect user input. This would only error if the parsed
            // command isn't properly augmented with ReplCommands
            let mut command = self._build_command();
            e.format(&mut command)
        })?;

        Ok(cmd)
    }

    fn print_long_help(&self) {
        if let Some(mut stdout) = self.get_stdout() {
            handle_write_fail(
                stdout.write_all(
                    self._build_command()
                        .bin_name("")
                        .color(clap::ColorChoice::Always)
                        .render_long_help()
                        .ansi()
                        .to_string()
                        .as_bytes(),
                ),
            );
        }
    }

}

// automatically implement ReplLineHandler for all ReplCommandHandlers, 
// parsing the line into a command and invoking user command handler.
impl<U> ReplLineHandler for U
where
    U: ReplCommandHandler,
{
    async fn handle_line(&self, line: String) -> anyhow::Result<()> {

        // parse the line into an argument list using shell syntax
        let args = match shell_words::split(line.as_str()) {
            Err(e) => {
                shelloutln!(self, AnsiColor::Red => "Syntax error: {e}");
                return Ok(());
            },
            Ok(args) => args
        };

        // pars args into a valid clap subcommand
        match self._try_parse_command(args) {
            Err(clap_error) => {
                // Clap errors are shown directly to the user with ANSI styling preserved.
                if let Some(stdout) = self.get_stdout() {
                    handle_write_fail(
                        stdout
                            .clone()
                            .write_all(clap_error.render().ansi().to_string().as_bytes()),
                    );
                }
            }
            // call user handler
            Ok(cmd) => self.handle_command(cmd).await?
        };

        Ok(())
    }

    // forward additional event callbacks from ReplLineHandler to ReplCommandHandler
    fn on_interrupt(&self) {
        ReplCommandHandler::on_interrupt(self)
    }

    fn on_eof(&self) {
        ReplCommandHandler::on_eof(self)
    }

    fn get_cancellation_token(&self) -> Option<CancellationToken> {
        ReplCommandHandler::get_cancellation_token(self)
    }
}