use crate::app_error::AppError;
use crate::tg::td_enums::{TdChatList, TdMessageReplyToMessage};
use crossterm::event::{KeyCode, KeyModifiers, MouseEvent};
use ratatui::layout::Rect;
use std::fmt::{self, Display, Formatter};
use std::{hash::Hash, str::FromStr};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
/// `Event` is an enum that represents the different types of events that can be
/// generated by the intraction with the terminal (`tui_backend`).
/// These events are used to drive the user interface and the application logic
/// and should be handled entirely.
pub enum Event {
    /// Unknown event.
    Unknown,
    /// Resize event with width and height.
    Resize(u16, u16),
    /// Key event with a `KeyCode` and `KeyModifiers`.
    Key(KeyCode, KeyModifiers),
    /// Paste event with a `String`.
    Paste(String),
    /// Mouse event with a `MouseEvent` struct.
    Mouse(MouseEvent),
    /// Init event.
    Init,
    /// Render event.
    Render,
    /// Focus Lost event.
    FocusLost,
    /// Focus Gained event.
    FocusGained,

    /// Update area event with a `Rect` struct.
    UpdateArea(Rect),
    /// EditMessage event with a `String`.
    /// This event is used to edit a message.
    /// The first parameter is the `message_id` and the second parameter is the `text`.
    EditMessage(i64, String),
    /// ReplyMessage event with a `String`.
    /// This event is used to reply to a message.
    /// The first parameter is the `message_id` and the second parameter is the `text`.
    ReplyMessage(i64, String),

    /// GetMe event.
    GetMe,
    /// Load chats event with a `ChatList` and a limit.
    LoadChats(TdChatList, i32),
    /// Send message event with a `String`.
    /// This event is used to send a message.
    /// The first parameter is the `text`.
    /// The second parameter is the `reply_to` field.
    SendMessage(String, Option<TdMessageReplyToMessage>),
    /// Send message edited event with a `i64` and a `String`.
    /// The first parameter is the `message_id` and the second parameter is the `text`.
    SendMessageEdited(i64, String),
    /// Get chat history event.
    GetChatHistory,
    /// Delete messages event with a `Vec<i64>` and a `bool`.
    /// The first parameter is the `message_ids` and the second parameter is the `revoke`.
    /// If `revoke` is true, the message will be deleted for everyone.
    /// If `revoke` is false, the message will be deleted only for the current user.
    DeleteMessages(Vec<i64>, bool),
    /// View all messages event.
    ViewAllMessages,
}
/// Implement the `Event` enum.
impl Event {
    /// Create an event with a key code and modifiers.
    ///
    /// # Arguments
    /// * `s` - A string that represents the key code.
    /// * `modifiers` - A `KeyModifiers` struct that represents the modifiers.
    ///
    /// # Returns
    /// * `Result<Event, AppError>` - An event or an error.
    pub fn event_with_modifiers(s: &str, modifiers: KeyModifiers) -> Result<Event, AppError<()>> {
        match s {
            "backspace" => Ok(Event::Key(KeyCode::Backspace, modifiers)),
            "enter" => Ok(Event::Key(KeyCode::Enter, modifiers)),
            "left" => Ok(Event::Key(KeyCode::Left, modifiers)),
            "right" => Ok(Event::Key(KeyCode::Right, modifiers)),
            "up" => Ok(Event::Key(KeyCode::Up, modifiers)),
            "down" => Ok(Event::Key(KeyCode::Down, modifiers)),
            "home" => Ok(Event::Key(KeyCode::Home, modifiers)),
            "end" => Ok(Event::Key(KeyCode::End, modifiers)),
            "page_up" => Ok(Event::Key(KeyCode::PageUp, modifiers)),
            "page_down" => Ok(Event::Key(KeyCode::PageDown, modifiers)),
            "tab" => Ok(Event::Key(KeyCode::Tab, modifiers)),
            "back_tab" => Ok(Event::Key(KeyCode::BackTab, modifiers)),
            "delete" => Ok(Event::Key(KeyCode::Delete, modifiers)),
            "insert" => Ok(Event::Key(KeyCode::Insert, modifiers)),
            "null" => Ok(Event::Key(KeyCode::Null, modifiers)),
            "esc" => Ok(Event::Key(KeyCode::Esc, modifiers)),
            "f1" => Ok(Event::Key(KeyCode::F(1), modifiers)),
            "f2" => Ok(Event::Key(KeyCode::F(2), modifiers)),
            "f3" => Ok(Event::Key(KeyCode::F(3), modifiers)),
            "f4" => Ok(Event::Key(KeyCode::F(4), modifiers)),
            "f5" => Ok(Event::Key(KeyCode::F(5), modifiers)),
            "f6" => Ok(Event::Key(KeyCode::F(6), modifiers)),
            "f7" => Ok(Event::Key(KeyCode::F(7), modifiers)),
            "f8" => Ok(Event::Key(KeyCode::F(8), modifiers)),
            "f9" => Ok(Event::Key(KeyCode::F(9), modifiers)),
            "f10" => Ok(Event::Key(KeyCode::F(10), modifiers)),
            "f11" => Ok(Event::Key(KeyCode::F(11), modifiers)),
            "f12" => Ok(Event::Key(KeyCode::F(12), modifiers)),
            e => {
                if e.len() == 1 && e.chars().next().unwrap().is_ascii() {
                    Ok(Event::Key(
                        KeyCode::Char(e.chars().next().unwrap()),
                        modifiers,
                    ))
                } else {
                    Err(AppError::InvalidEvent(e.to_string()))
                }
            }
        }
    }
}

