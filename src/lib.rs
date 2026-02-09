#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{
  ThreadsafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
};
use napi_derive::napi;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

// Re-export monio types
use monio::{
  display_at_point, displays, key_press, key_release, key_tap, mouse_click, mouse_move,
  mouse_position, mouse_press, mouse_release, primary_display, system_settings, Button,
  DisplayInfo, Event, EventType, Hook, Key, Rect, ScrollDirection, SystemSettings,
};

// ============================================================================
// Enums
// ============================================================================

#[napi]
pub enum EventTypeJs {
  HookEnabled,
  HookDisabled,
  KeyPressed,
  KeyReleased,
  KeyTyped,
  MousePressed,
  MouseReleased,
  MouseClicked,
  MouseMoved,
  MouseDragged,
  MouseWheel,
}

impl From<EventType> for EventTypeJs {
  fn from(et: EventType) -> Self {
    match et {
      EventType::HookEnabled => EventTypeJs::HookEnabled,
      EventType::HookDisabled => EventTypeJs::HookDisabled,
      EventType::KeyPressed => EventTypeJs::KeyPressed,
      EventType::KeyReleased => EventTypeJs::KeyReleased,
      EventType::KeyTyped => EventTypeJs::KeyTyped,
      EventType::MousePressed => EventTypeJs::MousePressed,
      EventType::MouseReleased => EventTypeJs::MouseReleased,
      EventType::MouseClicked => EventTypeJs::MouseClicked,
      EventType::MouseMoved => EventTypeJs::MouseMoved,
      EventType::MouseDragged => EventTypeJs::MouseDragged,
      EventType::MouseWheel => EventTypeJs::MouseWheel,
    }
  }
}

#[napi]
pub enum ButtonJs {
  Left,
  Right,
  Middle,
  Button4,
  Button5,
  Unknown,
}

impl From<Button> for ButtonJs {
  fn from(btn: Button) -> Self {
    match btn {
      Button::Left => ButtonJs::Left,
      Button::Right => ButtonJs::Right,
      Button::Middle => ButtonJs::Middle,
      Button::Button4 => ButtonJs::Button4,
      Button::Button5 => ButtonJs::Button5,
      Button::Unknown(_) => ButtonJs::Unknown,
    }
  }
}

impl From<ButtonJs> for Button {
  fn from(btn: ButtonJs) -> Self {
    match btn {
      ButtonJs::Left => Button::Left,
      ButtonJs::Right => Button::Right,
      ButtonJs::Middle => Button::Middle,
      ButtonJs::Button4 => Button::Button4,
      ButtonJs::Button5 => Button::Button5,
      ButtonJs::Unknown => Button::Unknown(0),
    }
  }
}

#[napi]
pub enum KeyJs {
  // Letters
  KeyA,
  KeyB,
  KeyC,
  KeyD,
  KeyE,
  KeyF,
  KeyG,
  KeyH,
  KeyI,
  KeyJ,
  KeyK,
  KeyL,
  KeyM,
  KeyN,
  KeyO,
  KeyP,
  KeyQ,
  KeyR,
  KeyS,
  KeyT,
  KeyU,
  KeyV,
  KeyW,
  KeyX,
  KeyY,
  KeyZ,

  // Numbers
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,

  // Function keys
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,

  // Special keys
  Escape,
  Space,
  Enter,
  Backspace,
  Tab,
  ShiftLeft,
  ShiftRight,
  ControlLeft,
  ControlRight,
  AltLeft,
  AltRight,
  MetaLeft,
  MetaRight,
  CapsLock,
  Delete,

  // Arrow keys
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,

  // Unknown (value = 67, kept for backward compat)
  Unknown,

  // ─── New keys (68+) ─────────────────────────────────────────────────

  // Navigation
  Insert,
  Home,
  End,
  PageUp,
  PageDown,

  // Lock keys
  NumLock,
  ScrollLock,
  PrintScreen,
  Pause,

  // Punctuation and symbols
  Grave,        // ` ~
  Minus,        // - _
  Equal,        // = +
  BracketLeft,  // [ {
  BracketRight, // ] }
  Backslash,    // \ |
  Semicolon,    // ; :
  Quote,        // ' "
  Comma,        // , <
  Period,       // . >
  Slash,        // / ?

  // Extended function keys
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,

  // Numpad
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadAdd,
  NumpadSubtract,
  NumpadMultiply,
  NumpadDivide,
  NumpadDecimal,
  NumpadEnter,
  NumpadEqual,

  // Media keys
  VolumeUp,
  VolumeDown,
  VolumeMute,
  MediaPlayPause,
  MediaStop,
  MediaNext,
  MediaPrevious,

  // Browser keys
  BrowserBack,
  BrowserForward,
  BrowserRefresh,
  BrowserStop,
  BrowserSearch,
  BrowserFavorites,
  BrowserHome,

  // Application keys
  LaunchMail,
  LaunchApp1,
  LaunchApp2,

  // International
  IntlBackslash,
  IntlYen,
  IntlRo,

