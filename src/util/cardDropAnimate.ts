import { animate, type ElementOrSelector } from 'motion-v'
import type { Directive } from 'vue'

const cardDropAnimate = (elements: ElementOrSelector) => {
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return
  animate(
    elements,
    { y: [-20, 0], opacity: [0, 1] },
    {
      delay: (index) => Math.min(index * 0.06, 0.24),
      type: 'spring',
      duration: 0.6,
      bounce: 0.49,
    },
  )
}
export default cardDropAnimate

export function getPageContent(root: Element): HTMLElement | null {
  return (
    root.querySelector<HTMLElement>(':scope > .view-content > .subview') ??
    root.querySelector<HTMLElement>(':scope > :not(#side-nav-background)')
  )
}

export function getPageAnimationElements(root: Element): Element[] {
  return Array.from(root.children).flatMap((child) =>
    child.matches('.mycard-container') || !child.querySelector('.mycard-container')
      ? [child]
      : getPageAnimationElements(child),
  )
}

const entrances = new WeakMap<
  HTMLElement,
  { reset: () => void; refresh: () => void; dispose: () => void }
>()

export function refreshPageEntrance(root: HTMLElement) {
  entrances.get(root)?.refresh()
}

// One owner for route entrances and content that arrives after a loader disappears.
export const vPageEntrance: Directive<HTMLElement, string> = {
  mounted(root) {
    let seen = new WeakSet<Element>()
    let queued = false
    let disposed = false
    function scan() {
      queued = false
      if (disposed) return
      const content = getPageContent(root)
      if (!content || content.style.pointerEvents === 'none') return
      const viewport = content.getBoundingClientRect()
      const elements = content.children.length ? getPageAnimationElements(content) : [content]
      const fresh = elements.filter((element) => {
        if (seen.has(element)) return false
        seen.add(element)
        const bounds = element.getBoundingClientRect()
        return bounds.height > 0 && bounds.bottom > viewport.top && bounds.top < viewport.bottom
      })
      if (fresh.length) cardDropAnimate(fresh)
    }
    function schedule() {
      if (!queued) {
        queued = true
        queueMicrotask(scan)
      }
    }
    const observer = new MutationObserver((records) => {
      if (
        records.some(
          (record) =>
            !(record.target instanceof Element && record.target.closest('.mycard-container')) &&
            Array.from(record.addedNodes).some((node) => node.nodeType === 1),
        )
      )
        schedule()
    })
    observer.observe(root, { childList: true, subtree: true })
    entrances.set(root, {
      refresh: schedule,
      reset: () => {
        seen = new WeakSet()
        schedule()
      },
      dispose: () => {
        disposed = true
        observer.disconnect()
      },
    })
    scan()
  },
  updated(root, { value, oldValue }) {
    if (value !== oldValue) entrances.get(root)?.reset()
  },
  unmounted(root) {
    entrances.get(root)?.dispose()
    entrances.delete(root)
  },
}
