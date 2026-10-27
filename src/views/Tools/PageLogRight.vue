<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import PCard from '@/components/widget/PCard.vue'
import PButton from '@/components/widget/PButton.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { pclText } from '@/api/pclLocale'
import { usePageSettings } from '@/composables/usePageSettings'
import sideTip from '@/composables/sideTip'
import { maxLogLines } from '@/util/logLines'
const props = withDefaults(defineProps<{ log?: string; stack?: string }>(), { log: '', stack: '' })
const text = pclText
const { values } = usePageSettings('PageLogRight', { SliderMaxLog: 13 })
const cleared = ref(false)
const autoScroll = ref(true)
const logElement = ref<HTMLElement>()
const maxLines = computed(() => maxLogLines(Number(values.SliderMaxLog)))
const content = computed(() =>
  cleared.value ? '' : props.log.split('\n').slice(-maxLines.value).join('\n'),
)
watch(
  [content, autoScroll],
  () => {
    if (autoScroll.value && logElement.value)
      logElement.value.scrollTop = logElement.value.scrollHeight
  },
  { flush: 'post' },
)
function exportText(value: string, filename: string) {
  if (!value) {
    sideTip.show(pclText('Proto.Empty.ExportLog'))
    return
  }
  const url = URL.createObjectURL(new Blob([value], { type: 'text/plain;charset=utf-8' }))
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}
</script>
<template>
  <article class="subview pcl-page log-page">
    <PCard hide-title class="log-content">
      <pre ref="logElement">{{ content || pclText('Proto.Empty.Log') }}</pre>
    </PCard>
    <PCard hide-title
      ><div class="log-actions">
        <PCheckBox
          id="CheckAutoScroll"
          v-model="autoScroll"
          :label="text('LogPage.Action.Scroll')"
        />
        <PButton inline :click="() => exportText(log, 'PCL.Proto.log')"
          ><PSvgIcon name="lucide/save" />{{ text('LogPage.Action.Export') }}</PButton
        >
        <PButton inline :click="() => (cleared = true)"
          ><PSvgIcon name="lucide/trash-2" />{{ text('LogPage.Action.Clear') }}</PButton
        >
        <PButton inline :click="() => sideTip.show(pclText('Proto.NotImplemented.Processes'))"
          ><PSvgIcon name="lucide/x" />{{ text('LogPage.Action.KillProcess') }}</PButton
        >
        <PButton inline :disabled="!stack" :click="() => exportText(stack, 'PCL.Proto-stack.log')"
          ><PSvgIcon name="lucide/package" />{{ text('LogPage.Action.ExportStack') }}</PButton
        >
      </div>
      <div class="log-limit">
        <label for="SliderMaxLog">{{ text('LogPage.MaxLines.Title') }}</label
        ><input
          id="SliderMaxLog"
          type="range"
          min="0"
          max="29"
          v-model.number="values.SliderMaxLog"
          :title="text('LogPage.MaxLines.ToolTip')"
        /><output>{{
          Number.isFinite(maxLines) ? maxLines : text('LogPage.MaxLines.Unlimited')
        }}</output>
      </div></PCard
    >
  </article>
</template>
<style scoped>
.log-page {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  height: 100%;
  overflow-y: auto;
}
.log-content {
  flex: 1;
  min-height: 120px;
}
.log-content pre {
  padding: 15px;
  white-space: pre-wrap;
  font: 12px/1.6 monospace;
  user-select: text;
  overflow: auto;
  max-height: calc(100vh - 220px);
}
.log-actions {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  flex-wrap: wrap;
  padding: 12px;
}
.log-limit {
  display: flex;
  gap: 25px;
  align-items: center;
  padding: 0 12px 12px;
}
.log-limit input {
  flex: 1;
}
</style>
