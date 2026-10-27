<script lang="ts">
import { defineComponent, onMounted, onUnmounted, useTemplateRef, ref, provide } from 'vue'
import useSideNavState from '@/stores/windowState'
import SideGroup from '@/components/widget/SideGroup.vue'
import { type INavItemGroup } from '@/types/naviOptions'
import cardDropAnimate, { getPageAnimationElements } from '@/util/cardDropAnimate'
import { useModal } from '@/composables/useModal'
import { useRoute } from 'vue-router'

export default defineComponent({
  name: 'SideNavLayout',
  components: {
    SideGroup,
  },
  props: {
    sideNavGroups: {
      type: Array as () => INavItemGroup[],
      required: true,
    },
  },
  setup() {
    const route = useRoute()
    let animatedPath = ''
    const revision = ref(0)
    provide('navAction', (kind: string) => {
      if (kind === 'refresh') revision.value++
      else
        void useModal().open({
          title: '提示',
          content: '初始化设置尚未接入 PCL.Proto。',
          plainText: true,
        })
    })
    let observer: ResizeObserver | null = null
    const asideRef = useTemplateRef<HTMLElement>('asideRef')
    const subviewRef = useTemplateRef<HTMLElement>('subviewRef')
    const sideNavState = useSideNavState()

    function updateAsideBackgroundWidth() {
      if (asideRef.value) {
        sideNavState.setWidth(asideRef.value.offsetWidth)
      }
    }

    function animateSubview() {
      if (subviewRef.value) {
        cardDropAnimate(getPageAnimationElements(subviewRef.value))
        animatedPath = route.path
      }
    }

    function animateOnNavigation() {
      if (animatedPath !== route.path) animateSubview()
    }

    onMounted(async () => {
      asideRef.value?.querySelectorAll('.sidenav-line').forEach((el_, i) => {
        let el = el_ as HTMLDivElement
        el.style.animationPlayState = 'paused'
        el.style.animationDelay = `${i * 0.02}s`
        requestAnimationFrame(() => {
          el.style.animationPlayState = 'running'
        })
      })
      observer = new ResizeObserver(updateAsideBackgroundWidth)
      observer.observe(asideRef.value!)
    })

    onUnmounted(() => {
      observer?.disconnect()
    })

    return {
      asideRef,
      subviewRef,
      animateSubview,
      animateOnNavigation,
      revision,
    }
  },
})
</script>

<template lang="pug">
.view-content
    aside(ref="asideRef")
        SideGroup(
            v-for="group in sideNavGroups"
            v-bind="group"
        )
    article.subview(ref="subviewRef")
        RouterView(:key="revision" v-slot="{ Component }")
            component(:is="Component" @vue:mounted="animateSubview" @vue:updated="animateOnNavigation")
</template>

<style scoped>
article {
  flex: 1 1 auto;
  min-width: 0;
  overflow-y: auto;
}

aside {
  padding: 12px 0 0;
  width: 152px;
  overflow-y: auto;
  height: 100%;
  flex: 0 0 auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

@keyframes fade-in-left {
  70% {
    transform: translateX(2px);
  }
  100% {
    opacity: 1;
    transform: translateX(0);
  }
}

aside :deep(.sidenav-line) {
  opacity: 0;
  will-change: transform, opacity;
  transform: translateX(-100%);
  animation: fade-in-left 0.45s ease-in-out forwards;
}
</style>
