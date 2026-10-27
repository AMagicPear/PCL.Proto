<script setup lang="ts">
import { computed, ref } from 'vue'
import PCard from '@/components/widget/PCard.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PSearchBox from '@/components/widget/PSearchBox.vue'
import PCompItem from '@/components/widget/PCompItem.vue'
import PButton from '@/components/widget/PButton.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import sideTip from '@/composables/sideTip'
import { usePageSettings } from '@/composables/usePageSettings'
import { useCompFavorites } from '@/stores/compFavorites'
import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'
const favorites = useCompFavorites()
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
const query = ref('')
const folder = ref('default')
const { act } = usePageSettings('PageDownloadCompFavorites')
const selected = ref(new Set<string>())
const selectedItems = computed(() =>
  favorites.items.filter((item) => selected.value.has(item.project_id)),
)
function select(id: string, checked: boolean) {
  if (checked) selected.value.add(id)
  else selected.value.delete(id)
}
async function share(ids: string[]) {
  if (!ids.length) {
    sideTip.show(text('Download.Comp.Favorites.Hint.NothingShared'))
    return
  }
  try {
    await navigator.clipboard.writeText(JSON.stringify(ids))
    sideTip.show('已成功复制！', 'success')
  } catch {
    sideTip.show('复制失败，请检查剪贴板权限。', 'warn')
  }
}
function unfavoriteSelected() {
  selectedItems.value.forEach((item) => favorites.toggle(item))
  selected.value.clear()
}
const items = computed(() =>
  favorites.items.filter((item) =>
    `${item.title} ${item.description}`.toLowerCase().includes(query.value.toLowerCase()),
  ),
)
</script>
<template>
  <section class="pcl-page favorites-page">
    <div class="search-row">
      <PSearchBox
        v-model="query"
        :placeholder="text('Download.Comp.Favorites.Search.Hint')"
      /><PCard hide-title class="folder-card"
        ><div class="folder-row">
          <PComboBox
            v-model="folder"
            :options="[
              { key: 'default', text: text('Download.Comp.Detail.Favorites.DefaultName') },
            ]"
          />
          <details class="manage-menu">
            <summary
              :aria-label="text('Download.Comp.Favorites.Manage')"
              :title="text('Download.Comp.Favorites.Manage')"
            >
              <PSvgIcon name="lucide/settings" :size="18" />
            </summary>
            <div class="menu-actions">
              <PButton
                v-for="action in ['New', 'Rename', 'Delete', 'Import']"
                :key="action"
                inline
                :click="() => act(`BtnFavorites${action}`)"
                >{{ text(`Download.Comp.Favorites.Menu.${action}`) }}</PButton
              >
              <PButton
                inline
                :click="() => share(favorites.items.map((item) => item.project_id))"
                >{{ text('Download.Comp.Favorites.Menu.Share') }}</PButton
              >
            </div>
          </details>
        </div></PCard
      >
    </div>
    <PCard
      v-if="items.length"
      :title="
        text('Download.Comp.Favorites.CardTitle')
          .replace('{0}', text('Download.Comp.Detail.Favorites.DefaultName'))
          .replace('{1}', String(items.length))
      "
      ><div class="favorite-items">
        <div v-for="item in items" :key="item.project_id" class="favorite-row">
          <PCheckBox
            :id="`favorite-${item.project_id}`"
            :model-value="selected.has(item.project_id)"
            :label="`选择 ${item.title}`"
            class="select-favorite"
            @update:model-value="(checked) => select(item.project_id, checked)"
          />
          <PCompItem :data="item" clickable /><PButton
            inline
            :click="() => favorites.toggle(item)"
            :title="text('Download.Comp.Detail.Unfavorite')"
            ><PSvgIcon name="lucide/heart-filled"
          /></PButton>
        </div></div></PCard
    ><PCard v-else hide-title class="empty-card"
      ><div class="empty-content">
        <h2>{{ text('Download.Comp.Favorites.Empty.Title') }}</h2>
        <hr />
        <p>{{ query ? '没有找到相关资源' : text('Download.Comp.Favorites.Empty.Description') }}</p>
      </div></PCard
    >
    <PCard v-if="selectedItems.length" hide-title class="selection-card"
      ><p>
        {{
          text('Download.Comp.Favorites.Hint.SelectedCount').replace(
            '{0}',
            String(selectedItems.length),
          )
        }}
      </p>
      <div class="selection-actions">
        <PButton inline :click="unfavoriteSelected"
          ><PSvgIcon name="lucide/heart-filled" />{{
            text('Download.Comp.Favorites.Cancel')
          }}</PButton
        >
        <PButton inline :click="() => share(selectedItems.map((item) => item.project_id))"
          ><PSvgIcon name="lucide/share-2" />{{ text('Download.Comp.Favorites.Share') }}</PButton
        >
        <PButton inline :click="() => act('BtnFavoritesDownload')"
          ><PSvgIcon name="lucide/download" />{{
            text('Download.Comp.Favorites.Download')
          }}</PButton
        >
        <PButton inline :click="() => selected.clear()"
          ><PSvgIcon name="lucide/x" />{{ text('Download.Comp.Favorites.Deselect') }}</PButton
        >
      </div></PCard
    >
  </section>
</template>
<style scoped>
.favorites-page {
  padding: 25px;
}
.search-row {
  display: flex;
  gap: 15px;
  margin-bottom: 15px;
}
.search-row > :first-child {
  flex: 1;
}
.folder-row {
  padding: 8px;
  display: flex;
  align-items: center;
  gap: 8px;
}
.manage-menu summary {
  cursor: pointer;
  list-style: none;
}
.folder-card,
.folder-card :deep(.mycard-content) {
  overflow: visible;
}
.manage-menu {
  position: relative;
}
.menu-actions {
  position: absolute;
  right: 0;
  top: 100%;
  z-index: 2;
  min-width: 160px;
  background: var(--color-background);
  box-shadow: var(--box-shadow);
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px;
}
.selection-card {
  position: sticky;
  bottom: 15px;
  margin: 15px auto 0;
  width: fit-content;
  max-width: 100%;
}
.selection-card p {
  padding: 9px;
  text-align: center;
}
.selection-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  padding: 0 5px 5px;
}
.select-favorite :deep(label) {
  font-size: 0;
}
.favorite-items {
  padding: 0 20px 15px;
}
.favorite-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.favorite-row > .comp-item {
  flex: 1;
  min-width: 0;
}
.empty-card {
  margin: 40px 15px;
}
.empty-content {
  padding: 17px 20px;
}
.empty-content h2 {
  font-size: 19px;
  font-weight: normal;
  color: var(--color-tint);
}
.empty-content hr {
  margin: 9px 0 15px;
  border: 0;
  height: 2px;
  background: var(--color-tint-light);
}
.empty-content p {
  padding: 0 10px;
}
</style>
