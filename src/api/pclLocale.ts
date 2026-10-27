import { readonly, ref, shallowReactive } from 'vue'
import zh from '../assets/languages/zh-CN.json'
import appZh from '../assets/messages/zh-CN.json'
import languageNames from '../assets/language-names.json'

const loaders = import.meta.glob('../assets/languages/*.json', { import: 'default' }) as Record<
  string,
  () => Promise<Record<string, string>>
>
const appLoaders = import.meta.glob('../assets/messages/*.json', { import: 'default' }) as Record<
  string,
  () => Promise<Record<string, string>>
>
export const pclStrings: Record<string, string> = { ...zh, ...appZh }
export const pclLanguages = languageNames
const currentLocale = ref('zh-CN')
export const pclLocale = readonly(currentLocale)
const loaded = shallowReactive<Record<string, Record<string, string>>>({ 'zh-CN': pclStrings })
const pending = new Map<string, Promise<void>>()
let languageChange = 0

function normalizeLocale(value: string): string {
  return pclLanguages.find((locale) => locale.toLowerCase() === value.toLowerCase()) || 'zh-CN'
}

export async function loadPclLocale(locale: string): Promise<void> {
  if (loaded[locale]) return
  const loader = loaders[`../assets/languages/${locale}.json`]
  if (!loader) throw new RangeError(`Unsupported language: ${locale}`)
  let request = pending.get(locale)
  if (!request) {
    request = Promise.all([loader(), appLoaders[`../assets/messages/${locale}.json`]?.()])
      .then(([strings, appStrings]) => {
        loaded[locale] = { ...strings, ...appStrings }
      })
      .finally(() => pending.delete(locale))
    pending.set(locale, request)
  }
  await request
}

export async function setPclLocale(locale: string): Promise<void> {
  const change = ++languageChange
  await loadPclLocale(locale)
  if (change !== languageChange) return
  currentLocale.value = locale
  if (typeof document !== 'undefined') document.documentElement.lang = locale
  try {
    localStorage.setItem('pcl-language', locale)
  } catch (error) {
    console.warn('Unable to save language preference', error)
  }
}

export async function initializePclLocale(): Promise<void> {
  let locale = 'zh-CN'
  try {
    locale = normalizeLocale(
      localStorage.getItem('pcl-language') || localStorage.getItem('ce-language') || locale,
    )
  } catch (error) {
    console.warn('Unable to read language preference', error)
  }
  try {
    await setPclLocale(locale)
  } catch (error) {
    console.warn('Unable to load language; using zh-CN', error)
    await setPclLocale('zh-CN')
  }
}

export function pclText(value = '', params: readonly unknown[] = []): string {
  const strings = loaded[currentLocale.value] || pclStrings
  const translated =
    strings[value] ??
    pclStrings[value] ??
    value.replace(
      /\{(?:Dynamic|Static)Resource ([^}]+)\}/g,
      (all, key: string) => strings[key] ?? pclStrings[key] ?? all,
    )
  return translated
    .replace(/PCL CE/g, 'PCL.Proto')
    .replace(/PCL-CE 文件夹/g, 'PCL.Proto 文件夹')
    .replace(/\{(\d+)\}/g, (all, index: string) =>
      params[Number(index)] == null ? all : String(params[Number(index)]),
    )
}
