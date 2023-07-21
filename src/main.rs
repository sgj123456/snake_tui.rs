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
struct Wall(u16, u16);
impl Wall {
    fn darw(&self) -> Result<()> {
        for i in 0..self.0 {
            stdout().queue(MoveTo(i, 0))?.queue(style::Print("█"))?;
            stdout()
                .queue(MoveTo(i, self.1))?
                .queue(style::Print("█"))?;
            stdout().queue(MoveTo(0, i))?.queue(style::Print("█"))?;
            stdout()
                .queue(MoveTo(self.0, i))?
                .queue(style::Print("█"))?;
        }
        Ok(())
    }
    fn is_collision(&self, head: (u16, u16)) -> bool {
        if head.0 > 0 && head.0 < self.0 && head.1 > 0 && head.1 < self.1 {
            false
        } else {
            true
        }
    }
}
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
    }
    fn update(&mut self, x: u16, y: u16) {
        for _ in 0..((self.score as i32) / 100 - self.body.len() as i32) {
            self.body.push_back((x, y));
        }
    }
}
struct Egg {
    direction: (u16, u16),
    value: u8,
}

impl Egg {
    fn range(wall: (u16, u16)) -> Self {
        Self {
            direction: (
                rand::thread_rng().gen_range(1..wall.0),
                (rand::thread_rng().gen_range(1..wall.1)),
            ),
            value: rand::thread_rng().gen_range(0..u8::MAX),
        }
    }
    fn set_range(&mut self, wall: (u16, u16)) {
        *self = Self::range(wall);
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
    let wall = Wall(110, 25);
    let mut snake = Snake::new(KeyCode::Right, (10, 10));
    let mut egg = Egg::range((wall.0, wall.1));
    enable_raw_mode()?;
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        SetSize(wall.0, wall.1),
        EnterAlternateScreen
    )?;
    wall.darw()?;
    loop {
        stdout()
            .queue(MoveTo(2, 2))?
            .queue(style::Print(format!("Score:{}", snake.score)))?;
        if poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => match code {
                    KeyCode::Esc => {
                        execute!(stdout(), LeaveAlternateScreen)?;
                        disable_raw_mode()?;
                        println!("Esc");
                        return Ok(());
                    }
                    _ => snake.set_direction(code),
                },
                _ => (),
            }
        } else {
            execute!(stdout(), BeginSynchronizedUpdate)?;
            let (old_x, old_y) = snake.move_body();
            if egg.is_be_eaten(snake.head) {
                snake.eat(egg.value);
                egg.set_range((wall.0, wall.1))
            }
            stdout()
                .queue(MoveTo(old_x, old_y))?
                .queue(style::Print(" "))?;
            stdout()
                .queue(MoveTo(egg.direction.0, egg.direction.1))?
                .queue(style::Print("$"))?;
            stdout()
                .queue(MoveTo(snake.head.0, snake.head.1))?
                .queue(style::Print("◎"))?;
            for &(x, y) in &snake.body {
                stdout().queue(MoveTo(x, y))?.queue(style::Print("●"))?;
                stdout().flush().unwrap();
            }
            snake.update(old_x, old_y);
            execute!(stdout(), EndSynchronizedUpdate)?;
            if snake.is_collision() || wall.is_collision(snake.head) {
                execute!(stdout(), LeaveAlternateScreen)?;
                disable_raw_mode()?;
                println!("Game Over!!!");
                return Ok(());
            }
            sleep(Duration::from_millis(100));
        }
    }
}
