<script setup lang="ts">
import { computed, onMounted, reactive, ref, shallowRef, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke, isTauri } from '@tauri-apps/api/core'
import PCard from '@/components/widget/PCard.vue'
import PVirtualList from '@/components/widget/PVirtualList.vue'
import PListItem from '@/components/widget/PListItem.vue'
import PVersionActions from '@/components/widget/PVersionActions.vue'
import PButton from '@/components/widget/PButton.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { getMinecraftVersions, type IVersionShow } from '@/api/gameVersions'
import { pclText } from '@/api/pclLocale'
import { useTaskManager } from '@/stores/task'
import sideTip from '@/composables/sideTip'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { showIconPath } from '@/types/gameInfo'
import type { McPluginReport } from '@/types/mcPlugin'
import { installWarnings, loaderVisible, selectLoader } from '@/api/installSelection'
import { saveMinecraftVersion } from '@/api/versionActions'

const props = defineProps<{ manual?: boolean; modify?: boolean }>()
const route = useRoute()
const router = useRouter()
const text = pclText
const versions = shallowRef<IVersionShow>()
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
    error.value = pclText('Proto.Error.Versions', [String(err)])
  } finally {
    loading.value = false
  }
}
function choose(id: string, type: string) {
  if (props.manual) {
    void saveMinecraftVersion(id)
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
      ['Forge', 'Fabric', 'LegacyFabric'].map(async (name) => {
        plugins[name] = { loading: true, versions: [], error: '' }
        try {
          if (!isTauri()) throw new Error(pclText('Proto.Desktop.Loaders'))
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
              error: pclText('Proto.Error.Versions', [String(err)]),
            }
        }
      }),
    )
  },
  { immediate: true },
)
async function install() {
  if (props.modify) {
    sideTip.show(pclText('Proto.NotImplemented.ModifyCore'))
    return
  }
  if (
    !instanceName.value.trim() ||
    /[\\/:*?"<>|\u0000-\u001f]/.test(instanceName.value) ||
    ['.', '..'].includes(instanceName.value.trim())
  ) {
    sideTip.show(pclText('Proto.Instance.InvalidName'), 'warn')
    return
  }
  if (!isTauri()) {
    sideTip.show(pclText('Proto.Desktop.Download'), 'warn')
    return
  }
  if (Object.entries(selected).some(([name, value]) => !['Fabric', 'LegacyFabric'].includes(name) && !!value)) {
    sideTip.show(pclText('Proto.NotImplemented.LoaderInstall'), 'warn')
    return
  }
  installing.value = true
  try {
    void useTaskManager()
      .startDownloadMCVersion(versionId.value, instanceName.value.trim(), selected.Fabric
        ? { family: 'fabric', version: selected.Fabric }
        : selected.LegacyFabric ? { family: 'legacyfabric', version: selected.LegacyFabric } : undefined)
      .catch((err) => sideTip.show(pclText('Proto.Error.Download', [String(err)]), 'warn'))
    await router.push('/downloading')
  } catch (err) {
    sideTip.show(pclText('Proto.Error.Download', [String(err)]), 'warn')
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
    <p v-if="modify && !versionId" class="pcl-empty">{{ pclText('Proto.Instance.SelectFirst') }}</p>
    <template v-else-if="!versionId">
      <PLoading v-if="loading" :loading-text="text('Download.Version.LoadingList')" />
      <PHint v-else-if="error" severity="error"
        >{{ error }}
        <PButton inline @click="load">{{
          pclText('Launch.Homepage.LoadFailed.Retry')
        }}</PButton></PHint
      >
      <template v-else-if="versions">
        <PCard :title="text('Download.Version.Latest.Title')" class="latest-card">
          <div class="version-list">
            <PListItem
              v-for="version in latest"
              :key="version.id"
              :title="version.id"
              :icon="version.type === 'release' ? showIconPath.grass : showIconPath.command"
              :subtitle="text(`Download.Version.Latest.${version.label}`, [version.releaseTime])"
              clickable
              @click="() => choose(version.id, version.type)"
              ><template #actions
                ><PVersionActions :version-id="version.id" :manual="manual" /></template
            ></PListItem>
          </div>
        </PCard>
        <template v-for="category in categories" :key="category.key">
          <PCard
            v-if="versions[category.key].length"
            :title="`${text(`Download.Version.Type.${category.label}`)} (${versions[category.key].length})`"
            default-fold-status="fold"
          >
            <div class="version-list">
              <PVirtualList
                :items="versions[category.key]"
                :item-key="(version) => version.id"
                v-slot="{ item: version }"
              >
                <PListItem
                  :key="version.id"
                  :title="version.id"
                  :icon="category.icon"
                  :subtitle="version.releaseTime"
                  clickable
                  @click="() => choose(version.id, category.key)"
                  ><template #actions
                    ><PVersionActions :version-id="version.id" :manual="manual" /></template
                ></PListItem>
              </PVirtualList>
            </div>
          </PCard>
        </template>
      </template>
    </template>
    <template v-else>
      <PCard hide-title>
        <div class="instance-name-row">
          <button
            type="button"
            class="install-back"
            @click="() => router.push('/download/game')"
            :aria-label="pclText('Common.Action.Back')"
          >
            <PSvgIcon name="lucide/arrow-left" :size="16" />
          </button>
          <img :src="showIconPath.grass" alt="" />
          <PTextBox v-model="instanceName" :aria-label="pclText('Proto.Label.InstanceName')" />
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
            selected[loader] ||
            (plugins[loader]?.loading
              ? pclText('Proto.Download.Loading')
              : pclText('Proto.Download.SelectVersion'))
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
              @click="() => selectLoader(selected, loader, '')"
              >{{ pclText('Proto.Action.ClearSelection') }}</PButton
            >
            <PVirtualList
              :items="plugins[loader]!.versions"
              :item-key="(version) => version.version"
              v-slot="{ item: version }"
            >
              <PListItem
                :key="version.version"
                :title="version.version"
                :subtitle="version.stable ? text('Download.Version.Type.Stable') : undefined"
                clickable
                @click="() => selectLoader(selected, loader, version.version)"
              />
            </PVirtualList>
          </template>
          <p v-else class="pcl-empty">
            {{
              ['Forge', 'Fabric', 'LegacyFabric'].includes(loader)
                ? pclText('Download.Install.State.NoAvailableVersion')
                : pclText('Proto.NotImplemented.Loader')
            }}
          </p>
        </div>
      </PCard>
      <PButton type="tint" class="install-start" :disabled="installing" @click="install"
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
.install-back {
  flex: 0 0 26px;
  width: 26px;
  height: 26px;
  padding: 5px;
  display: grid;
  place-items: center;
  border: 0;
  border-radius: 50%;
  background: transparent;
  color: rgba(0, 0, 0, 0.62745);
  opacity: 0.6;
  cursor: pointer;
  transition: color 150ms;
}
.install-back:hover,
.install-back:focus-visible {
  color: rgba(0, 0, 0, 0.90196);
  transition-duration: 120ms;
}
.install-back:focus-visible {
  outline: 2px solid var(--color-tint);
  outline-offset: 2px;
}
.install-back :deep(.pcl-icon) {
  transition: transform 250ms;
}
.install-back:active :deep(.pcl-icon) {
  transform: scale(0.8);
}
@media (prefers-color-scheme: dark) {
  .install-back {
    color: rgba(255, 255, 255, 0.62745);
  }
  .install-back:hover,
  .install-back:focus-visible {
    color: rgba(255, 255, 255, 0.90196);
  }
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
