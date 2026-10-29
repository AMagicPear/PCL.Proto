<script setup lang="ts">
import type { INavItem } from '@/types/naviOptions'
import { RouterLink } from 'vue-router'
import { inject } from 'vue'
import PSelectionIndicator from './PSelectionIndicator.vue'
const runAction = inject<(kind: string) => void>('navAction')
defineProps<INavItem>()
</script>

<template>
  <div class="sidenav-line">
    <component
      :is="linkto ? RouterLink : 'a'"
      :to="linkto"
      @click="clickCallback"
      :class="{ custom: !linkto }"
    >
      <PSelectionIndicator v-if="linkto" />

      <i class="side-nav-icon" v-if="icon"><icon /></i>
      <p>{{ text }}</p>
    </component>
    <button
      v-if="action"
      class="refresh-icon pcl-icon-button"
      type="button"
      :title="action.title"
      :aria-label="action.title"
      @click="runAction?.(action.kind)"
    >
      <component :is="action.icon" />
    </button>
  </div>
</template>

<style scoped>
a {
  width: 100%;
  display: flex;
  gap: 11px;
  align-items: center;
  justify-content: flex-start;
  font-size: 13px;
  color: var(--color-text-black);
  height: 36px;
  background-color: transparent;
  transition: 0.4s;
  padding-inline-end: 40px;
}

.sidenav-line:hover > a {
  background-color: var(--color-tint-lighter);
}

a.router-link-active {
  color: var(--color-tint);
}

a.custom {
  /* left: 11px gap + 4px width */
  padding-inline-start: 15px;
  cursor: pointer;
}

.sidenav-line {
  position: relative;
}
.refresh-icon {
  position: absolute;
  right: 8px;
  top: 7px;
  width: 22px;
  height: 22px;
  padding: 3px;
  opacity: 0;
  transition:
    opacity 0.4s,
    color 150ms,
    transform 250ms;
}

.sidenav-line:has(a.router-link-active):hover .refresh-icon,
.refresh-icon:focus-visible {
  opacity: 1;
}

.side-nav-icon {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 修复Safari图标不显示的问题 */
:deep(.side-nav-icon > svg) {
  max-width: 100%;
  max-height: 100%;
}
</style>
