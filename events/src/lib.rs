use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(duration) => {
                let seconds = duration.num_seconds();
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let seconds = seconds % 60;
                let content = format!(
                    "You have {}H:{}M:{}S left before the registration ends",
                    hours, minutes, seconds
                );
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content,
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = Color::TrueColor {
            r: self.color.0,
            g: self.color.1,
            b: self.color.2,
        };
        let colored_content = self.content.clone().color(color);
        write!(f, "({:?}, {}, {})", self.position, self.size, colored_content)
    }
}