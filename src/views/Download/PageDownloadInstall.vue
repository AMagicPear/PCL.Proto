<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke, isTauri } from '@tauri-apps/api/core'
import PCard from '@/components/widget/PCard.vue'
import PListItem from '@/components/widget/PListItem.vue'
import PButton from '@/components/widget/PButton.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { getMinecraftVersions, type IVersionShow } from '@/api/gameVersions'
import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'
import { useTaskManager } from '@/stores/task'
import sideTip from '@/composables/sideTip'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { showIconPath } from '@/types/gameInfo'
import type { McPluginReport } from '@/types/mcPlugin'
import { installWarnings, loaderVisible, selectLoader } from '@/api/installSelection'

const props = defineProps<{ manual?: boolean; modify?: boolean }>()
const route = useRoute()
const router = useRouter()
const { locale } = useI18n()
const text = (key: string) => pclText(`{DynamicResource ${key}}`, locale.value)
const versions = ref<IVersionShow>()
const loading = ref(false)
const error = ref('')
const installing = ref(false)
const instance = useSelectedInstance()
const versionId = computed(() =>
  props.modify ? instance.instance_info?.version || '' : String(route.query.version_id || ''),
)
const instanceName = ref(versionId.value)
const plugins = reactive<
  Record<string, { loading: boolean; versions: McPluginReport[]; error: string }>
