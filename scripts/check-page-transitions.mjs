import assert from 'node:assert/strict'
import { createServer } from 'vite'
import { createSSRApp, effectScope, nextTick, shallowRef } from 'vue'
import { createMemoryHistory, createRouter } from 'vue-router'

const server = await createServer({
  server: { middlewareMode: true, hmr: false, watch: null },
  appType: 'custom',
})
const scope = effectScope()
try {
  const { usePageTransition } = await server.ssrLoadModule('/src/composables/usePageTransition.ts')
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      ...['a', 'b', 'c'].map((path) => ({ path: `/${path}`, component: {} })),
      {
        path: '/group',
        component: {},
        children: [
          { path: 'one', component: {} },
          { path: 'two', component: {} },
        ],
      },
    ],
  })
  const app = createSSRApp({})
  app.use(router)
  await router.push('/a')
  const animations = []
  const timers = []
  let reducedMotion = false
  let expectedDelay = 110
  globalThis.window = {
    matchMedia: () => ({ matches: reducedMotion }),
    setTimeout: (callback, delay) => {
      assert.equal(delay, expectedDelay)
      timers.push(callback)
    },
  }
  globalThis.getComputedStyle = () => ({ opacity: '1', transform: 'none' })
  const block = () => ({
    children: [],
    matches: () => true,
    animate: (frames, options) => {
      const animation = {
        frames,
        options,
        cancelled: false,
        cancel() {
          this.cancelled = true
        },
      }
      animations.push(animation)
      return animation
    },
  })
  const content = { style: { pointerEvents: '' }, children: [block(), block(), block()] }
  const page = shallowRef({ querySelector: () => content })
  app.runWithContext(() => scope.run(() => usePageTransition(page)))
  const flush = () => new Promise((resolve) => setImmediate(resolve))
  const finish = () => timers.splice(0).forEach((callback) => callback())

  const first = router.push('/b')
  await flush()
  assert.equal(router.currentRoute.value.path, '/a', 'keep outgoing content until exit finishes')
  assert.equal(content.style.pointerEvents, 'none')
  assert.deepEqual(
    animations.map(({ options }) => options.delay),
    [0, 15, 30],
  )
  assert.ok(
    animations.every(
      ({ options, frames }) =>
        options.duration === 70 &&
        frames[1].opacity === 0 &&
        frames[1].transform.trim() === 'translateY(-6px)',
    ),
  )
  const second = router.push('/c')
  await flush()
  assert.equal(animations.length, 3, 'rapid navigation shares one exit')
  finish()
  await Promise.all([first, second])
  await nextTick()
  assert.equal(router.currentRoute.value.path, '/c')
  assert.equal(content.style.pointerEvents, '')
  assert.ok(animations.every(({ cancelled }) => cancelled))

  const pending = router.push('/b')
  await flush()
  await router.push('/c')
  await nextTick()
  assert.equal(content.style.pointerEvents, '', 'duplicate navigation restores the current page')
  finish()
  await pending

  const removeAbort = router.beforeResolve(() => false)
  const aborted = router.push('/b')
  await flush()
  finish()
  await aborted
  await nextTick()
  assert.equal(router.currentRoute.value.path, '/c')
  assert.equal(content.style.pointerEvents, '', 'aborted navigation restores interaction')
  assert.ok(animations.every(({ cancelled }) => cancelled))
  removeAbort()

  const count = animations.length
  await router.push('/c?filter=test')
  reducedMotion = true
  await router.push('/a')
  assert.equal(animations.length, count, 'query updates and reduced motion skip the exit')
  reducedMotion = false

  const standalone = { ...block(), style: { pointerEvents: '' } }
  page.value = { querySelector: (selector) => (selector.includes('.subview') ? null : standalone) }
  const fullPage = router.push('/b')
  await flush()
  assert.equal(animations.length, count + 1, 'standalone content such as the editor also exits')
  finish()
  await fullPage
  await nextTick()
  assert.equal(standalone.style.pointerEvents, '')

  const enterGroup = router.push('/group/one')
  await flush()
  finish()
  await enterGroup
  expectedDelay = 130
  const subpage = router.push('/group/two')
  await flush()
  finish()
  await subpage
  console.log(
    'Page exit timing, rapid/duplicate/aborted navigation, full pages and reduced motion passed.',
  )
} finally {
  scope.stop()
  delete globalThis.window
  delete globalThis.getComputedStyle
  await server.close()
}
