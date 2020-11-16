// Client

use crossterm::{Result, style, queue, execute, cursor, event, terminal};

use std::io::{self, ErrorKind, Read, Write, stdout, Stdout};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use std::sync; // Arc, Mutex


const LOCAL: &str = "45.79.53.62:6000";
const MSG_SIZE: usize = 100;

struct Display {
    write: Stdout,
}

impl Display {
    fn new() -> Self {
        let mut write = stdout();
        execute!(&mut write, terminal::EnterAlternateScreen).expect("Failed to enter alternate screen mode");
        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        Self {
            write,
        }
    }

    fn message_board(&mut self, messages: &[String]) {
        queue!(self.write, cursor::SavePosition, cursor::Hide).expect("Failed to Hide Cursor");
        for (idx, message) in messages.iter().enumerate() {
            queue!(self.write, cursor::MoveTo(0, idx as u16), style::Print(message)).expect("Failed to write to message board");
        }
        queue!(self.write, cursor::Show, cursor::RestorePosition).expect("Failed to Show Cursor");
        self.write.flush();
    }

    fn chat_bar(&mut self, msg: &str) {
        queue!(self.write, cursor::MoveTo(0, 20), terminal::Clear(terminal::ClearType::CurrentLine)).expect("Failed to clear line");
        execute!(self.write, cursor::MoveTo(0, 20), style::Print(msg)).expect("Failed to write to char bar");
    }

    fn users_list(&mut self) {
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Faild to disable raw mode");
        execute!(&mut self.write, terminal::LeaveAlternateScreen).expect("Faild to leave alternate screen mode");
        println!("bye bye!");
    }
}

struct ClientData {
    current_messages: sync::Arc<sync::Mutex<Vec<String>>>,
    buff: String,
    display: Display,
    tx: mpsc::Sender<String>,
}


impl ClientData {
    fn new(current_messages: sync::Arc<sync::Mutex<Vec<String>>>, tx: mpsc::Sender<String>) -> Self {
        let display = Display::new();

        Self {
            current_messages,
            buff: String::new(),
            display,
            tx,
        }
    }

    fn mainloop(&mut self) -> Result<()> {
        loop {
            self.display.message_board(&self.current_messages.lock().unwrap());
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    match key {
                        event::KeyEvent { code, modifiers: _ } => {
                            match code {
                                event::KeyCode::Esc => {break;},
                                event::KeyCode::Backspace => {
                                    self.buff.pop();
                                    self.display.chat_bar(&self.buff);
                                },
                                event::KeyCode::Enter => {
                                    let msg = self.buff.trim().to_string();
                                    if msg == ":quit" || self.tx.send(msg).is_err() {break}
                                    self.buff.clear();
                                    self.display.chat_bar(&self.buff);
                                },
                                event::KeyCode::Char(c) => {
                                    self.buff.push(c);
                                    self.display.chat_bar(&self.buff);
                                }
                                _ => {},
                            }
                        },
                    }
                }
            }
        }
        Ok(())
    }
}


fn main() -> Result<()> {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    let current_messages = sync::Arc::new(sync::Mutex::new(Vec::new()));
    let cm = sync::Arc::clone(&current_messages);
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                let mut data = cm.lock().unwrap(); // TODO: this needs to not unwrap.
                data.push(msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            // Channel recived message to thread and now sends to Server.
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });
    let mut client_data = ClientData::new(current_messages, tx);
    client_data.mainloop()?;
    Ok(())
}
