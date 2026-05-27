use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind},
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::time::Duration;

const BALL: &str = "●";
const TRAIL: &str = "○";
const HORIZONTAL: &str = "━";
const VERTICAL: &str = "┃";
const TOP_LEFT: &str = "┏";
const BOTTOM_LEFT: &str = "┗";

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            dx: 1.0,
            dy: 1.0,
        }
    }

    fn update(&mut self, width: u16, height: u16) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x <= 1.0 {
            self.x = 1.0;
            self.dx = self.dx.abs();
        }
        if self.x >= (width - 2) as f32 {
            self.x = (width - 2) as f32;
            self.dx = -self.dx.abs();
        }
        if self.y <= 1.0 {
            self.y = 1.0;
            self.dy = self.dy.abs();
        }
        if self.y >= (height - 2) as f32 {
            self.y = (height - 2) as f32;
            self.dy = -self.dy.abs();
        }
    }

    fn trail_positions(&self) -> Vec<(u16, u16)> {
        let mut positions = Vec::new();
        for i in 1..=3 {
            let tx = (self.x - self.dx * i as f32).round() as u16;
            let ty = (self.y - self.dy * i as f32).round() as u16;
            if tx > 0 && ty > 0 {
                positions.push((tx, ty));
            }
        }
        positions
    }
}

fn draw_frame(width: u16, height: u16) {
    let mut out = stdout();
    let w = width as usize;
    let h = height as usize;

    // Top border
    let _ = out.execute(Print(format!(
        "{}{}",
        TOP_LEFT,
        HORIZONTAL.repeat(w.saturating_sub(2))
    )));

    // Middle rows
    for _ in 1..h.saturating_sub(1) {
        let _ = out.execute(Print(VERTICAL.to_string()));
        let _ = out.execute(Clear(ClearType::UntilNewLine));
        let _ = out.execute(Print(VERTICAL.to_string()));
    }

    // Bottom border
    if h > 1 {
        let _ = out.execute(Print(format!(
            "{}{}",
            BOTTOM_LEFT,
            HORIZONTAL.repeat(w.saturating_sub(2))
        )));
    }
}

fn clear_area() {
    let mut out = stdout();
    let _ = out.execute(Clear(ClearType::All));
}

fn main() -> std::io::Result<()> {
    let mut out = stdout();

    // Enter alternate screen and hide cursor
    out.execute(EnterAlternateScreen)?;
    out.execute(Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    // Cleanup on drop
    struct Cleanup;
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let mut out = stdout();
            let _ = crossterm::terminal::disable_raw_mode();
            let _ = out.execute(Show);
            let _ = out.execute(LeaveAlternateScreen);
            let _ = out.flush();
        }
    }
    let _cleanup = Cleanup;

    let mut ball = Ball::new(5.0, 5.0);

    loop {
        let (width, height) = size().unwrap_or((40, 20));

        if width < 10 || height < 10 {
            let mut out = stdout();
            let _ = out.execute(Clear(ClearType::All));
            let msg = "Terminal too small (min 10x10). Resize or press q to quit.";
            let _ = out.execute(Print(msg));
            let _ = out.flush();

            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                            break;
                        }
                    }
                }
            }
            continue;
        }

        clear_area();

        // Draw frame
        draw_frame(width, height);

        // Draw trail
        for (tx, ty) in ball.trail_positions() {
            if tx > 0 && ty > 0 && tx < width - 1 && ty < height - 1 {
                let mut out = stdout();
                let _ = out.execute(crossterm::cursor::MoveTo(tx, ty));
                let _ = out.execute(Print(TRAIL));
            }
        }

        // Draw ball
        let bx = ball.x.round() as u16;
        let by = ball.y.round() as u16;
        let mut out = stdout();
        let _ = out.execute(crossterm::cursor::MoveTo(bx, by));
        let _ = out.execute(Print(BALL));
        let _ = out.flush();

        // Update ball
        ball.update(width, height);

        // Check for quit
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}