  // Context menu
  ContextMenu,
}
impl From<Key> for KeyJs {
  fn from(key: Key) -> Self {
    match key {
      Key::KeyA => KeyJs::KeyA,
      Key::KeyB => KeyJs::KeyB,
      Key::KeyC => KeyJs::KeyC,
      Key::KeyD => KeyJs::KeyD,
      Key::KeyE => KeyJs::KeyE,
      Key::KeyF => KeyJs::KeyF,
      Key::KeyG => KeyJs::KeyG,
      Key::KeyH => KeyJs::KeyH,
      Key::KeyI => KeyJs::KeyI,
      Key::KeyJ => KeyJs::KeyJ,
      Key::KeyK => KeyJs::KeyK,
      Key::KeyL => KeyJs::KeyL,
      Key::KeyM => KeyJs::KeyM,
      Key::KeyN => KeyJs::KeyN,
      Key::KeyO => KeyJs::KeyO,
      Key::KeyP => KeyJs::KeyP,
      Key::KeyQ => KeyJs::KeyQ,
      Key::KeyR => KeyJs::KeyR,
      Key::KeyS => KeyJs::KeyS,
      Key::KeyT => KeyJs::KeyT,
      Key::KeyU => KeyJs::KeyU,
      Key::KeyV => KeyJs::KeyV,
      Key::KeyW => KeyJs::KeyW,
      Key::KeyX => KeyJs::KeyX,
      Key::KeyY => KeyJs::KeyY,
      Key::KeyZ => KeyJs::KeyZ,
      Key::Num0 => KeyJs::Num0,
      Key::Num1 => KeyJs::Num1,
      Key::Num2 => KeyJs::Num2,
      Key::Num3 => KeyJs::Num3,
      Key::Num4 => KeyJs::Num4,
      Key::Num5 => KeyJs::Num5,
      Key::Num6 => KeyJs::Num6,
      Key::Num7 => KeyJs::Num7,
      Key::Num8 => KeyJs::Num8,
      Key::Num9 => KeyJs::Num9,
      Key::F1 => KeyJs::F1,
      Key::F2 => KeyJs::F2,
      Key::F3 => KeyJs::F3,
      Key::F4 => KeyJs::F4,
      Key::F5 => KeyJs::F5,
      Key::F6 => KeyJs::F6,
      Key::F7 => KeyJs::F7,
      Key::F8 => KeyJs::F8,
      Key::F9 => KeyJs::F9,
      Key::F10 => KeyJs::F10,
      Key::F11 => KeyJs::F11,
      Key::F12 => KeyJs::F12,
      Key::Escape => KeyJs::Escape,
      Key::Space => KeyJs::Space,
      Key::Enter => KeyJs::Enter,
      Key::Backspace => KeyJs::Backspace,
      Key::Tab => KeyJs::Tab,
      Key::ShiftLeft => KeyJs::ShiftLeft,
      Key::ShiftRight => KeyJs::ShiftRight,
      Key::ControlLeft => KeyJs::ControlLeft,
      Key::ControlRight => KeyJs::ControlRight,
      Key::AltLeft => KeyJs::AltLeft,
      Key::AltRight => KeyJs::AltRight,
      Key::MetaLeft => KeyJs::MetaLeft,
      Key::MetaRight => KeyJs::MetaRight,
      Key::CapsLock => KeyJs::CapsLock,
      Key::Delete => KeyJs::Delete,
      Key::ArrowLeft => KeyJs::ArrowLeft,
      Key::ArrowRight => KeyJs::ArrowRight,
      Key::ArrowUp => KeyJs::ArrowUp,
      Key::ArrowDown => KeyJs::ArrowDown,
      // Navigation
      Key::Insert => KeyJs::Insert,
      Key::Home => KeyJs::Home,
      Key::End => KeyJs::End,
      Key::PageUp => KeyJs::PageUp,
      Key::PageDown => KeyJs::PageDown,
      // Lock keys
      Key::NumLock => KeyJs::NumLock,
      Key::ScrollLock => KeyJs::ScrollLock,
      Key::PrintScreen => KeyJs::PrintScreen,
      Key::Pause => KeyJs::Pause,
      // Punctuation
      Key::Grave => KeyJs::Grave,
      Key::Minus => KeyJs::Minus,
      Key::Equal => KeyJs::Equal,
      Key::BracketLeft => KeyJs::BracketLeft,
      Key::BracketRight => KeyJs::BracketRight,
      Key::Backslash => KeyJs::Backslash,
      Key::Semicolon => KeyJs::Semicolon,
      Key::Quote => KeyJs::Quote,
      Key::Comma => KeyJs::Comma,
      Key::Period => KeyJs::Period,
      Key::Slash => KeyJs::Slash,
      // Extended F-keys
      Key::F13 => KeyJs::F13,
      Key::F14 => KeyJs::F14,
      Key::F15 => KeyJs::F15,
      Key::F16 => KeyJs::F16,
      Key::F17 => KeyJs::F17,
      Key::F18 => KeyJs::F18,
      Key::F19 => KeyJs::F19,
      Key::F20 => KeyJs::F20,
      Key::F21 => KeyJs::F21,
      Key::F22 => KeyJs::F22,
      Key::F23 => KeyJs::F23,
      Key::F24 => KeyJs::F24,
      // Numpad
      Key::Numpad0 => KeyJs::Numpad0,
      Key::Numpad1 => KeyJs::Numpad1,
      Key::Numpad2 => KeyJs::Numpad2,
      Key::Numpad3 => KeyJs::Numpad3,
      Key::Numpad4 => KeyJs::Numpad4,
      Key::Numpad5 => KeyJs::Numpad5,
      Key::Numpad6 => KeyJs::Numpad6,
      Key::Numpad7 => KeyJs::Numpad7,
      Key::Numpad8 => KeyJs::Numpad8,
      Key::Numpad9 => KeyJs::Numpad9,
      Key::NumpadAdd => KeyJs::NumpadAdd,
      Key::NumpadSubtract => KeyJs::NumpadSubtract,
      Key::NumpadMultiply => KeyJs::NumpadMultiply,
      Key::NumpadDivide => KeyJs::NumpadDivide,
      Key::NumpadDecimal => KeyJs::NumpadDecimal,
      Key::NumpadEnter => KeyJs::NumpadEnter,
      Key::NumpadEqual => KeyJs::NumpadEqual,
      // Media
      Key::VolumeUp => KeyJs::VolumeUp,
      Key::VolumeDown => KeyJs::VolumeDown,
      Key::VolumeMute => KeyJs::VolumeMute,
      Key::MediaPlayPause => KeyJs::MediaPlayPause,
      Key::MediaStop => KeyJs::MediaStop,
      Key::MediaNext => KeyJs::MediaNext,
      Key::MediaPrevious => KeyJs::MediaPrevious,
      // Browser
      Key::BrowserBack => KeyJs::BrowserBack,
      Key::BrowserForward => KeyJs::BrowserForward,
      Key::BrowserRefresh => KeyJs::BrowserRefresh,
      Key::BrowserStop => KeyJs::BrowserStop,
      Key::BrowserSearch => KeyJs::BrowserSearch,
      Key::BrowserFavorites => KeyJs::BrowserFavorites,
      Key::BrowserHome => KeyJs::BrowserHome,
      // Application
      Key::LaunchMail => KeyJs::LaunchMail,
      Key::LaunchApp1 => KeyJs::LaunchApp1,
      Key::LaunchApp2 => KeyJs::LaunchApp2,
      // International
      Key::IntlBackslash => KeyJs::IntlBackslash,
      Key::IntlYen => KeyJs::IntlYen,
      Key::IntlRo => KeyJs::IntlRo,
      // Context menu
      Key::ContextMenu => KeyJs::ContextMenu,
      // Truly unknown
      Key::Unknown(_) => KeyJs::Unknown,
    }
  }
}

