import sourceDefaults from '@/assets/setting-defaults.json'
import { computed, reactive, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { pclImagePath } from '@/api/pclAssets'
import { pclText } from '@/api/pclLocale'
import { useSetup } from '@/stores/setup'
import { useSelectedInstance } from '@/stores/gameLaunch'
import sideTip from '@/composables/sideTip'
import { useModal } from '@/composables/useModal'
import { useI18n } from 'vue-i18n'

export function usePageSettings(
  page: string,
  defaults: Record<string, string | boolean | number> = {},
) {
  const pageDefaults =
    (sourceDefaults as Record<string, Record<string, string | boolean | number>>)[page] || {}
  // Runtime selectors use frontend IDs; retain those instead of source Java/font paths.
  const initialDefaults = {
    ...defaults,
    ...Object.fromEntries(
      Object.entries(pageDefaults).filter(
        ([key]) => !['ComboArgumentJava', 'ComboUiFont', 'ComboUiMotdFont'].includes(key),
      ),
    ),
  }
  const route = useRoute()
  const router = useRouter()
  const setup = useSetup()
  const { locale } = useI18n()
  const instance = useSelectedInstance()
  const storageKey = computed(
    () =>
      `pcl-ui:${page}:${page.startsWith('PageInstance') ? instance.instance_info?.id || 'none' : 'global'}`,
  )
  function loadValues() {
    try {
      const parsed: unknown = JSON.parse(localStorage.getItem(storageKey.value) || '{}')
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed))
        return Object.fromEntries(
          Object.entries(parsed).filter(
            ([key, v]) =>
              !/password|token|secret|cdk/i.test(key) &&
              ['string', 'number', 'boolean'].includes(typeof v),
          ),
        ) as Record<string, string | number | boolean>
    } catch {
      /* Keep invalid saved UI preferences from preventing the page from opening. */
    }
    return {}
  }
  const values = reactive<Record<string, string | boolean | number>>({
    ...initialDefaults,
    ...(page === 'PageSetupLaunch'
      ? {
          TextArgumentTitle: setup.launch.gameWindowTitle,
          TextArgumentInfo: setup.launch.customInfo,
        }
      : {}),
    ...loadValues(),
  })
  const launchFields: Record<string, keyof typeof setup.launch> = {
    TextArgumentTitle: 'gameWindowTitle',
    TextArgumentInfo: 'customInfo',
  }
  function save() {
    if (page === 'PageSetupLaunch')
      Object.entries(launchFields).forEach(
        ([control, field]) => (setup.launch[field] = String(values[control] || '')),
      )
    try {
      localStorage.setItem(
        storageKey.value,
        JSON.stringify(
          Object.fromEntries(
            Object.entries(values).filter(([key]) => !/password|token|secret|cdk/i.test(key)),
          ),
        ),
      )
    } catch {
      sideTip.show('无法保存设置，请检查可用存储空间。', 'warn')
    }
  }
  watch(values, save, { deep: true })
  watch(storageKey, () => {
    Object.keys(values).forEach((key) => delete values[key])
    Object.assign(values, initialDefaults, loadValues())
  })
  function change(id: string, value: string | number | boolean) {
    if (id) values[id] = value
  }
  async function act(id: string, event = '', eventType = '', eventData = '') {
    if (eventType === 'OpenUrl' || eventType === '打开网页') {
      if (eventData && /^https?:\/\//i.test(eventData))
        window.open(eventData, '_blank', 'noopener,noreferrer')
      return
    }
    if (eventType === '复制文本') {
      try {
        await navigator.clipboard.writeText(eventData || '')
        sideTip.show('已成功复制！', 'success')
      } catch {
        sideTip.show('复制失败，请检查剪贴板权限。', 'warn')
      }
      return
    }
    if (id === 'BtnHintClose') {
      values.hideCommunityHint = true
      return
    }
    if (
      /Btn(?:EmptyDownload|DownloadNew|ManageDownload|HintDownload)/.test(id) ||
      event === 'BtnDownloadNew_Click'
    ) {
      const targets: Record<string, string> = {
        PageInstanceSaves: 'world',
        PageInstanceSavesDatapack: 'datapack',
        PageInstanceScreenshot: 'resourcepack',
      }
      const category = String(route.meta.downloadType || 'mod')
      await router.push(
        `/download/${targets[page] || (category === 'schematic' ? 'mod' : category)}`,
      )
      return
    }
    if (id === 'BtnBack' || id === 'BtnEmptyBack') {
      router.back()
      return
    }
    if (id === 'BtnDownload' && page === 'PageInstanceModDisabled') {
      await router.push('/download/game')
      return
    }
    if (id === 'BtnVersion' && page === 'PageInstanceModDisabled') {
      await router.push('/instance_select')
      return
    }
    if (id === 'BtnSchematicVersionSelect') {
      await router.push('/instance_select')
      return
    }
    if (id === 'BtnCheckAgain' || id === 'BtnChangelog') {
      window.open(
        'https://github.com/PCL-Community/PCL.Proto/releases',
        '_blank',
        'noopener,noreferrer',
      )
      return
    }
    if (id === 'BtnCustomTutorial') {
      window.open(
        'https://github.com/Hex-Dragon/PCL2/wiki/自定义主页',
        '_blank',
        'noopener,noreferrer',
      )
      return
    }
    if (id === 'BtnCustomRefresh') {
      await router.push('/home')
      return
    }
    if (id === 'BtnOverrideCancel') {
      Object.keys(values).forEach((key) => delete values[key])
      Object.assign(values, initialDefaults)
      localStorage.removeItem(storageKey.value)
      return
    }
    if (id === 'BtnSwitch') {
      await router.push(page === 'PageSetupLaunch' ? '/instance_setting/setting' : '/setup/launch')
      return
    }
    if (id === 'BtnAdvanceJvmReset') {
      change('TextAdvanceJvm', initialDefaults.TextAdvanceJvm || '')
      return
    }
    if (event === 'Feedback_Click') {
      window.open(
        'https://github.com/PCL-Community/PCL.Proto/issues',
        '_blank',
        'noopener,noreferrer',
      )
      return
    }
    if (eventType === '今日人品') {
      await router.push('/tools/wonderbox')
      return
    }
    await useModal().open({
      title: '提示',
      content: '此操作尚未接入 PCL.Proto。',
      buttons: [{ type: 'tint', content: '确定' }],
    })
  }
  const text = (value: string) => pclText(value, locale.value)
  const image = (value: string) => pclImagePath(value, import.meta.env.BASE_URL)
  return { values, change, act, text, image }
}
