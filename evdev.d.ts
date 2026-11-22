export {
  ButtonJs,
  computeEventMask,
  EVENT_MASK_ALL,
  EVENT_MASK_KEYBOARD,
  EVENT_MASK_MOUSE_ALL,
  EVENT_MASK_MOUSE_BUTTONS,
  EVENT_MASK_MOUSE_MOVEMENT,
  EVENT_MASK_MOUSE_WHEEL,
  EventTypeJs,
  KeyJs,
  ScrollDirectionJs,
  simulateKeyPress,
  simulateKeyRelease,
  simulateKeyTap,
  simulateMouseClick,
  simulateMousePress,
  simulateMouseRelease,
  startListen,
} from './index.js'
export type { EventJs, HookJs } from './index.js'

/** Move by signed 32-bit relative input counts. The desktop applies pointer acceleration and scaling. */
export declare function simulateMouseMoveRelative(deltaX: number, deltaY: number): void
