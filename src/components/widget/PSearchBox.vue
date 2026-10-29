<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import PButton from './PButton.vue'
import PCard from './PCard.vue'
import PSvgIcon from './PSvgIcon.vue'
defineProps<{ placeholder?: string }>()
const emit = defineEmits(['search'])
const query = defineModel<string>({ default: '' })
</script>

<template>
  <PCard hide-title>
    <section class="search-card">
      <PSvgIcon name="lucide/search" :size="14" class="search-icon" />
      <input
        type="text"
        :placeholder="placeholder"
        :aria-label="placeholder || pclText('Common.Action.Search')"
        maxlength="50"
        class="search-input"
        v-model="query"
        v-on:keydown.enter="emit('search', query)"
      />
      <PButton type="tint" class="search-button" @click="emit('search', query)">{{
        pclText('Common.Action.Search')
      }}</PButton>
    </section>
  </PCard>
</template>

<style scoped>
.search-card {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 10px;
  height: 40px;
  align-items: center;
  padding: 4px 4px 4px 14px;
}

.search-icon {
  color: var(--color-text-grey);
}

.search-input {
  flex: 1;
  min-width: 0;
  height: 100%;
  padding: 0;
  font-size: 13.5px;
  text-align: left;
  color: var(--color-text-black);
  font-family: inherit;
  border: none;
  outline: none;
  background-color: transparent;
}
.search-button {
  width: 60px;
  height: 32px;
  padding: 0;
  flex-shrink: 0;
}
</style>
