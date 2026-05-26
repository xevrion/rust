use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::{
    collections::VecDeque,
    io::{self, Write},
    time::{Duration, Instant},
};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;
const TICK_MS: u64 = 120;

#[derive(Clone, Copy, PartialEq)]
struct Pos {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

struct Game {
    snake: VecDeque<Pos>,
    dir: Dir,
    next_dir: Dir,
    food: Pos,
    score: u32,
    rng_seed: u64,
    alive: bool,
}

impl Game {
    fn new() -> Self {
        let head = Pos { x: WIDTH / 2, y: HEIGHT / 2 };
        let mut snake = VecDeque::new();
        snake.push_back(head);
        snake.push_back(Pos { x: head.x - 1, y: head.y });
        snake.push_back(Pos { x: head.x - 2, y: head.y });

        let mut g = Game {
            snake,
            dir: Dir::Right,
            next_dir: Dir::Right,
            food: Pos { x: 0, y: 0 },
            score: 0,
            rng_seed: 12345,
            alive: true,
        };
        g.food = g.spawn_food();
        g
    }

    fn lcg_next(&mut self) -> u64 {
        self.rng_seed = self.rng_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.rng_seed
    }

    fn spawn_food(&mut self) -> Pos {
        loop {
            let x = (self.lcg_next() % (WIDTH as u64 - 2) + 1) as u16;
            let y = (self.lcg_next() % (HEIGHT as u64 - 2) + 1) as u16;
            let candidate = Pos { x, y };
            if !self.snake.contains(&candidate) {
                return candidate;
            }
        }
    }

    fn set_dir(&mut self, d: Dir) {
        if d != self.dir.opposite() {
            self.next_dir = d;
        }
    }

    fn step(&mut self) {
        self.dir = self.next_dir;
        let head = *self.snake.front().unwrap();
        let new_head = match self.dir {
            Dir::Up => Pos { x: head.x, y: head.y.wrapping_sub(1) },
            Dir::Down => Pos { x: head.x, y: head.y + 1 },
            Dir::Left => Pos { x: head.x.wrapping_sub(1), y: head.y },
            Dir::Right => Pos { x: head.x + 1, y: head.y },
        };

        if new_head.x == 0
            || new_head.x >= WIDTH - 1
            || new_head.y == 0
            || new_head.y >= HEIGHT - 1
            || self.snake.contains(&new_head)
        {
            self.alive = false;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 10;
            self.food = self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }
}

fn draw(stdout: &mut io::Stdout, game: &Game) -> io::Result<()> {
    queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

    queue!(stdout, SetForegroundColor(Color::Cyan), Print("╔"))?;
    for _ in 1..WIDTH - 1 {
        queue!(stdout, Print("═"))?;
    }
    queue!(stdout, Print("╗\r\n"))?;

    for row in 1..HEIGHT - 1 {
        queue!(stdout, SetForegroundColor(Color::Cyan), Print("║"))?;
        for col in 1..WIDTH - 1 {
            let p = Pos { x: col, y: row };
            if game.snake.front() == Some(&p) {
                queue!(stdout, SetForegroundColor(Color::Green), Print("◉"))?;
            } else if game.snake.contains(&p) {
                queue!(stdout, SetForegroundColor(Color::Green), Print("█"))?;
            } else if p == game.food {
                queue!(stdout, SetForegroundColor(Color::Red), Print("●"))?;
            } else {
                queue!(stdout, SetForegroundColor(Color::DarkGrey), Print("·"))?;
            }
        }
        queue!(stdout, SetForegroundColor(Color::Cyan), Print("║\r\n"))?;
    }

    queue!(stdout, SetForegroundColor(Color::Cyan), Print("╚"))?;
    for _ in 1..WIDTH - 1 {
        queue!(stdout, Print("═"))?;
    }
    queue!(stdout, Print("╝\r\n"))?;

    queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!(
            " Score: {:>4}   Length: {:>3}   WASD / Arrow keys   Q to quit\r\n",
            game.score,
            game.snake.len()
        ))
    )?;

    stdout.flush()?;
    Ok(())
}

fn draw_game_over(stdout: &mut io::Stdout, score: u32) -> io::Result<()> {
    let banner = [
        "  ██████╗  █████╗ ███╗   ███╗███████╗     ██████╗ ██╗   ██╗███████╗██████╗  ",
        " ██╔════╝ ██╔══██╗████╗ ████║██╔════╝    ██╔═══██╗██║   ██║██╔════╝██╔══██╗ ",
        " ██║  ███╗███████║██╔████╔██║█████╗      ██║   ██║██║   ██║█████╗  ██████╔╝ ",
        " ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝      ██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗ ",
        " ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗    ╚██████╔╝ ╚████╔╝ ███████╗██║  ██║ ",
        "  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝     ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝ ",
    ];

    queue!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 3))?;
    for line in &banner {
        queue!(stdout, SetForegroundColor(Color::Red), Print(format!("{}\r\n", line)))?;
    }
    queue!(
        stdout,
        cursor::MoveTo(16, 11),
        SetForegroundColor(Color::Yellow),
        Print(format!(
            "Final Score: {}   Press R to restart or Q to quit",
            score
        ))
    )?;
    stdout.flush()?;
    Ok(())
}

fn run_game() -> io::Result<bool> {
    let mut stdout = io::stdout();
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All))?;

    loop {
        let elapsed = last_tick.elapsed();
        let tick_dur = Duration::from_millis(TICK_MS);

        if event::poll(tick_dur.saturating_sub(elapsed))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        terminal::disable_raw_mode()?;
                        execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                        return Ok(false);
                    }
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        game.set_dir(Dir::Up)
                    }
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        game.set_dir(Dir::Down)
                    }
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                        game.set_dir(Dir::Left)
                    }
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                        game.set_dir(Dir::Right)
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_dur {
            last_tick = Instant::now();
            game.step();

            if !game.alive {
                draw_game_over(&mut stdout, game.score)?;
                loop {
                    if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                        match code {
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                terminal::disable_raw_mode()?;
                                execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                                return Ok(true);
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                terminal::disable_raw_mode()?;
                                execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                                return Ok(false);
                            }
                            _ => {}
                        }
                    }
                }
            }

            draw(&mut stdout, &game)?;
        }
    }
}

fn main() {
    loop {
        match run_game() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
