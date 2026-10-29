import sourceDefaults from '@/assets/setting-defaults.json'
import { computed, reactive, watch } from 'vue'
import { pclText } from '@/api/pclLocale'
import { useSelectedInstance } from '@/stores/gameLaunch'
import sideTip from '@/composables/sideTip'

type SettingValue = string | boolean | number
type PageDefaults<Page extends string> = Page extends keyof typeof sourceDefaults
  ? (typeof sourceDefaults)[Page]
  : Record<never, never>
type Settings<T> = {
  [Key in keyof T]: T[Key] extends boolean ? boolean : T[Key] extends number ? number : string
} & Record<string, SettingValue>

export function usePageSettings<
  Page extends string,
  Defaults extends Record<string, SettingValue> = Record<never, never>,
>(page: Page, defaults: Defaults = {} as Defaults) {
  const pageDefaults = (sourceDefaults as Record<string, Record<string, SettingValue>>)[page] || {}
  const initialDefaults = {
    ...Object.fromEntries(
      Object.entries(pageDefaults).filter(
        ([key]) =>
          !['ComboArgumentJava', 'CheckAdvanceJava', 'ComboUiFont', 'ComboUiMotdFont'].includes(
            key,
          ),
      ),
    ),
    ...defaults,
  }
  const instance = page.startsWith('PageInstance') ? useSelectedInstance() : null
  const storageKey = computed(
    () => `pcl-ui:${page}:${instance ? instance.instance_info?.id || 'none' : 'global'}`,
  )
  function loadValues() {
    try {
      const parsed: unknown = JSON.parse(localStorage.getItem(storageKey.value) || '{}')
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed))
        return Object.fromEntries(
          Object.entries(parsed).filter(
            ([key, value]) =>
              !/password|token|secret|cdk/i.test(key) &&
              !['ComboArgumentJava', 'CheckAdvanceJava'].includes(key) &&
              ['string', 'number', 'boolean'].includes(typeof value) &&
              (!(key in initialDefaults) || typeof value === typeof initialDefaults[String(key)]),
          ),
        )
    } catch {
      // Invalid saved preferences must not prevent the page from opening.
    }
    return {}
  }
  const values = reactive({ ...initialDefaults, ...loadValues() }) as Settings<
    PageDefaults<Page> & Defaults
  >
  watch(
    values,
    () => {
      try {
        const legacy = JSON.parse(localStorage.getItem(storageKey.value) || '{}')
        const javaLegacy = Object.fromEntries(
          Object.entries(legacy || {}).filter(([key]) =>
            ['ComboArgumentJava', 'CheckAdvanceJava'].includes(key),
          ),
        )
        localStorage.setItem(
          storageKey.value,
          JSON.stringify({
            ...javaLegacy,
            ...Object.fromEntries(
              Object.entries(values).filter(([key]) => !/password|token|secret|cdk/i.test(key)),
            ),
          }),
        )
      } catch {
        sideTip.show(pclText('Proto.Storage.SettingsFailed'), 'warn')
      }
    },
    { deep: true },
  )
  watch(storageKey, () => {
    Object.keys(values).forEach((key) => delete values[key])
    Object.assign(values, initialDefaults, loadValues())
  })
  function reset(key?: keyof typeof values) {
    if (key) {
      Object.assign(values, { [key]: initialDefaults[String(key)] })
    } else {
      Object.keys(values).forEach((key) => delete values[key])
      Object.assign(values, initialDefaults)
    }
  }
  return { values, reset }
}
