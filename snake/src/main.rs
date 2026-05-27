use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::{execute, QueueableCommand};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;
use rand::Rng;

// ─── Data Structures ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }
}

struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    fn new(start: Point, direction: Direction) -> Self {
        let mut body = VecDeque::new();
        // Snake length 3: head, then body segments behind
        body.push_back(start);
        body.push_back(Point::new(start.x - 1, start.y));
        body.push_back(Point::new(start.x - 2, start.y));
        Snake { body, direction }
    }

    fn head(&self) -> Point {
        *self.body.front().unwrap()
    }

    fn move_forward(&mut self) -> Point {
        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => Point::new(head.x, head.y - 1),
            Direction::Down => Point::new(head.x, head.y + 1),
            Direction::Left => Point::new(head.x - 1, head.y),
            Direction::Right => Point::new(head.x + 1, head.y),
        };
        self.body.push_front(new_head);
        new_head
    }

    fn pop_tail(&mut self) -> Option<Point> {
        self.body.pop_back()
    }

    fn reverse_direction(&self) -> Option<Direction> {
        match self.direction {
            Direction::Up => Some(Direction::Down),
            Direction::Down => Some(Direction::Up),
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::Left),
        }
    }
}

struct Game {
    snake: Snake,
    food: Point,
    width: i16,
    height: i16,
    score: u32,
    game_over: bool,
}

impl Game {
    fn new(width: i16, height: i16) -> Self {
        let center = Point::new(width / 2, height / 2);
        let snake = Snake::new(center, Direction::Right);
        let food = Self::spawn_food(width, height, &snake);
        Game {
            snake,
            food,
            width,
            height,
            score: 0,
            game_over: false,
        }
    }

    fn spawn_food(width: i16, height: i16, snake: &Snake) -> Point {
        loop {
            // rand::Rng is implemented for ThreadRng
            let x = rand::thread_rng().gen_range(1..=width);
            let y = rand::thread_rng().gen_range(1..=height);
            let p = Point::new(x, y);
            if !snake.body.contains(&p) {
                return p;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let _new_head = self.snake.move_forward();

        // Check wall collision (grid is 1..=width, 1..=height for playable area)
        let head = self.snake.head();
        if head.x < 1 || head.x > self.width || head.y < 1 || head.y > self.height {
            self.game_over = true;
            return;
        }

        // Check self collision (exclude tail which will move)
        let body_vec: Vec<&Point> = self.snake.body.iter().skip(1).collect();
        if body_vec.contains(&&head) {
            self.game_over = true;
            return;
        }

        // Check food
        if head == self.food {
            self.score += 1;
            self.food = Self::spawn_food(self.width, self.height, &self.snake);
            // Don't pop tail — snake grows
        } else {
            self.snake.pop_tail();
        }
    }

    fn set_direction(&mut self, dir: Direction) {
        if Some(dir) != self.snake.reverse_direction() {
            self.snake.direction = dir;
        }
    }
}

// ─── Rendering ──────────────────────────────────────────────────────────────

fn render(game: &Game) -> std::io::Result<()> {
    let mut out = stdout();
    out.queue(Clear(ClearType::All))?;
    out.queue(MoveTo(0, 0))?;

    let width = game.width;
    let height = game.height;

    // Top border
    print!("{}", "┏".to_string() + &"━".repeat(width as usize) + "┓\r\n");

    // Rows
    for y in 1..=height {
        print!("┃");
        for x in 1..=width {
            let p = Point::new(x, y);
            if p == game.snake.head() {
                print!("■");
            } else if game.snake.body.contains(&p) {
                print!("□");
            } else if p == game.food {
                print!("●");
            } else {
                print!(" ");
            }
        }
        println!("┃");
    }

    // Bottom border
    print!("{}", "┗".to_string() + &"━".repeat(width as usize) + "┛\r\n");
    print!("Score: {}\r\n", game.score);

    if game.game_over {
        println!("GAME OVER! Press q to quit.");
    }

    out.flush()?;
    Ok(())
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() -> std::io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    // Game state
    let mut game = Game::new(20, 20);

    // Initial render
    render(&game)?;

    loop {
        // Non-blocking poll for input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break;
                    }
                    KeyCode::Up => game.set_direction(Direction::Up),
                    KeyCode::Down => game.set_direction(Direction::Down),
                    KeyCode::Left => game.set_direction(Direction::Left),
                    KeyCode::Right => game.set_direction(Direction::Right),
                    _ => {}
                }
            }
        }

        if !game.game_over {
            game.update();
        }

        render(&game)?;

        if game.game_over {
            // Wait for quit key
            loop {
                if event::poll(Duration::from_secs(1))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        if let KeyCode::Char('q') | KeyCode::Char('Q') = key.code {
                            break;
                        }
                    }
                }
            }
            break;
        }
    }

    // Cleanup
    execute!(stdout(), Show)?;
    disable_raw_mode()?;
    Ok(())
}