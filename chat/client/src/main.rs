use crossterm::{Result, style, queue, execute, cursor, event, terminal};

use std::io::{self, ErrorKind, Read, Write, stdout, Stdout};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;


const LOCAL: &str = "45.79.53.62:6000";
const MSG_SIZE: usize = 32;


fn main() -> Result<()> {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                execute!(stdout(), cursor::MoveTo(0, 3), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
                execute!(&mut stdout(), cursor::MoveTo(0, 3), style::Print(&msg)).unwrap();
                //println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                // println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    let mut stdout = stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    println!("Write a Message:");
    let mut buff = String::new();
    loop {
        //io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key {
                    event::KeyEvent { code, modifiers: _ } => {
                        match code {
                            event::KeyCode::Esc => break,
                            event::KeyCode::Enter => {
                                let msg = buff.trim().to_string();
                                if msg == ":quit" || tx.send(msg).is_err() {break}
                                buff.clear();
                            },
                            event::KeyCode::Char(c) => {
                                buff.push(c);
                                execute!(&mut stdout, cursor::MoveTo(0, 10), terminal::Clear(terminal::ClearType::CurrentLine))?;
                                execute!(
                                    &mut stdout,
                                    cursor::MoveTo(0, 10),
                                    style::Print(&buff),
                                    )?;
                            }
                            _ => {},
                        }
                    },
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(&mut stdout, terminal::EnterAlternateScreen)?;
    println!("bye bye!");
    Ok(())
}
