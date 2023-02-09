/// Source: https://github.com/wyhaya/loading/blob/main/src/lib.rs
/// Formatting altered slightly

use std::io::{Write, stderr};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Loading {
    sender: Sender<Signal>,
}

impl Default for Loading {
    fn default() -> Self {
        Self::new(Spinner::default())
    }
}

impl Loading {
    /// Create a stdout loading
    pub fn new(spinner: Spinner) -> Self {
        let (sender, receiver) = mpsc::channel();

        Self::update_stdout(receiver);
        Self::update_animation(sender.clone(), spinner);

        Self { sender }
    }

    /// End loading
    pub fn end(&self) {
        let (sender, receiver) = mpsc::channel();
        let _ = self.sender.send(Signal::Exit(sender));
        // Waiting for the sub -thread to exit
        let _ = receiver.recv();
    }

    /// Modify the currently displayed text
    pub fn text<T: ToString>(&self, text: T) {
        let _ = self.sender.send(Signal::Text(text.to_string()));
    }

    /// Save the current line as 'success' and continue to load on the next line
    pub fn success<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Success, text.to_string()));
    }

    /// Save the current line as 'fail' and continue to load on the next line
    pub fn fail<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Fail, text.to_string()));
    }

    /// Save the current line as 'warn' and continue to load on the next line
    pub fn warn<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Warn, text.to_string()));
    }

    /// Save the current line as 'info' and continue to load on the next line
    pub fn info<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Info, text.to_string()));
    }

    /// Save the current line as 'debug' and continue to load on the next line
    pub fn debug<T: ToString>(&self, text: T) {
        let text = format!("\x1B[90m{}\x1B[0m", text.to_string());

        let _ = self
            .sender
            .send(Signal::Next(Status::Debug, text.to_string()));
    }

    fn update_animation(sender: Sender<Signal>, mut spinner: Spinner) {
        thread::spawn(move || {
            while sender.send(Signal::Frame(spinner.next())).is_ok() {
                thread::sleep(spinner.interval);
            }
        });
    }

    fn update_stdout(receiver: Receiver<Signal>) {
        thread::spawn(move || {
            let mut output = stderr();
            let mut frame = "";
            let mut text = String::new();

            macro_rules! write_content {
                () => {
                    let _ = output.write(b"\x1B[2K\x1B[0G");
                    let _ = output.flush();
                };
                ($($arg:tt)*) => {
                    let _ = output.write(b"\x1B[2K\x1B[0G");
                    let _ = output.write(format!($($arg)*).as_bytes());
                    let _ = output.flush();
                };
            }

            let mut show_loader = true;
            while let Ok(signal) = receiver.recv() {
                match signal {
                    Signal::Frame(s) => {
                        frame = s;
                        if show_loader { write_content!("[{}] {}", frame, text); }
                    }
                    Signal::Text(s) => {
                        if show_loader { write_content!("[{}] {}", frame, s); }
                        text = s;
                    }
                    Signal::Next(status, s) => {
                        write_content!("[{}] {}\n", status.as_str(), s);
                    }
                    Signal::Exit(sender) => {
                        write_content!();
                        show_loader = false;
                        let _ = sender.send(());
                        // break;
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
enum Signal {
    Frame(&'static str),
    Text(String),
    Next(Status, String),
    Exit(Sender<()>),
}

#[derive(Debug, Clone)]
pub struct Spinner {
    index: usize,
    frames: Vec<&'static str>,
    interval: Duration,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    }
}

impl Spinner {
    pub fn new(frames: Vec<&'static str>) -> Self {
        Self {
            index: 0,
            frames,
            interval: Duration::from_millis(80),
        }
    }

    /// Change the interval between two frames
    pub fn interval(&mut self, interval: Duration) {
        self.interval = interval
    }

    fn next(&mut self) -> &'static str {
        match self.frames.get(self.index) {
            Some(s) => {
                self.index += 1;
                s
            }
            None => {
                self.index = 1;
                self.frames[0]
            }
        }
    }
}

#[derive(Debug)]
enum Status {
    Success,
    Fail,
    Warn,
    Info,
    Debug,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Success => "\x1B[92m+\x1B[0m",
            Status::Fail => "\x1B[91mFAIL\x1B[0m",
            Status::Warn => "\x1B[93m!\x1B[0m",
            Status::Info => "\x1B[94m*\x1B[0m",
            Status::Debug => "\x1B[90m \x1B[0m",
        }
    }
}