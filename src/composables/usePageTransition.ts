import { nextTick, onScopeDispose, type Ref } from 'vue'
import { isNavigationFailure, NavigationFailureType, useRouter } from 'vue-router'
import {
  getPageAnimationElements,
  getPageContent,
  refreshPageEntrance,
} from '@/util/cardDropAnimate'

export function usePageTransition(page: Ref<HTMLElement | null>) {
  const router = useRouter()
  let exit: { finished: Promise<void>; restore: () => void } | undefined

  function restore() {
    exit?.restore()
    exit = undefined
  }

  const removeGuard = router.beforeResolve(async (to, from) => {
    if (to.path === from.path || window.matchMedia('(prefers-reduced-motion: reduce)').matches)
      return
    const content = page.value && getPageContent(page.value)
    if (!content) return

    // CE: 70 ms / -6 px per block, 15 ms stagger; main/subpage switches wait 110/130 ms.
    // Share the exit when navigation is requested again before the old page leaves.
    if (!exit) {
      const duration = to.matched[0] === from.matched[0] ? 130 : 110
      const pointerEvents = content.style.pointerEvents
      content.style.pointerEvents = 'none'
      const viewport = content.getBoundingClientRect()
      const right = (
        content.children.length ? getPageAnimationElements(content) : [content]
      ).filter((element) => {
        const bounds = element.getBoundingClientRect()
        return bounds.height > 0 && bounds.bottom > viewport.top && bounds.top < viewport.bottom
      })
      const aside =
        to.matched[0] !== from.matched[0]
          ? page.value?.querySelector<HTMLElement>(':scope > .view-content > aside')
          : null
      const elements = aside ? [...right, aside] : right
      // Read all styles before starting animations to avoid forcing layout once per card.
      const frames = elements.map((element) => {
        const { opacity, transform } = getComputedStyle(element)
        const end =
          element === aside
            ? element.classList.contains('launch-left')
              ? 'scale(0.95)'
              : 'translateX(-6px)'
            : 'translateY(-6px)'
        return [
          { opacity, transform },
          { opacity: 0, transform: `${transform === 'none' ? '' : transform} ${end}` },
        ]
      })
      const animations = elements.map((element, index) =>
        element.animate(frames[index]!, {
          duration: 70,
          delay: elements[index] === aside ? 0 : Math.min(index * 15, duration - 70),
          easing: 'linear',
          fill: 'forwards',
        }),
      )
      exit = {
        finished: new Promise((resolve) => window.setTimeout(resolve, duration)),
        restore: () => {
          animations.forEach((animation) => animation.cancel())
          content.style.pointerEvents = pointerEvents
        },
      }
    }
    await exit.finished
  })

  const removeAfter = router.afterEach(async (_to, _from, failure) => {
    if (isNavigationFailure(failure, NavigationFailureType.cancelled)) return
    const previous = exit
    exit = undefined
    // Reused page components must render and start their entrance before removing the exit.
    await nextTick()
    previous?.restore()
    if (page.value) refreshPageEntrance(page.value)
  })
  const removeError = router.onError(restore)

  onScopeDispose(() => {
    removeGuard()
    removeAfter()
    removeError()
    restore()
  })
}
