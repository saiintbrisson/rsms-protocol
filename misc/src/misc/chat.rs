use std::borrow::Cow;

use protocol_internal::ProtocolSupport;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatComponent<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ChatColor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ChatEvent<ClickEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<ChatEvent<HoverEvent>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<ChatComponent<'a>>,
}

impl<'a> ChatComponent<'a> {
    pub fn new<I: Into<Cow<'a, str>>>(text: I) -> Self {
        Self {
            text: Some(text.into()),
            ..Default::default()
        }
    }
    pub fn append<I: Into<Cow<'a, str>>>(mut self, text: I) -> Self {
        self.extra.push(ChatComponent::new(text));
        self
    }
    pub fn append_extra(mut self, component: ChatComponent<'a>) -> Self {
        self.extra.push(component);
        self
    }
}

impl<'a> ChatComponent<'a> {
    pub fn color(mut self, color: ChatColor) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.color = Some(color);
        } else {
            self.color = Some(color);
        }
        self
    }
    pub fn bold(mut self, bold: bool) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.bold = Some(bold);
        } else {
            self.bold = Some(bold);
        }
        self
    }
    pub fn italic(mut self, italic: bool) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.italic = Some(italic);
        } else {
            self.italic = Some(italic);
        }
        self
    }
    pub fn underlined(mut self, underlined: bool) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.underlined = Some(underlined);
        } else {
            self.underlined = Some(underlined);
        }
        self
    }
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.strikethrough = Some(strikethrough);
        } else {
            self.strikethrough = Some(strikethrough);
        }
        self
    }
    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.obfuscated = Some(obfuscated);
        } else {
            self.obfuscated = Some(obfuscated);
        }
        self
    }
    pub fn insertion(mut self, insertion: String) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.insertion = Some(insertion);
        } else {
            self.insertion = Some(insertion);
        }
        self
    }
    pub fn click_event(mut self, click_event: ClickEvent, value: &str) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.click_event = Some(ChatEvent::new(click_event, value));
        } else {
            self.click_event = Some(ChatEvent::new(click_event, value));
        }
        self
    }

    pub fn hover_event(mut self, hover_event: HoverEvent, value: &str) -> Self {
        if let Some(component) = self.extra.last_mut() {
            component.hover_event = Some(ChatEvent::new(hover_event, value));
        } else {
            self.hover_event = Some(ChatEvent::new(hover_event, value));
        }
        self
    }
}

impl<'a> ProtocolSupport for ChatComponent<'a> {
    fn calculate_len(&self) -> usize {
        <String as ProtocolSupport>::calculate_len(&serde_json::to_string(self).unwrap())
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        <String as ProtocolSupport>::serialize(
            &serde_json::to_string(self)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?,
            dst,
        )
    }

    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        serde_json::from_str(&<String as ProtocolSupport>::deserialize(src)?)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatColor {
    Black = '0' as u8,
    DarkBlue = '1' as u8,
    DarkGreen = '2' as u8,
    DarkCyan = '3' as u8,
    DarkRed = '4' as u8,
    Purple = '5' as u8,
    Gold = '6' as u8,
    Gray = '7' as u8,
    DarkGray = '8' as u8,
    Blue = '9' as u8,
    BrightGreen = 'A' as u8,
    Cyan = 'B' as u8,
    Red = 'C' as u8,
    Pink = 'D' as u8,
    Yellow = 'E' as u8,
    White = 'F' as u8,
}

impl ChatColor {
    pub fn to_code(&self) -> char {
        *self as u8 as char
    }
}

impl Default for ChatColor {
    fn default() -> Self {
        Self::White
    }
}

impl From<char> for ChatColor {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Black,
            '1' => Self::DarkBlue,
            '2' => Self::DarkGreen,
            '3' => Self::DarkCyan,
            '4' => Self::DarkRed,
            '5' => Self::Purple,
            '6' => Self::Gold,
            '7' => Self::Gray,
            '8' => Self::DarkGray,
            '9' => Self::Blue,
            'A' => Self::BrightGreen,
            'B' => Self::Cyan,
            'C' => Self::Red,
            'D' => Self::Pink,
            'E' => Self::Yellow,
            'F' => Self::White,
            _ => panic!("did not expect {}", c),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatEvent<T: Sized> {
    action: T,
    value: String,
}

impl<T: Sized> ChatEvent<T> {
    pub fn new(action: T, value: &str) -> ChatEvent<T> {
        ChatEvent {
            action,
            value: value.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickEvent {
    OpenUrl,
    RunCommand,
    SuggestCommand,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverEvent {
    ShowText,
    ShowItem,
    ShowAchievement,
}

#[repr(i8)]
#[derive(Clone, Copy, Debug, protocol_derive::ProtocolSupport)]
pub enum ChatPosition {
    Chat = 0,
    SystemMessage = 1,
    AboveHotbar = 2,
}

impl Default for ChatPosition {
    fn default() -> Self {
        Self::Chat
    }
}
