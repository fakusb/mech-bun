use crate::data::{
    grid::{Direction, GroundTile, TileItem},
    level_state::TileContent,
    LevelState, world::WorldState,
};

use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant}, slice::{Iter, IterMut}, vec::IntoIter,
};

use crossterm::{
    cursor::{self, MoveLeft, MoveTo, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    ExecutableCommand, QueueableCommand,
};

//const DRAW_HEIGHT: u16 = crate::data::grid::LEVEL_HEIGHT as u16 + 3;
//const DRAW_WIDTH: u16 = LEVEL_WIDTH as u16 + 2;

pub(crate) fn run_level(state: &mut LevelState) -> io::Result<()> {
    let (cols, rows) = size()?;
    enable_raw_mode()?;
    //thread::sleep(Duration::from_secs(2));
    let mut tick: usize = 0;
    let delta_t = Duration::from_millis(50);
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

    let mut display_queue: std::iter::Peekable<IntoIter<LevelState>> = Vec::new().into_iter().peekable();
    let duration_step = Duration::from_millis(100);
    let mut next_animation_step = Instant::now();

    while !abort {
        let mut applied_input = false;
        let frame_start = Instant::now();
        let next_frame = frame_start + delta_t;
        if display_queue.peek().is_some() && frame_start > next_animation_step {
            display_queue.next();
            next_animation_step = frame_start + duration_step;
        }
        // stdout
        //     //.queue(Clear(ClearType::All))?
        //     .queue(cursor::Hide)?.flush()?;
        // stdout.execute(Clear(ClearType::All))?;

        stdout
            .queue(MoveTo(1, 1))?
            // .queue(SetStyle(
            //     ContentStyle::new().with(Color::White).on(Color::Black),
            // ))?
            .flush()?;

        queue_print_level(&mut stdout, display_queue.peek().unwrap_or(state))?
            .queue(cursor::Hide)?
            //.queue(PrintStyledContent(String::from("HAHA").dark_blue()))?
            .queue(MoveToNextLine(1))?
            .queue(cursor::Hide)?
            .flush()?;
        // print!("{}", ansi_escapes::EraseLines());
        // println!("{}", state.to_unicode_string());
        println!("{tick}");
        tick += 1;

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
                    if let Ok::<Direction, _>(dir) = c.try_into() {
                        if applied_input {
                            continue;
                        }
                        if let Ok(res) = state.move_to(dir) {
                            display_queue = res.into_iter().peekable();
                            next_animation_step = Instant::now() + duration_step;
                            applied_input = true
                        };
                    }
                }
                _ => (),
            }
        }
    }

    io::stdout().queue(SetSize(cols, rows))?.flush()?;
    disable_raw_mode()?;
    Ok(())
}


pub(crate) fn run_world(state: &mut WorldState) -> io::Result<()> {
    run_level(&mut state.level_state)
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();

    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::Up | KeyCode::Char('w') => Ok(Direction::Up),
            KeyCode::Down | KeyCode::Char('s') => Ok(Direction::Down),
            KeyCode::Left | KeyCode::Char('a') => Ok(Direction::Left),
            KeyCode::Right | KeyCode::Char('d') => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

pub fn queue_print_level<'a, W>(mut out: &'a mut W, level: &LevelState) -> io::Result<&'a mut W>
where
    W: QueueableCommand,
{
    //let flipflop = false;
    let (col, row) = cursor::position()?;
    let mut highlight = false;
    for (p, t) in level.content() {
        if p.x() == 0 {
            out = out.queue(cursor::MoveTo(col, row + p.y() as u16))?;
            highlight = p.y() % 2 == 0;
        }
        out = queue_tile(out, t, highlight)?;
        highlight ^= true;
    }
    Ok(out)
}

pub fn queue_tile<W>(out: &mut W, (tile, item): TileContent, highlight: bool) -> io::Result<&mut W>
where
    W: QueueableCommand,
{
    out.queue(SetBackgroundColor(if highlight {
        Color::Rgb {
            r: 32,
            g: 32,
            b: 32,
        }
    } else {
        Color::Black
    }))?;
    //out.queue(SetForegroundColor(if highlight {Color::Rgb { r: 245, g: 245, b: 245 }} else {Color::White}))?;
    match item {
        None => match tile {
            GroundTile::Hole => out.queue(Print("🕳  "))?.queue(MoveLeft(1)),
            GroundTile::Wall {
                breakable: true, ..
            } => out.queue(Print("░░░")),
            GroundTile::Wall {
                breakable: false, ..
            } => out.queue(Print("▓▓▓")),
            GroundTile::Floor { is_entry: false } => out.queue(Print("   ")),
            GroundTile::Floor { is_entry: true } => out.queue(Print(" ꜛ ")),
        },
        Some(TileItem::Paquerette) => out.queue(Print("👧 ")),
        Some(TileItem::Bun) => out.queue(Print("🐰 ")),
        Some(TileItem::Bunstack) => out.queue(Print("🗼  ")),
    }
}
