use font8x8::legacy::BASIC_LEGACY;
use minifb::{Key, Window, WindowOptions};
use std::io::Read;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::sync::mpsc::channel;
use std::thread;

const WIDTH: usize = 1600;
const HEIGHT: usize = 896;

fn write_char_to_buffer(chr: Option<char>, buffer: &mut [char], i: &mut usize) {
    if chr.is_none() {
        return;
    }
    if *i >= (WIDTH / 16) * (HEIGHT / 16) {
        buffer.rotate_left(WIDTH / 16);
        for e in buffer
            .iter_mut()
            .take(WIDTH / 16 * HEIGHT / 16)
            .skip(WIDTH / 16 * (HEIGHT / 16 - 1))
        {
            *e = ' ';
        }
        *i = WIDTH / 16 * (HEIGHT / 16 - 1);
        buffer[*i] = chr.unwrap();
        buffer[*i + 1] = '_'; // cursor handling
        *i += 1;
    } else {
        buffer[*i] = chr.unwrap();
        if *i < ((WIDTH / 16) * (HEIGHT / 16)) - 1 {
            buffer[*i + 1] = '_'; // cursor handling
        } else {
            buffer[0] = '_';
        }
        *i += 1;
    }
}

fn get_key_char(key: &Key) -> Option<char> {
    match key {
        Key::Key0 => Some('0'),
        Key::Key1 => Some('1'),
        Key::Key2 => Some('2'),
        Key::Key3 => Some('3'),
        Key::Key4 => Some('4'),
        Key::Key5 => Some('5'),
        Key::Key6 => Some('6'),
        Key::Key7 => Some('7'),
        Key::Key8 => Some('8'),
        Key::Key9 => Some('9'),
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::Apostrophe => Some('\''),
        Key::Backquote => Some('`'),
        Key::Slash => Some('/'),
        Key::Backslash => Some('\\'),
        Key::Comma => Some(','),
        Key::Equal => Some('='),
        Key::LeftBracket => Some('['),
        Key::Minus => Some('-'),
        Key::Period => Some('.'),
        Key::RightBracket => Some(']'),
        Key::Semicolon => Some(';'),
        Key::Space => Some(' '),
        Key::NumPad0 => Some('0'),
        Key::NumPad1 => Some('1'),
        Key::NumPad2 => Some('2'),
        Key::NumPad3 => Some('3'),
        Key::NumPad4 => Some('4'),
        Key::NumPad5 => Some('5'),
        Key::NumPad6 => Some('6'),
        Key::NumPad7 => Some('7'),
        Key::NumPad8 => Some('8'),
        Key::NumPad9 => Some('9'),
        Key::NumPadDot => Some('.'),
        Key::NumPadSlash => Some('/'),
        Key::NumPadAsterisk => Some('*'),
        Key::NumPadMinus => Some('-'),
        Key::NumPadPlus => Some('+'),
        _ => None,
    }
}

fn get_key_char_shift(key: &Key) -> Option<char> {
    match key {
        Key::A => Some('A'),
        Key::B => Some('B'),
        Key::C => Some('C'),
        Key::D => Some('D'),
        Key::E => Some('E'),
        Key::F => Some('F'),
        Key::G => Some('G'),
        Key::H => Some('H'),
        Key::I => Some('I'),
        Key::J => Some('J'),
        Key::K => Some('K'),
        Key::L => Some('L'),
        Key::M => Some('M'),
        Key::N => Some('N'),
        Key::O => Some('O'),
        Key::P => Some('P'),
        Key::Q => Some('Q'),
        Key::R => Some('R'),
        Key::S => Some('S'),
        Key::T => Some('T'),
        Key::U => Some('U'),
        Key::V => Some('V'),
        Key::W => Some('W'),
        Key::X => Some('X'),
        Key::Y => Some('Y'),
        Key::Z => Some('Z'),
        Key::Key0 => Some(')'),
        Key::Key1 => Some('!'),
        Key::Key2 => Some('@'),
        Key::Key3 => Some('#'),
        Key::Key4 => Some('$'),
        Key::Key5 => Some('%'),
        Key::Key6 => Some('^'),
        Key::Key7 => Some('&'),
        Key::Key8 => Some('*'),
        Key::Key9 => Some('('),
        Key::Minus => Some('_'),
        Key::Equal => Some('+'),
        Key::LeftBracket => Some('{'),
        Key::RightBracket => Some('}'),
        Key::Backslash => Some('|'),
        Key::Semicolon => Some(':'),
        Key::Apostrophe => Some('"'),
        Key::Backquote => Some('~'),
        Key::Comma => Some('<'),
        Key::Period => Some('>'),
        Key::Slash => Some('?'),
        _ => None,
    }
}

