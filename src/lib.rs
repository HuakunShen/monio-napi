#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeCallContext, ThreadsafeFunctionCallMode};
use napi_derive::napi;
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

  // Unknown
  Unknown,
}

// Conversion functions for KeyJs (truncated for brevity - you'd add all keys)
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
      _ => KeyJs::Unknown,
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
      KeyJs::Unknown => Key::Unknown(0),
    }
  }
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
}

// ============================================================================
// Event Listening
// ============================================================================

/// Start listening for input events with a callback.
/// Returns a HookJs instance that can be used to stop the listener.
#[napi(ts_return_type = "HookJs")]
pub fn start_listen(
  #[napi(ts_arg_type = "(event: EventJs) => void")] callback: Function<(), ()>,
) -> Result<HookJs> {
  let tsfn = callback
    .build_threadsafe_function()
    .build_callback(|ctx: ThreadsafeCallContext<EventJs>| Ok(vec![ctx.value]))?;

  let hook = Hook::new();
  hook
    .run_async(move |event: &Event| {
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
  })
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
