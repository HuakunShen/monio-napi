import { getDisplays, getPrimaryDisplay, startListen, getMousePosition, getDisplayAtPoint } from '../index'

const displays = getDisplays()
console.log(displays)

const mousePos = getMousePosition()
console.log(mousePos)
const display = getDisplayAtPoint(mousePos.x, mousePos.y)
console.log(display)
// const hook = startListen((event) => {
//   console.log(event)
//   /* ... */
// })
// setTimeout(() => {
//   hook.stop()
// }, 5000)
