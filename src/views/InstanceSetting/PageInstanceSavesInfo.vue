<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import PCard from '@/components/widget/PCard.vue'
withDefaults(defineProps<{ details?: Record<string, string | number> }>(), { details: () => ({}) })

const text = pclText
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 10px 25px 10px 25px; display: flex; flex-direction: column"
        >
          <slot name="warnings" />
          <PCard
            data-control="PanContent"
            style="margin: 15px 0px 0px 0px"
            :title="text('Instance.Saves.Info.Details.Title')"
          >
            <dl
              v-if="Object.keys(details).length"
              data-control="PanList"
              style="
                margin: 0px 15px 15px 15px;
                display: grid;
                grid-template-columns: auto 24px auto;
              "
            >
              <template v-for="(value, label) in details" :key="label"
                ><dt>{{ label }}</dt>
                <dd style="grid-column: 3">{{ value }}</dd></template
              >
            </dl>

            <p v-else class="pcl-empty">{{ pclText('Proto.Empty.SaveInfo') }}</p></PCard
          >
          <PCard
            v-if="$slots.settings"
            :title="text('Instance.Saves.Info.Settings.Title')"
            style="margin-top: 15px"
            ><slot name="settings"
          /></PCard>
        </div>
      </div>
    </div>
  </div>
</template>
