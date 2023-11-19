use std::{io::stdout, thread, time::Duration};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    event::{read, Event, poll},
    ExecutableCommand, QueueableCommand, cursor::MoveTo,
};


fn main() -> std::io::Result<()> {
    // using the macro
    let mut stdout = stdout();
    let (mut width, mut height) = crossterm::terminal::size()?;
    let mut bar = "-".repeat(width as usize);
    let label_1 = "Styled text here 1";

    loop {
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => println!("{:?}", event),
                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                    bar = "-".repeat(w as usize);
                    println!("New size {}x{}", width, height);
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available

        stdout.queue(Clear(ClearType::All))?;
        execute!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, height-2),
            Print(&bar),
            ResetColor
        )?;
        }
    }

    // stdout.queue(Clear(ClearType::All))?;
    //
    // execute!(
    //     stdout,
    //     SetForegroundColor(Color::White),
    //     SetBackgroundColor(Color::DarkGrey),
    //     MoveTo(width/2 - label_1.len() as u16/2, height/2),
    //     Print(label_1),
    //     ResetColor
    // )?;
    //
    // println!();
    // // or using functions
    // stdout
    //     .execute(SetForegroundColor(Color::Blue))?
    //     .execute(SetBackgroundColor(Color::Red))?
    //     .execute(Print("Styled text here 2"))?
    //     .execute(ResetColor)?;
    //
    // thread::sleep(Duration::from_secs(3));
}
