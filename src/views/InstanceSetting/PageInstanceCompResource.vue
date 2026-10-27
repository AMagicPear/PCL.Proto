<script setup lang="ts">
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import { computed, ref } from 'vue'
import PLocalModItem from '@/components/widget/PLocalModItem.vue'
import PSearchBox from '@/components/widget/PSearchBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useRoute } from 'vue-router'
const route = useRoute()
const props = withDefaults(
  defineProps<{
    loading?: boolean
    schematicAvailable?: boolean
    error?: string
    items?: {
      id: string
      title: string
      description?: string
      icon?: string
      enabled?: boolean
      updatable?: boolean
      error?: string
      duplicate?: boolean
    }[]
  }>(),
  { loading: false, error: '', items: () => [], schematicAvailable: false },
)
const query = ref('')
const filter = ref('All')
const selected = ref(new Set<string>())
const sorted = ref(false)
const visibleItems = computed(() => {
  const items = props.items.filter(
    (item) =>
      `${item.title} ${item.description || ''}`.toLowerCase().includes(query.value.toLowerCase()) &&
      (filter.value === 'All' ||
        (filter.value === 'Enabled' && item.enabled !== false) ||
        (filter.value === 'Disabled' && item.enabled === false) ||
        (filter.value === 'Updatable' && item.updatable) ||
        (filter.value === 'Error' && item.error) ||
        (filter.value === 'Duplicate' && item.duplicate)),
  )
  return sorted.value ? [...items].sort((a, b) => a.title.localeCompare(b.title)) : items
})
function select(id: string, value: boolean) {
  if (value) selected.value.add(id)
  else selected.value.delete(id)
}
const { values, text, change, act, image } = usePageSettings('PageInstanceCompResource', {})
const selectionActions = [
  { id: 'Update', key: 'Instance.Resource.Update', icon: 'upload' },
  { id: 'Enable', key: 'Instance.Resource.Enable', icon: 'circle-check' },
  { id: 'Disable', key: 'Instance.Resource.Disable', icon: 'circle-minus' },
  { id: 'Favorites', key: 'Instance.Resource.Favorite', icon: 'heart' },
  { id: 'Share', key: 'Instance.Resource.ShareSelected', icon: 'share-2' },
  { id: 'Delete', key: 'Common.Action.Delete', icon: 'trash-2' },
]
</script>
<template>
  <div class="pcl-page">
    <div>
      <PCard
        v-if="route.meta.resourceType === 'Schematic' && !schematicAvailable"
        hide-title
        class="pcl-unavailable"
        ><h2>{{ text('Instance.Resource.Schematic.Unavailable.Title') }}</h2>
        <div class="pcl-divider" />
        <p>{{ text('Instance.Resource.Schematic.Unavailable.Message') }}</p>
        <div class="resource-actions">
          <PButton type="tint" :click="() => act('BtnHintDownload')">{{
            text('Instance.Resource.Schematic.DownloadMod')
          }}</PButton
          ><PButton :click="() => act('BtnSchematicVersionSelect')">{{
            text('Instance.Resource.Mod.Disabled.SelectInstance')
          }}</PButton>
        </div></PCard
      >
      <div v-else style="display: grid">
        <div
          data-control="PanAllBack"
          style="grid-row: 1 / span 1; grid-column: 1 / span 1; display: grid"
        >
          <div data-control="PanBack" style="grid-row: 1 / span 1; grid-column: 1 / span 1">
            <div v-if="items.length && !loading && !error" class="resource-main">
              <PSearchBox
                :placeholder="text('Instance.Resource.Search.Hint')"
                @search="(value) => (query = value || '')"
              />
              <PCard hide-title
                ><div class="resource-actions">
                  <PButton
                    v-for="action in [
                      { id: 'BtnManageOpen', key: 'Common.Action.OpenFolder' },
                      { id: 'BtnManageInstall', key: 'Instance.Resource.InstallFromFiles' },
                      { id: 'BtnManageDownload', key: 'Instance.Resource.DownloadNew' },
                      { id: 'BtnManageInfoExport', key: 'Instance.Resource.ExportInfo' },
                      { id: 'BtnManageCheck', key: 'Instance.Resource.CheckMods' },
                    ]"
                    :key="action.id"
                    :click="() => act(action.id)"
                    >{{ text(action.key) }}</PButton
                  ><PButton :click="() => visibleItems.forEach((item) => selected.add(item.id))">{{
                    text('Instance.Resource.SelectAll')
                  }}</PButton
                  ><RouterLink
                    to="/instance_setting/mod/jarinjar"
                    :title="text('Instance.Resource.Mod.JarInJar.Entry.ToolTip')"
                    >{{ text('Instance.Resource.Mod.JarInJar.Entry') }}</RouterLink
                  >
                </div></PCard
              >
              <PCard hide-title
                ><div class="resource-actions">
                  <PComboBox
                    v-model="filter"
                    :options="
                      ['All', 'Enabled', 'Disabled', 'Updatable', 'Error', 'Duplicate'].map(
                        (value) => ({
                          key: value,
                          text: text(`Instance.Resource.Filter.${value}`),
                        }),
                      )
                    "
                  /><PButton inline :click="() => (sorted = !sorted)">{{
                    text('Instance.Resource.Sort.Text')
                  }}</PButton>
                </div>
                <div class="resource-list">
                  <PLocalModItem
                    v-for="item in visibleItems"
                    :key="item.id"
                    v-bind="item"
                    :selected="selected.has(item.id)"
                    @select="(value) => select(item.id, value)"
                    @toggle="act('BtnToggle')"
                    @update="act('BtnUpdate')"
                  />
                  <p v-if="!visibleItems.length" class="pcl-empty">没有符合条件的结果</p>
                </div></PCard
              >
            </div>
          </div>
          <PCard v-if="loading" hide-title class="resource-status"
            ><PLoading :card="false" :loading-text="text('Instance.Resource.Loading')"
          /></PCard>
          <PHint v-else-if="error" severity="error" class="resource-status"
            >{{ error
            }}<PButton inline :click="() => act('BtnRefresh')">{{
              text('Common.Action.Refresh')
            }}</PButton></PHint
          >
          <PCard
            v-if="!loading && !error && !items.length"
            data-control="PanEmpty"
            style="
              margin: 40px;
              grid-row: 1 / span 1;
              grid-column: 1 / span 1;
              justify-self: center;
              align-self: center;
            "
            hide-title
          >
            <div style="margin: 17px 20px; display: flex; flex-direction: column">
              <span
                data-control="TxtEmptyTitle"
                style="margin: 0px 0px 9px 0px; font-size: 19px; color: var(--color-tint)"
                class="pcl-text"
                >{{ text('Instance.Resource.Empty.Title') }}</span
              >
              <div style="height: 2px" class="pcl-divider" />
              <span
                data-control="TxtEmptyDescription"
                style="margin: 10px 10px 0px 10px"
                class="pcl-text"
                >{{ text('Instance.Resource.Empty.Description') }}</span
              >
              <div
                style="
                  margin: 20px 10px 5px 10px;
                  display: flex;
                  flex-direction: row;
                  align-items: center;
                  flex-wrap: wrap;
                "
              >
                <PButton
                  data-control="BtnHintInstall"
                  style="height: 35px; min-width: 130px; margin: 0px 5px; padding: 0px 12px"
                  :click="() => act('BtnHintInstall', '', '', '')"
                  type="tint"
                  >{{ text('Instance.Resource.InstallFromFiles') }}</PButton
                >
                <PButton
                  data-control="BtnHintDownload"
                  style="height: 35px; min-width: 130px; margin: 0px 5px; padding: 0px 12px"
                  :click="() => act('BtnHintDownload', '', '', '')"
                  >{{ text('Instance.Resource.DownloadNew') }}</PButton
                >
                <PButton
                  data-control="BtnHintOpen"
                  style="height: 35px; min-width: 130px; margin: 0px 5px; padding: 0px 12px"
                  :click="() => act('BtnHintOpen', '', '', '')"
                  >{{ text('Common.Action.OpenFolder') }}</PButton
                >
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
    <PCard v-if="selected.size" hide-title class="pcl-selection-toolbar"
      ><p>{{ text('Instance.Resource.SelectedCount').replace('{0}', String(selected.size)) }}</p>
      <div>
        <PButton
          v-for="action in selectionActions"
          :key="action.id"
          inline
          :click="() => act(`BtnSelect${action.id}`)"
          ><PSvgIcon :name="`lucide/${action.icon}`" :size="18" />{{ text(action.key) }}</PButton
        ><PButton inline :click="() => selected.clear()"
          ><PSvgIcon name="lucide/x" :size="18" />{{
            text('Instance.Resource.CancelSelection')
          }}</PButton
        >
      </div></PCard
    >
  </div>
</template>

<style scoped>
.resource-main {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.resource-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 15px;
  padding: 15px;
}
.resource-list {
  padding: 0 20px 20px;
}
.resource-status {
  margin: 40px;
  grid-area: 1 / 1;
}
</style>
