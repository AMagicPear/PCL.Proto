<template>
  <div class="list-actions" @click.stop><slot /></div>
</template>

<style scoped>
.list-actions {
  display: flex;
  align-items: center;
  opacity: 0;
  pointer-events: none;
  transition: opacity 72ms;
}
:global(.gameinfo-container:hover .list-actions),
:global(.gameinfo-container:focus-within .list-actions) {
  opacity: 1;
  pointer-events: auto;
  transition-duration: 84ms;
  transition-delay: 36ms;
}
.list-actions > :deep(button) {
  width: 25px;
  height: 25px;
  padding: 5px;
  border: 0;
  border-radius: 50%;
  background: transparent;
  color: var(--color-tint-light, var(--color-tint));
  cursor: pointer;
  display: grid;
  place-items: center;
  transition:
    color 150ms,
    transform 250ms;
}
.list-actions > :deep(button:not(:disabled):hover),
.list-actions > :deep(button:not(:disabled):focus-visible) {
  color: var(--color-tint-dark, var(--color-tint));
  transition-duration: 120ms, 250ms;
}
.list-actions > :deep(button:not(:disabled):active) {
  transform: scale(0.8);
}
.list-actions > :deep(button:focus-visible) {
  outline: 2px solid var(--color-tint);
  outline-offset: 2px;
}
.list-actions > :deep(button:disabled) {
  color: var(--color-text-grey);
  cursor: not-allowed;
}
@media (hover: none) {
  .list-actions {
    opacity: 1;
    pointer-events: auto;
  }
}
@media (prefers-reduced-motion: reduce) {
  .list-actions,
  .list-actions > :deep(button) {
    transition: none;
  }
}
</style>
