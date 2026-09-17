use std::{io, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use khaos_core::{analyze, generate, EntropyReport};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::ui;

pub struct App {
    sample: Option<Vec<u8>>,
    report: Option<EntropyReport>,
    output: Option<Vec<u8>>,
    error: Option<String>,
    should_quit: bool,
}

impl App {
    pub fn new(sample: Option<Vec<u8>>) -> Self {
        let report = sample.as_deref().map(analyze);

        Self {
            sample,
            report,
            output: None,
            error: None,
            should_quit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| ui::render(frame, self))?;

            if event::poll(Duration::from_millis(100))? {
                self.handle_event(event::read()?);
            }
        }

        Ok(())
    }

    pub fn report(&self) -> Option<&EntropyReport> {
        self.report.as_ref()
    }

    pub fn output(&self) -> Option<&[u8]> {
        self.output.as_deref()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn has_sample(&self) -> bool {
        self.sample.is_some()
    }

    fn handle_event(&mut self, event: Event) {
        let Event::Key(key) = event else {
            return;
        };

        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('g') => self.generate(),
            _ => {}
        }
    }

    fn generate(&mut self) {
        let Some(sample) = self.sample.as_deref() else {
            self.error = Some("Start khaos with a sample file before generating bytes".to_owned());
            return;
        };

        match generate(sample, 32) {
            Ok(bytes) => {
                self.output = Some(bytes);
                self.error = None;
            }
            Err(error) => {
                self.output = None;
                self.error = Some(error.to_string());
            }
        }
    }
}