impl From<KeyJs> for Key {
  fn from(key: KeyJs) -> Self {
    match key {
      KeyJs::KeyA => Key::KeyA,
      KeyJs::KeyB => Key::KeyB,
      KeyJs::KeyC => Key::KeyC,
      KeyJs::KeyD => Key::KeyD,
      KeyJs::KeyE => Key::KeyE,
      KeyJs::KeyF => Key::KeyF,
      KeyJs::KeyG => Key::KeyG,
      KeyJs::KeyH => Key::KeyH,
      KeyJs::KeyI => Key::KeyI,
      KeyJs::KeyJ => Key::KeyJ,
      KeyJs::KeyK => Key::KeyK,
      KeyJs::KeyL => Key::KeyL,
      KeyJs::KeyM => Key::KeyM,
      KeyJs::KeyN => Key::KeyN,
      KeyJs::KeyO => Key::KeyO,
      KeyJs::KeyP => Key::KeyP,
      KeyJs::KeyQ => Key::KeyQ,
      KeyJs::KeyR => Key::KeyR,
      KeyJs::KeyS => Key::KeyS,
      KeyJs::KeyT => Key::KeyT,
      KeyJs::KeyU => Key::KeyU,
      KeyJs::KeyV => Key::KeyV,
      KeyJs::KeyW => Key::KeyW,
      KeyJs::KeyX => Key::KeyX,
      KeyJs::KeyY => Key::KeyY,
      KeyJs::KeyZ => Key::KeyZ,
      KeyJs::Num0 => Key::Num0,
      KeyJs::Num1 => Key::Num1,
      KeyJs::Num2 => Key::Num2,
      KeyJs::Num3 => Key::Num3,
      KeyJs::Num4 => Key::Num4,
      KeyJs::Num5 => Key::Num5,
      KeyJs::Num6 => Key::Num6,
      KeyJs::Num7 => Key::Num7,
      KeyJs::Num8 => Key::Num8,
      KeyJs::Num9 => Key::Num9,
      KeyJs::F1 => Key::F1,
      KeyJs::F2 => Key::F2,
      KeyJs::F3 => Key::F3,
      KeyJs::F4 => Key::F4,
      KeyJs::F5 => Key::F5,
      KeyJs::F6 => Key::F6,
      KeyJs::F7 => Key::F7,
      KeyJs::F8 => Key::F8,
      KeyJs::F9 => Key::F9,
      KeyJs::F10 => Key::F10,
      KeyJs::F11 => Key::F11,
      KeyJs::F12 => Key::F12,
      KeyJs::Escape => Key::Escape,
      KeyJs::Space => Key::Space,
      KeyJs::Enter => Key::Enter,
      KeyJs::Backspace => Key::Backspace,
      KeyJs::Tab => Key::Tab,
      KeyJs::ShiftLeft => Key::ShiftLeft,
      KeyJs::ShiftRight => Key::ShiftRight,
      KeyJs::ControlLeft => Key::ControlLeft,
      KeyJs::ControlRight => Key::ControlRight,
      KeyJs::AltLeft => Key::AltLeft,
      KeyJs::AltRight => Key::AltRight,
      KeyJs::MetaLeft => Key::MetaLeft,
      KeyJs::MetaRight => Key::MetaRight,
      KeyJs::CapsLock => Key::CapsLock,
      KeyJs::Delete => Key::Delete,
      KeyJs::ArrowLeft => Key::ArrowLeft,
      KeyJs::ArrowRight => Key::ArrowRight,
      KeyJs::ArrowUp => Key::ArrowUp,
      KeyJs::ArrowDown => Key::ArrowDown,
      // Navigation
      KeyJs::Insert => Key::Insert,
      KeyJs::Home => Key::Home,
      KeyJs::End => Key::End,
      KeyJs::PageUp => Key::PageUp,
      KeyJs::PageDown => Key::PageDown,
      // Lock keys
      KeyJs::NumLock => Key::NumLock,
      KeyJs::ScrollLock => Key::ScrollLock,
      KeyJs::PrintScreen => Key::PrintScreen,
      KeyJs::Pause => Key::Pause,
      // Punctuation
      KeyJs::Grave => Key::Grave,
      KeyJs::Minus => Key::Minus,
      KeyJs::Equal => Key::Equal,
      KeyJs::BracketLeft => Key::BracketLeft,
      KeyJs::BracketRight => Key::BracketRight,
      KeyJs::Backslash => Key::Backslash,
      KeyJs::Semicolon => Key::Semicolon,
      KeyJs::Quote => Key::Quote,
      KeyJs::Comma => Key::Comma,
      KeyJs::Period => Key::Period,
      KeyJs::Slash => Key::Slash,
      // Extended F-keys
      KeyJs::F13 => Key::F13,
      KeyJs::F14 => Key::F14,
      KeyJs::F15 => Key::F15,
      KeyJs::F16 => Key::F16,
      KeyJs::F17 => Key::F17,
      KeyJs::F18 => Key::F18,
      KeyJs::F19 => Key::F19,
      KeyJs::F20 => Key::F20,
      KeyJs::F21 => Key::F21,
      KeyJs::F22 => Key::F22,
      KeyJs::F23 => Key::F23,
      KeyJs::F24 => Key::F24,
      // Numpad
      KeyJs::Numpad0 => Key::Numpad0,
      KeyJs::Numpad1 => Key::Numpad1,
      KeyJs::Numpad2 => Key::Numpad2,
      KeyJs::Numpad3 => Key::Numpad3,
      KeyJs::Numpad4 => Key::Numpad4,
      KeyJs::Numpad5 => Key::Numpad5,
      KeyJs::Numpad6 => Key::Numpad6,
      KeyJs::Numpad7 => Key::Numpad7,
      KeyJs::Numpad8 => Key::Numpad8,
      KeyJs::Numpad9 => Key::Numpad9,
      KeyJs::NumpadAdd => Key::NumpadAdd,
      KeyJs::NumpadSubtract => Key::NumpadSubtract,
      KeyJs::NumpadMultiply => Key::NumpadMultiply,
      KeyJs::NumpadDivide => Key::NumpadDivide,
      KeyJs::NumpadDecimal => Key::NumpadDecimal,
      KeyJs::NumpadEnter => Key::NumpadEnter,
      KeyJs::NumpadEqual => Key::NumpadEqual,
      // Media
      KeyJs::VolumeUp => Key::VolumeUp,
      KeyJs::VolumeDown => Key::VolumeDown,
      KeyJs::VolumeMute => Key::VolumeMute,
      KeyJs::MediaPlayPause => Key::MediaPlayPause,
      KeyJs::MediaStop => Key::MediaStop,
      KeyJs::MediaNext => Key::MediaNext,
      KeyJs::MediaPrevious => Key::MediaPrevious,
      // Browser
      KeyJs::BrowserBack => Key::BrowserBack,
      KeyJs::BrowserForward => Key::BrowserForward,
      KeyJs::BrowserRefresh => Key::BrowserRefresh,
      KeyJs::BrowserStop => Key::BrowserStop,
      KeyJs::BrowserSearch => Key::BrowserSearch,
      KeyJs::BrowserFavorites => Key::BrowserFavorites,
      KeyJs::BrowserHome => Key::BrowserHome,
      // Application
      KeyJs::LaunchMail => Key::LaunchMail,
      KeyJs::LaunchApp1 => Key::LaunchApp1,
      KeyJs::LaunchApp2 => Key::LaunchApp2,
      // International
      KeyJs::IntlBackslash => Key::IntlBackslash,
      KeyJs::IntlYen => Key::IntlYen,
      KeyJs::IntlRo => Key::IntlRo,
      // Context menu
      KeyJs::ContextMenu => Key::ContextMenu,
      // Unknown
      KeyJs::Unknown => Key::Unknown(0),
    }
  }
}

// ============================================================================
// Display Name Helpers (single source of truth for key/button presentation)
// ============================================================================

