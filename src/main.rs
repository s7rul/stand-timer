use chrono::{Duration, Local};
use clap::Parser;
use std::io;
use strum_macros::Display;
use tui_big_text::{BigText, PixelSize};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, style::Stylize, symbols::border, text::Line, widgets::Block,
};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Sit time in minutes
    #[arg(long)]
    pub sit_time: u32,
    /// Stand time in minutes
    #[arg(long)]
    pub stand_time: u32,
}


fn main() {
    let args = Args::parse();

    let mut app = App::new(args.sit_time, args.stand_time);
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
    fn new(sit_time: u32, stand_time: u32) -> Self {
        Self {
            sit_time: Duration::minutes(sit_time as i64),
            stand_time: Duration::minutes(stand_time as i64),
            timer: Duration::minutes(sit_time as i64),
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
