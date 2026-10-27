<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import PListItem from '@/components/widget/PListItem.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PSearchBox from '@/components/widget/PSearchBox.vue'
const props = withDefaults(
  defineProps<{ saves?: { id: string; name: string; description?: string; icon?: string }[] }>(),
  { saves: () => [] },
)
const router = useRouter()
const sort = ref('name')
const query = ref('')
const matching = computed(() =>
  props.saves.filter((save) =>
    `${save.name} ${save.description || ''}`.toLowerCase().includes(query.value.toLowerCase()),
  ),
)
const savesSorted = computed(() =>
  sort.value === 'name'
    ? [...matching.value].sort((a, b) => a.name.localeCompare(b.name))
    : matching.value,
)
const { values, text, change, act, image } = usePageSettings('PageInstanceSaves', {})
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div style="display: grid">
          <div v-if="saves.length" class="saves-list">
            <PSearchBox v-model="query" :placeholder="text('Instance.Saves.SearchHint')" />
            <PCard :title="text('Instance.Saves.QuickActions')"
              ><div class="saves-actions">
                <PButton :click="() => act('BtnOpenFolder_Click')">{{
                  text('Instance.Saves.OpenFolder')
                }}</PButton
                ><PButton :click="() => act('BtnDownloadNew_Click')">{{
                  text('Instance.Resource.DownloadNew')
                }}</PButton
                ><PButton :click="() => act('BtnPaste_Click')">{{
                  text('Instance.Saves.PasteFile')
                }}</PButton>
              </div></PCard
            >
            <PCard :title="text('Instance.Saves.ListTitle')"
              ><div class="saves-actions">
                <label>{{ text('Instance.Saves.SortBy') }}</label
                ><PComboBox
                  v-model="sort"
                  :options="[
                    { key: 'name', text: pclText('Proto.Label.Name') },
                    { key: 'default', text: text('Common.Option.Default') },
                  ]"
                />
              </div>
              <div class="save-items">
                <PListItem
                  v-for="save in savesSorted"
                  :key="save.id"
                  :title="save.name"
                  :subtitle="save.description"
                  :icon="save.icon"
                  :click="
                    () => router.push({ path: '/instance_save/info', query: { id: save.id } })
                  "
                /></div
            ></PCard>
          </div>
          <PCard
            v-if="!saves.length"
            data-control="PanNoWorld"
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
                grid-template-columns: minmax(0, 1fr) auto auto auto minmax(0, 1fr);
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
                >{{ text('Instance.Saves.Empty.Title') }}</span
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
                style="
                  margin: 15px 10px 5px 10px;
                  grid-row: 3 / span 1;
                  grid-column: 1 / span 4;
                  justify-self: center;
                "
                class="pcl-text"
                >{{ text('Instance.Saves.Empty.Message') }}</span
              >
              <PButton
                style="
                  height: 35px;
                  min-width: 140px;
                  margin: 10px 10px 0px 10px;
                  padding: 0px 13px;
                  grid-row: 4 / span 1;
                  grid-column: 2 / span 1;
                  justify-self: center;
                "
                :click="() => act('BtnOpenFolder_Click', 'BtnOpenFolder_Click', '', '')"
                type="tint"
                >{{ text('Instance.Saves.OpenFolder') }}</PButton
              >
              <PButton
                style="
                  height: 35px;
                  min-width: 140px;
                  margin: 10px 10px 0px 10px;
                  padding: 0px 13px;
                  grid-row: 4 / span 1;
                  grid-column: 3 / span 1;
                  justify-self: center;
                "
                :click="() => act('BtnDownloadNew_Click', 'BtnDownloadNew_Click', '', '')"
                >{{ text('Instance.Resource.DownloadNew') }}</PButton
              >
              <PButton
                style="
                  height: 35px;
                  min-width: 140px;
                  margin: 10px 10px 0px 10px;
                  padding: 0px 13px;
                  grid-row: 4 / span 1;
                  grid-column: 4 / span 1;
                  justify-self: center;
                "
                :click="() => act('BtnPaste_Click', 'BtnPaste_Click', '', '')"
                >{{ text('Instance.Saves.PasteFile') }}</PButton
              >
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.saves-list {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.saves-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 15px;
  padding: 0 25px 15px;
}
.save-items {
  padding: 0 20px 20px;
}
</style>
