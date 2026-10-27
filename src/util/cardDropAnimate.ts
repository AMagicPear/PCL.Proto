import { animate, stagger, type ElementOrSelector } from 'motion-v'
import type { Directive, RendererNode } from 'vue'

const cardDropAnimate = (element: ElementOrSelector | RendererNode) => {
  animate(
    element as ElementOrSelector,
    { y: [-20, 0], opacity: [0, 1] },
    {
      delay: stagger(0.06, { startDelay: 0 }),
      type: 'spring',
      duration: 0.6,
      bounce: 0.49,
    },
  )
}

export default cardDropAnimate

export function getPageAnimationElements(root: Element): Element[] {
  return Array.from(root.children).flatMap((child) =>
    child.matches('.mycard-container') || !child.querySelector('.mycard-container')
      ? [child]
      : getPageAnimationElements(child),
  )
}

// directives that should be register at main.ts and can be used anywhere
export const vAnimateDrop: Directive<HTMLElement, void> = {
  mounted(el) {
    cardDropAnimate(el)
  },
}

export const vAnimateChildrenDrop: Directive<HTMLElement, boolean | undefined> = {
  mounted(el, { value }) {
    if (value !== false) cardDropAnimate(getPageAnimationElements(el))
  },
  updated(el, { value, oldValue }) {
    if (value && value !== oldValue) cardDropAnimate(getPageAnimationElements(el))
  },
}
