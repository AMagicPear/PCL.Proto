<template>
  <button class="pcl-primary-action" type="button"><slot /></button>
</template>

<style scoped>
.pcl-primary-action {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  align-self: center;
  gap: 8px;
  min-width: 120px;
  height: 36px;
  padding: 0 12px;
  border: 0;
  border-radius: 30px;
  background: var(--tint-blue);
  color: var(--vt-c-white);
  font-family: inherit;
  font-size: 14px;
  box-shadow: var(--box-shadow);
  cursor: pointer;
  transition: background-color 0.2s, transform 0.2s;
}
.pcl-primary-action:hover:not(:disabled) { background-color: var(--light-blue); }
.pcl-primary-action:active:not(:disabled) { transform: scale(0.95); }
.pcl-primary-action:focus-visible { outline: 2px solid var(--color-tint); outline-offset: 2px; }
.pcl-primary-action:disabled { opacity: 0.5; cursor: not-allowed; }
@media (prefers-reduced-motion: reduce) {
  .pcl-primary-action { transition: none; }
}
</style>