fn key_display_name(key: KeyJs) -> &'static str {
  match key {
    // Letters
    KeyJs::KeyA => "A",
    KeyJs::KeyB => "B",
    KeyJs::KeyC => "C",
    KeyJs::KeyD => "D",
    KeyJs::KeyE => "E",
    KeyJs::KeyF => "F",
    KeyJs::KeyG => "G",
    KeyJs::KeyH => "H",
    KeyJs::KeyI => "I",
    KeyJs::KeyJ => "J",
    KeyJs::KeyK => "K",
    KeyJs::KeyL => "L",
    KeyJs::KeyM => "M",
    KeyJs::KeyN => "N",
    KeyJs::KeyO => "O",
    KeyJs::KeyP => "P",
    KeyJs::KeyQ => "Q",
    KeyJs::KeyR => "R",
    KeyJs::KeyS => "S",
    KeyJs::KeyT => "T",
    KeyJs::KeyU => "U",
    KeyJs::KeyV => "V",
    KeyJs::KeyW => "W",
    KeyJs::KeyX => "X",
    KeyJs::KeyY => "Y",
    KeyJs::KeyZ => "Z",
    // Numbers
    KeyJs::Num0 => "0",
    KeyJs::Num1 => "1",
    KeyJs::Num2 => "2",
    KeyJs::Num3 => "3",
    KeyJs::Num4 => "4",
    KeyJs::Num5 => "5",
    KeyJs::Num6 => "6",
    KeyJs::Num7 => "7",
    KeyJs::Num8 => "8",
    KeyJs::Num9 => "9",
    // Function keys
    KeyJs::F1 => "F1",
    KeyJs::F2 => "F2",
    KeyJs::F3 => "F3",
    KeyJs::F4 => "F4",
    KeyJs::F5 => "F5",
    KeyJs::F6 => "F6",
    KeyJs::F7 => "F7",
    KeyJs::F8 => "F8",
    KeyJs::F9 => "F9",
    KeyJs::F10 => "F10",
    KeyJs::F11 => "F11",
    KeyJs::F12 => "F12",
    // Special keys
    KeyJs::Escape => "Esc",
    KeyJs::Space => "Space",
    KeyJs::Enter => "\u{21b5}",     // ↵
    KeyJs::Backspace => "\u{232b}", // ⌫
    KeyJs::Tab => "Tab",
    // Modifiers
    KeyJs::ShiftLeft => "Shift",
    KeyJs::ShiftRight => "Shift",
    KeyJs::ControlLeft => "Ctrl",
    KeyJs::ControlRight => "Ctrl",
    KeyJs::AltLeft => "Alt",
    KeyJs::AltRight => "Alt",
    KeyJs::MetaLeft => "\u{2318}",  // ⌘
    KeyJs::MetaRight => "\u{2318}", // ⌘
    KeyJs::CapsLock => "Caps",
    KeyJs::Delete => "Del",
    // Arrows
    KeyJs::ArrowLeft => "\u{2190}",  // ←
    KeyJs::ArrowRight => "\u{2192}", // →
    KeyJs::ArrowUp => "\u{2191}",    // ↑
    KeyJs::ArrowDown => "\u{2193}",  // ↓
    // Navigation
    KeyJs::Insert => "Ins",
    KeyJs::Home => "Home",
    KeyJs::End => "End",
    KeyJs::PageUp => "PgUp",
    KeyJs::PageDown => "PgDn",
    // Lock keys
    KeyJs::NumLock => "NumLk",
    KeyJs::ScrollLock => "ScrLk",
    KeyJs::PrintScreen => "PrtSc",
    KeyJs::Pause => "Pause",
    // Punctuation
    KeyJs::Grave => "`",
    KeyJs::Minus => "-",
    KeyJs::Equal => "=",
    KeyJs::BracketLeft => "[",
    KeyJs::BracketRight => "]",
    KeyJs::Backslash => "\\",
    KeyJs::Semicolon => ";",
    KeyJs::Quote => "'",
    KeyJs::Comma => ",",
    KeyJs::Period => ".",
    KeyJs::Slash => "/",
    // Extended function keys
    KeyJs::F13 => "F13",
    KeyJs::F14 => "F14",
    KeyJs::F15 => "F15",
    KeyJs::F16 => "F16",
    KeyJs::F17 => "F17",
    KeyJs::F18 => "F18",
    KeyJs::F19 => "F19",
    KeyJs::F20 => "F20",
    KeyJs::F21 => "F21",
    KeyJs::F22 => "F22",
    KeyJs::F23 => "F23",
    KeyJs::F24 => "F24",
    // Numpad
    KeyJs::Numpad0 => "Num0",
    KeyJs::Numpad1 => "Num1",
    KeyJs::Numpad2 => "Num2",
    KeyJs::Numpad3 => "Num3",
    KeyJs::Numpad4 => "Num4",
    KeyJs::Numpad5 => "Num5",
    KeyJs::Numpad6 => "Num6",
    KeyJs::Numpad7 => "Num7",
    KeyJs::Numpad8 => "Num8",
    KeyJs::Numpad9 => "Num9",
    KeyJs::NumpadAdd => "Num+",
    KeyJs::NumpadSubtract => "Num-",
    KeyJs::NumpadMultiply => "Num*",
    KeyJs::NumpadDivide => "Num/",
    KeyJs::NumpadDecimal => "Num.",
    KeyJs::NumpadEnter => "NumEnter",
    KeyJs::NumpadEqual => "Num=",
    // Media
    KeyJs::VolumeUp => "Vol+",
    KeyJs::VolumeDown => "Vol-",
    KeyJs::VolumeMute => "Mute",
    KeyJs::MediaPlayPause => "Play",
    KeyJs::MediaStop => "Stop",
    KeyJs::MediaNext => "Next",
    KeyJs::MediaPrevious => "Prev",
    // Browser
    KeyJs::BrowserBack => "BrBack",
    KeyJs::BrowserForward => "BrFwd",
    KeyJs::BrowserRefresh => "BrRefresh",
    KeyJs::BrowserStop => "BrStop",
    KeyJs::BrowserSearch => "BrSearch",
    KeyJs::BrowserFavorites => "BrFav",
    KeyJs::BrowserHome => "BrHome",
    // Application
    KeyJs::LaunchMail => "Mail",
    KeyJs::LaunchApp1 => "App1",
    KeyJs::LaunchApp2 => "App2",
    // International
    KeyJs::IntlBackslash => "IntlBksl",
    KeyJs::IntlYen => "\u{00a5}", // ¥
    KeyJs::IntlRo => "IntlRo",
    // Context menu
    KeyJs::ContextMenu => "Menu",
    // Unknown
    KeyJs::Unknown => "Unknown",
  }
}

fn button_display_name(button: ButtonJs) -> &'static str {
  match button {
    ButtonJs::Left => "MouseL",
    ButtonJs::Right => "MouseR",
    ButtonJs::Middle => "MouseM",
    ButtonJs::Button4 => "Mouse4",
    ButtonJs::Button5 => "Mouse5",
    ButtonJs::Unknown => "Mouse?",
  }
}

fn key_category(key: KeyJs) -> &'static str {
  let k: Key = key.into();
  if k.is_modifier() {
    return "modifier";
  }
  if k.is_letter() {
    return "letter";
  }
  if k.is_number() {
    return "number";
  }
  if k.is_function_key() {
    return "function";
  }
  if k.is_numpad() {
    return "numpad";
  }
  if k.is_media() {
    return "media";
  }
  // monio's is_navigation includes arrows, home, end, pageup, pagedown
  if k.is_navigation() {
    // Distinguish arrow keys from other navigation
    return match k {
      Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => "arrow",
      _ => "navigation",
    };
  }
  // Additional categories not covered by monio's built-in methods
  match k {
    Key::Grave
    | Key::Minus
    | Key::Equal
    | Key::BracketLeft
    | Key::BracketRight
    | Key::Backslash
    | Key::Semicolon
    | Key::Quote
    | Key::Comma
    | Key::Period
    | Key::Slash => "punctuation",
    Key::CapsLock | Key::NumLock | Key::ScrollLock => "lock",
    Key::BrowserBack
    | Key::BrowserForward
    | Key::BrowserRefresh
    | Key::BrowserStop
    | Key::BrowserSearch
    | Key::BrowserFavorites
    | Key::BrowserHome => "browser",
    Key::LaunchMail | Key::LaunchApp1 | Key::LaunchApp2 => "application",
    Key::IntlBackslash | Key::IntlYen | Key::IntlRo => "international",
    Key::Escape
    | Key::Space
    | Key::Enter
    | Key::Backspace
    | Key::Tab
    | Key::Delete
    | Key::PrintScreen
    | Key::Pause
    | Key::ContextMenu => "special",
    _ => "unknown",
  }
}

/// Get the display name for a key.
#[napi]
pub fn get_key_display_name(key: KeyJs) -> String {
  key_display_name(key).to_string()
}

/// Get the display name for a mouse button.
#[napi]
pub fn get_button_display_name(button: ButtonJs) -> String {
  button_display_name(button).to_string()
}

/// Get the category for a key (e.g. "letter", "modifier", "arrow", "function").
#[napi]
pub fn get_key_category(key: KeyJs) -> String {
  key_category(key).to_string()
}

/// Check if a key is a modifier key.
#[napi]
pub fn is_modifier_key(key: KeyJs) -> bool {
  let k: Key = key.into();
  k.is_modifier()
}

#[napi(object)]
pub struct KeyDisplayInfo {
  pub key: u32,
  pub display_name: String,
  pub category: String,
}

/// Total number of named KeyJs variants (0 through 137 inclusive).
/// IMPORTANT: Update this when adding new KeyJs variants, and add matching
/// arms to key_from_i32, key_display_name, and key_category.
const KEY_JS_COUNT: i32 = 138;

