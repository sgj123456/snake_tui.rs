use crossterm::{
    cursor::MoveTo,
    event::{self, poll, Event, KeyCode, KeyEvent},
    execute, style,
    terminal::{self, *},
    QueueableCommand, Result,
};
use rand::Rng;
use std::{
    collections::VecDeque,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
#[derive(Debug)]
struct Snake {
    body: VecDeque<(u16, u16)>,
    direction: KeyCode,
    head: (u16, u16),
    score: u32,
}
impl Snake {
    fn new(direction: KeyCode, head: (u16, u16)) -> Self {
        assert!([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]
            .iter()
            .any(|&element| element == direction));
        Self {
            body: VecDeque::from([(0, 0)]),
            head,
            direction,
            score: 0,
        }
    }
    fn is_collision(&self) -> bool {
        self.body.iter().any(|&element| element == self.head)
    }
    fn move_body(&mut self) -> (u16, u16) {
        self.body.push_back(self.head);
        match self.direction {
            KeyCode::Left => self.head.0 -= 1,
            KeyCode::Right => self.head.0 += 1,
            KeyCode::Up => self.head.1 -= 1,
            KeyCode::Down => self.head.1 += 1,
            _ => panic!(),
        };
        self.body.pop_front().unwrap()
    }
    fn set_direction(&mut self, direction: KeyCode) {
        assert!([KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down]
            .iter()
            .any(|&element| element == direction));
        match (&mut self.direction, &direction) {
            (old, new) if old == new => return,
            (KeyCode::Left, KeyCode::Right)
            | (KeyCode::Right, KeyCode::Left)
            | (KeyCode::Up, KeyCode::Down)
            | (KeyCode::Down, KeyCode::Up) => return,
            (old, new) => {
                *old = *new;
            }
        }
    }
    fn eat(&mut self, value: u8) {
        self.score += value as u32;
        self.body.push_back(*self.body.back().unwrap());
    }
}
struct Egg {
    direction: (u16, u16),
    value: u8,
}

impl Egg {
    fn range() -> Self {
        Self {
            direction: (
                rand::thread_rng().gen_range(0..50),
                (rand::thread_rng().gen_range(0..15)),
            ),
            value: rand::thread_rng().gen_range(0..u8::MAX),
        }
    }
    fn set_range(&mut self) {
        *self = Self::range();
    }
    fn is_be_eaten(&self, head: (u16, u16)) -> bool {
        if self.direction == head {
            true
        } else {
            false
        }
    }
}

fn main() -> Result<()> {
    let mut snake = Snake::new(KeyCode::Right, (0, 0));
    let mut egg = Egg::range();
    enable_raw_mode()?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    loop {
        if poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => snake.set_direction(code),
                _ => (),
            }
        } else {
            execute!(stdout(), BeginSynchronizedUpdate)?;
            let (old_x, old_y) = snake.move_body();
            if egg.is_be_eaten(snake.head) {
                snake.eat(egg.value);
                egg.set_range()
            }
            stdout()
                .queue(MoveTo(old_x, old_y))?
                .queue(style::Print(" "))?;
            stdout()
                .queue(MoveTo(egg.direction.0, egg.direction.1))?
                .queue(style::Print("$"))?;
            for &(x, y) in &snake.body {
                stdout().queue(MoveTo(x, y))?.queue(style::Print("*"))?;
                stdout().flush().unwrap();
            }
            sleep(Duration::from_millis(200));
        }
        execute!(stdout(), EndSynchronizedUpdate)?;
        if snake.is_collision() {
            panic!()
        }
    }
}
