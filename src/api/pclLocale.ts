import { shallowReactive } from 'vue'
import zh from '../assets/languages/zh-CN.json'
const loaders = import.meta.glob(
  ['../assets/languages/*.json', '!../assets/languages/zh-CN.json'],
  { import: 'default' },
) as Record<string, () => Promise<Record<string, string>>>
export const pclStrings: Record<string, string> = zh
const loaded = shallowReactive<Record<string, Record<string, string>>>({ 'zh-CN': zh })
export async function loadPclLocale(locale: string) {
  const loader = loaders[`../assets/languages/${locale}.json`]
  if (!loaded[locale] && loader) loaded[locale] = await loader()
}
export function pclText(value = '', locale = 'zh-CN'): string {
  const strings = loaded[locale] || pclStrings
  if (strings[value] !== undefined || pclStrings[value] !== undefined)
    return appText(strings[value] ?? pclStrings[value]!)
  return appText(
    value.replace(
      /\{(?:Dynamic|Static)Resource ([^}]+)\}/g,
      (all, key: string) => strings[key] ?? pclStrings[key] ?? all,
    ),
  )
}

function appText(value: string): string {
  return value.replace(/PCL CE/g, 'PCL.Proto').replace(/PCL-CE 文件夹/g, 'PCL.Proto 文件夹')
}
