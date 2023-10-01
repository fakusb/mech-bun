use crate::data::{
    grid::{Direction, GroundTile, TileItem, LEVEL_WIDTH},
    level_state::TileContent,
    LevelState,
};

use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    self,
    cursor::{self, MoveTo, MoveToNextLine, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, ContentStyle, Print, PrintStyledContent, SetStyle, StyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, SetSize,
    },
    Command, ExecutableCommand, QueueableCommand,
};

const DRAW_HEIGHT: u16 = crate::data::grid::LEVEL_HEIGHT as u16 + 3;
const DRAW_WIDTH: u16 = LEVEL_WIDTH as u16 + 2;

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

    while !abort {
        let frame_start = Instant::now();
        let next_frame = frame_start + delta_t;
        let mut applied_input = false;
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

        queue_print_level(&mut stdout, state)?
            .queue(cursor::Hide)?
            //.queue(PrintStyledContent(String::from("HAHA").dark_blue()))?
            .queue(MoveToNextLine(1))?
            .queue(cursor::Hide)?
            .flush()?;
        // print!("{}", ansi_escapes::EraseLines());
        // println!("{}", state.to_unicode_string());
        println!("{tick}");
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

    io::stdout()
        .queue(SetSize(cols, rows))?
        .flush()?;
    disable_raw_mode()?;
    Ok(())
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();

    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::Up | KeyCode::Char('w')=> Ok(Direction::Up),
            KeyCode::Down | KeyCode::Char('s') => Ok(Direction::Down),
            KeyCode::Left | KeyCode::Char('a') => Ok(Direction::Left),
            KeyCode::Right | KeyCode::Char('d') => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

pub fn queue_print_level<'a, W>(
    mut out: &'a mut W,
    level: &LevelState,
) -> io::Result<&'a mut W>
where
    W: QueueableCommand,
{
    let mut flipflop = false;
    let (col, row) = cursor::position()?;
    for (p, t) in level.content() {
        if p.x() == 0 {
            out = out.queue(cursor::MoveTo(col, row + p.y() as u16))?;
        }
        out = queue_tile(out, t)?;
    }
    Ok(out)
}

pub fn queue_tile<W>(out: &mut W, (tile, item): TileContent) -> io::Result<&mut W>
where
    W: QueueableCommand,
{
    match item {
        None => match tile {
            GroundTile::Hole => out.queue(Print("🕳  ")),
            GroundTile::Wall { breakable:true,.. } => out.queue(Print("▒▒▒")),
            GroundTile::Wall { breakable:false,.. } => out.queue(Print("▓▓▓")),
            GroundTile::Floor {isEntry:false} => out.queue(Print("   ")),
            GroundTile::Floor {isEntry:true} => out.queue(Print(" ꜛ ")),
        },
        Some(TileItem::Paquerette) => out.queue(Print("👧 ")),
        Some(TileItem::Bun) => out.queue(Print("🐰 ")),
        Some(TileItem::Bunstack) => out.queue(Print("🗼  ")),
    }
}
