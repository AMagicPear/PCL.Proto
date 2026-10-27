<script setup lang="ts">
import PListItem from '@/components/widget/PListItem.vue'
import PCard from '@/components/widget/PCard.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PSearchBox from '@/components/widget/PSearchBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PButton from '@/components/widget/PButton.vue'
import sideTip from '@/composables/sideTip'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { useRepositoriesStore } from '@/stores/repositories'
import type GameInstance from '@/types/gameInstance'
import { showIconPath } from '@/types/gameInfo'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'

const route = useRoute()
const router = useRouter()
const repos = useRepositoriesStore()
const selected = useSelectedInstance()
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
const instances = ref<GameInstance[]>([])
const loading = ref(false)
const error = ref('')
const query = ref('')
const filtered = computed(() =>
  instances.value.filter((item) =>
    `${item.name} ${item.version} ${Object.keys(item.pluginsVersion || {}).join(' ')}`
      .toLowerCase()
      .includes(query.value.toLowerCase()),
  ),
)
let request = 0
async function load() {
  const current = ++request
  loading.value = true
  error.value = ''
  instances.value = []
  try {
    if (!isTauri()) return
    if (!repos.repositires) await repos.fetchFromBackend()
    const index = Number(route.params.repository || 0)
    if (!Number.isInteger(index) || index < 0 || !repos.repositires?.[index]) return
    const items = await repos.getInstancesInRepository(index)
    if (current === request) instances.value = items
  } catch (err) {
    if (current === request) error.value = `无法读取游戏实例：${String(err)}`
  } finally {
    if (current === request) loading.value = false
  }
}
async function select(instance: GameInstance) {
  try {
    await invoke('select_instance', {
      repository_index: Number(route.params.repository || 0),
      instance_id: instance.id,
    })
    selected.$patch({ instance_info: instance })
    sideTip.show(`已选择 ${instance.name}`, 'success')
    await router.push('/home')
  } catch (err) {
    sideTip.show(`选择实例失败：${String(err)}`, 'warn')
  }
}
watch(
  () => route.params.repository,
  () => {
    query.value = ''
    void load()
  },
  { immediate: true },
)
</script>
<template>
  <section class="pcl-page select-page">
    <PSearchBox v-model="query" :placeholder="text('Select.Instance.Search.Hint')" />
    <PCard v-if="loading" hide-title class="empty-card"
      ><PLoading :card="false" :loading-text="text('Select.Instance.Loading')"
    /></PCard>
    <PHint v-else-if="error" severity="error"
      >{{ error }}<PButton inline :click="load">重试</PButton></PHint
    >
    <PCard v-else-if="!instances.length" hide-title class="empty-card"
      ><div class="empty-content">
        <h2>{{ text('Select.Instance.Empty.Title') }}</h2>
        <hr />
        <p>{{ text('Select.Instance.Empty.Message') }}</p>
        <PButton type="tint" :click="() => router.push('/download/game')">{{
          text('Select.Instance.Empty.Download')
        }}</PButton>
      </div></PCard
    >
    <PCard v-else-if="!filtered.length" hide-title class="empty-card"
      ><div class="empty-content">
        <h2>{{ text('Select.Instance.EmptySearch.Title') }}</h2>
        <hr />
        <p>{{ text('Select.Instance.Search.NoResult').replace('{0}', query) }}</p>
      </div></PCard
    >
    <PCard v-else :title="`${text('Launch.Home.SelectInstance')} (${filtered.length})`"
      ><div class="instance-list">
        <PListItem
          v-for="item in filtered"
          :key="item.id"
          :title="item.name"
          :subtitle="item.version"
          :icon="showIconPath.grass"
          :click="() => select(item)"
        /></div
    ></PCard>
  </section>
</template>
<style scoped>
.select-page {
  padding: 15px 25px 25px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.empty-card {
  margin: 25px 15px;
}
.empty-content {
  padding: 17px 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.empty-content h2 {
  font-size: 18px;
  font-weight: normal;
}
.empty-content p {
  padding: 0 10px;
  white-space: pre-line;
}
.empty-content hr {
  border: 0;
  height: 2px;
  background: var(--color-tint-light);
}
.instance-list {
  padding: 0 20px 15px;
}
</style>