>({})
const selected = reactive<Record<string, string>>({})
const warnings = computed(() => installWarnings(selected, versionId.value))
const loaders = [
  'Forge',
  'Cleanroom',
  'NeoForge',
  'Fabric',
  'LegacyFabric',
  'FabricApi',
  'LegacyFabricApi',
  'LabyMod',
  'OptiFine',
  'OptiFabric',
  'LiteLoader',
]
const visibleLoaders = computed(() =>
  loaders.filter((loader) => loaderVisible(loader, versionId.value, selected)),
)
const categories = [
  { key: 'release', label: 'Release', icon: showIconPath.grass },
  { key: 'snapshot', label: 'Development', icon: showIconPath.command },
  { key: 'old', label: 'BeforeRelease', icon: showIconPath.stone },
  { key: 'fool', label: 'AprilFools', icon: showIconPath.gold },
] as const
const latest = computed(() => {
  const data = versions.value
  if (!data) return []
  const list = data.latest.release
    ? [{ ...data.latest.release, type: 'release', label: 'Release' }]
    : []
  if (
    data.latest.snapshot &&
    (!data.latest.release || data.latest.snapshot.timestamp > data.latest.release.timestamp)
  )
    list.push({ ...data.latest.snapshot, type: 'snapshot', label: 'Development' })
  return list
})
async function load() {
  loading.value = true
  error.value = ''
  try {
    versions.value = await getMinecraftVersions()
  } catch (err) {
    error.value = `无法获取版本列表：${String(err)}`
  } finally {
    loading.value = false
  }
}
function choose(id: string, type: string) {
  if (props.manual) {
    sideTip.show('Minecraft 安装包另存为尚未接入 PCL.Proto。')
    return
  }
  void router.push({ path: '/download/game/inner', query: { version_id: id, version_type: type } })
}
let request = 0
watch(
  versionId,
  async (id) => {
    instanceName.value = id
    Object.keys(selected).forEach((key) => delete selected[key])
    const current = ++request
    if (!id) return
    await Promise.all(
      ['Forge', 'Fabric'].map(async (name) => {
        plugins[name] = { loading: true, versions: [], error: '' }
        try {
          if (!isTauri()) throw new Error('请在桌面客户端中查询加载器版本。')
          const list = await invoke<McPluginReport[]>('get_plugin_versions', {
            plugin_type: name.toLowerCase(),
            mc_version: id,
          })
          if (current === request) plugins[name] = { loading: false, versions: list, error: '' }
        } catch (err) {
          if (current === request)
            plugins[name] = {
              loading: false,
              versions: [],
              error: `无法获取版本列表：${String(err)}`,
            }
        }
      }),
    )
  },
  { immediate: true },
)
async function install() {
  if (props.modify) {
    sideTip.show('实例核心修改尚未接入 PCL.Proto。')
    return
  }
  if (
    !instanceName.value.trim() ||
    /[\\/:*?"<>|\u0000-\u001f]/.test(instanceName.value) ||
    ['.', '..'].includes(instanceName.value.trim())
  ) {
    sideTip.show('请输入有效的实例名称。', 'warn')
    return
  }
  if (!isTauri()) {
    sideTip.show('请在桌面客户端中下载游戏。', 'warn')
    return
  }
  if (Object.values(selected).some(Boolean)) {
    sideTip.show('加载器安装尚未接入 PCL.Proto，请先下载原版游戏。', 'warn')
    return
  }
  installing.value = true
  try {
    void useTaskManager()
      .startDownloadMCVersion(versionId.value, instanceName.value.trim())
      .catch((err) => sideTip.show(`下载失败：${String(err)}`, 'warn'))
    await router.push('/downloading')
  } catch (err) {
    sideTip.show(`下载失败：${String(err)}`, 'warn')
  } finally {
    installing.value = false
  }
}
onMounted(() => {
  if (!versionId.value) void load()
})
watch(versionId, (id) => {
  if (!id && !versions.value) void load()
})
</script>
<template>
  <section class="pcl-page install-page">
    <p v-if="modify && !versionId" class="pcl-empty">请先选择一个游戏实例</p>
    <template v-else-if="!versionId">
      <PCard v-if="loading" hide-title
        ><PLoading :card="false" :loading-text="text('Download.Version.LoadingList')"
      /></PCard>
      <PHint v-else-if="error" severity="error"
        >{{ error }} <PButton inline :click="load">重试</PButton></PHint
      >
      <template v-else-if="versions">
        <PCard :title="text('Download.Version.Latest.Title')" class="latest-card">
          <div class="version-list">
            <PListItem
              v-for="version in latest"
              :key="version.id"
              :title="version.id"
              :icon="version.type === 'release' ? showIconPath.grass : showIconPath.command"
              :subtitle="
                text(`Download.Version.Latest.${version.label}`).replace('{0}', version.releaseTime)
              "
              :click="() => choose(version.id, version.type)"
            />
          </div>
        </PCard>
        <template v-for="category in categories" :key="category.key">
          <PCard
            v-if="versions[category.key].length"
            :title="`${text(`Download.Version.Type.${category.label}`)} (${versions[category.key].length})`"
            default-fold-status="fold"
          >
            <div class="version-list">
              <PListItem
                v-for="version in versions[category.key]"
                :key="version.id"
                :title="version.id"
                :icon="category.icon"
                :subtitle="version.releaseTime"
                :click="() => choose(version.id, category.key)"
              />
            </div>
          </PCard>
        </template>
      </template>
    </template>
    <template v-else>
      <PCard hide-title>
        <div class="instance-name-row">
          <PButton inline :click="() => router.push('/download/game')" aria-label="返回"
            ><PSvgIcon name="lucide/arrow-left"
          /></PButton>
          <img :src="showIconPath.grass" alt="" />
          <PTextBox v-model="instanceName" aria-label="实例名称" />
          <span v-if="modify">{{ text('Instance.Install.Action.ModifyLabel') }}</span>
        </div>
        <div v-if="warnings.length" class="install-warnings">
          <PHint
            v-for="warning in warnings"
            :key="warning"
            :severity="warning.endsWith('Api') ? 'error' : 'warning'"
            >{{ text(`Download.Install.Warning.${warning}`) }}</PHint
          >
        </div>
      </PCard>
      <PCard
        v-for="loader in visibleLoaders"
        :key="`${versionId}-${loader}`"
        :title="text(`Common.Installation.${loader}`)"
        default-fold-status="fold"
      >
        <template #description
          ><span class="loader-status">{{
            selected[loader] || (plugins[loader]?.loading ? '加载中…' : '选择版本')
          }}</span></template
        >
        <div class="version-list">
          <PLoading v-if="plugins[loader]?.loading" :card="false" />
          <PHint v-else-if="plugins[loader]?.error" severity="error">{{
            plugins[loader]?.error
          }}</PHint>
          <template v-else-if="plugins[loader]?.versions.length">
            <PButton
              v-if="selected[loader]"
              inline
              :click="() => selectLoader(selected, loader, '')"
              >清除选择</PButton
            >
            <PListItem
              v-for="version in plugins[loader]?.versions"
              :key="version.version"
              :title="version.version"
              :subtitle="version.stable ? text('Download.Version.Type.Stable') : undefined"
              :click="() => selectLoader(selected, loader, version.version)"
            />
          </template>
          <p v-else class="pcl-empty">
            {{
              ['Forge', 'Fabric'].includes(loader) ? '没有可用版本' : '此加载器尚未接入 PCL.Proto。'
            }}
          </p>
        </div>
      </PCard>
      <PButton type="tint" class="install-start" :disabled="installing" :click="install"
        ><PSvgIcon name="lucide/download" />{{
          text(modify ? 'Instance.Install.Action.StartModify' : 'Download.Install.StartDownload')
        }}</PButton
      >
    </template>
  </section>
</template>
<style scoped>
.install-page {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.version-list {
  padding: 0 18px 12px 20px;
}
.instance-name-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 15px 16px 15px 13px;
}
.instance-name-row > img {
  width: 32px;
  height: 32px;
}
.instance-name-row > :last-child {
  flex: 1;
}
.loader-status {
  color: var(--color-text-grey);
  font-size: 12px;
}
.install-warnings {
  padding: 0 16px 15px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.install-start {
  align-self: center;
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
