const mainLoaders = ['Forge', 'Cleanroom', 'NeoForge', 'Fabric', 'LegacyFabric', 'LabyMod']

export function fabricInstanceName(version: string, selected: Record<string, string>) {
  return version + (selected.Fabric
    ? `-Fabric_${selected.Fabric.replace('+build', '')}`
    : selected.LegacyFabric ? `-LegacyFabric_${selected.LegacyFabric}` : '')
}

// Mirrors PageDownloadInstall.SelectModLoader, including loader-specific add-ons.
export function selectLoader(selected: Record<string, string>, loader: string, version: string) {
  if (mainLoaders.includes(loader)) {
    for (const name of mainLoaders) if (name !== loader) delete selected[name]
    if (loader !== 'Fabric') {
      delete selected.FabricApi
      delete selected.OptiFabric
    }
    if (loader !== 'LegacyFabric') delete selected.LegacyFabricApi
  }
  selected[loader] = version
  if (!selected.Fabric) {
    delete selected.FabricApi
    delete selected.OptiFabric
  }
  if (!selected.LegacyFabric) delete selected.LegacyFabricApi
  if (!selected.OptiFine) delete selected.OptiFabric
}

// McInstanceInfo.VersionToDrop(..., allowSnapshot: true).
export function versionDrop(version: string) {
  const lower = version.toLowerCase()
  if (lower.startsWith('1.rv')) return 90
  if (lower.startsWith('3d shareware')) return 140
  const snapshot = /^(\d{2})w\d/.exec(lower)
  if (snapshot) {
    const year = Number(snapshot[1])
    return year <= 16
      ? Math.max(year * 10 - 70, 20)
      : year === 17
        ? 120
        : Math.min(130 + (year - 18) * 10, 210)
  }
  const match = /^(\d+)\.(\d+)/.exec(lower)
  if (!match) return -1
  const major = Number(match[1]),
    minor = Number(match[2])
  return major === 1 ? minor * 10 : major >= 25 ? major * 10 + minor : major === 2 ? 50 : -1
}

// McVersionComparer.CompareVersion: prereleases sort before the corresponding release.
function compareVersions(left: string, right: string) {
  left = left.toLowerCase()
  right = right.toLowerCase()
  const tokens = (value: string) =>
    value
      .replace(/快照/g, 'snapshot')
      .replace(/预览版/g, 'pre')
      .match(/[a-z]+|[0-9]+/g) || []
  const a = tokens(left),
    b = tokens(right)
  const prereleases: Record<string, number> = { rc: -1, pre: -2, snapshot: -3, experimental: -4 }
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    const x = a[i] || '0',
      y = b[i] || '0'
    if (x === y) continue
    const nx = prereleases[x] ?? (Number(x) || 0),
      ny = prereleases[y] ?? (Number(y) || 0)
    const delta = nx === 0 && ny === 0 ? (x > y ? 1 : x < y ? -1 : 0) : nx - ny
    if (delta) return delta
  }
  return left > right ? 1 : left < right ? -1 : 0
}

export function loaderVisible(loader: string, version: string, selected: Record<string, string>) {
  const drop = versionDrop(version)
  const formatted = /^1\.\d/.test(version) || Number(/^[2-9]\d\.\d+/.exec(version)?.[0]) > 25
  switch (loader) {
    case 'OptiFine':
      return compareVersions(version, '1.7.2') >= 0
    case 'Forge':
      return formatted && compareVersions(version, '1.1') >= 0
    case 'NeoForge':
      return compareVersions(version, '1.20.1') >= 0
    case 'Cleanroom':
      return version === '1.12.2'
    case 'LiteLoader':
      return (
        formatted &&
        compareVersions(version, '1.5.2') >= 0 &&
        compareVersions(version, '1.12.2') <= 0
      )
    case 'Fabric':
      return drop > 130 || (drop === 130 && compareVersions(version, '18w43b') >= 0)
    case 'LegacyFabric':
      return drop >= 30 && drop <= 130
    case 'LabyMod':
      return formatted && compareVersions(version, '1.8.9') >= 0
    case 'FabricApi':
      return !!selected.Fabric
    case 'LegacyFabricApi':
      return !!selected.LegacyFabric
    case 'OptiFabric':
      return !!selected.Fabric && !!selected.OptiFine
    default:
      return false
  }
}

export function installWarnings(selected: Record<string, string>, version: string) {
  const warnings: string[] = []
  const drop = versionDrop(version)
  if (selected.Fabric && !selected.FabricApi) warnings.push('FabricApi')
  if (selected.LegacyFabric && !selected.LegacyFabricApi) warnings.push('LegacyFabricApi')
  if ((selected.Fabric || selected.LegacyFabric) && selected.OptiFine && !selected.OptiFabric)
    warnings.push(
      drop >= 140 && drop <= 150
        ? 'OptiFabricOld'
        : selected.LegacyFabric
          ? 'LegacyOptiFabric'
          : 'OptiFabric',
    )
  if (drop >= 160 && selected.OptiFine && (selected.Forge || selected.Fabric))
    warnings.push('ModOptiFine')
  return warnings
}
