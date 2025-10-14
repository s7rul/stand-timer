use chrono::{Duration, Local};
use std::io;
use strum_macros::Display;
use tui_big_text::{BigText, PixelSize};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, style::Stylize, symbols::border, text::Line, widgets::Block,
};

fn main() {
    let mut app = App::new();
    let mut terminal = ratatui::init();
    app.run(&mut terminal).unwrap();
    ratatui::restore();
}

#[derive(Display)]
enum SitStanState {
    #[strum(to_string = "Stand")]
    Stand,
    #[strum(to_string = "Sit")]
    Sit,
}

struct App {
    sit_stand: SitStanState,
    sit_time: Duration,
    stand_time: Duration,
    timer: Duration,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            sit_time: Duration::minutes(20),
            stand_time: Duration::minutes(5),
            timer: Duration::minutes(20),
            sit_stand: SitStanState::Sit,
            exit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_switch = Local::now();
        while !self.exit {
            let now = Local::now();
            let delta = now - last_switch;
            self.timer = match self.sit_stand {
                SitStanState::Stand => self.stand_time - delta,
                SitStanState::Sit => self.sit_time - delta,
            };

            if self.timer < Duration::zero() {
                last_switch = now;

                match self.sit_stand {
                    SitStanState::Stand => {
                        self.sit_stand = SitStanState::Sit;
                    }
                    SitStanState::Sit => {
                        self.sit_stand = SitStanState::Stand;
                    }
                }
            }

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let title = Line::from(" Sit stand timer ".bold());
        let help_text = Line::from(" q to quit ".bold());
        let main_block = Block::bordered()
            .title(title.centered())
            .title_bottom(help_text.centered())
            .border_set(border::THICK);
        let inner = main_block.inner(area);

        let timer_text = format!(
            "{}:{:0>2}",
            self.timer.num_minutes(),
            self.timer.num_seconds() % 60
        );

        let clock = BigText::builder()
            .pixel_size(PixelSize::Full)
            .centered()
            .lines(vec![
                timer_text.into(),
                format!("{}", self.sit_stand).into(),
            ])
            .build();

        frame.render_widget(main_block, area);
        frame.render_widget(clock, inner);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(5)).unwrap() {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Char('e') => todo!(),
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }
}
