import assert from 'node:assert/strict'
import { createServer } from 'vite'
import { createSSRApp, effectScope, h, nextTick, ref } from 'vue'
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
    get length() { return storage.size },
    key: (index) => [...storage.keys()][index] ?? null,
  },
})
const scopes = []
try {
  const load = async (name) =>
    (await server.ssrLoadModule(`/src/components/widget/${name}.vue`)).default
  const [PTextBox, PComboBox, PCheckBox, SetupItem] = await Promise.all(
    ['PTextBox', 'PComboBox', 'PCheckBox', 'SetupItem'].map(load),
  )
  const [PCard, PVirtualList] = await Promise.all(['PCard', 'PVirtualList'].map(load))
  for (const state of ['fold', 'unfold', 'unfoldable']) {
    let rendered = 0
    const html = await renderToString(
      createSSRApp({
        render: () =>
          h(
            PCard,
            { defaultFoldStatus: state, title: 'Versions' },
            {
              default: () => {
                rendered++
                return h('input', { value: 'saved' })
              },
            },
          ),
      }),
    )
    assert.equal(rendered, state === 'fold' ? 0 : 1, 'folded content is lazy')
    assert.equal(html.includes('value="saved"'), state !== 'fold')
    if (state === 'unfoldable')
      assert.doesNotMatch(html, /height:40px/, 'ordinary cards size naturally')
  }
  for (const [count, rowHeight] of [
    [0, 42],
    [1, 42],
    [20, 42],
    [21, 42],
    [4000, 42],
    [40, 64],
  ]) {
    const html = await renderToString(
      createSSRApp({
        render: () =>
          h(
            PVirtualList,
            { items: Array.from({ length: count }, (_, i) => i), itemKey: String, rowHeight },
            {
              default: ({ item }) => h('p', { class: 'page-row' }, String(item)),
            },
          ),
      }),
    )
    assert.equal(
      [...html.matchAll(/class="page-row"/g)].length,
      Math.min(count, 16),
      'large backend responses never create an unbounded list',
    )
    assert.equal(html.includes('aria-label="分页"'), false)
    assert.match(
      html,
      new RegExp(`height:${count * rowHeight}px`),
      'the full continuous scroll range remains available',
    )
  }
  console.log('Lazy folded cards, natural card sizing and bounded list rendering passed.')
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
  const firstKey = 'pcl-instance-ui:PageInstanceSetup:%2Frepo%20one%2Fversions%2Fshared'
  const secondKey = 'pcl-instance-ui:PageInstanceSetup:%2Frepo%20two%2Fversions%2Fshared'
  storage.set('pcl-ui:PageInstanceSetup:shared', JSON.stringify({ TextArgumentInfo: 'Legacy' }))
  instance.instance_info = { id: 'shared', directory: '/repo one/versions/shared' }
  const scoped = open('PageInstanceSetup')
  scoped.values.TextArgumentInfo = 'First instance'
  storage.set(secondKey, JSON.stringify({ TextArgumentInfo: 'Second instance' }))
  instance.instance_info = { id: 'shared', directory: '/repo two/versions/shared' }
  assert.equal(scoped.values.TextArgumentInfo, 'Second instance')
  scoped.values.TextArgumentInfo = 'Second edited'
  instance.instance_info = { id: 'shared', directory: '/repo one/versions/shared' }
  assert.equal(scoped.values.TextArgumentInfo, 'First instance')
  assert.equal(JSON.parse(storage.get(firstKey)).TextArgumentInfo, 'First instance')
  assert.equal(JSON.parse(storage.get(secondKey)).TextArgumentInfo, 'Second edited')
  assert.equal(JSON.parse(storage.get('pcl-ui:PageInstanceSetup:shared')).TextArgumentInfo, 'Legacy')
  instance.instance_info = null
  await nextTick()
  scoped.values.TextArgumentInfo = 'No instance'
  await nextTick()
  assert.equal(storage.has('pcl-ui:PageInstanceSetup:none'), false)
  instance.instance_info = { id: 'shared', directory: '/repo one/versions/shared' }
  await nextTick()
  assert.equal(scoped.values.TextArgumentInfo, 'First instance')
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

  const { compTags, compTagOptions, compFacets, compVersions, latestCompFile, groupCompVersions } =
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
  const groupedVersions = groupCompVersions(
    [
      {
        ...files[0],
        id: 'older',
        date_published: '2024-01-01',
        game_versions: ['1.20.1', '1.20.1'],
        loaders: ['fabric', 'forge'],
      },
      {
        ...files[0],
        id: 'newer',
        date_published: '2025-01-01',
        game_versions: ['1.21.1', '1.20.1'],
        loaders: ['fabric'],
      },
      {
        ...files[0],
        id: 'snapshot',
        date_published: '2025-01-01',
        game_versions: ['25w01a', '25w02a'],
        loaders: ['fabric'],
      },
    ],
    '',
    '',
    true,
  )
  assert.deepEqual(
    groupedVersions.map((group) => group.name),
    ['Fabric 1.21.1', 'Forge 1.20.1', 'Fabric 1.20.1', '快照版'],
  )
  assert.deepEqual(
    groupedVersions[2].versions.map((version) => version.id),
    ['newer', 'older'],
  )
  assert.equal(groupedVersions[3].versions.length, 1, 'one file appears once per game-version card')
  assert.equal(groupCompVersions(files, 'missing').length, 0)
  assert.equal(groupCompVersions(files, '', 'missing').length, 0)
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
    auto_select_instance: true,
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
      CheckDownloadAutoSelectVersion: false,
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
  const { downloadGameJava } = await server.ssrLoadModule('/src/api/javaReq.ts')
  const javaDownload = downloadGameJava('/games/current', () => {})
  assert.equal(calls[0].command, 'download_game_java')
  assert.equal(calls[0].args.instanceDirectory, '/games/current')
  assert.equal(calls[0].args.downloadSettings, undefined)
  calls.shift().resolve({})
  await javaDownload
  const {
    initializeDownloadSettings,
    readDownloadSettings,
    saveDownloadSettings,
    downloadSaveError,
    downloadSettingValues,
  } = await server.ssrLoadModule('/src/api/downloadSettings.ts')
  const init = initializeDownloadSettings()
  assert.equal(calls[0].command, 'initialize_download_settings')
  assert.equal(calls[0].args.legacy.max_connections, 16)
  assert.equal(calls[0].args.legacy.auto_select_instance, false)
  calls.shift().resolve(downloadDefaults)
  await init
  const existingInit = initializeDownloadSettings()
  calls.shift().resolve(downloadDefaults)
  assert.deepEqual(await existingInit, downloadDefaults)
  assert.equal(calls.length, 0)
  assert.deepEqual(downloadSettings(downloadSettingValues(downloadDefaults)), downloadDefaults)
  const firstSettings = { ...downloadDefaults, max_connections: 9 }
  const secondSettings = { ...downloadDefaults, max_connections: 10 }
  const firstSave = saveDownloadSettings(firstSettings)
  const secondSave = saveDownloadSettings(secondSettings)
  await Promise.resolve()
  assert.equal(calls.length, 1)
  assert.equal(calls[0].args.settings.max_connections, 9)
  calls.shift().resolve(firstSettings)
  await firstSave
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].args.settings.max_connections, 10)
  calls.shift().reject(new Error('disk full'))
  await assert.rejects(secondSave, /disk full/)
  assert.match(downloadSaveError.value, /disk full/)
  assert.deepEqual(await readDownloadSettings(), secondSettings)
  const retrySave = saveDownloadSettings(secondSettings)
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().resolve(secondSettings)
  await retrySave
  assert.equal(downloadSaveError.value, '')
  const { initializeWindowSettings, legacyWindowSettings, readWindowSettings, saveWindowSettings, windowSaveError } =
    await server.ssrLoadModule('/src/api/windowSettings.ts')
  storage.set('pcl-ui:PageSetupLaunch:global', JSON.stringify({ ComboArgumentWindowType: '3', TextArgumentWindowWidth: '1280', TextArgumentWindowHeight: '720' }))
  const windowInit = initializeWindowSettings()
  calls.shift().resolve(null)
  await new Promise((resolve) => setTimeout(resolve, 0))
  const customWindow = { mode: 'custom', width: 1280, height: 720 }
  assert.deepEqual(calls[0].args.settings, customWindow)
  calls.shift().resolve(customWindow)
  await windowInit
  assert.deepEqual(legacyWindowSettings(), customWindow)
  const failedWindow = saveWindowSettings({ ...customWindow, width: 1600 })
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().reject(new Error('disk full'))
  await assert.rejects(failedWindow, /disk full/)
  assert.match(windowSaveError.value, /disk full/)
  assert.equal((await readWindowSettings()).width, 1600)
  const { migrateLegacyInstanceJava } = await server.ssrLoadModule('/src/api/javaMigration.ts')
  storage.set('pcl-ui:PageInstanceSetup:legacy', JSON.stringify({ ComboArgumentJava: '/jdk25/bin/java', CheckAdvanceJava: true }))
  storage.set('pcl-ui:PageInstanceSetup:none', JSON.stringify({ ComboArgumentJava: '/bad' }))
  const migration = migrateLegacyInstanceJava()
  assert.equal(calls[0].command, 'migrate_instance_java_settings')
  assert.deepEqual(calls[0].args.legacy.legacy, { preference: { kind: 'exist', java_exe: '/jdk25/bin/java' }, ignore_compatibility: true })
  assert.equal(calls[0].args.legacy.none, undefined)
  calls.shift().resolve(['legacy'])
  assert.deepEqual(await migration, ['legacy'])
  const { migrateLegacyInstancePages } = await server.ssrLoadModule('/src/api/instancePageMigration.ts')
  const oldPage = 'pcl-ui:PageInstanceOverall:unique'
  const oldDuplicate = 'pcl-ui:PageInstanceOverall:duplicate'
  storage.set(oldPage, JSON.stringify({ ComboDisplayLogo: '2', unknown: 'ignored' }))
  storage.set(oldDuplicate, JSON.stringify({ ComboDisplayLogo: '1' }))
  storage.set('pcl-ui:PageInstanceSavesDatapack:unique', JSON.stringify({ filter: 'Enabled', SearchBox: 'stone' }))
  const existingPage = 'pcl-instance-ui:PageInstanceSavesDatapack:%2Frepo%2Fversions%2Fone'
  storage.set(existingPage, JSON.stringify({ filter: 'Disabled' }))
  const pageMigration = migrateLegacyInstancePages()
  assert.equal(calls[0].command, 'resolve_legacy_instance_ids')
  calls.shift().resolve({ unique: '/repo/versions/one' })
  assert.equal(await pageMigration, 1)
  assert.deepEqual(JSON.parse(storage.get('pcl-instance-ui:PageInstanceOverall:%2Frepo%2Fversions%2Fone')), { ComboDisplayLogo: '2' })
  assert.equal(JSON.parse(storage.get(existingPage)).filter, 'Disabled')
  assert.equal(JSON.parse(storage.get(oldPage)).unknown, 'ignored')
  const retryPages = migrateLegacyInstancePages()
  assert.deepEqual(calls[0].args.ids, ['duplicate'])
  calls.shift().reject(new Error('repository unavailable'))
  assert.equal(await retryPages, 1)
  const { initializeMemorySettings, memoryTickToMib, saveMemorySettings, readMemorySettings, previewMemorySettings } =
    await server.ssrLoadModule('/src/api/memorySettings.ts')
  assert.equal(memoryTickToMib(0), 307)
  assert.equal(memoryTickToMib(13), 2048)
  assert.equal(memoryTickToMib(49), 49152)
  const previewRequest = previewMemorySettings('/repo/a', { mode: 'auto' }, false)
  assert.deepEqual(calls[0].args, { instanceDirectory: '/repo/a', settings: { mode: 'auto' }, is64Bit: false })
  calls.shift().resolve({ total_bytes: 8192, available_bytes: 4096, allocation_mib: 1024 })
  assert.equal((await previewRequest).allocation_mib, 1024)
  storage.set('pcl-ui:PageSetupLaunch:global', JSON.stringify({ LaunchRamType: '1', SliderRamCustom: 13, SliderRamInitialCustom: 20 }))
  storage.set('pcl-ui:PageInstanceSetup:memory', JSON.stringify({ VersionRamType: '0' }))
  storage.set('pcl-ui:PageInstanceSetup:follow', JSON.stringify({ VersionRamType: '2' }))
  const memoryInit = initializeMemorySettings()
  assert.deepEqual(calls[0].args.settings, { mode: 'custom', maximum_mib: 2048, initial_mib: 2048 })
  assert.equal(calls[0].args.custom.initial_mib, memoryTickToMib(20))
  calls.shift().resolve(null)
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].command, 'migrate_instance_memory_settings')
  assert.deepEqual(calls[0].args.legacy.memory.settings, { mode: 'auto' })
  assert.equal(calls[0].args.legacy.follow.settings, null)
  assert.equal(calls[0].args.legacy.none, undefined)
  calls.shift().resolve(['memory'])
  assert.equal(await memoryInit, 1)
  const firstMemory = { settings: { mode: 'custom', maximum_mib: 2048, initial_mib: null }, custom: { maximum_mib: 2048, initial_mib: null } }
  const secondMemory = { settings: { mode: 'custom', maximum_mib: 3072, initial_mib: 1024 }, custom: { maximum_mib: 3072, initial_mib: 1024 } }
  const saveFirstMemory = saveMemorySettings('/repo/a', firstMemory)
  const saveSecondMemory = saveMemorySettings('/repo/a', secondMemory)
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.deepEqual(calls[0].args.settings, firstMemory.settings)
  calls.shift().resolve(firstMemory)
  await saveFirstMemory
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.deepEqual(calls[0].args.settings, secondMemory.settings)
  calls.shift().reject(new Error('memory disk full'))
  await assert.rejects(saveSecondMemory, /memory disk full/)
  assert.deepEqual(await readMemorySettings('/repo/a'), { ...secondMemory, error: 'Error: memory disk full' })
  const inherited = { settings: null, custom: { maximum_mib: 6144, initial_mib: 4096 } }
  const inheritFirst = saveMemorySettings('/repo/inherit', inherited)
  const inheritSecond = saveMemorySettings('/repo/inherit', inherited)
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().resolve(null)
  await inheritFirst
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().reject(new Error('inherit failed'))
  await assert.rejects(inheritSecond, /inherit failed/)
  assert.deepEqual(await readMemorySettings('/repo/inherit'), { ...inherited, error: 'Error: inherit failed' })
  globalThis.isTauri = true
  const { useMemorySettings } = await server.ssrLoadModule('/src/composables/useMemorySettings.ts')
  const memoryScope = effectScope()
  scopes.push(memoryScope)
  const memoryPath = ref('/repo/old')
  const memoryForm = memoryScope.run(() => useMemorySettings(memoryPath))
  await new Promise((resolve) => setTimeout(resolve, 0))
  const oldRead = calls.shift()
  memoryPath.value = '/repo/new'
  await new Promise((resolve) => setTimeout(resolve, 0))
  const newRead = calls.shift()
  assert.equal(newRead.args.instanceDirectory, '/repo/new')
  newRead.resolve(secondMemory)
  await new Promise((resolve) => setTimeout(resolve, 0))
  oldRead.resolve({ settings: { mode: 'auto' }, custom: { maximum_mib: 2048, initial_mib: null } })
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(memoryForm.mode.value, 'custom')
  assert.equal(memoryForm.maximum.value, 15)
  memoryForm.mode.value = 'auto'
  memoryForm.save()
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.deepEqual(calls[0].args.custom, secondMemory.custom)
  calls.shift().resolve({ settings: { mode: 'auto' }, custom: secondMemory.custom })
  await new Promise((resolve) => setTimeout(resolve, 0))
  const reloadMemory = memoryForm.load()
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().resolve({ settings: { mode: 'auto' }, custom: secondMemory.custom })
  await reloadMemory
  assert.equal(memoryForm.mode.value, 'auto')
  assert.equal(memoryForm.maximum.value, 15)
  assert.equal(memoryTickToMib(memoryForm.initial.value), 1024)
  const { useLaunchArguments } = await server.ssrLoadModule('/src/composables/useLaunchArguments.ts')
  const { readLaunchArguments } = await server.ssrLoadModule('/src/api/launchArguments.ts')
  const argumentScope = effectScope()
  scopes.push(argumentScope)
  const argumentPath = ref('/repo/argument-old')
  const argumentForm = argumentScope.run(() => useLaunchArguments(argumentPath))
  await new Promise((resolve) => setTimeout(resolve, 0))
  const oldArguments = calls.shift()
  argumentPath.value = '/repo/argument-new'
  await new Promise((resolve) => setTimeout(resolve, 0))
  const newArguments = calls.shift()
  newArguments.resolve({ jvm: '-Dnew=true', game: '', classpath: '' })
  await new Promise((resolve) => setTimeout(resolve, 0))
  oldArguments.resolve({ jvm: '-Dold=true', game: '', classpath: '' })
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(argumentForm.jvm.value, '-Dnew=true')
  argumentForm.game.value = "--server 'my host'"
  argumentForm.save()
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].command, 'set_launch_arguments')
  assert.equal(calls[0].args.instanceDirectory, '/repo/argument-new')
  calls.shift().reject(new Error('arguments disk full'))
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.match(argumentForm.error.value, /arguments disk full/)
  assert.equal((await readLaunchArguments('/repo/argument-new')).settings.game, "--server 'my host'")
  argumentForm.game.value = '--demo'
  argumentForm.save()
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].args.settings.game, '--demo')
  calls.shift().resolve(null)
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(argumentForm.error.value, '')
  const { initializeLogSettings, logSlider, logSaveError, updateLogSlider, saveLogSettings } =
    await server.ssrLoadModule('/src/api/logSettings.ts')
  globalThis.isTauri = true
  storage.set('pcl-ui:PageLogRight:global', JSON.stringify({ SliderMaxLog: 5 }))
  storage.set('pcl-ui:PageSetupLauncherMisc:global', JSON.stringify({ SliderMaxLog: 13 }))
  const logInit = initializeLogSettings()
  calls.shift().resolve(null)
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].args.lines, 100)
  calls.shift().resolve(100)
  await logInit
  assert.equal(logSlider.value, 5)
  const logExisting = initializeLogSettings()
  calls.shift().resolve(500)
  await logExisting
  assert.equal(logSlider.value, 13)
  assert.equal(calls.length, 0)
  updateLogSlider(29)
  saveLogSettings()
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(calls[0].args.lines, 0)
  calls.shift().reject(new Error('disk full'))
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.match(logSaveError.value, /disk full/)
  saveLogSettings()
  await new Promise((resolve) => setTimeout(resolve, 0))
  calls.shift().resolve(0)
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(logSaveError.value, '')
  delete globalThis.isTauri
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
    await javaStore.load('/games/legacy')
    assert.equal(javaStore.instances['/games/legacy'].preference.kind, 'global')
    const migrations = javaCalls.filter(
      ({ command }) => command === 'set_instance_java_settings',
    ).length
    await javaStore.load('/games/legacy')
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
  assert.equal(calls[0].args.download_settings, undefined)
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
  const fabricInstall = manager.startDownloadMCVersion('1.20.1', 'Fabric', { family: 'fabric', version: '0.16.14' })
  assert.deepEqual(calls[5].args.fabric, { family: 'fabric', version: '0.16.14' })
  calls[5].resolve()
  await fabricInstall
  const legacyInstall = manager.startDownloadMCVersion('1.12.2', 'Legacy', { family: 'legacyfabric', version: '0.20.1' })
  assert.deepEqual(calls[6].args.fabric, { family: 'legacyfabric', version: '0.20.1' })
  calls[6].resolve()
  await legacyInstall
  const selectedInstall = manager.startDownloadMCVersion('1.20.1', 'selected')
  calls[7].resolve({ selected: true, selection_error: null })
  await selectedInstall
  assert.equal(calls[8].command, 'get_active_instance')
  calls[8].resolve({ id: 'selected' })
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(useSelectedInstance().instance_info.id, 'selected')
  const selectedStore = useSelectedInstance()
  const staleFetch = selectedStore.fetch()
  selectedStore.setInstance({ id: 'new-selection' })
  calls[9].resolve({ id: 'old-selection' })
  await staleFetch
  assert.equal(selectedStore.instance_info.id, 'new-selection')
  const firstFetch = selectedStore.fetch()
  const secondFetch = selectedStore.fetch()
  calls[11].resolve({ id: 'newest' })
  await secondFetch
  calls[10].resolve({ id: 'older' })
  await firstFetch
  assert.equal(selectedStore.instance_info.id, 'newest')
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
