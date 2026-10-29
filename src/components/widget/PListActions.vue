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
:global(.comp-item:hover .list-actions),
:global(.comp-item:focus-within .list-actions) {
  opacity: 1;
  pointer-events: auto;
}
@media (hover: none) {
  .list-actions {
    opacity: 1;
    pointer-events: auto;
  }
}
@media (prefers-reduced-motion: reduce) {
  .list-actions {
    transition: none;
  }
}
</style>
