import { invoke, isTauri } from '@tauri-apps/api/core'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'

/**
 * Resolve a link that leaves the app WebView.
 * Resolve relative links too, keeping same-origin routes inside the app.
 */
export function externalUrl(value: string | URL | null | undefined): string | undefined {
  if (!value) return undefined
  try {
    const url = value instanceof URL ? value : new URL(value, window.location.href)
    if (url.protocol !== 'http:' && url.protocol !== 'https:') return undefined
    if (url.origin === window.location.origin) return undefined
    return url.href
  } catch {
    return undefined
  }
}

/** Open external links in the system browser under Tauri, or a new browser tab on the web. */
export async function openExternal(value: string | URL | null | undefined): Promise<boolean> {
  const url = externalUrl(value)
  if (!url) return false
  try {
    const parsed = new URL(url)
    if (parsed.username || parsed.password) throw new Error('Invalid external URL')
    if (isTauri()) {
      await invoke('open_external_url', { url })
    } else {
      window.open(url, '_blank', 'noopener,noreferrer')
    }
    return true
  } catch (error) {
    console.error('Failed to open external link', error)
    sideTip.show(pclText('Proto.Error.OpenUrl'), 'warn')
    return false
  }
}

/**
 * Keep plain <a> links from navigating the app WebView to a remote page.
 * Capture phase so handlers that stop propagation on the target cannot bypass it.
 */
export function installExternalLinkHandler(): () => void {
  if (!isTauri()) return () => {}
  const handle = (event: MouseEvent) => {
    if (event.defaultPrevented) return
    if (event.type === 'click' ? event.button !== 0 : event.button !== 1) return
    const target = event.target
    const element = target instanceof Element ? target : (target as Node | null)?.parentElement
    const anchor = element?.closest<HTMLAnchorElement>('a[href]')
    const url = anchor ? externalUrl(anchor.href) : undefined
    if (!url) return
    event.preventDefault()
    event.stopPropagation()
    void openExternal(url)
  }
  document.addEventListener('click', handle, true)
  document.addEventListener('auxclick', handle, true)
  return () => {
    document.removeEventListener('click', handle, true)
    document.removeEventListener('auxclick', handle, true)
  }
}
