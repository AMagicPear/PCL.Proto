<script setup lang="ts">
import { showUnavailable } from '@/util/showUnavailable'
import { pclText } from '@/api/pclLocale'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
withDefaults(defineProps<{ screenshots?: { id: string; url: string; name: string }[] }>(), {
  screenshots: () => [],
})
const safeUrl = (value: string) =>
  /^(https?:|blob:|asset:|data:image\/)/i.test(value) ? value : undefined

const text = pclText
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div style="display: grid">
          <div v-if="screenshots.length" class="screenshots-content">
            <PCard :title="text('Instance.Screenshot.QuickActions')"
              ><div class="screenshot-actions">
                <PButton type="tint" @click="showUnavailable">{{
                  text('Instance.Screenshot.OpenFolder')
                }}</PButton>
              </div></PCard
            >
            <div class="screenshots">
              <a
                v-for="screenshot in screenshots"
                :key="screenshot.id"
                :href="safeUrl(screenshot.url)"
                target="_blank"
                rel="noopener noreferrer"
                ><img
                  :src="safeUrl(screenshot.url)"
                  :alt="screenshot.name"
                  loading="lazy"
                /><span>{{ screenshot.name }}</span></a
              >
            </div>
          </div>
          <PCard
            v-if="!screenshots.length"
            data-control="PanNoPic"
            style="
              margin: 40px;
              grid-row: 1 / span 1;
              grid-column: 1 / span 1;
              justify-self: center;
              align-self: center;
            "
            hide-title
          >
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
                >{{ text('Instance.Screenshot.Empty.Title') }}</span
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
                >{{ text('Instance.Screenshot.Empty.Message') }}</span
              >
              <PButton
                data-control="BtnOpenFolderTop"
                style="
                  height: 35px;
                  min-width: 140px;
                  margin: 10px 10px 0px 10px;
                  padding: 0px 13px;
                  grid-row: 4 / span 1;
                  grid-column: 2 / span 1;
                  justify-self: center;
                "
                @click="showUnavailable"
                type="tint"
                >{{ text('Instance.Screenshot.OpenFolder') }}</PButton
              >
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.screenshots-content {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.screenshot-actions {
  padding: 0 25px 15px;
}
.screenshots {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 15px;
}
.screenshots a {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.screenshots img {
  width: 100%;
  border-radius: 6px;
}
</style>
