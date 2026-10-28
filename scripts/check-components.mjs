import assert from 'node:assert/strict'
import { createServer } from 'vite'
import { createSSRApp, effectScope, h, nextTick } from 'vue'
import { createPinia, setActivePinia } from 'pinia'
import { renderToString } from 'vue/server-renderer'

const server = await createServer({
  server: { middlewareMode: true, hmr: false, watch: null },
  appType: 'custom',
})
const savedStorage = Object.getOwnPropertyDescriptor(globalThis, 'localStorage')
const storage = new Map()
Object.defineProperty(globalThis, 'localStorage', {
  configurable: true,
  value: {
    getItem: (key) => storage.get(key) ?? null,
    setItem: (key, value) => storage.set(key, value),
  },
})
const scopes = []
try {
  const load = async (name) =>
    (await server.ssrLoadModule(`/src/components/widget/${name}.vue`)).default
  const [PTextBox, PComboBox, PCheckBox, SetupItem] = await Promise.all(
    ['PTextBox', 'PComboBox', 'PCheckBox', 'SetupItem'].map(load),
  )
  for (const [control, extra, tag] of [
    [PTextBox, {}, 'input'],
    [PTextBox, { multiline: true }, 'textarea'],
    [PComboBox, { options: [{ key: 'a', text: 'A' }] }, 'select'],
  ]) {
    const html = await renderToString(
      createSSRApp({
        render: () =>
          h(control, {
            ...extra,
            id: 'native-control',
            name: 'setting',
            class: 'outer',
            style: { margin: '5px' },
            'aria-labelledby': 'label',
            'data-control': 'source-id',
            required: true,
            disabled: true,
          }),
      }),
    )
    const native = html.match(new RegExp(`<${tag}[^>]*>`))[0]
    assert.match(native, /id="native-control"/)
    assert.match(native, /name="setting"/)
    assert.match(native, /aria-labelledby="label"/)
    assert.match(native, / required/)
    assert.match(native, / disabled/)
    assert.doesNotMatch(native, /class="outer"|margin:|data-control/)
    assert.match(html, /class="[^"]*outer"[^>]*style="margin:5px;"[^>]*data-control="source-id"/)
  }
  const rows = await renderToString(
    createSSRApp({
      render: () =>
        h('div', [
          h(SetupItem, { label: 'Name' }, { default: ({ id }) => h(PTextBox, { inputId: id }) }),
          h(
            SetupItem,
            { label: 'Mode' },
            { default: ({ id }) => h(PComboBox, { inputId: id, options: [] }) },
          ),
          h(PCheckBox, { label: 'First' }),
          h(PCheckBox, { label: 'Second' }),
        ]),
    }),
  )
  const ids = [...rows.matchAll(/\bid="([^"]+)"/g)].map((match) => match[1])
  const labels = [...rows.matchAll(/\bfor="([^"]+)"/g)].map((match) => match[1])
  assert.equal(new Set(ids).size, 4, 'each native control gets a unique ID')
  assert.deepEqual(labels, ids, 'labels target the native controls')

  const { usePageSettings } = await server.ssrLoadModule('/src/composables/usePageSettings.ts')
  const { useSelectedInstance } = await server.ssrLoadModule('/src/stores/gameLaunch.ts')
  const { default: sourceDefaults } = await server.ssrLoadModule(
    '/src/assets/setting-defaults.json',
  )
  setActivePinia(createPinia())
  const open = (page, defaults) => {
    const scope = effectScope()
    scopes.push(scope)
    return scope.run(() => usePageSettings(page, defaults))
  }
  const key = 'pcl-ui:PageSetupLaunch:global'
  storage.set(
    key,
    JSON.stringify({
      TextArgumentTitle: 'Saved',
      CheckAdvanceRunWait: 'wrong-type',
      Password: 'private',
      nested: {},
      list: [],
    }),
  )
  const settings = open('PageSetupLaunch', { TextArgumentInfo: 'Runtime info' })
  assert.equal(settings.values.TextArgumentTitle, 'Saved')
  assert.equal(settings.values.TextArgumentInfo, 'Runtime info')
  assert.equal(
    settings.values.CheckAdvanceRunWait,
    sourceDefaults.PageSetupLaunch.CheckAdvanceRunWait,
  )
  assert.equal(settings.values.Password, undefined)
  assert.equal(settings.values.nested, undefined)
  assert.equal(settings.values.list, undefined)
  settings.values.TextArgumentTitle = 'Changed'
  settings.values.Token = 'private'
  await nextTick()
  assert.equal(JSON.parse(storage.get(key)).TextArgumentTitle, 'Changed')
  assert.equal(JSON.parse(storage.get(key)).Token, undefined)
  settings.reset('TextArgumentTitle')
  assert.equal(settings.values.TextArgumentTitle, sourceDefaults.PageSetupLaunch.TextArgumentTitle)
  settings.reset()
  assert.equal(settings.values.TextArgumentInfo, 'Runtime info')
  assert.equal(settings.values.Token, undefined)
  for (const invalid of ['{broken', 'null', '[]']) {
    storage.set('pcl-ui:Invalid:global', invalid)
    assert.equal(open('Invalid', { enabled: true }).values.enabled, true)
  }
  const instance = useSelectedInstance()
  instance.instance_info = { id: 'first' }
  const scoped = open('PageInstanceSetup')
  scoped.values.TextArgumentInfo = 'First instance'
  await nextTick()
  storage.set(
    'pcl-ui:PageInstanceSetup:second',
    JSON.stringify({ TextArgumentInfo: 'Second instance' }),
  )
  instance.instance_info = { id: 'second' }
  await nextTick()
  assert.equal(scoped.values.TextArgumentInfo, 'Second instance')
  scoped.values.TextArgumentInfo = 'Second edited'
  await nextTick()
  instance.instance_info = { id: 'first' }
  await nextTick()
  assert.equal(scoped.values.TextArgumentInfo, 'First instance')
  assert.equal(
    JSON.parse(storage.get('pcl-ui:PageInstanceSetup:second')).TextArgumentInfo,
    'Second edited',
  )
  console.log(
    'Component attributes, labels, settings persistence/reset and instance isolation passed.',
  )
} finally {
  scopes.forEach((scope) => scope.stop())
  if (savedStorage) Object.defineProperty(globalThis, 'localStorage', savedStorage)
  else delete globalThis.localStorage
  await server.close()
}
