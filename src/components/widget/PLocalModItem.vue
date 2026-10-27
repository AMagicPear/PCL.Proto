<script setup lang="ts">
import PListItem from './PListItem.vue'
import PButton from './PButton.vue'
import PSvgIcon from './PSvgIcon.vue'
import { pclText } from '@/api/pclLocale'
defineProps<{
  id: string
  title: string
  description?: string
  icon?: string
  enabled?: boolean
  updatable?: boolean
  error?: string
  selected?: boolean
}>()
const emit = defineEmits<{ select: [boolean]; toggle: []; update: [] }>()
const fallback = `${import.meta.env.BASE_URL}pcl/images/Icons/NoIcon.png`
</script>
<template>
  <PListItem
    class="local-resource"
    :title="title"
    :subtitle="error || description"
    :class="{ disabled: enabled === false }"
  >
    <template #icon
      ><input
        type="checkbox"
        :aria-label="pclText('Proto.Label.Select', [title])"
        :checked="selected"
        @change="emit('select', ($event.target as HTMLInputElement).checked)" /><img
        :src="icon || fallback"
        alt=""
        @error="($event.target as HTMLImageElement).src = fallback"
    /></template>
    <template #actions
      ><PButton
        v-if="updatable"
        inline
        :click="() => emit('update')"
        :title="pclText('Instance.Resource.Filter.Updatable')"
        ><PSvgIcon name="lucide/upload" :size="18" /></PButton
      ><PButton
        inline
        :click="() => emit('toggle')"
        :title="
          pclText(
            enabled === false
              ? 'Instance.Resource.Filter.Enabled'
              : 'Instance.Resource.Filter.Disabled',
          )
        "
        ><PSvgIcon :name="enabled === false ? 'lucide/check' : 'lucide/x'" :size="18" /></PButton
      ><slot name="actions"
    /></template>
  </PListItem>
</template>
<style scoped>
.local-resource {
  min-height: 44px;
}
.local-resource :deep(img) {
  width: 34px;
  height: 34px;
  border-radius: 6px;
}
.local-resource :deep(.title) {
  font-size: 14px;
}
.local-resource :deep(.right) {
  display: flex;
  gap: 6px;
}
.disabled {
  opacity: 0.5;
}
</style>
