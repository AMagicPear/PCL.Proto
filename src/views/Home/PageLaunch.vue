<script setup lang="ts">
import { onMounted } from 'vue'
import useSideNavState from '@/stores/windowState'
import PageLaunchLeft from './PageLaunchLeft.vue'
onMounted(() => useSideNavState().setWidthOfPageDefault('home'))
</script>
<template>
  <div class="view-content">
    <PageLaunchLeft />
    <article class="subview launch-right"><RouterView /></article>
  </div>
</template>
<style scoped>
.launch-right {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
}
</style>