/// Map an integer to a KeyJs variant. Returns None for out-of-range values.
fn key_from_i32(v: i32) -> Option<KeyJs> {
  match v {
    0 => Some(KeyJs::KeyA),
    1 => Some(KeyJs::KeyB),
    2 => Some(KeyJs::KeyC),
    3 => Some(KeyJs::KeyD),
    4 => Some(KeyJs::KeyE),
    5 => Some(KeyJs::KeyF),
    6 => Some(KeyJs::KeyG),
    7 => Some(KeyJs::KeyH),
    8 => Some(KeyJs::KeyI),
    9 => Some(KeyJs::KeyJ),
    10 => Some(KeyJs::KeyK),
    11 => Some(KeyJs::KeyL),
    12 => Some(KeyJs::KeyM),
    13 => Some(KeyJs::KeyN),
    14 => Some(KeyJs::KeyO),
    15 => Some(KeyJs::KeyP),
    16 => Some(KeyJs::KeyQ),
    17 => Some(KeyJs::KeyR),
    18 => Some(KeyJs::KeyS),
    19 => Some(KeyJs::KeyT),
    20 => Some(KeyJs::KeyU),
    21 => Some(KeyJs::KeyV),
    22 => Some(KeyJs::KeyW),
    23 => Some(KeyJs::KeyX),
    24 => Some(KeyJs::KeyY),
    25 => Some(KeyJs::KeyZ),
    26 => Some(KeyJs::Num0),
    27 => Some(KeyJs::Num1),
    28 => Some(KeyJs::Num2),
    29 => Some(KeyJs::Num3),
    30 => Some(KeyJs::Num4),
    31 => Some(KeyJs::Num5),
    32 => Some(KeyJs::Num6),
    33 => Some(KeyJs::Num7),
    34 => Some(KeyJs::Num8),
    35 => Some(KeyJs::Num9),
    36 => Some(KeyJs::F1),
    37 => Some(KeyJs::F2),
    38 => Some(KeyJs::F3),
    39 => Some(KeyJs::F4),
    40 => Some(KeyJs::F5),
    41 => Some(KeyJs::F6),
    42 => Some(KeyJs::F7),
    43 => Some(KeyJs::F8),
    44 => Some(KeyJs::F9),
    45 => Some(KeyJs::F10),
    46 => Some(KeyJs::F11),
    47 => Some(KeyJs::F12),
    48 => Some(KeyJs::Escape),
    49 => Some(KeyJs::Space),
    50 => Some(KeyJs::Enter),
    51 => Some(KeyJs::Backspace),
    52 => Some(KeyJs::Tab),
    53 => Some(KeyJs::ShiftLeft),
    54 => Some(KeyJs::ShiftRight),
    55 => Some(KeyJs::ControlLeft),
    56 => Some(KeyJs::ControlRight),
    57 => Some(KeyJs::AltLeft),
    58 => Some(KeyJs::AltRight),
    59 => Some(KeyJs::MetaLeft),
    60 => Some(KeyJs::MetaRight),
    61 => Some(KeyJs::CapsLock),
    62 => Some(KeyJs::Delete),
    63 => Some(KeyJs::ArrowLeft),
    64 => Some(KeyJs::ArrowRight),
    65 => Some(KeyJs::ArrowUp),
    66 => Some(KeyJs::ArrowDown),
    67 => Some(KeyJs::Unknown),
    68 => Some(KeyJs::Insert),
    69 => Some(KeyJs::Home),
    70 => Some(KeyJs::End),
    71 => Some(KeyJs::PageUp),
    72 => Some(KeyJs::PageDown),
    73 => Some(KeyJs::NumLock),
    74 => Some(KeyJs::ScrollLock),
    75 => Some(KeyJs::PrintScreen),
    76 => Some(KeyJs::Pause),
    77 => Some(KeyJs::Grave),
    78 => Some(KeyJs::Minus),
    79 => Some(KeyJs::Equal),
    80 => Some(KeyJs::BracketLeft),
    81 => Some(KeyJs::BracketRight),
    82 => Some(KeyJs::Backslash),
    83 => Some(KeyJs::Semicolon),
    84 => Some(KeyJs::Quote),
    85 => Some(KeyJs::Comma),
    86 => Some(KeyJs::Period),
    87 => Some(KeyJs::Slash),
    88 => Some(KeyJs::F13),
    89 => Some(KeyJs::F14),
    90 => Some(KeyJs::F15),
    91 => Some(KeyJs::F16),
    92 => Some(KeyJs::F17),
    93 => Some(KeyJs::F18),
    94 => Some(KeyJs::F19),
    95 => Some(KeyJs::F20),
    96 => Some(KeyJs::F21),
    97 => Some(KeyJs::F22),
    98 => Some(KeyJs::F23),
    99 => Some(KeyJs::F24),
    100 => Some(KeyJs::Numpad0),
    101 => Some(KeyJs::Numpad1),
    102 => Some(KeyJs::Numpad2),
    103 => Some(KeyJs::Numpad3),
    104 => Some(KeyJs::Numpad4),
    105 => Some(KeyJs::Numpad5),
    106 => Some(KeyJs::Numpad6),
    107 => Some(KeyJs::Numpad7),
    108 => Some(KeyJs::Numpad8),
    109 => Some(KeyJs::Numpad9),
    110 => Some(KeyJs::NumpadAdd),
    111 => Some(KeyJs::NumpadSubtract),
    112 => Some(KeyJs::NumpadMultiply),
    113 => Some(KeyJs::NumpadDivide),
    114 => Some(KeyJs::NumpadDecimal),
    115 => Some(KeyJs::NumpadEnter),
    116 => Some(KeyJs::NumpadEqual),
    117 => Some(KeyJs::VolumeUp),
    118 => Some(KeyJs::VolumeDown),
    119 => Some(KeyJs::VolumeMute),
    120 => Some(KeyJs::MediaPlayPause),
    121 => Some(KeyJs::MediaStop),
    122 => Some(KeyJs::MediaNext),
    123 => Some(KeyJs::MediaPrevious),
    124 => Some(KeyJs::BrowserBack),
    125 => Some(KeyJs::BrowserForward),
    126 => Some(KeyJs::BrowserRefresh),
    127 => Some(KeyJs::BrowserStop),
    128 => Some(KeyJs::BrowserSearch),
    129 => Some(KeyJs::BrowserFavorites),
    130 => Some(KeyJs::BrowserHome),
    131 => Some(KeyJs::LaunchMail),
    132 => Some(KeyJs::LaunchApp1),
    133 => Some(KeyJs::LaunchApp2),
    134 => Some(KeyJs::IntlBackslash),
    135 => Some(KeyJs::IntlYen),
    136 => Some(KeyJs::IntlRo),
    137 => Some(KeyJs::ContextMenu),
    _ => None,
  }
}

/// Get display info for all known keys.
#[napi]
pub fn get_all_key_display_info() -> Vec<KeyDisplayInfo> {
  (0..KEY_JS_COUNT)
    .filter_map(|i| {
      // Two key_from_i32 calls because KeyJs is not Copy (napi enum).
      // Both are O(1) match lookups — acceptable for a bulk init function.
      let display_name = key_display_name(key_from_i32(i)?).to_string();
      let category = key_category(key_from_i32(i)?).to_string();
      Some(KeyDisplayInfo {
        key: i as u32,
        display_name,
        category,
      })
    })
    .collect()
}

// ============================================================================
// Structs
// ============================================================================

#[napi(object)]
pub struct KeyboardDataJs {
  pub key: KeyJs,
  pub raw_code: u32,
}

#[napi(object)]
pub struct MouseDataJs {
  pub x: f64,
  pub y: f64,
  pub button: Option<ButtonJs>,
}

#[napi]
pub enum ScrollDirectionJs {
  Up,
  Down,
  Left,
  Right,
}

impl From<ScrollDirection> for ScrollDirectionJs {
  fn from(dir: ScrollDirection) -> Self {
    match dir {
      ScrollDirection::Up => ScrollDirectionJs::Up,
      ScrollDirection::Down => ScrollDirectionJs::Down,
      ScrollDirection::Left => ScrollDirectionJs::Left,
      ScrollDirection::Right => ScrollDirectionJs::Right,
    }
  }
}

