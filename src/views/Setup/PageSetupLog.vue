<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
const { values, text, change, act, image } = usePageSettings('PageSetupLog', {})
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div style="display: flex; flex-direction: column">
          <PCard style="margin: 25px 25px 5px 25px" :title="text('Setup.Log.Operations')">
            <div
              style="
                margin: 0px 20px 20px 25px;
                display: flex;
                flex-direction: row;
                align-items: center;
                flex-wrap: wrap;
              "
            >
              <PButton
                style="min-width: 100px; margin: 0px 15px 0px 0px"
                :click="() => act('ButtonExport_OnClick', 'ButtonExport_OnClick', '', '')"
                type="tint"
                >{{ text('Setup.Log.Export') }}</PButton
              >
              <PButton
                style="min-width: 120px; margin: 0px 15px 0px 0px"
                :click="() => act('ButtonExportAll_OnClick', 'ButtonExportAll_OnClick', '', '')"
                >{{ text('Setup.Log.ExportAll') }}</PButton
              >
              <PButton
                style="min-width: 120px; margin: 0px 15px 0px 0px"
                :click="() => act('ButtonOpenDir_OnClick', 'ButtonOpenDir_OnClick', '', '')"
                >{{ text('Setup.Log.OpenDirectory') }}</PButton
              >
              <PButton
                style="min-width: 120px"
                :click="() => act('ButtonClean_OnClick', 'ButtonClean_OnClick', '', '')"
                type="warn"
                >{{ text('Setup.Log.ClearHistory') }}</PButton
              >
            </div>
          </PCard>
          <PCard
            data-control="PanContent"
            style="margin: 10px 25px 25px 25px"
            :title="text('Setup.Log.AllLogs')"
          >
            <div
              data-control="PanList"
              style="margin: 0px 18px 18px 20px; display: flex; flex-direction: column"
            >
              <p class="pcl-empty">{{ pclText('Proto.Empty.LogFiles') }}</p>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
