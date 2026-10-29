import assert from 'node:assert/strict'
import { createServer } from 'vite'
import { createSSRApp, effectScope, h, nextTick } from 'vue'
import { createPinia, setActivePinia } from 'pinia'
import { renderToString } from 'vue/server-renderer'
import { createRouter, createMemoryHistory } from 'vue-router'

const server = await createServer({
  server: { middlewareMode: true, hmr: false, watch: null },
  appType: 'custom',
})
const savedStorage = Object.getOwnPropertyDescriptor(globalThis, 'localStorage')
const savedWindow = Object.getOwnPropertyDescriptor(globalThis, 'window')
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
  const PListItem = await load('PListItem')
  for (const selected of [undefined, false, true]) {
    const html = await renderToString(
      createSSRApp({
        render: () =>
          h(PListItem, {
            title: 'JDK 25',
            tags: ['64 Bit', 'ARM64', 'Eclipse'],
            selected,
            clickable: true,
          }),
      }),
    )
    if (selected === undefined) assert.doesNotMatch(html, /aria-pressed|class="[^"]*selectable/)
    else assert.match(html, new RegExp(`aria-pressed="${selected}"`))
    assert.match(html, /class="list-tag"[^>]*>ARM64<\/span>/)
    assert.doesNotMatch(html, /<input/)
  }
  const PVersionActions = await load('PVersionActions')
  for (const manual of [false, true]) {
    const html = await renderToString(
      createSSRApp({ render: () => h(PVersionActions, { versionId: '1.21.1', manual }) }),
    )
    assert.equal([...html.matchAll(/<button\b/g)].length, manual ? 2 : 3)
    assert.match(html, /aria-label="更新日志"/)
    assert.match(html, /aria-label="下载服务端"/)
    if (!manual) assert.match(html, /aria-label="另存为"/)
  }

  const { compTags, compTagOptions, compFacets, compVersions, latestCompFile } =
    await server.ssrLoadModule('/src/api/compResources.ts')
  const project = {
    project_id: 'sodium',
    title: 'Sodium',
    description: 'Fast rendering',
    icon_url: '',
    project_type: 'mod',
    downloads: 100,
    date_modified: '2026-10-01',
    categories: ['fabric', 'optimization', 'decoration', 'optimization'],
    versions: ['1.19.4', '1.20.1', '1.21.1'],
  }
  assert.deepEqual(compTags(project), ['性能优化', '装饰'])
  assert.deepEqual(
    compTags({ ...project, project_type: 'resourcepack', categories: ['decoration', '16x'] }),
    ['16x'],
  )
  assert.deepEqual(compTags({ ...project, categories: ['fabric', 'unknown'] }), ['其他'])
  assert.deepEqual(compTags({ ...project, project_type: 'shader', categories: ['iris', 'high'] }), [
    'Iris',
  ])
  assert.ok(compTagOptions('mod').some((option) => option.key === '6814/optimization'))
  assert.ok(compTagOptions('mod').every((option) => !option.key || option.key.split('/')[1]))
  const headings = compTagOptions('resourcepack').filter((option) => option.disabled)
  assert.equal(headings.length, 3)
  assert.equal(new Set(headings.map((option) => option.key)).size, headings.length)
  assert.deepEqual(
    compFacets('mod', { loader: 'fabric', version: ' 1.21.1 ', tag: '6814/optimization' }),
    [['project_type:mod'], ['categories:fabric'], ['versions:1.21.1'], ['categories:optimization']],
  )
  assert.deepEqual(compFacets('datapack', { loader: 'fabric', version: '', tag: '/adventure' }), [
    ['project_type:mod'],
    ['categories:datapack'],
    ['categories:adventure'],
  ])
  assert.equal(compVersions(project.versions), '1.19~1.21')
  assert.equal(compVersions(['1.21.1', '1.21.4', '1.19.2']), '1.21, 1.19')
  assert.equal(compVersions(['26.1', '26.2', '1.21.1']), '26.1~26.2, 1.21')
  assert.equal(compVersions(['25w01a']), '仅快照版本')
  assert.equal(compVersions(), '未知版本')
  const files = [
    {
      id: 'old',
      date_published: '2026-01-01',
      game_versions: ['1.21.1'],
      loaders: ['fabric'],
      files: [{ url: 'https://example.com/old.jar', filename: 'old.jar', primary: true }],
    },
    {
      id: 'new',
      date_published: '2026-10-01',
      game_versions: ['26.1'],
      loaders: ['neoforge'],
      files: [{ url: 'https://example.com/new.jar', filename: 'new.jar', primary: true }],
    },
  ]
  assert.equal(latestCompFile(files).filename, 'new.jar')
  assert.equal(latestCompFile(files, '1.21.1', 'fabric').filename, 'old.jar')
  assert.equal(latestCompFile(files, '1.21.1', 'forge'), undefined)
  assert.equal(
    latestCompFile([{ ...files[0], files: [{ url: 'javascript:alert(1)' }] }]),
    undefined,
  )
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [{ path: '/', component: {} }],
  })
  const PCompItem = await load('PCompItem')
  const app = createSSRApp({ render: () => h(PCompItem, { data: project, clickable: true }) })
  app.use(router).use(createPinia())
  const resourceHtml = await renderToString(app)
  assert.equal([...resourceHtml.matchAll(/class="inline-icon"/g)].length, 4)
  assert.match(resourceHtml, /<span[^>]*>Fabric 1.19~1.21<\/span>/)
  assert.match(resourceHtml, /aria-label="快速下载最新版本"/)
  assert.match(resourceHtml, /aria-label="收藏"/)
  assert.doesNotMatch(resourceHtml, /<span class="desc-tag"[^>]*>fabric<\/span>/)
  console.log(
    'Resource categories, facet intersections, version summaries and compatible quick downloads passed.',
  )

  const { downloadSettings } = await server.ssrLoadModule('/src/api/downloadSettings.ts')
  const downloadDefaults = {
    max_connections: 64,
    file_connections: 8,
    speed_limit: 0,
    http_mode: 0,
    file_source: 1,
    version_source: 1,
  }
  assert.deepEqual(downloadSettings({}), downloadDefaults)
  assert.deepEqual(downloadSettings(sourceDefaults.PageSetupGameManage), downloadDefaults)
  assert.equal(downloadSettings({ ComboDownloadSource: '' }).file_source, 1)
  for (const [value, bytes] of [
    [0, 104858],
    [14, 1572864],
    [15, 2097152],
    [31, 10485760],
    [32, 11534336],
    [41, 20971520],
    [42, 0],
  ]) {
    assert.equal(downloadSettings({ SliderDownloadSpeed: value }).speed_limit, bytes)
  }
  assert.deepEqual(
    downloadSettings({
      SliderDownloadThread: 256,
      SliderDownloadFileConnection: -1,
      SliderDownloadSpeed: null,
      ComboDownloadHttpMode: 'invalid',
      ComboDownloadSource: false,
    }),
    downloadDefaults,
  )
  storage.set(
    'pcl-ui:PageSetupGameManage:global',
    JSON.stringify({
      SliderDownloadThread: 15,
      SliderDownloadFileConnection: 1,
      ComboDownloadHttpMode: '1',
      ComboDownloadVersion: '0',
    }),
  )
  assert.equal(downloadSettings().max_connections, 16)
  assert.equal(downloadSettings().file_connections, 2)
  assert.equal(downloadSettings().http_mode, 1)
  assert.equal(downloadSettings().version_source, 0)

  const calls = []
  Object.defineProperty(globalThis, 'window', {
    configurable: true,
    value: {
      __TAURI_INTERNALS__: {
        transformCallback: () => 1,
        invoke: (command, args) =>
          new Promise((resolve, reject) => calls.push({ command, args, resolve, reject })),
      },
    },
  })
  const originalInvoke = window.__TAURI_INTERNALS__.invoke
  const javaCalls = []
  const javaRuntime = {
    java_exe: '/jdk25/bin/java',
    directory_path: '/jdk25/bin',
    version: '25.0.1',
    slug_version: 25,
    is_enabled: true,
    is_jdk: true,
    is_64_bit: true,
    architecture: 'Arm64',
    compability: 'Perfect',
    implementor: 'Eclipse',
    is_user_imported: false,
  }
  const javaBackend = { global: null, instances: {}, fail: false }
  let pauseJavaList
  window.__TAURI_INTERNALS__.invoke = async (command, args) => {
    javaCalls.push({ command, args })
    if (command === 'get_java_list' || command === 'refresh_java_list') {
      if (pauseJavaList) {
        const pause = pauseJavaList
        pauseJavaList = null
        return pause
      }
      return [{ ...javaRuntime }]
    }
    if (command === 'get_java_settings')
      return {
        global_java: javaBackend.global,
        has_instance_settings: !!javaBackend.instances[args.instanceDirectory],
        global_is_64_bit: true,
        instance_is_64_bit: true,
        instance: javaBackend.instances[args.instanceDirectory] || {
          preference: { kind: 'global' },
          ignore_compatibility: false,
        },
      }
    if (javaBackend.fail) throw new Error('disk full')
    if (command === 'set_global_java') javaBackend.global = args.javaExe
    else if (command === 'set_instance_java_settings')
      javaBackend.instances[args.instanceDirectory] = args.settings
    else if (command === 'set_java_enabled') javaRuntime.is_enabled = args.enabled
    else throw new Error(`Unexpected Java command: ${command}`)
  }
  try {
    const { useJavaStore } = await server.ssrLoadModule('/src/stores/java.ts')
    const javaStore = useJavaStore()
    storage.set(
      'pcl-ui:PageInstanceSetup:legacy',
      JSON.stringify({
        ComboArgumentJava: '/jdk25/bin/java',
        CheckAdvanceJava: true,
        TextArgumentTitle: 'keep',
      }),
    )
    await javaStore.load('/games/legacy', false, 'legacy')
    assert.equal(javaStore.instances['/games/legacy'].preference.java_exe, '/jdk25/bin/java')
    assert.equal(javaStore.instances['/games/legacy'].ignore_compatibility, true)
    const migrations = javaCalls.filter(
      ({ command }) => command === 'set_instance_java_settings',
    ).length
    await javaStore.load('/games/legacy', false, 'legacy')
    assert.equal(
      javaCalls.filter(({ command }) => command === 'set_instance_java_settings').length,
      migrations,
    )
    await javaStore.selectGlobal('/jdk25/bin/java')
    assert.equal(javaStore.globalJava, '/jdk25/bin/java')
    await javaStore.setEnabled(javaStore.runtimes[0])
    assert.equal(javaStore.runtimes[0].is_enabled, false)
    javaBackend.fail = true
    await assert.rejects(javaStore.selectGlobal(null), /disk full/)
    assert.equal(
      javaStore.globalJava,
      '/jdk25/bin/java',
      'failed persistence must not change the displayed selection',
    )
    javaBackend.fail = false
    let releaseJava
    pauseJavaList = new Promise((resolve) => {
      releaseJava = resolve
    })
    const staleJava = javaStore.load('/games/stale')
    await javaStore.load('/games/current')
    releaseJava([{ ...javaRuntime }])
    await staleJava
    assert.equal(
      javaStore.instances['/games/stale'],
      undefined,
      'late responses do not overwrite the next instance',
    )
    assert.equal(javaStore.instances['/games/current'].preference.kind, 'global')
    await javaStore.saveInstance('/games/current', {
      preference: { kind: 'relative', relative_path: 'runtime/bin/java' },
      ignore_compatibility: false,
    })
    assert.deepEqual(javaBackend.instances['/games/current'].preference, {
      kind: 'relative',
      relative_path: 'runtime/bin/java',
    })
    const combo = await renderToString(
      createSSRApp({
        render: () =>
          h(PComboBox, {
            options: [{ key: 'disabled', text: 'Disabled Java', disabled: true }],
          }),
      }),
    )
    assert.match(combo, /<option[^>]*value="disabled"[^>]*disabled/)
  } finally {
    window.__TAURI_INTERNALS__.invoke = originalInvoke
  }
  const { useTaskManager, TaskStatus } = await server.ssrLoadModule('/src/stores/task.ts')
  const manager = useTaskManager()
  const { useFloatButton } = await server.ssrLoadModule('/src/composables/useFloatButton.ts')
  const button = useFloatButton().floatButtonState
  const clientDownload = manager.exportMinecraftVersion('1.21.1', '/exports', false)
  const serverDownload = manager.exportMinecraftVersion('1.21.1', '/exports', true)
  assert.equal(manager.activeTaskCount, 2)
  assert.throws(() => manager.exportMinecraftVersion('1.21.1', '/exports', false))
  assert.deepEqual(
    calls.map(({ command, args }) => [command, args.task_id, args.server]),
    [
      ['export_minecraft_version', 0, false],
      ['export_minecraft_version', 1, true],
    ],
  )
  assert.equal(calls[0].args.download_settings.max_connections, 16)
  assert.equal(calls[0].args.download_settings.file_connections, 2)
  assert.equal(manager.tasks[0].items.length, 2)
  calls[0].args.on_event.onmessage({
    task_id: 0,
    item_id: 1,
    files_remaining: 1,
    progress: 0.5,
    status: TaskStatus.Running,
    speed: 100,
  })
  assert.equal(manager.tasks[0].progress, 0.25)
  assert.equal(manager.tasks[1].progress, 0)
  assert.equal(manager.totalSpeed, 100)
  calls[0].resolve('/exports/1.21.1')
  assert.equal(await clientDownload, '/exports/1.21.1')
  assert.equal(manager.tasks[0].status, TaskStatus.Completed)
  calls[1].reject(new Error('SHA-1 mismatch'))
  await assert.rejects(serverDownload, /SHA-1/)
  assert.equal(manager.tasks[1].status, TaskStatus.Failed)
  assert.equal(manager.activeTaskCount, 0)
  assert.equal(manager.totalSpeed, 0)
  assert.equal(manager.totalRemaining, 0)
  assert.match(manager.tasks[1].error, /SHA-1/)
  const install = manager.startDownloadMCVersion('1.21.1', 'instance')
  assert.equal(manager.tasks[2].items.length, 4)
  assert.equal(calls[2].command, 'download_minecraft_version')
  assert.equal(calls[2].args.instance_name, 'instance')
  assert.equal(button.visible, true)
  for (let item = 0; item < 4; item++) {
    calls[2].args.on_event.onmessage({
      task_id: 2,
      item_id: item,
      files_remaining: 0,
      progress: 1,
      status: TaskStatus.Completed,
      speed: 0,
    })
  }
  assert.equal(
    manager.tasks[2].status,
    TaskStatus.Running,
    'downloaded files must wait for native extraction and instance publication',
  )
  const failed = assert.rejects(install, /native extraction failed/)
  calls[2].reject(new Error('native extraction failed'))
  await failed
  assert.equal(manager.tasks[2].status, TaskStatus.Failed)
  assert.match(manager.tasks[2].error, /native extraction failed/)
  calls[2].args.on_event.onmessage({
    task_id: 2,
    item_id: 0,
    files_remaining: 1,
    progress: 0.5,
    status: TaskStatus.Running,
    speed: 500,
  })
  assert.equal(manager.totalSpeed, 0, 'late IPC cannot restart terminal tasks')
  const nextInstall = manager.startDownloadMCVersion('1.21.1', 'second')
  const concurrent = manager.startDownloadMCVersion('1.21.1', 'third')
  calls[3].args.on_event.onmessage({
    task_id: 4,
    item_id: 1,
    files_remaining: 1,
    progress: 0.5,
    status: TaskStatus.Failed,
    speed: 0,
  })
  assert.equal(manager.tasks[3].items[1].status, TaskStatus.Pending, 'IPC IDs stay isolated')
  calls[3].resolve()
  await nextInstall
  assert.equal(manager.tasks[3].status, TaskStatus.Completed)
  assert.equal(manager.tasks[3].progress, 1)
  assert.equal(button.visible, true, 'another installation is still running')
  calls[4].resolve()
  await concurrent
  assert.equal(manager.tasks[4].status, TaskStatus.Completed)
  assert.equal(manager.totalSpeed, 0)
  assert.equal(manager.totalRemaining, 0)
  assert.equal(button.visible, false)
  console.log(
    'Version actions, IPC isolation, download progress, publication failures and concurrent tasks passed.',
  )
  console.log(
    'Component attributes, labels, settings persistence/reset and instance isolation passed.',
  )
} finally {
  scopes.forEach((scope) => scope.stop())
  if (savedStorage) Object.defineProperty(globalThis, 'localStorage', savedStorage)
  else delete globalThis.localStorage
  if (savedWindow) Object.defineProperty(globalThis, 'window', savedWindow)
  else delete globalThis.window
  await server.close()
}
