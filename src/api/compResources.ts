import categories from '@/assets/download-categories.json'
import { pclLocale, pclText } from './pclLocale'
import type { ISearchHit, IVersion } from './modrinthApi'

export type CompType = keyof typeof categories

const categoryKeys: Record<string, string> = Object.fromEntries(
  Object.values(categories).flatMap((items) =>
    items
      .filter((item) => item.tag.split('/')[1])
      .map((item) => [item.tag.split('/')[1], item.text]),
  ),
)
Object.assign(categoryKeys, {
  iris: 'Download.Comp.Category.Iris',
  optifine: 'Download.Comp.Category.Optifine',
  vanilla: 'Download.Comp.Filter.Loader.VanillaAvailable',
})
const hiddenTags = ['datapack', 'themed', 'cursed', 'entities', 'potato', 'low', 'medium', 'high']
const loaderNames: Record<string, string> = {
  forge: 'Forge',
  neoforge: 'NeoForge',
  fabric: 'Fabric',
  quilt: 'Quilt',
}

export function compTagOptions(type: CompType) {
  return categories[type]
    .filter((item) => !item.tag || item.tag.split('/')[1])
    .map((item) => ({
      key: item.tag || (item.text === 'Common.Option.All' ? '' : item.text),
      text: pclText(item.text),
      disabled: !item.tag && item.text !== 'Common.Option.All',
    }))
}

export function compTags(project: ISearchHit) {
  const tags = project.categories.flatMap((category) => {
    if (hiddenTags.includes(category)) return []
    if (
      project.project_type === 'resourcepack' &&
      ['decoration', 'mobs', 'equipment'].includes(category)
    )
      return []
    const key = categoryKeys[category]
    return key ? [pclText(key)] : []
  })
  return tags.length
    ? [...new Set(tags)].sort((a, b) => a.localeCompare(b, pclLocale.value))
    : [pclText('Download.Comp.Category.Other')]
}

export function compFacets(
  type: CompType,
  values: { loader: string; version: string; tag: string },
) {
  const facets = [[`project_type:${type === 'datapack' ? 'mod' : type}`]]
  if (type === 'datapack') facets.push(['categories:datapack'])
  if (values.loader && ['mod', 'modpack', 'shader'].includes(type))
    facets.push([`categories:${values.loader}`])
  if (values.version.trim()) facets.push([`versions:${values.version.trim()}`])
  const tag = values.tag.split('/')[1]
  if (tag) facets.push([`categories:${tag}`])
  return facets
}

export function compLoaders(project: ISearchHit) {
  return [...new Set([...(project.loaders || []), ...project.categories])]
    .flatMap((loader) => (loaderNames[loader] ? [loaderNames[loader]] : []))
    .join(' / ')
}

export function compVersions(versions: string[] = []) {
  const releases = [
    ...new Set(
      versions.flatMap((version) =>
        /^\d+\.\d+(?:\.\d+)?$/.test(version) ? [version.split('.').slice(0, 2).join('.')] : [],
      ),
    ),
  ].sort((a, b) => b.localeCompare(a, 'en', { numeric: true }))
  if (!releases.length)
    return pclText(
      versions.length
        ? 'Download.Comp.Detail.CompItem.SnapshotOnly'
        : 'Download.Comp.Detail.CompItem.UnknownVersion',
    )
  const segments: string[] = []
  for (let i = 0; i < releases.length; i++) {
    const start = releases[i]!
    let end = start
    while (i + 1 < releases.length) {
      const next = releases[i + 1]!
      if (
        next.split('.')[0] !== end.split('.')[0] ||
        Number(next.split('.')[1]) !== Number(end.split('.')[1]) - 1
      )
        break
      end = next
      i++
    }
    segments.push(start === end ? start : `${end}~${start}`)
  }
  return segments.join(', ')
}

export function latestCompFile(versions: IVersion[], gameVersion = '', loader = '') {
  const compatible = versions
    .filter(
      (version) =>
        (!gameVersion || version.game_versions.includes(gameVersion)) &&
        (!loader || version.loaders.includes(loader)),
    )
    .sort((a, b) => Date.parse(b.date_published) - Date.parse(a.date_published))
  const files = compatible[0]?.files || []
  const file = files.find((file) => file.primary) || files[0]
  return file && /^https:\/\//i.test(file.url) ? file : undefined
}

export function groupCompVersions(
  versions: readonly IVersion[],
  gameVersion = '',
  loader = '',
  splitLoaders = false,
) {
  const groups = new Map<
    string,
    { key: string; name: string; gameVersion: string; versions: IVersion[] }
  >()
  const comparer = new Intl.Collator('en', { numeric: true })
  for (const version of [...versions].sort(
    (a, b) => Date.parse(b.date_published) - Date.parse(a.date_published),
  )) {
    if (loader && !version.loaders.includes(loader)) continue
    for (const game of new Set(version.game_versions.length ? version.game_versions : [''])) {
      if (gameVersion && game !== gameVersion) continue
      const name = !game
        ? pclText('Download.Comp.Detail.VersionGroup.Other')
        : game.includes('w')
          ? pclText('Download.Comp.Detail.VersionGroup.Snapshot')
          : game
      const loaders =
        splitLoaders && game.includes('.')
          ? [...new Set(version.loaders)].filter((value) => !loader || value === loader)
          : ['']
      for (const value of loaders.length ? loaders : ['']) {
        const key = `${value}:${name}`
        let group = groups.get(key)
        if (!group) {
          group = {
            key,
            name: `${loaderNames[value] || value}${value ? ' ' : ''}${name}`,
            gameVersion: name,
            versions: [],
          }
          groups.set(key, group)
        }
        if (group.versions[group.versions.length - 1] !== version) group.versions.push(version)
      }
    }
  }
  return [...groups.values()].sort(
    (a, b) =>
      Number(b.gameVersion.includes('.')) - Number(a.gameVersion.includes('.')) ||
      comparer.compare(b.gameVersion, a.gameVersion) ||
      comparer.compare(b.name, a.name),
  )
}
