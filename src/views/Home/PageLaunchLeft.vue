<script setup lang="ts">
import PButton from '@/components/widget/PButton.vue'
import PageLoginProfileSkin from './PageLoginProfileSkin.vue'
import PageLoginProfile from './PageLoginProfile.vue'
import PageLoginOffline from './PageLoginOffline.vue'
import PageLoginMs from './PageLoginMs.vue'
import PageLoginAuth from './PageLoginAuth.vue'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { useRouter } from 'vue-router'
import type { ProfileView } from '@/types/profile'
import { invoke } from '@tauri-apps/api/core'
import { prepareGameJava, downloadGameJava, type JavaInstallProgress } from '@/api/javaReq'
import { useModal } from '@/composables/useModal'
import PHint from '@/components/widget/PHint.vue'
import { computed, ref } from 'vue'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
defineProps<{
  loadingInstances?: boolean
  launching?: {
    name: string
    progress?: number
    step?: string
    method?: string
    speed?: string
    trivia?: string
  }
}>()
const router = useRouter()
const instance = useSelectedInstance()
const text = pclText
const profileView = ref<ProfileView>('skin')
const accountViews = {
  skin: PageLoginProfileSkin,
  profiles: PageLoginProfile,
  offline: PageLoginOffline,
  microsoft: PageLoginMs,
  auth: PageLoginAuth,
}
const accountView = computed(() => accountViews[profileView.value])
const modal = useModal()
const preparing = ref(false)
const javaCancelled = ref(false)
const javaProgress = ref<JavaInstallProgress | null>(null)
async function launch() {
  if (preparing.value) return
  if (!instance.instance_info) {
    await router.push('/download/game')
    return
  }
  const instanceDirectory = instance.instance_info.directory
  preparing.value = true
  javaCancelled.value = false
  try {
    let info = await prepareGameJava(instanceDirectory)
    if (!info.runtime) {
      if (!info.download_major) {
        await modal.open({
          title: text('Minecraft.Launch.Java.NotFound.Title'),
          content: text('Proto.Java.ManualRequired', [
            info.requirements.min[0],
            info.requirements.max[0],
          ]),
          plainText: true,
          buttons: [{ content: text('Common.Action.Close'), operation: () => modal.close() }],
        })
        return
      }
      const answer = await modal.open({
        title: text('Minecraft.Launch.Java.AutoDownload.Title'),
        content: text('Minecraft.Launch.Java.AutoDownload.Message', [
          `Java ${info.download_major}`,
        ]),
        plainText: true,
        buttons: [
          {
            content: text('Minecraft.Launch.Java.AutoDownload.Action'),
            operation: () => modal.close(true),
          },
          { content: text('Common.Action.Cancel'), operation: () => modal.close(false) },
        ],
      })
      if (!answer.confirmed || instance.instance_info?.directory !== instanceDirectory) return
      javaProgress.value = { completed: 0, total: 0 }
      await downloadGameJava(instanceDirectory, (progress) => {
        javaProgress.value = {
          completed: Math.max(javaProgress.value?.completed || 0, progress.completed),
          total: progress.total,
        }
      })
      if (javaCancelled.value) return
      javaProgress.value = null
      info = await prepareGameJava(instanceDirectory)
      if (!info.runtime) throw new Error(text('Proto.Java.ReadFailed'))
    }
    for (const warning of info.warnings)
      sideTip.show(text(warning.key, [warning.version, warning.min, warning.max]), 'warn')
    await invoke('launch_game', { instanceDirectory })
  } catch (err) {
    if (!javaCancelled.value) sideTip.show(text('Proto.Error.Launch', [String(err)]), 'warn')
  } finally {
    javaProgress.value = null
    preparing.value = false
  }
}
async function cancelJava() {
  javaCancelled.value = true
  await invoke('cancel_java_download')
}
function settings() {
  if (instance.instance_info) void router.push('/instance_setting')
  else sideTip.show(pclText('Proto.Instance.SelectFirstHint'))
}
</script>
<template>
  <aside class="launch-left">
    <div v-if="launching" class="launch-status">
      <h2>{{ text('Launch.Status.Title.Launching') }}</h2>
      <p>{{ launching.name }}</p>
      <progress
        :value="launching.progress"
        max="1"
        :aria-label="text('Launch.Status.LaunchProgress')"
      />
      <dl>
        <dt>{{ text('Launch.Status.CurrentStep') }}</dt>
        <dd>{{ launching.step || '—' }}</dd>
        <dt>{{ text('Launch.Status.LoginMethod') }}</dt>
        <dd>{{ launching.method || '—' }}</dd>
        <dt>{{ text('Launch.Status.LaunchProgress') }}</dt>
        <dd>
          {{ launching.progress == null ? '—' : `${Math.round(launching.progress * 100)} %` }}
        </dd>
        <dt>{{ text('Launch.Status.DownloadSpeed') }}</dt>
        <dd>{{ launching.speed || '—' }}</dd>
      </dl>
      <h3>{{ text('Launch.Status.Trivia') }}</h3>
      <p>{{ launching.trivia || text('Launch.Status.Trivia.Placeholder') }}</p>
      <PButton @click="() => sideTip.show(pclText('Proto.NotImplemented.CancelLaunch'))">{{
        text('Common.Action.Cancel')
      }}</PButton>
    </div>
    <template v-else
      ><div class="login-panel"><component :is="accountView" @view="profileView = $event" /></div>
      <PHint v-if="preparing" severity="info" aria-live="polite">
        {{
          javaProgress
            ? text('Proto.Java.DownloadProgress', [javaProgress.completed, javaProgress.total])
            : text('Proto.Java.Preparing')
        }}
        <PButton v-if="javaProgress" inline @click="cancelJava">{{
          text('Common.Action.Cancel')
        }}</PButton>
      </PHint>
      <div class="launch-actions">
        <PButton
          class="launch"
          type="tint"
          @click="launch"
          :disabled="loadingInstances || preparing"
          ><span>{{
            text(
              loadingInstances
                ? 'Launch.Home.Button.Loading'
                : instance.instance_info
                  ? 'Launch.Home.Button.Launch'
                  : 'Launch.Home.Button.Download',
            )
          }}</span
          ><small>{{
            loadingInstances
              ? text('Launch.Home.VersionList.Loading')
              : instance.instance_info?.name || text('Select.Instance.Empty.Title')
          }}</small></PButton
        ><PButton @click="() => router.push('/instance_select')">{{
          text('Launch.Home.SelectInstance')
        }}</PButton
        ><PButton @click="settings">{{ text('Launch.Home.InstanceSettings') }}</PButton>
      </div></template
    >
  </aside>
</template>
<style scoped>
.launch-left {
  width: 300px;
  flex: 0 0 300px;
  padding: 0 20px 20px;
  display: flex;
  flex-direction: column;
  animation: pcl-zoom-in 0.4s ease forwards;
}
.login-panel {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 0;
  overflow-y: auto;
  padding: 15px 0;
}
.login-panel:has(.profile-actions details[open]) {
  overflow: visible;
  z-index: 1;
}
.launch-actions {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}
.launch-actions > button {
  min-height: 35px;
}
.launch {
  grid-column: 1 / -1;
  height: 54px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.launch small {
  font-size: 11px;
  color: var(--color-text-grey);
}
.launch-status {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-top: 20px;
  text-align: center;
}
.launch-status h2 {
  font-size: 20px;
  color: var(--color-tint);
}
.launch-status progress {
  width: 100%;
  height: 4px;
}
.launch-status dl {
  display: grid;
  grid-template-columns: auto auto;
  gap: 5px 15px;
  text-align: left;
}
.launch-status dt {
  color: var(--color-text-grey);
  text-align: right;
}
.launch-status dd {
  margin: 0;
}
.launch-status h3 {
  font-size: 12.5px;
  color: var(--color-text-grey);
  font-weight: normal;
}
</style>