#[napi(object)]
pub struct WheelDataJs {
  pub x: f64,
  pub y: f64,
  pub direction: ScrollDirectionJs,
  pub delta: f64,
}

#[napi(object)]
pub struct EventJs {
  pub event_type: EventTypeJs,
  pub time: f64,
  pub keyboard: Option<KeyboardDataJs>,
  pub mouse: Option<MouseDataJs>,
  pub wheel: Option<WheelDataJs>,
}

impl From<&Event> for EventJs {
  fn from(event: &Event) -> Self {
    let time = event
      .time
      .duration_since(UNIX_EPOCH)
      .map(|d| d.as_secs_f64())
      .unwrap_or(0.0);

    EventJs {
      event_type: event.event_type.into(),
      time,
      keyboard: event.keyboard.as_ref().map(|kb| KeyboardDataJs {
        key: kb.key.into(),
        raw_code: kb.raw_code,
      }),
      mouse: event.mouse.as_ref().map(|m| MouseDataJs {
        x: m.x,
        y: m.y,
        button: m.button.map(|b| b.into()),
      }),
      wheel: event.wheel.as_ref().map(|w| WheelDataJs {
        x: w.x,
        y: w.y,
        direction: w.direction.into(),
        delta: w.delta,
      }),
    }
  }
}

#[napi(object)]
pub struct RectJs {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl From<&Rect> for RectJs {
  fn from(rect: &Rect) -> Self {
    RectJs {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    }
  }
}

#[napi(object)]
pub struct DisplayInfoJs {
  pub id: u32,
  pub bounds: RectJs,
  pub scale_factor: f64,
  pub refresh_rate: Option<u32>,
  pub is_primary: bool,
}

impl From<&DisplayInfo> for DisplayInfoJs {
  fn from(info: &DisplayInfo) -> Self {
    DisplayInfoJs {
      id: info.id,
      bounds: (&info.bounds).into(),
      scale_factor: info.scale_factor,
      refresh_rate: info.refresh_rate,
      is_primary: info.is_primary,
    }
  }
}

#[napi(object)]
pub struct SystemSettingsJs {
  pub keyboard_repeat_rate: Option<u32>,
  pub keyboard_repeat_delay: Option<u32>,
  pub mouse_sensitivity: Option<f64>,
  pub mouse_acceleration: Option<f64>,
  pub mouse_acceleration_threshold: Option<f64>,
  pub double_click_time: Option<u32>,
  pub keyboard_layout: Option<String>,
}

impl From<&SystemSettings> for SystemSettingsJs {
  fn from(settings: &SystemSettings) -> Self {
    SystemSettingsJs {
      keyboard_repeat_rate: settings.keyboard_repeat_rate,
      keyboard_repeat_delay: settings.keyboard_repeat_delay,
      mouse_sensitivity: settings.mouse_sensitivity,
      mouse_acceleration: settings.mouse_acceleration,
      mouse_acceleration_threshold: settings.mouse_acceleration_threshold,
      double_click_time: settings.double_click_time,
      keyboard_layout: settings.keyboard_layout.clone(),
    }
  }
}

// ============================================================================
// Hook Management
// ============================================================================

#[napi]
pub struct HookJs {
  hook: Arc<Mutex<Option<Hook>>>,
  mask: Arc<AtomicU32>,
}

#[napi]
impl HookJs {
  #[napi]
  pub fn stop(&self) -> Result<()> {
    let mut guard = self.hook.lock().unwrap();
    if let Some(hook) = guard.take() {
      hook.stop().map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to stop hook: {}", e),
        )
      })?;
    }
    Ok(())
  }

  #[napi(getter)]
  pub fn is_running(&self) -> bool {
    let guard = self.hook.lock().unwrap();
    guard.as_ref().is_some_and(|h| h.is_running())
  }

  /// Update the event filter bitmask at runtime without restarting the hook.
  /// Each bit corresponds to an EventTypeJs value (bit 0 = HookEnabled, bit 2 = KeyPressed, etc.).
  /// Set to 0x7FF (2047) for all events, or combine specific bits.
  #[napi]
  pub fn set_event_mask(&self, mask: u32) {
    self.mask.store(mask, Ordering::Relaxed);
  }

  /// Get the current event filter bitmask.
  #[napi(getter)]
  pub fn event_mask(&self) -> u32 {
    self.mask.load(Ordering::Relaxed)
  }
}

// ============================================================================
// Event Listening
// ============================================================================

/// Convert an EventType to its bitmask bit.
fn event_type_bit(et: &EventType) -> u32 {
  match et {
    EventType::HookEnabled => 1 << 0,
    EventType::HookDisabled => 1 << 1,
    EventType::KeyPressed => 1 << 2,
    EventType::KeyReleased => 1 << 3,
    EventType::KeyTyped => 1 << 4,
    EventType::MousePressed => 1 << 5,
    EventType::MouseReleased => 1 << 6,
    EventType::MouseClicked => 1 << 7,
    EventType::MouseMoved => 1 << 8,
    EventType::MouseDragged => 1 << 9,
    EventType::MouseWheel => 1 << 10,
  }
}

/// Predefined event masks for common subscription patterns.
/// Use these with `startListen`'s `eventMask` parameter or `HookJs.setEventMask()`.
///
/// - `EVENT_MASK_ALL` (0x7FF): All events
/// - `EVENT_MASK_KEYBOARD` (0x1C): KeyPressed | KeyReleased | KeyTyped
/// - `EVENT_MASK_MOUSE_BUTTONS` (0xE0): MousePressed | MouseReleased | MouseClicked
/// - `EVENT_MASK_MOUSE_MOVEMENT` (0x300): MouseMoved | MouseDragged
/// - `EVENT_MASK_MOUSE_WHEEL` (0x400): MouseWheel
/// - `EVENT_MASK_MOUSE_ALL` (0x7E0): All mouse events
#[napi]
pub const EVENT_MASK_ALL: u32 = 0x7FF;

#[napi]
pub const EVENT_MASK_KEYBOARD: u32 = (1 << 2) | (1 << 3) | (1 << 4);

#[napi]
pub const EVENT_MASK_MOUSE_BUTTONS: u32 = (1 << 5) | (1 << 6) | (1 << 7);

#[napi]
pub const EVENT_MASK_MOUSE_MOVEMENT: u32 = (1 << 8) | (1 << 9);

#[napi]
pub const EVENT_MASK_MOUSE_WHEEL: u32 = 1 << 10;

#[napi]
pub const EVENT_MASK_MOUSE_ALL: u32 =
  (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10);

/// Check whether a subscription pattern is input-related (keyboard or mouse).
#[napi]
pub fn is_input_pattern(pattern: String) -> bool {
  pattern.starts_with("keyboard:") || pattern.starts_with("mouse:")
}

/// Compute an event mask from a list of subscription pattern strings.
///
/// Recognized patterns:
/// - `"keyboard:*"` or any `"keyboard:..."` → keyboard events
/// - `"mouse:down"`, `"mouse:up"`, `"mouse:click"` → mouse button events
/// - `"mouse:move"` → mouse movement events
/// - `"mouse:scroll"` → mouse wheel events
/// - `"mouse:*"` or other `"mouse:..."` → all mouse events
///
/// Returns `EVENT_MASK_ALL` if no patterns match (safe default).
#[napi]
pub fn compute_event_mask(patterns: Vec<String>) -> u32 {
  let mut mask = 0u32;
  for p in &patterns {
    if p.starts_with("keyboard:") {
      mask |= EVENT_MASK_KEYBOARD;
    } else if p == "mouse:down" || p == "mouse:up" || p == "mouse:click" {
      mask |= EVENT_MASK_MOUSE_BUTTONS;
    } else if p == "mouse:move" {
      mask |= EVENT_MASK_MOUSE_MOVEMENT;
    } else if p == "mouse:scroll" {
      mask |= EVENT_MASK_MOUSE_WHEEL;
    } else if p.starts_with("mouse:") {
      // "mouse:*" or any other mouse pattern → all mouse
      mask |= EVENT_MASK_MOUSE_ALL;
    }
  }
  if mask == 0 {
    EVENT_MASK_ALL
  } else {
    mask
  }
}

