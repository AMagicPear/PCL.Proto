<script lang="ts" setup>
import { useId } from 'vue'
import shapes from '@/assets/control-geometry.json'
defineProps<{ label: string; id?: string; disabled?: boolean }>()
const controlId = useId()
const model = defineModel<boolean>({ default: false })
</script>
<template>
  <div class="option" :class="{ disabled }">
    <input
      type="checkbox"
      :disabled="disabled"
      :id="id || controlId"
      class="checkable"
      v-model="model"
    />
    <span class="checkbox-shape" aria-hidden="true"
      ><svg viewBox="0 0 12 12">
        <path :d="shapes['MyCheckBox.ShapeCheck']" fill="currentColor" /></svg
    ></span>
    <label :for="id || controlId">{{ label }}</label>
  </div>
</template>
<style scoped>
.option {
  margin-block: 4px;
  display: flex;
  align-items: center;
  gap: 8px;
  position: relative;
  min-height: 20px;
}
.checkable {
  position: absolute;
  left: 1px;
  width: 18px;
  height: 18px;
  opacity: 0;
  cursor: pointer;
}
.checkbox-shape {
  margin-left: 1px;
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  border: 1.1px solid currentColor;
  border-radius: 3px;
  display: grid;
  place-items: center;
  pointer-events: none;
}
.checkbox-shape svg {
  width: 12px;
  height: 12px;
  opacity: 0;
  transition: opacity 0.2s;
}
input:checked + .checkbox-shape {
  color: var(--color-tint);
}
input:checked + .checkbox-shape svg {
  opacity: 1;
}
input:focus-visible + .checkbox-shape {
  outline: 2px solid var(--color-tint);
  outline-offset: 3px;
}
label {
  cursor: pointer;
}
.disabled {
  opacity: 0.5;
}
.disabled input,
.disabled label {
  cursor: not-allowed;
}
</style>
