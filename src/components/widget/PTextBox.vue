<script lang="ts" setup>
import { useAttrs, useId } from 'vue'
const attrs = useAttrs()
const suggestionsId = useId()
const model = defineModel<string>()
defineProps<{
  placeholder?: string
  disabled?: boolean
  type?: 'text' | 'password'
  inputId?: string
  multiline?: boolean
  suggestions?: string[]
}>()
</script>

<template>
  <div class="input-wrapper">
    <textarea
      v-if="multiline"
      :id="inputId"
      :aria-label="String(attrs['aria-label'] || attrs.title || '')"
      :disabled="disabled"
      :placeholder
      v-model="model"
    />
    <input
      v-else
      :id="inputId"
      :aria-label="String(attrs['aria-label'] || attrs.title || '')"
      :disabled="disabled"
      :type="type || 'text'"
      :list="suggestions?.length ? suggestionsId : undefined"
      :placeholder
      v-model="model"
    />
    <datalist v-if="suggestions?.length" :id="suggestionsId">
      <option v-for="item in suggestions" :key="item" :value="item" />
    </datalist>
  </div>
</template>

<style scoped>
input,
textarea {
  background-color: transparent;
  font-size: 12px;
  padding: 0.3rem 0.8rem;
  border-radius: 4px;
  border: 1px solid var(--color-tint-lightist);
  transition: background-color 0.3s ease;
  color: var(--color-text-black);
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  width: 100%;
}

textarea {
  height: 100%;
  min-height: 80px;
  resize: vertical;
}

input:hover,
textarea:hover {
  background-color: var(--color-tint-lighter);
}

input:focus,
textarea:focus {
  background-color: var(--color-tint-lighter);
}
</style>
