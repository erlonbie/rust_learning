use crossterm::event::KeyModifiers;
use crossterm::terminal;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};
use std::{io::stdout, thread, time::Duration};

#[derive(Debug, Default)]
struct Point(u16, u16);

#[derive(Debug, Default)]
struct DrawBox {
    upper_left: Point,
    upper_right: Point,
    lower_left: Point,
    lower_right: Point,
}

impl DrawBox {
    fn new(upper_left: Point, upper_right: Point, lower_left: Point, lower_right: Point) -> Self {
        Self {
            upper_left,
            upper_right,
            lower_left,
            lower_right,
        }
    }
}

fn draw_boxes(
    bar: &str,
    stdout: &mut Stdout,
    prompt: &str,
    chat: &[String],
    prompt_drawbox: &DrawBox,
    chat_drawbox: &DrawBox,
) {
    for (i, line) in chat.iter().enumerate() {
        let _ = execute!(stdout, MoveTo(2, i as u16 + 1), Print(line));
    }

    let _ = execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        MoveTo(chat_drawbox.upper_left.0, chat_drawbox.upper_right.1 - 1),
        Print("┏"),
        MoveTo(
            chat_drawbox.upper_left.0 + 1,
            chat_drawbox.upper_right.1 - 1
        ),
        Print(&bar),
        Print("┓"),
    );

    let distance = chat_drawbox.lower_left.1 - chat_drawbox.upper_left.1;
    for i in 0..distance {
        let _ = execute!(
            stdout,
            MoveTo(chat_drawbox.upper_left.0, chat_drawbox.upper_left.1 + i),
            Print("┃"),
            MoveTo(chat_drawbox.upper_right.0, chat_drawbox.upper_right.1 + i),
            Print("┃"),
        );
    }

    let _ = execute!(
        stdout,
        MoveTo(chat_drawbox.lower_left.0, chat_drawbox.lower_left.1),
        Print("┗"),
        MoveTo(chat_drawbox.lower_left.0 + 1, chat_drawbox.lower_left.1),
        Print(&bar),
        Print("┛"),
    );

    let _ = execute!(
        stdout,
        SetForegroundColor(Color::Green),
        MoveTo(prompt_drawbox.upper_left.0, prompt_drawbox.upper_left.1),
        Print("┏"),
        MoveTo(prompt_drawbox.upper_left.0 + 1, prompt_drawbox.upper_left.1),
        Print(&bar),
        Print("┓"),
        MoveTo(prompt_drawbox.upper_left.0, prompt_drawbox.lower_left.1 - 1),
        Print("┃"),
        // MoveTo(width - 1, height - 2),
        MoveTo(
            prompt_drawbox.lower_right.0,
            prompt_drawbox.lower_left.1 - 1
        ),
        Print("┃"),
        // MoveTo(0, height - 1),
        MoveTo(prompt_drawbox.lower_left.0, prompt_drawbox.lower_left.1),
        Print("┗"),
        // MoveTo(1, height - 1),
        MoveTo(prompt_drawbox.lower_left.0 + 1, prompt_drawbox.lower_left.1),
        Print(&bar),
        Print("┛"),
        // MoveTo(1, height - 2),
        MoveTo(
            prompt_drawbox.lower_left.0 + 1,
            prompt_drawbox.lower_left.1 - 1
        ),
        ResetColor,
        Print(&prompt),
    )
    .map_err(|err| {
        eprintln!("draw_boxes ERROR: {}", err);
    });
}

fn main() -> std::io::Result<()> {
    // using the macro
    let mut stdout = stdout();
    let (mut width, mut height) = crossterm::terminal::size()?;
    let mut bar = "━".repeat(width as usize - 2);
    let mut prompt = String::new();
    let mut prompt_drawbox = DrawBox::new(
        Point(width.wrapping_sub(width), height - 3),
        Point(width, height.wrapping_sub(height) + 3),
        Point(width.wrapping_sub(width), height - 1),
        Point(width, height.wrapping_sub(height) + 1),
    );
    let mut chat = Vec::new();
    let mut chat_drawbox = DrawBox::new(
        Point(width.wrapping_sub(width), height.wrapping_sub(height) + 1),
        Point(width, height.wrapping_sub(height) + 1),
        Point(width.wrapping_sub(width), prompt_drawbox.upper_left.1 - 1),
        Point(width, height.wrapping_sub(prompt_drawbox.upper_right.1) + 1),
    );
    terminal::enable_raw_mode().unwrap();

    loop {
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => match event.code {
                    crossterm::event::KeyCode::Enter => match prompt.len() {
                        0 => {}
                        _ => {
                            chat.push(prompt.clone());
                            prompt.clear();
                        }
                    },
                    crossterm::event::KeyCode::Char(c) => {
                        if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
                        } else {
                            prompt.push(c);
                        }
                    }
                    crossterm::event::KeyCode::Backspace => {
                        prompt.pop();
                    }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                    prompt_drawbox = DrawBox::new(
                        Point(w.wrapping_sub(w), h - 3),
                        Point(w, h.wrapping_sub(h) + 3),
                        Point(w.wrapping_sub(w), h - 1),
                        Point(w, h.wrapping_sub(h) + 1),
                    );
                    chat_drawbox = DrawBox::new(
                        Point(w.wrapping_sub(w), h.wrapping_sub(h) + 1),
                        Point(w, h.wrapping_sub(h) + 1),
                        Point(w.wrapping_sub(w), prompt_drawbox.upper_left.1 - 1),
                        Point(w, h.wrapping_sub(prompt_drawbox.upper_right.1) + 1),
                    );
                    bar = "━".repeat(w as usize - 2);
                    println!("New size {}x{}", width, height);
                }
                _ => {}
            }
        }
        stdout.queue(Clear(ClearType::All))?;

        chat_window(&mut stdout, &chat, Rec {
            x: 1,
            y: 1,
            width,
            height: height - 5,
        } );

        draw_boxes(&bar, &mut stdout, &prompt, &chat, &prompt_drawbox, &chat_drawbox);

        thread::sleep(Duration::from_millis(33));
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

struct Rec {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

fn chat_window(stdout: &mut impl Write, chat: &[String], drawbox: Rec) {
    let n = chat.len();
    let m = n.checked_sub(drawbox.height as usize).unwrap_or(0);
    for (y, line) in chat.iter().skip(m).enumerate() {
        stdout.queue(MoveTo(drawbox.x, drawbox.y + y as u16)).unwrap();
        // stdout.queue(Print(line)).unwrap();
        stdout.write_all(" ".as_bytes()).unwrap();
    }
}