/// Start listening for input events with a callback.
/// Returns a HookJs instance that can be used to stop the listener.
///
/// `event_mask` is an optional bitmask that filters events on the native side before
/// crossing the NAPI boundary. This is a performance optimization — high-frequency
/// events like MouseMoved never reach JS if the corresponding bit is not set.
///
/// Use the `EVENT_MASK_*` constants to compose masks. If `None`, all events are forwarded.
/// The mask can be updated at runtime via `HookJs.setEventMask()`.
#[napi(ts_return_type = "HookJs")]
pub fn start_listen(
  #[napi(ts_arg_type = "(event: EventJs) => void")] callback: Function<(), ()>,
  event_mask: Option<u32>,
) -> Result<HookJs> {
  let tsfn = callback
    .build_threadsafe_function()
    .build_callback(|ctx: ThreadsafeCallContext<EventJs>| Ok(vec![ctx.value]))?;

  let mask = Arc::new(AtomicU32::new(event_mask.unwrap_or(EVENT_MASK_ALL)));
  let mask_clone = mask.clone();

  let hook = Hook::new();
  hook
    .run_async(move |event: &Event| {
      // Filter on the Rust side — skip NAPI boundary for unwanted events
      let bit = event_type_bit(&event.event_type);
      if mask_clone.load(Ordering::Relaxed) & bit == 0 {
        return;
      }
      let event_js = EventJs::from(event);
      let _ = tsfn.call(event_js, ThreadsafeFunctionCallMode::NonBlocking);
    })
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to start listener: {}", e),
      )
    })?;

  Ok(HookJs {
    hook: Arc::new(Mutex::new(Some(hook))),
    mask,
  })
}

// ============================================================================
// EventEmitter-style InputHook (per-event-type callbacks, Rust-side dispatch)
// ============================================================================

/// Keyboard event payload for onKeyDown / onKeyUp callbacks.
#[napi(object)]
pub struct KeyboardEventJs {
  pub key: KeyJs,
  pub raw_code: u32,
  pub time: f64,
}

/// Mouse button event payload for onMouseDown / onMouseUp / onClick callbacks.
#[napi(object)]
pub struct MouseButtonEventJs {
  pub x: f64,
  pub y: f64,
  pub button: ButtonJs,
  pub time: f64,
}

/// Mouse move event payload for onMouseMove callbacks.
#[napi(object)]
pub struct MouseMoveEventJs {
  pub x: f64,
  pub y: f64,
  pub time: f64,
}

/// Wheel event payload for onWheel callbacks.
#[napi(object)]
pub struct WheelEventJs {
  pub x: f64,
  pub y: f64,
  pub direction: ScrollDirectionJs,
  pub delta: f64,
  pub time: f64,
}

// Type aliases for the per-event threadsafe functions.
// Each TSFN carries its own typed payload, avoiding the generic EventJs.
// build_callback() produces: ThreadsafeFunction<T, (), Vec<T>, Status, false>
type KeyboardTsFn = ThreadsafeFunction<KeyboardEventJs, (), Vec<KeyboardEventJs>, Status, false>;
type MouseButtonTsFn =
  ThreadsafeFunction<MouseButtonEventJs, (), Vec<MouseButtonEventJs>, Status, false>;
type MouseMoveTsFn = ThreadsafeFunction<MouseMoveEventJs, (), Vec<MouseMoveEventJs>, Status, false>;
type WheelTsFn = ThreadsafeFunction<WheelEventJs, (), Vec<WheelEventJs>, Status, false>;

/// Internal storage for per-event-type callbacks.
struct InputHookCallbacks {
  key_down: Option<KeyboardTsFn>,
  key_up: Option<KeyboardTsFn>,
  mouse_down: Option<MouseButtonTsFn>,
  mouse_up: Option<MouseButtonTsFn>,
  mouse_click: Option<MouseButtonTsFn>,
  mouse_move: Option<MouseMoveTsFn>,
  mouse_wheel: Option<WheelTsFn>,
}

// SAFETY: All fields are Option<ThreadsafeFunction<...>>, which is designed for
// cross-thread use. If a non-Send/Sync field is ever added to this struct,
// these impls must be revisited — the compiler will NOT catch the violation.
unsafe impl Send for InputHookCallbacks {}
unsafe impl Sync for InputHookCallbacks {}

impl InputHookCallbacks {
  fn new() -> Self {
    Self {
      key_down: None,
      key_up: None,
      mouse_down: None,
      mouse_up: None,
      mouse_click: None,
      mouse_move: None,
      mouse_wheel: None,
    }
  }

  /// Compute the event mask from which callbacks are registered.
  fn compute_mask(&self) -> u32 {
    let mut mask = 0u32;
    if self.key_down.is_some() {
      mask |= 1 << 2;
    } // KeyPressed
    if self.key_up.is_some() {
      mask |= 1 << 3;
    } // KeyReleased
    if self.mouse_down.is_some() {
      mask |= 1 << 5;
    } // MousePressed
    if self.mouse_up.is_some() {
      mask |= 1 << 6;
    } // MouseReleased
    if self.mouse_click.is_some() {
      mask |= 1 << 7;
    } // MouseClicked
    if self.mouse_move.is_some() {
      mask |= (1 << 8) | (1 << 9);
    } // MouseMoved | MouseDragged
    if self.mouse_wheel.is_some() {
      mask |= 1 << 10;
    } // MouseWheel
    mask
  }
}

/// EventEmitter-style input hook with per-event-type callbacks.
///
/// Unlike `startListen()` which sends all events through a single callback,
/// `InputHook` dispatches events to typed callbacks registered via `onKeyDown()`,
/// `onMouseMove()`, etc. Only registered event types cross the NAPI boundary —
/// the event mask is computed automatically from which callbacks are set.
///
/// ```js
/// const hook = new InputHook();
/// hook.onKeyDown((data) => console.log("key:", data.key, data.rawCode));
/// hook.onMouseMove((data) => console.log("mouse:", data.x, data.y));
/// hook.start();
/// // ... later:
/// hook.stop();
/// ```
#[napi]
pub struct InputHook {
  hook: Arc<Mutex<Option<Hook>>>,
  callbacks: Arc<Mutex<InputHookCallbacks>>,
  mask: Arc<AtomicU32>,
}

impl Default for InputHook {
  fn default() -> Self {
    Self::new()
  }
}

