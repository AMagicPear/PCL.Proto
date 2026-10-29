export interface DownloadSettings {
  max_connections: number
  file_connections: number
  speed_limit: number
  http_mode: number
  file_source: number
  version_source: number
}

export interface DownloadStats {
  active_connections: number
  max_connections: number
  speed: number
  downloaded_bytes: number
}

export function downloadSettings(values?: Record<string, unknown>): DownloadSettings {
  if (!values) {
    try {
      values = JSON.parse(localStorage.getItem('pcl-ui:PageSetupGameManage:global') || '{}')
    } catch {
      // Invalid persisted preferences use the same defaults as the settings page.
    }
  }
  const settings = values || {}
  const integer = (key: string, fallback: number, max: number) => {
    const raw = settings[key]
    const value =
      typeof raw === 'number' || (typeof raw === 'string' && raw.trim()) ? Number(raw) : NaN
    return Number.isInteger(value) && value >= 0 && value <= max ? value : fallback
  }
  const speed = integer('SliderDownloadSpeed', 42, 42)
  const mib = 1024 * 1024
  return {
    max_connections: integer('SliderDownloadThread', 63, 255) + 1,
    file_connections: integer('SliderDownloadFileConnection', 7, 7) + 1,
    speed_limit:
      speed <= 14
        ? Math.round((speed + 1) * 0.1 * mib)
        : speed <= 31
          ? Math.round((speed - 11) * 0.5 * mib)
          : speed <= 41
            ? (speed - 21) * mib
            : 0,
    http_mode: integer('ComboDownloadHttpMode', 0, 2),
    file_source: integer('ComboDownloadSource', 1, 2),
    version_source: integer('ComboDownloadVersion', 1, 2),
  }
}
