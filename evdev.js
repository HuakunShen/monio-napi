'use strict'

if (process.platform !== 'linux' || process.arch !== 'x64') {
  throw new Error('monio-napi/evdev requires Linux x64 with glibc')
}

const binary = 'monio-napi.evdev.linux-x64-gnu.node'
let bindingPath
try {
  bindingPath = require.resolve(`./${binary}`)
} catch (error) {
  if (error.code !== 'MODULE_NOT_FOUND') throw error
  try {
    bindingPath = require.resolve(`monio-napi-linux-x64-gnu/${binary}`)
  } catch (error) {
    if (error.code !== 'MODULE_NOT_FOUND') throw error
    throw new Error('monio-napi/evdev binary is missing. Install with optional dependencies or run pnpm build:evdev.')
  }
}

const binding = require(bindingPath)

exports.startListen = binding.startListen
exports.simulateKeyPress = binding.simulateKeyPress
exports.simulateKeyRelease = binding.simulateKeyRelease
exports.simulateKeyTap = binding.simulateKeyTap
exports.simulateMousePress = binding.simulateMousePress
exports.simulateMouseRelease = binding.simulateMouseRelease
exports.simulateMouseClick = binding.simulateMouseClick
exports.simulateMouseMoveRelative = binding.simulateMouseMoveRelative
exports.EventTypeJs = binding.EventTypeJs
exports.KeyJs = binding.KeyJs
exports.ButtonJs = binding.ButtonJs
exports.ScrollDirectionJs = binding.ScrollDirectionJs
exports.computeEventMask = binding.computeEventMask
exports.EVENT_MASK_ALL = binding.EVENT_MASK_ALL
exports.EVENT_MASK_KEYBOARD = binding.EVENT_MASK_KEYBOARD
exports.EVENT_MASK_MOUSE_ALL = binding.EVENT_MASK_MOUSE_ALL
exports.EVENT_MASK_MOUSE_BUTTONS = binding.EVENT_MASK_MOUSE_BUTTONS
exports.EVENT_MASK_MOUSE_MOVEMENT = binding.EVENT_MASK_MOUSE_MOVEMENT
exports.EVENT_MASK_MOUSE_WHEEL = binding.EVENT_MASK_MOUSE_WHEEL
