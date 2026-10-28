<script setup lang="ts">
import modrinthApi, { type ISearchHit } from '@/api/modrinthApi'
import PCompItem from '@/components/widget/PCompItem.vue'
import PCard from '@/components/widget/PCard.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PButton from '@/components/widget/PButton.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import PSearchBox from '@/components/widget/PSearchBox.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PHint from '@/components/widget/PHint.vue'
import PLoading from '@/components/widget/PLoading.vue'
import categories from '@/assets/download-categories.json'
import { computed, reactive, ref, watch } from 'vue'
import { pclText } from '@/api/pclLocale'

const props = defineProps<{
  projectType: 'mod' | 'modpack' | 'resourcepack' | 'shader' | 'datapack' | 'world'
}>()
const text = pclText
const values = reactive({ source: '0', sort: '0', loader: '', version: '', query: '', tag: '' })
const hits = ref<ISearchHit[]>([])
const error = ref('')
const loading = ref(false)
const page = ref(0)
const total = ref(0)
let request = 0
const limit = 20
const tagOptions = computed(() =>
  categories[props.projectType].map((item) => ({ key: item.tag, text: text(item.text) })),
)
const loaderOptions = computed(() =>
  (props.projectType === 'shader'
    ? ['', 'iris', 'optifine', 'canvas', 'vanilla']
    : ['', 'forge', 'neoforge', 'fabric', 'quilt']
  ).map((value) => ({ key: value, text: value || text('Common.Option.All') })),
)
async function search() {
  const current = ++request
  error.value = ''
  loading.value = true
  try {
    if (values.source === '1') throw new Error(pclText('Proto.Resource.CurseForgeUnavailable'))
    if (props.projectType === 'world') throw new Error(pclText('Proto.Resource.WorldUnavailable'))
    const facets = [
      [`project_type:${props.projectType === 'datapack' ? 'mod' : props.projectType}`],
    ]
    if (props.projectType === 'datapack') facets.push(['categories:datapack'])
    if (values.loader) facets.push([`categories:${values.loader}`])
    if (values.version.trim()) facets.push([`versions:${values.version.trim()}`])
    if (values.tag) {
      const slug = values.tag.split('/')[1]
      if (!slug) throw new Error(pclText('Proto.Resource.CategoryUnavailable'))
      facets.push([`categories:${slug}`])
    }
    const sort =
      (['relevance', 'downloads', 'follows', 'newest', 'updated'] as const)[Number(values.sort)] ||
      'relevance'
    const data = await modrinthApi.searchProjects({
      query: values.query,
      facets,
      index: sort,
      offset: page.value * limit,
      limit,
    })
    if (current !== request) return
    hits.value = data.hits
    total.value = data.total_hits
  } catch (err) {
    if (current !== request) return
    hits.value = []
    total.value = 0
    error.value =
      err instanceof Error ? err.message : pclText('Proto.Error.Resources', [String(err)])
  } finally {
    if (current === request) loading.value = false
  }
}
function reset() {
  Object.assign(values, { source: '0', sort: '0', loader: '', version: '', query: '', tag: '' })
  page.value = 0
  void search()
}
function firstSearch() {
  page.value = 0
  void search()
}
function turn(next: number) {
  page.value = Math.max(0, next)
  void search()
}
watch(() => props.projectType, reset, { immediate: true })
</script>
<template>
  <section class="pcl-page comp-page">
    <PSearchBox
      v-model="values.query"
      :placeholder="text('Download.Comp.Search.Hint')"
      @search="firstSearch"
    />
    <div class="filters">
      <label>{{ text('Download.Comp.Filter.Source') }}</label>
      <PComboBox
        v-model="values.source"
        :options="[
          { key: '0', text: 'Modrinth' },
          { key: '1', text: 'CurseForge' },
        ]"
        @update:model-value="firstSearch"
      />
      <label>{{ text('Download.Comp.Filter.Tag') }}</label>
      <PComboBox v-model="values.tag" :options="tagOptions" @update:model-value="firstSearch" />
      <label>{{ text('Download.Comp.Filter.Sort') }}</label>
      <PComboBox
        v-model="values.sort"
        :options="
          ['Relevance', 'Downloads', 'Follows', 'Latest', 'Updated'].map((item, index) => ({
            key: String(index),
            text: text(`Download.Comp.Sort.${item}`),
          }))
        "
        @update:model-value="firstSearch"
      />
      <label>{{ text('Download.Comp.Filter.Version') }}</label>
      <PTextBox
        v-model="values.version"
        :placeholder="text('Common.Option.All')"
        @change="firstSearch"
        @keydown.enter="firstSearch"
      />
      <template v-if="['mod', 'modpack', 'shader'].includes(projectType)">
        <label>{{ text('Download.Comp.Filter.Loader') }}</label>
        <PComboBox
          v-model="values.loader"
          :options="loaderOptions"
          @update:model-value="firstSearch"
        />
      </template>
      <PButton inline @click="reset" :title="text('Common.Action.Reset')"
        ><PSvgIcon name="lucide/rotate-ccw"
      /></PButton>
    </div>
    <PLoading v-if="loading" :loading-text="text('Download.Version.LoadingList')" />
    <PHint v-else-if="error" severity="error"
      >{{ error }}
      <PButton inline @click="search">{{
        pclText('Launch.Homepage.LoadFailed.Retry')
      }}</PButton></PHint
    >
    <template v-else>
      <PCard hide-title
        ><div class="projects">
          <PCompItem v-for="project in hits" :key="project.project_id" :data="project" clickable />
          <p v-if="!hits.length" class="pcl-empty">{{ pclText('Proto.Empty.Resources') }}</p>
        </div></PCard
      >
      <PCard hide-title
        ><nav class="pages" :aria-label="pclText('Proto.Label.Pagination')">
          <PButton
            inline
            :disabled="!page"
            @click="() => turn(0)"
            :aria-label="pclText('Proto.Action.First')"
            ><PSvgIcon name="lucide/chevrons-left"
          /></PButton>
          <PButton
            inline
            :disabled="!page"
            @click="() => turn(page - 1)"
            :aria-label="pclText('Proto.Action.Previous')"
            ><PSvgIcon name="lucide/chevron-left"
          /></PButton>
          <span>{{ page + 1 }}</span>
          <PButton
            inline
            :disabled="(page + 1) * limit >= total"
            @click="() => turn(page + 1)"
            :aria-label="pclText('Proto.Action.Next')"
            ><PSvgIcon name="lucide/chevron-right"
          /></PButton></nav
      ></PCard>
    </template>
  </section>
</template>
<style scoped>
.comp-page {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.filters {
  display: grid;
  grid-template-columns: auto minmax(80px, 1fr) auto minmax(80px, 1fr);
  align-items: center;
  gap: 10px 12px;
}
.projects {
  padding: 12px 14px;
}
.pages {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15px;
  padding: 10px;
}
@media (max-width: 700px) {
  .filters {
    grid-template-columns: auto minmax(80px, 1fr);
  }
}
</style>