/// Implement the `FromStr` trait for `Event`.
impl FromStr for Event {
    type Err = AppError<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modifiers = s.split('+').collect::<Vec<&str>>();
        if modifiers.len() > 1 {
            let key = modifiers[modifiers.len() - 1];
            let modifiers = modifiers[..modifiers.len() - 1]
                .iter()
                .map(|m| match *m {
                    "ctrl" => KeyModifiers::CONTROL,
                    "alt" => KeyModifiers::ALT,
                    "shift" => KeyModifiers::SHIFT,
                    "super" => KeyModifiers::SUPER,
                    "meta" => KeyModifiers::META,
                    "hyper" => KeyModifiers::HYPER,
                    _ => KeyModifiers::NONE,
                })
                .fold(KeyModifiers::NONE, |acc, m| acc | m);
            Self::event_with_modifiers(key, modifiers)
        } else {
            Self::event_with_modifiers(s, KeyModifiers::NONE)
        }
    }
}

/// Implement the `Display` trait for `Event`.
impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Event::Unknown => write!(f, "Unknown"),
            Event::Init => write!(f, "Init"),
            Event::Render => write!(f, "Render"),
            Event::Resize(width, height) => {
                write!(f, "Resize({}, {})", width, height)
            }
            Event::Key(key, modifiers) => {
                let k = if let KeyCode::Char(c) = key {
                    c.to_string()
                } else {
                    format!("{:?}", key)
                };

                match *modifiers {
                    KeyModifiers::NONE => write!(f, "{}", k),
                    KeyModifiers::CONTROL => write!(f, "Ctrl+{}", k),
                    KeyModifiers::ALT => write!(f, "Alt+{}", k),
                    KeyModifiers::SHIFT => write!(f, "Shift+{}", k),
                    KeyModifiers::SUPER => write!(f, "Super+{}", k),
                    KeyModifiers::META => write!(f, "Meta+{}", k),
                    KeyModifiers::HYPER => write!(f, "Hyper+{}", k),
                    _ => write!(f, "{:?}+{}", modifiers, k),
                }
            }
            Event::Mouse(mouse) => write!(f, "Mouse({:?})", mouse),
            Event::UpdateArea(area) => write!(f, "UpdateArea({:?})", area),
            Event::Paste(s) => write!(f, "Paste({})", s),
            Event::FocusLost => write!(f, "FocusLost"),
            Event::FocusGained => write!(f, "FocusGained"),
            Event::GetMe => write!(f, "GetMe"),
            Event::LoadChats(chat_list, limit) => {
                write!(f, "LoadChats({:?}, {})", chat_list, limit)
            }
            Event::SendMessage(s, reply_to) => {
                write!(f, "SendMessage({}, {:?})", s, reply_to)
            }
            Event::SendMessageEdited(message_id, s) => {
                write!(f, "SendMessageEdited({}, {})", message_id, s)
            }
            Event::GetChatHistory => {
                write!(f, "GetChatHistory")
            }
            Event::DeleteMessages(message_ids, revoke) => {
                write!(f, "DeleteMessages({:?}, {})", message_ids, revoke)
            }
            Event::EditMessage(message_id, text) => {
                write!(f, "EditMessage({}, {})", message_id, text)
            }
            Event::ReplyMessage(message_id, text) => {
                write!(f, "ReplyMessage({}, {})", message_id, text)
            }
            Event::ViewAllMessages => {
                write!(f, "ViewAllMessages")
            }
        }
    }
}
