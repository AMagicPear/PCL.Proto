<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { getMinecraftVersions } from '@/api/gameVersions'
import type { McPluginReport } from '@/types/mcPlugin'
import PCard from '@/components/widget/PCard.vue'
import PListItem from '@/components/widget/PListItem.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PButton from '@/components/widget/PButton.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PHint from '@/components/widget/PHint.vue'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
const props = defineProps<{ loader: string; website: string; supported?: boolean }>()
const localeName = computed(() => (props.loader === 'OptiFine' ? 'Optifine' : props.loader))
const text = pclText
const minecraft = ref('')
const minecraftOptions = ref<{ key: string; text: string }[]>([])
const versions = ref<McPluginReport[]>([])
const error = ref('')
const loading = ref(false)
let request = 0
async function load() {
  if (!props.supported || !minecraft.value) return
  const current = ++request
  loading.value = true
  error.value = ''
  try {
    if (!isTauri()) throw new Error(pclText('Proto.Desktop.Loaders'))
    const data = await invoke<McPluginReport[]>('get_plugin_versions', {
      plugin_type: props.loader.toLowerCase(),
      mc_version: minecraft.value,
    })
    if (current === request) versions.value = data
  } catch (err) {
    if (current === request) error.value = pclText('Proto.Error.Versions', [String(err)])
  } finally {
    if (current === request) loading.value = false
  }
}
function openWebsite() {
  window.open(props.website, '_blank', 'noopener,noreferrer')
}
async function initialize() {
  if (!props.supported) return
  loading.value = true
  try {
    const data = await getMinecraftVersions()
    minecraftOptions.value = [...data.release, ...data.snapshot, ...data.old].map((version) => ({
      key: version.id,
      text: version.id,
    }))
    minecraft.value = data.latest.release?.id || minecraftOptions.value[0]?.key || ''
    if (!minecraft.value) loading.value = false
  } catch (err) {
    error.value = pclText('Proto.Error.GameVersion', [String(err)])
    loading.value = false
  }
}
onMounted(initialize)
function retry() {
  return minecraft.value ? load() : initialize()
}
watch(minecraft, load)
</script>
<template>
  <section class="pcl-page loader-page">
    <PCard :title="text(`Download.Version.${localeName}.Intro.Title`)"
      ><div class="intro">
        <p>{{ text(`Download.Version.${localeName}.Intro.Description`) }}</p>
        <PButton type="tint" @click="openWebsite">{{
          text('Download.Version.OpenWebsite')
        }}</PButton>
      </div></PCard
    ><template v-if="supported"
      ><PComboBox
        v-if="minecraftOptions.length"
        v-model="minecraft"
        :options="minecraftOptions"
        :aria-label="pclText('Proto.Label.MinecraftVersion')"
      /><PCard v-if="loading" hide-title
        ><PLoading
          :card="false"
          :loading-text="text(`Download.Version.${localeName}.LoadingList`)" /></PCard
      ><PHint v-else-if="error" severity="error"
        >{{ error
        }}<PButton inline @click="retry">{{
          pclText('Launch.Homepage.LoadFailed.Retry')
        }}</PButton></PHint
      ><PCard v-else :title="minecraft || loader"
        ><div class="version-list">
          <PListItem
            v-for="version in versions"
            :key="version.version"
            :title="version.version"
            :subtitle="version.stable ? text('Download.Version.Type.Stable') : undefined"
            clickable
            @click="() => sideTip.show(pclText('Proto.NotImplemented.Installer'))"
          />
          <p v-if="!versions.length" class="pcl-empty">
            {{ pclText('Download.Install.State.NoAvailableVersion') }}
          </p>
        </div></PCard
      ></template
    ><PHint v-else severity="info">{{
      text('Proto.Download.InstallerUnavailable', [loader])
    }}</PHint>
  </section>
</template>
<style scoped>
.loader-page {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.intro {
  padding: 0 25px 20px;
}
.intro > p {
  white-space: pre-line;
  margin-bottom: 11px;
}
.version-list {
  padding: 0 20px 15px;
}
</style>