#[napi]
impl InputHook {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      hook: Arc::new(Mutex::new(None)),
      callbacks: Arc::new(Mutex::new(InputHookCallbacks::new())),
      mask: Arc::new(AtomicU32::new(0)),
    }
  }

  // ─── Per-event-type callback registration ──────────────────────────

  #[napi]
  pub fn on_key_down(
    &self,
    #[napi(ts_arg_type = "(data: KeyboardEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<KeyboardEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.key_down = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_key_up(
    &self,
    #[napi(ts_arg_type = "(data: KeyboardEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<KeyboardEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.key_up = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_mouse_down(
    &self,
    #[napi(ts_arg_type = "(data: MouseButtonEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<MouseButtonEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_down = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_mouse_up(
    &self,
    #[napi(ts_arg_type = "(data: MouseButtonEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<MouseButtonEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_up = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_click(
    &self,
    #[napi(ts_arg_type = "(data: MouseButtonEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<MouseButtonEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_click = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_mouse_move(
    &self,
    #[napi(ts_arg_type = "(data: MouseMoveEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<MouseMoveEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_move = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  #[napi]
  pub fn on_wheel(
    &self,
    #[napi(ts_arg_type = "(data: WheelEventJs) => void")] callback: Function<(), ()>,
  ) -> Result<()> {
    let tsfn = callback
      .build_threadsafe_function()
      .build_callback(|ctx: ThreadsafeCallContext<WheelEventJs>| Ok(vec![ctx.value]))?;
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_wheel = Some(tsfn);
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
    Ok(())
  }

  // ─── Callback removal ──────────────────────────────────────────────

  #[napi]
  pub fn off_key_down(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.key_down = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_key_up(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.key_up = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_mouse_down(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_down = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_mouse_up(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_up = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_click(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_click = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_mouse_move(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_move = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn off_wheel(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    cbs.mouse_wheel = None;
    self.mask.store(cbs.compute_mask(), Ordering::Relaxed);
  }

  #[napi]
  pub fn remove_all_listeners(&self) {
    let mut cbs = self.callbacks.lock().unwrap();
    *cbs = InputHookCallbacks::new();
    self.mask.store(0, Ordering::Relaxed);
  }

  // ─── Lifecycle ─────────────────────────────────────────────────────

  #[napi]
  pub fn start(&self) -> Result<()> {
    let mut hook_guard = self.hook.lock().unwrap();
    if hook_guard.is_some() {
      return Err(Error::new(
        Status::GenericFailure,
        "Hook is already running",
      ));
    }

    let callbacks = self.callbacks.clone();
    let mask = self.mask.clone();

    let hook = Hook::new();
    hook
      .run_async(move |event: &Event| {
        // Check the mask BEFORE acquiring the lock
        let bit = event_type_bit(&event.event_type);
        if mask.load(Ordering::Relaxed) & bit == 0 {
          return;
        }

        let time = event
          .time
          .duration_since(UNIX_EPOCH)
          .map(|d| d.as_secs_f64())
          .unwrap_or(0.0);

        let cbs = callbacks.lock().unwrap();

        match event.event_type {
          EventType::KeyPressed => {
            if let (Some(ref tsfn), Some(ref kb)) = (&cbs.key_down, &event.keyboard) {
              let data = KeyboardEventJs {
                key: kb.key.into(),
                raw_code: kb.raw_code,
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::KeyReleased => {
            if let (Some(ref tsfn), Some(ref kb)) = (&cbs.key_up, &event.keyboard) {
              let data = KeyboardEventJs {
                key: kb.key.into(),
                raw_code: kb.raw_code,
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::MousePressed => {
            if let (Some(ref tsfn), Some(ref m)) = (&cbs.mouse_down, &event.mouse) {
              let data = MouseButtonEventJs {
                x: m.x,
                y: m.y,
                button: m.button.unwrap_or(Button::Left).into(),
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::MouseReleased => {
            if let (Some(ref tsfn), Some(ref m)) = (&cbs.mouse_up, &event.mouse) {
              let data = MouseButtonEventJs {
                x: m.x,
                y: m.y,
                button: m.button.unwrap_or(Button::Left).into(),
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::MouseClicked => {
            if let (Some(ref tsfn), Some(ref m)) = (&cbs.mouse_click, &event.mouse) {
              let data = MouseButtonEventJs {
                x: m.x,
                y: m.y,
                button: m.button.unwrap_or(Button::Left).into(),
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::MouseMoved | EventType::MouseDragged => {
            if let (Some(ref tsfn), Some(ref m)) = (&cbs.mouse_move, &event.mouse) {
              let data = MouseMoveEventJs {
                x: m.x,
                y: m.y,
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          EventType::MouseWheel => {
            if let (Some(ref tsfn), Some(ref w)) = (&cbs.mouse_wheel, &event.wheel) {
              let data = WheelEventJs {
                x: w.x,
                y: w.y,
                direction: w.direction.into(),
                delta: w.delta,
                time,
              };
              let _ = tsfn.call(data, ThreadsafeFunctionCallMode::NonBlocking);
            }
          }
          _ => {} // HookEnabled, HookDisabled, KeyTyped — ignored
        }
      })
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to start hook: {}", e),
        )
      })?;

    *hook_guard = Some(hook);
    Ok(())
  }

  #[napi]
  pub fn stop(&self) -> Result<()> {
    let mut hook_guard = self.hook.lock().unwrap();
    if let Some(hook) = hook_guard.take() {
      hook.stop().map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to stop hook: {}", e),
        )
      })?;
    }
    Ok(())
  }

  #[napi(getter)]
  pub fn is_running(&self) -> bool {
    let guard = self.hook.lock().unwrap();
    guard.as_ref().is_some_and(|h| h.is_running())
  }

  #[napi(getter)]
  pub fn event_mask(&self) -> u32 {
    self.mask.load(Ordering::Relaxed)
  }
}

// ============================================================================
// Display Functions
// ============================================================================

/// Get all displays
#[napi]
pub fn get_displays() -> Result<Vec<DisplayInfoJs>> {
  displays()
    .map(|infos| infos.iter().map(|info| info.into()).collect())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get displays: {}", e),
      )
    })
}

/// Get the primary display
#[napi]
pub fn get_primary_display() -> Result<DisplayInfoJs> {
  primary_display().map(|info| (&info).into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to get primary display: {}", e),
    )
  })
}

/// Get display at a specific point
#[napi]
pub fn get_display_at_point(x: f64, y: f64) -> Result<Option<DisplayInfoJs>> {
  display_at_point(x, y)
    .map(|opt| opt.as_ref().map(|info| info.into()))
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get display at point: {}", e),
      )
    })
}

/// Get system settings
#[napi]
pub fn get_system_settings() -> Result<SystemSettingsJs> {
  system_settings()
    .map(|settings| (&settings).into())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get system settings: {}", e),
      )
    })
}

// ============================================================================
// Event Simulation
// ============================================================================

/// Move the mouse to absolute coordinates
#[napi]
pub fn simulate_mouse_move(x: f64, y: f64) -> Result<()> {
  mouse_move(x, y).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to move mouse: {}", e),
    )
  })
}

/// Press a mouse button
#[napi]
pub fn simulate_mouse_press(button: ButtonJs) -> Result<()> {
  mouse_press(button.into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to press mouse button: {}", e),
    )
  })
}

/// Release a mouse button
#[napi]
pub fn simulate_mouse_release(button: ButtonJs) -> Result<()> {
  mouse_release(button.into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to release mouse button: {}", e),
    )
  })
}

/// Click a mouse button (press + release)
#[napi]
pub fn simulate_mouse_click(button: ButtonJs) -> Result<()> {
  mouse_click(button.into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to click mouse button: {}", e),
    )
  })
}

/// Press a key
#[napi]
pub fn simulate_key_press(key: KeyJs) -> Result<()> {
  key_press(key.into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to press key: {}", e),
    )
  })
}

/// Release a key
#[napi]
pub fn simulate_key_release(key: KeyJs) -> Result<()> {
  key_release(key.into()).map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to release key: {}", e),
    )
  })
}

/// Tap a key (press + release)
#[napi]
pub fn simulate_key_tap(key: KeyJs) -> Result<()> {
  key_tap(key.into())
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to tap key: {}", e)))
}

/// Get the current mouse cursor position
#[napi]
pub fn get_mouse_position() -> Result<MouseDataJs> {
  let (x, y) = mouse_position().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to get mouse position: {}", e),
    )
  })?;
  Ok(MouseDataJs { x, y, button: None })
}
