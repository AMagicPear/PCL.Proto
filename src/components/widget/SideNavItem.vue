<script setup lang="ts">
import type { INavItem } from '@/types/naviOptions'
import { RouterLink } from 'vue-router'
import { inject } from 'vue'
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
      <svg class="indicator" width="4" height="23" viewBox="0 0 4 23" v-if="linkto">
        <line
          x1="2"
          y1="2"
          x2="2"
          y2="21"
          stroke="currentColor"
          stroke-width="4"
          stroke-linecap="round"
        />
      </svg>

      <i class="side-nav-icon" v-if="icon"><icon /></i>
      <p>{{ text }}</p>
    </component>
    <button
      v-if="action"
      class="refresh-icon button-animated"
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
  padding-inline-end: 10px;
}

a:hover {
  background-color: var(--color-tint-lighter);
}

a.router-link-active {
  color: var(--color-tint);
}

a.custom {
  /* left: 11px gap + 4px width */
  padding-inline: 15px 22px;
  cursor: pointer;
}

.indicator {
  visibility: hidden;
  transform: scaleY(0);
  transform-origin: center;
  transition: none;
}

a.router-link-active .indicator {
  visibility: visible;
  transform: scaleY(1);
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
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
  border: none;
  background: transparent;
  cursor: pointer;
  color: var(--color-tint-light);
  opacity: 0;
  transition: 0.4s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.refresh-icon:hover {
  color: var(--color-titlebar);
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
