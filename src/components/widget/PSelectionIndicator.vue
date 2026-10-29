<template>
  <svg class="selection-indicator" width="4" height="23" viewBox="0 0 4 23" aria-hidden="true">
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
</template>

<style scoped>
.selection-indicator {
  flex-shrink: 0;
  color: var(--color-tint);
  visibility: hidden;
  transform: scaleY(0);
  transform-origin: center;
  transition: none;
}
:global(a.router-link-active > .selection-indicator),
:global(.gameinfo-container[aria-pressed='true'] > .selection-indicator) {
  visibility: visible;
  transform: scaleY(1);
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}
@media (prefers-reduced-motion: reduce) {
  .selection-indicator {
    transition: none !important;
  }
}
</style>
