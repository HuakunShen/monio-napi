import { getDisplays, getPrimaryDisplay, startListen } from '../index'

const displays = getDisplays()
console.log(displays)
const hook = startListen((event) => {
  console.log(event)
  /* ... */
})
setTimeout(() => {
  hook.stop()
}, 5000)
