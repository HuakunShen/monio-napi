---
name: monio-napi
description: Use monio-napi for Node.js global input hooks, drag-aware mouse events, display queries, and keyboard/mouse simulation.
---

# monio-napi

Use this skill when working with the [`monio-napi`](https://www.npmjs.com/package/monio-napi) Node.js package for cross-platform input monitoring and event simulation.

Start by checking `README.md` and `examples/monio.ts` for baseline usage and platform behavior before proposing custom event flow.

## Core APIs

- `startListen(callback, eventMask?)` for callback-based global input listening.
- `HookJs.stop()` to stop a running listener.
- `HookJs.setEventMask(mask)` to change filtered event types at runtime.
- `EVENT_MASK_ALL`, `EVENT_MASK_KEYBOARD`, `EVENT_MASK_MOUSE_BUTTONS`, `EVENT_MASK_MOUSE_MOVEMENT`, `EVENT_MASK_MOUSE_WHEEL`, `EVENT_MASK_MOUSE_ALL`.
- `computeEventMask(patterns)` for deriving masks from string patterns.
- `EventTypeJs` and `EventJs` for event-type switching.
- `InputHook` for per-event-type listeners (`onKeyDown`, `onMouseMove`, `onWheel`, etc.).
- `InputHook` lifecycle: `start()`, `stop()`, `isRunning`, and callback removal (`off...`, `removeAllListeners()`).
- Keyboard simulation helpers: `simulateKeyPress`, `simulateKeyRelease`, `simulateKeyTap`.
- Mouse simulation helpers: `simulateMouseMove`, `simulateMousePress`, `simulateMouseRelease`, `simulateMouseClick`.
- Display/system helpers: `getDisplays`, `getPrimaryDisplay`, `getDisplayAtPoint`, `getSystemSettings`, `getMousePosition`.
- Key/button metadata helpers: `getKeyDisplayName`, `getButtonDisplayName`, `getKeyCategory`, `isModifierKey`, `getAllKeyDisplayInfo`.
- Button/key enums are `ButtonJs` and `KeyJs`.

## Example-guided workflows

1. Basic callback listener: use `examples/monio.ts` and call `startListen`, inspect `event.eventType`, then branch on `EventTypeJs` values.
2. Selective callback hooks: use `InputHook` only for needed callbacks and call `start()` for typed dispatch.
3. Performance-oriented filtering: pass `computeEventMask(...)` output to `startListen` to avoid crossing the NAPI boundary for noisy event types.
4. Mouse/keyboard simulation: use `simulateMouseMove`, `simulateMouseClick`, and `simulateKeyTap` for deterministic automation tasks.
5. Display and input state queries: use `getDisplays`, `getDisplayAtPoint`, `getPrimaryDisplay`, `getMousePosition`, and `getSystemSettings`.

## Quick snippets

```js
import { startListen, EventTypeJs } from 'monio-napi'

const hook = startListen((event) => {
  if (event.eventType === EventTypeJs.MouseDragged) {
    console.log('Dragging at', event.mouse?.x, event.mouse?.y)
  }
})

setTimeout(() => hook.stop(), 3000)
```

```js
import { InputHook, EventTypeJs } from 'monio-napi'

const hook = new InputHook()

hook.onMouseMove((data) => {
  console.log('Move', data.x, data.y)
})
hook.onKeyDown((data) => {
  console.log('KeyDown', data.key, data.rawCode)
})
hook.start()

setTimeout(() => hook.stop(), 5000)
```
