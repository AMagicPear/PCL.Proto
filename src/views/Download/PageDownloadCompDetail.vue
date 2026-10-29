<script setup lang="ts">
import modrinthApi, { type IProject, type IVersion } from '@/api/modrinthApi'
import PCard from '@/components/widget/PCard.vue'
import PCompItem from '@/components/widget/PCompItem.vue'
import PListItem from '@/components/widget/PListItem.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PButton from '@/components/widget/PButton.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useRoute } from 'vue-router'
import { computed, ref, watch } from 'vue'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
import { useCompFavorites } from '@/stores/compFavorites'
const favorites = useCompFavorites()

const route = useRoute()
const text = pclText
const project = ref<IProject>()
const versions = ref<IVersion[]>([])
const loading = ref(false)
const error = ref('')
const versionFilter = ref('')
const loaderFilter = ref('')
const item = computed(
  () =>
    project.value && {
      ...project.value,
      date_modified: project.value.updated,
      project_id: project.value.id,
      versions: gameVersions.value,
    },
)
const gameVersions = computed(() => [
  ...new Set(versions.value.flatMap((version) => version.game_versions)),
])
const loaders = computed(() => [...new Set(versions.value.flatMap((version) => version.loaders))])
const filtered = computed(() =>
  versions.value.filter(
    (version) =>
      (!versionFilter.value || version.game_versions.includes(versionFilter.value)) &&
      (!loaderFilter.value || version.loaders.includes(loaderFilter.value)),
  ),
)
const projectUrl = computed(() =>
  project.value ? `https://modrinth.com/${project.value.project_type}/${project.value.slug}` : '',
)
function safeUrl(value: string) {
  return /^https:\/\//i.test(value) ? value : undefined
}
let request = 0
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
    versionFilter.value = route.query.version === 'all' ? '' : String(route.query.version || '')
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
      <PCard hide-title>
        <div class="filters">
          <PComboBox
            v-model="versionFilter"
            :options="[
              { key: '', text: text('Common.Option.All') },
              ...gameVersions.map((value) => ({ key: value, text: value })),
            ]"
            :aria-label="pclText('Proto.Label.GameVersion')"
          />
          <PComboBox
            v-model="loaderFilter"
            :options="[
              { key: '', text: text('Common.Option.All') },
              ...loaders.map((value) => ({ key: value, text: value })),
            ]"
            :aria-label="pclText('Download.Comp.Filter.Loader')"
          />
        </div>
      </PCard>
      <PCard v-for="version in filtered" :key="version.id" :title="version.name">
        <div class="version-files">
          <PListItem
            v-for="file in version.files"
            :key="file.url"
            :title="file.filename"
            :subtitle="`${version.game_versions.join(', ')} · ${version.loaders.join(', ')} · ${new Date(version.date_published).toLocaleString()}`"
          >
            <template #actions
              ><a
                v-if="safeUrl(file.url)"
                :href="safeUrl(file.url)"
                target="_blank"
                rel="noopener noreferrer"
                :download="file.filename"
                class="file-download"
                ><PSvgIcon name="lucide/download" />{{ pclText('Common.Action.Download') }}</a
              ></template
            >
          </PListItem>
        </div>
      </PCard>
      <p v-if="!filtered.length" class="pcl-empty">{{ pclText('Proto.Empty.FilteredVersions') }}</p>
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
  gap: 12px;
  flex-wrap: wrap;
  padding: 15px;
}
.filters > * {
  flex: 1;
  min-width: 150px;
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
