import test from 'ava'
import { getDisplays, getPrimaryDisplay, startListen } from '../index'

test('sync function from native code', (t) => {
  t.is(1, 1)
})
