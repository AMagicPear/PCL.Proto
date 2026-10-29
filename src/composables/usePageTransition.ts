import { nextTick, onScopeDispose, type Ref } from 'vue'
import { isNavigationFailure, NavigationFailureType, useRouter } from 'vue-router'
import { getPageAnimationElements } from '@/util/cardDropAnimate'

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
    const content =
      page.value?.querySelector<HTMLElement>(':scope > .view-content > .subview') ??
      page.value?.querySelector<HTMLElement>(':scope > :not(#side-nav-background)')
    if (!content) return

    // CE: 70 ms / -6 px per block, 15 ms stagger; main/subpage switches wait 110/130 ms.
    // Share the exit when navigation is requested again before the old page leaves.
    if (!exit) {
      const pointerEvents = content.style.pointerEvents
      content.style.pointerEvents = 'none'
      const elements = content.children.length ? getPageAnimationElements(content) : [content]
      const animations = elements.map((element, index) => {
        const { opacity, transform } = getComputedStyle(element)
        return element.animate(
          [
            { opacity, transform },
            { opacity: 0, transform: `${transform === 'none' ? '' : transform} translateY(-6px)` },
          ],
          { duration: 70, delay: index * 15, easing: 'linear', fill: 'forwards' },
        )
      })
      exit = {
        finished: new Promise((resolve) =>
          window.setTimeout(resolve, to.matched[0] === from.matched[0] ? 130 : 110),
        ),
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
  })
  const removeError = router.onError(restore)

  onScopeDispose(() => {
    removeGuard()
    removeAfter()
    removeError()
    restore()
  })
}
