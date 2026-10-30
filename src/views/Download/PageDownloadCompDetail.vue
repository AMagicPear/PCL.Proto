<script setup lang="ts">
import modrinthApi, { type IProject, type IVersion } from '@/api/modrinthApi'
import PCard from '@/components/widget/PCard.vue'
import PVirtualList from '@/components/widget/PVirtualList.vue'
import {
  compVersionFilters,
  groupCompVersions,
  groupedVersionName,
  loaderNames,
} from '@/api/compResources'
import PCompItem from '@/components/widget/PCompItem.vue'
import PListItem from '@/components/widget/PListItem.vue'
import PButton from '@/components/widget/PButton.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useRoute } from 'vue-router'
import { computed, onScopeDispose, ref, shallowRef, watch } from 'vue'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
import { useCompFavorites } from '@/stores/compFavorites'
const favorites = useCompFavorites()

const route = useRoute()
const text = pclText
const project = shallowRef<IProject>()
const versions = shallowRef<IVersion[]>([])
const loading = ref(false)
const error = ref('')
const versionFilter = ref('')
const requestedVersion = ref('')
const loaderFilter = ref('')
const item = computed(
  () =>
    project.value && {
      ...project.value,
      date_modified: project.value.updated,
      project_id: project.value.id,
      versions: [...new Set(versions.value.flatMap((version) => version.game_versions))],
    },
)
// 筛选选项按 CE 规则逐级合并（1.21.1 → 1.21 → 远古版）
const versionFilters = computed(() =>
  compVersionFilters(versions.value.flatMap((version) => version.game_versions)),
)
const loaders = computed(() => [...new Set(versions.value.flatMap((version) => version.loaders))])
const isMod = computed(() => project.value?.project_type === 'mod')
const groups = computed(() =>
  groupCompVersions(
    versions.value,
    versionFilter.value,
    loaderFilter.value,
    isMod.value && loaders.value.length > 1,
    versionFilters.value.byDrop,
    versionFilters.value.foldOld,
  ),
)
const projectUrl = computed(() =>
  project.value ? `https://modrinth.com/${project.value.project_type}/${project.value.slug}` : '',
)
function safeUrl(value: string) {
  return /^https:\/\//i.test(value) ? value : undefined
}
let request = 0
onScopeDispose(() => {
  request++
})
async function load() {
  const current = ++request
  loading.value = true
  error.value = ''
  project.value = undefined
  versions.value = []
  try {
    const id = String(route.query.id || '')
    if (!/^[a-zA-Z0-9_-]+$/.test(id)) throw new Error(pclText('Proto.Resource.InvalidId'))
    const [data, files] = await Promise.all([
      modrinthApi.getProject(id),
      modrinthApi.getProjectVersions(id),
    ])
    if (current !== request) return
    project.value = data
    versions.value = files
    // 路由带入的是实例的具体版本号，需要归到筛选选项上
    if (requestedVersion.value)
      versionFilter.value = groupedVersionName(
        requestedVersion.value,
        versionFilters.value.byDrop,
        versionFilters.value.foldOld,
      )
  } catch (err) {
    if (current === request) error.value = pclText('Proto.Error.Resource', [String(err)])
  } finally {
    if (current === request) loading.value = false
  }
}
async function copy(value: string) {
  try {
    await navigator.clipboard.writeText(value)
    sideTip.show(pclText('Common.Hint.Copied'), 'success')
  } catch {
    sideTip.show(pclText('Proto.Clipboard.Unavailable'), 'warn')
  }
}
function openUrl(url: string) {
  if (safeUrl(url)) window.open(url, '_blank', 'noopener,noreferrer')
}
function openProject() {
  if (projectUrl.value) window.open(projectUrl.value, '_blank', 'noopener,noreferrer')
}
watch(
  () => route.query.id,
  () => {
    requestedVersion.value = route.query.version === 'all' ? '' : String(route.query.version || '')
    versionFilter.value = requestedVersion.value
    loaderFilter.value = String(route.query.loader || '')
    void load()
  },
  { immediate: true },
)
</script>
<template>
  <article class="subview pcl-page detail-page">
    <PLoading v-if="loading" :loading-text="text('Download.Version.LoadingList')" />
    <PHint v-else-if="error" severity="error"
      >{{ error
      }}<PButton inline @click="load">{{
        pclText('Launch.Homepage.LoadFailed.Retry')
      }}</PButton></PHint
    >
    <template v-else-if="project && item">
      <PCard hide-title>
        <div class="intro">
          <PCompItem :data="item" />
          <div class="actions">
            <PButton inline @click="openProject"><PSvgIcon name="lucide/globe" />Modrinth</PButton>
            <PButton
              inline
              @click="
                () => openUrl(`https://search.mcmod.cn/s?key=${encodeURIComponent(project!.title)}`)
              "
              >{{ text('Download.Comp.Detail.McMod') }}</PButton
            >
            <PButton
              inline
              @click="
                () =>
                  openUrl(
                    `https://translate.google.com/?sl=auto&amp;tl=zh-CN&amp;text=${encodeURIComponent(project!.description)}`,
                  )
              "
              >{{ text('Download.Comp.Detail.TranslateDescription') }}</PButton
            >
            <PButton inline @click="() => copy(project!.title)">{{
              text('Download.Comp.Detail.CopyName')
            }}</PButton>
            <PButton inline @click="() => favorites.toggle(item!)"
              ><PSvgIcon
                :name="favorites.has(project.id) ? 'lucide/heart-filled' : 'lucide/heart'"
              />{{
                text(
                  favorites.has(project.id)
                    ? 'Download.Comp.Detail.Unfavorite'
                    : 'Download.Comp.Detail.Favorite',
                )
              }}</PButton
            >
            <PButton inline @click="() => copy(projectUrl)">{{
              text('Download.Comp.Detail.CopyLink')
            }}</PButton>
          </div>
        </div>
      </PCard>
      <PCard v-if="versionFilters.list.length >= 2" hide-title>
        <div class="filters">
          <div class="filter-row">
            <span v-if="isMod" class="filter-label">{{
              text('Download.Comp.Detail.InstanceFilter')
            }}</span>
            <button
              type="button"
              class="pcl-chip"
              :class="{ active: !versionFilter }"
              @click="versionFilter = ''"
            >
              {{ text('Common.Option.All') }}
            </button>
            <button
              v-for="value in versionFilters.list"
              :key="value"
              type="button"
              class="pcl-chip"
              :class="{ active: versionFilter === value }"
              @click="versionFilter = value"
            >
              {{ value }}
            </button>
          </div>
          <div v-if="isMod" class="filter-row">
            <span class="filter-label">{{ text('Download.Comp.Detail.LoaderFilter') }}</span>
            <button
              type="button"
              class="pcl-chip"
              :class="{ active: !loaderFilter }"
              @click="loaderFilter = ''"
            >
              {{ text('Common.Option.All') }}
            </button>
            <button
              v-for="value in loaders"
              :key="value"
              type="button"
              class="pcl-chip"
              :class="{ active: loaderFilter === value }"
              @click="loaderFilter = value"
            >
              {{ loaderNames[value] || value }}
            </button>
          </div>
        </div>
      </PCard>
      <PCard
        v-for="group in groups"
        :key="`${versionFilter}:${loaderFilter}:${group.key}`"
        :title="group.name"
        :default-fold-status="groups.length === 1 ? 'unfold' : 'fold'"
      >
        <div class="version-files">
          <PVirtualList
            :items="
              group.versions.flatMap((version) => version.files.map((file) => ({ version, file })))
            "
            :item-key="(row) => `${row.version.id}:${row.file.url}`"
            v-slot="{ item: { version, file } }"
          >
            <PListItem
              :title="version.name"
              :subtitle="`${file.filename} · ${version.loaders.join(', ')} · ${new Date(version.date_published).toLocaleString()}`"
            >
              <template #actions>
                <a
                  v-if="safeUrl(file.url)"
                  :href="safeUrl(file.url)"
                  target="_blank"
                  rel="noopener noreferrer"
                  :download="file.filename"
                  class="file-download"
                >
                  <PSvgIcon name="lucide/download" />{{ pclText('Common.Action.Download') }}
                </a>
              </template>
            </PListItem>
          </PVirtualList>
        </div>
      </PCard>
      <p v-if="!groups.length" class="pcl-empty">{{ pclText('Proto.Empty.FilteredVersions') }}</p>
    </template>
  </article>
</template>
<style scoped>
.detail-page {
  padding: 25px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 15px;
  height: 100%;
}
.intro {
  padding: 15px 22px;
}
.actions {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  padding-top: 10px;
}
.actions :deep(button) {
  display: flex;
  align-items: center;
  gap: 6px;
}
.filters {
  display: flex;
  flex-direction: column;
  padding: 5px 0;
}
.filter-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  margin: 5px 10px;
}
.filter-label {
  margin: 0 2px 0 2px;
  color: var(--color-text-black);
}
.version-files {
  padding: 0 20px 15px;
}
.file-download {
  display: flex;
  gap: 6px;
  align-items: center;
  color: var(--color-tint);
}
</style>
