use crate::data::{
    grid::{Direction, LEVEL_WIDTH},
    LevelState,
};

use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    self,
    cursor::{self, MoveTo, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, SetSize,
    },
    ExecutableCommand, QueueableCommand,
};

const DRAW_HEIGHT: u16 = crate::data::grid::LEVEL_HEIGHT as u16 + 3;
const DRAW_WIDTH: u16 = LEVEL_WIDTH as u16 + 2;

pub(crate) fn run_level(state: &mut impl LevelState) -> io::Result<()> {
    let (cols, rows) = size()?;
    enable_raw_mode()?;
    //thread::sleep(Duration::from_secs(2));
    let mut tick: usize = 0;
    let delta_t = Duration::from_millis(100);
    let mut stdout = stdout();
    let mut abort: bool = false;

    stdout
        //.queue(EnterAlternateScreen)?
        .queue(cursor::DisableBlinking)?
        .queue(cursor::Hide)?
        .queue(Clear(ClearType::All))?
        //.queue(SetSize(DRAW_WIDTH, DRAW_HEIGHT))?
        .flush()?;

    stdout.execute(Clear(ClearType::All))?;

    while !abort {
        let frame_start = Instant::now();
        let next_frame = frame_start + delta_t;
        let mut applied_input = false;
        stdout
            //.queue(Clear(ClearType::All))?
            .queue(cursor::Hide)?.flush()?;
        stdout.execute(Clear(ClearType::All))?;

        stdout
            .queue(MoveTo(0, 0))?
            .queue(PrintStyledContent(state.to_unicode_string().white()))?
            .queue(cursor::Hide)?.flush()?;
        stdout.queue(MoveToNextLine(1))?
            .queue(PrintStyledContent(String::new().dark_blue()))?
            .queue(MoveToNextLine(1))?
            .queue(cursor::Hide)?
            .flush()?;
        // print!("{}", ansi_escapes::EraseLines());
        // println!("{}", state.to_unicode_string());
        // println!("{tick}");
        tick = tick + 1;

        loop {
            if abort || !event::poll(next_frame.saturating_duration_since(Instant::now()))? {
                break;
            }
            let e = event::read()?;
            //eprint!("{e:?}\n");
            match e {
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => abort = true,
                Event::Key(KeyEvent {
                    code: c,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if c == KeyCode::Char('c') {
                        abort = true;
                    } else if let Ok::<Direction, _>(dir) = c.try_into() {
                        if applied_input {
                            continue;
                        }
                        if state.move_to(dir).is_ok() {
                            applied_input = true;
                        }
                    }
                }
                _ => (),
            }
        }
    }
    //todo!("Hardcode first level, to main game loop, and test printing of level. Then implement game transition logic.");
    //println!("Hello, world!");

    io::stdout()
        .queue(SetSize(cols, rows))?
        .queue(Clear(ClearType::All))?
        .flush()?;
    disable_raw_mode()?;
    Ok(())
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();

    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::Up => Ok(Direction::Up),
            KeyCode::Down => Ok(Direction::Down),
            KeyCode::Left => Ok(Direction::Left),
            KeyCode::Right => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