fn render_text_buffer(buffer: &[char]) -> Vec<u32> {
    let mut out = Vec::new();
    for row in 0..HEIGHT / 16 {
        for line in 0..16 {
            for chr in &buffer[(row * WIDTH / 16)..((row * WIDTH / 16) + WIDTH / 16)] {
                for bit in 0..8 {
                    if *chr as usize >= 128 {
                        for _ in 0..2 {
                            out.push(0x282828);
                        }
                        continue;
                    }
                    match BASIC_LEGACY[*chr as usize][line / 2] & (1 << bit) {
                        0 => {
                            for _ in 0..2 {
                                out.push(0x282828) // gruvbox like
                            }
                        }
                        _ => {
                            for _ in 0..2 {
                                out.push(0xe5e5e5) // gruvbox like
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("EG-Term", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_key_repeat_delay(0.5);
    window.set_key_repeat_rate(0.05);

    // 6250 - 160 fps, 13330 - 75 fps, 16600 - 60fps, 33300 - 30fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(6250)));

    // starting the shell process
    let mut shell = Command::new("/bin/bash")
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("ERROR starting the shell process");
    let mut shell_stdin = shell.stdin.take().unwrap_or_else(|| {
        eprintln!("ERROR grabbing stdin of the shell process");
        exit(1);
    });
    let (queue_tx, queue_rx) = channel::<String>();
    thread::spawn(move || {
        let mut shell_stdout = shell.stdout.take().unwrap_or_else(|| {
            eprintln!("ERROR grabbing stdout of the shell process");
            exit(1);
        });
        loop {
            let mut buf = [0; 65536];
            let r = shell_stdout.read(&mut buf).unwrap_or_else(|e| {
                eprintln!("ERROR reading from shell stdout: {e}");
                exit(1);
            });
            let str = std::str::from_utf8(&buf[..r])
                .unwrap_or_else(|e| {
                    eprintln!("ERROR converting stdout to utf-8: {e}");
                    exit(1);
                })
                .to_string();
            queue_tx.send(str.clone()).unwrap_or_else(|_| exit(0));
        }
    });

    let mut text_buffer = [' '; WIDTH / 16 * HEIGHT / 16];
    let mut text_buffer_pointer: usize = 0;
    let mut user_typed_line = String::new();
    let mut shift_state = false;

    fn redraw_window(win: &mut Window, buf: &mut [u32], txt_buf: &mut [char]) {
        // dbg!("REDRAW");
        let rendered_text_buffer = render_text_buffer(txt_buf);
        for (i, pixel) in buf.iter_mut().enumerate() {
            *pixel = rendered_text_buffer[i];
        }
        win.update_with_buffer(buf, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                eprintln!("ERROR updating the window with the buffer: {e}");
                exit(1);
            });
    }

    // initial redraw with just cursor
    write_char_to_buffer(Some('>'), &mut text_buffer, &mut text_buffer_pointer);
    write_char_to_buffer(Some(' '), &mut text_buffer, &mut text_buffer_pointer);
    text_buffer[2] = '_';
    redraw_window(&mut window, &mut buffer, &mut text_buffer);

    while window.is_open() {
        if shift_state
            && !window.is_key_down(Key::LeftShift)
            && !window.is_key_down(Key::RightShift)
        {
            shift_state = false;
        }

        window
            .get_keys_pressed(minifb::KeyRepeat::Yes)
            .iter()
            .for_each(|key| {
                let mut key_char: Option<char>;
                match key {
                    Key::LeftShift | Key::RightShift => shift_state = true,
                    Key::Enter | Key::NumPadEnter => {
                        // println!("Enter pressed, string: {user_typed_line}");
                        shell_stdin
                            .write_all(format!("{user_typed_line}\n").as_bytes())
                            .unwrap_or_else(|_| exit(0));
                        // insert new line
                        while text_buffer_pointer % (WIDTH / 16) != 0 {
                            write_char_to_buffer(
                                Some(' '),
                                &mut text_buffer,
                                &mut text_buffer_pointer,
                            );
                        }
                        let mut received = queue_rx.recv().unwrap_or_else(|e| {
                            eprintln!("ERROR receiving data from mpsc sender: {e}");
                            exit(1);
                        });
                        if received.starts_with('\u{1b}') && !user_typed_line.is_empty() {
                            received = queue_rx.recv().unwrap_or_else(|e| {
                                eprintln!("ERROR receiving data from mpsc sender: {e}");
                                exit(1);
                            });
                        }
                        // dbg!(&received);
                        for chr in received.chars() {
                            if chr == '\n' {
                                // insert new line
                                while text_buffer_pointer % (WIDTH / 16) != 0 {
                                    write_char_to_buffer(
                                        Some(' '),
                                        &mut text_buffer,
                                        &mut text_buffer_pointer,
                                    );
                                }
                            } else {
                                write_char_to_buffer(
                                    Some(chr),
                                    &mut text_buffer,
                                    &mut text_buffer_pointer,
                                );
                            }
                        }
                        user_typed_line.clear();
                        write_char_to_buffer(Some('>'), &mut text_buffer, &mut text_buffer_pointer);
                        write_char_to_buffer(Some(' '), &mut text_buffer, &mut text_buffer_pointer);
                    }
                    Key::Backspace => {
                        if text_buffer_pointer >= 1 {
                            text_buffer_pointer -= 1;
                            text_buffer[text_buffer_pointer] = '_'; // cursor handling
                            text_buffer[text_buffer_pointer + 1] = ' ';
                            user_typed_line.pop();
                        }
                    }
                    _ => {
                        if shift_state {
                            key_char = get_key_char_shift(key);
                            if key_char.is_none() {
                                key_char = get_key_char(key); // fallback
                            }
                        } else {
                            key_char = get_key_char(key);
                        }
                        if let Some(k) = key_char {
                            user_typed_line.push(k);
                        }
                        write_char_to_buffer(key_char, &mut text_buffer, &mut text_buffer_pointer);
                    }
                };
                redraw_window(&mut window, &mut buffer, &mut text_buffer);
            });

        window.update();
    }
}
