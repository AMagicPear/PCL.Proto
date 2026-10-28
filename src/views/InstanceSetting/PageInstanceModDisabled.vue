<script setup lang="ts">
import { useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'

const router = useRouter()
const text = pclText
</script>
<template>
  <div class="pcl-page">
    <div>
      <PCard data-control="PanMain" style="margin: 40px" hide-title>
        <div
          style="
            margin: 17px 20px;
            display: grid;
            grid-template-columns: minmax(0, 1fr) auto auto minmax(0, 1fr);
            grid-template-rows: auto auto auto auto;
          "
        >
          <span
            style="
              margin: 0px 0px 9px 0px;
              font-size: 19px;
              grid-row: 1 / span 1;
              grid-column: 1 / span 4;
              justify-self: center;
              color: var(--color-tint);
            "
            class="pcl-text"
            >{{ text('Instance.Resource.Mod.Disabled.Title') }}</span
          >
          <div
            style="
              height: 2px;
              grid-row: 2 / span 1;
              grid-column: 1 / span 4;
              justify-self: stretch;
            "
            class="pcl-divider"
          />
          <span
            style="margin: 15px 10px 5px 10px; grid-row: 3 / span 1; grid-column: 1 / span 4"
            class="pcl-text"
            >{{ text('Instance.Resource.Mod.Disabled.Message') }}</span
          >
          <PButton
            data-control="BtnDownload"
            style="
              height: 35px;
              min-width: 140px;
              margin: 10px 10px 0px 10px;
              padding: 0px 13px;
              grid-row: 4 / span 1;
              grid-column: 2 / span 1;
              justify-self: center;
            "
            @click="() => router.push('/download/game')"
            type="tint"
            >{{ text('Instance.Resource.Mod.Disabled.GoDownload') }}</PButton
          >
          <PButton
            data-control="BtnVersion"
            style="
              height: 35px;
              min-width: 140px;
              margin: 10px 10px 0px 10px;
              padding: 0px 13px;
              grid-row: 4 / span 1;
              grid-column: 3 / span 1;
              justify-self: center;
            "
            @click="() => router.push('/instance_select')"
            >{{ text('Instance.Resource.Mod.Disabled.SelectInstance') }}</PButton
          >
        </div>
      </PCard>
    </div>
  </div>
</template>
