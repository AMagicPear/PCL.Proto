<script setup lang="ts">
import PButton from '@/components/widget/PButton.vue'
import PLoading from '@/components/widget/PLoading.vue'
import PageLoginProfileSkin from './PageLoginProfileSkin.vue'
import PageLoginProfile from './PageLoginProfile.vue'
import PageLoginOffline from './PageLoginOffline.vue'
import PageLoginMs from './PageLoginMs.vue'
import PageLoginAuth from './PageLoginAuth.vue'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { useRouter } from 'vue-router'
import type { ProfileView } from '@/types/profile'
import { Channel, invoke } from '@tauri-apps/api/core'
import { prepareGameJava, downloadGameJava, type JavaInstallProgress } from '@/api/javaReq'
import { useModal } from '@/composables/useModal'
import { computed, onUnmounted, ref } from 'vue'
import { pclText } from '@/api/pclLocale'
import type { DownloadStats } from '@/api/downloadSettings'
import sideTip from '@/composables/sideTip'

defineProps<{ loadingInstances?: boolean }>()

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

type Launching = {
  name: string
  progress: number
  stage: string
  method: string
  downloading: boolean
}
type LaunchProgress = Pick<Launching, 'progress' | 'stage' | 'downloading'>

const launching = ref<Launching | null>(null)
const downloadSpeed = ref<string | null>(null)
let speedTimer: ReturnType<typeof setTimeout> | undefined
const leavingLaunch = ref(false)
let launchLeaveTimer: ReturnType<typeof setTimeout> | undefined

async function getLaunchMethod() {
  try {
    const account = await invoke<Record<string, Record<string, string>> | null>('get_account')
    if (!account) return text('Launch.Account.Type.Offline')
    if (account.Microsoft) return text('Launch.Account.Type.Microsoft')
    const thirdParty = account.Yggdrasil || account.AuthlibInjector
    return thirdParty
      ? `${text('Launch.Account.Type.ThirdParty')}${thirdParty.server_name ? ` / ${thirdParty.server_name}` : ''}`
      : text('Launch.Account.Type.Offline')
  } catch {
    return text('Launch.Account.Type.Offline')
  }
}
function updateLaunching(update: LaunchProgress) {
  if (launching.value) launching.value = { ...launching.value, ...update }
}
function formatSpeed(speed: number) {
  if (speed < 1024) return `${speed.toFixed(0)} B/s`
  if (speed < 1048576) return `${(speed / 1024).toFixed(1)} KiB/s`
  return `${(speed / 1048576).toFixed(1)} MiB/s`
}
async function refreshSpeed() {
  if (!launching.value?.downloading) {
    downloadSpeed.value = null
    return
  }
  try {
    const stats = await invoke<DownloadStats>('get_download_stats')
    if (launching.value?.downloading) downloadSpeed.value = formatSpeed(stats.speed)
  } catch {
    downloadSpeed.value = null
  } finally {
    if (launching.value?.downloading) speedTimer = setTimeout(refreshSpeed, 300)
  }
}
function startLaunching() {
  clearTimeout(speedTimer)
  downloadSpeed.value = null
  launching.value = {
    name: instance.instance_info?.name || '',
    progress: 0,
    stage: 'Common.Action.Initialize',
    method: text('Launch.Account.Type.Offline'),
    downloading: false,
  }
  void getLaunchMethod().then((method) => {
    if (launching.value) launching.value = { ...launching.value, method }
  })
}
function stopLaunching() {
  clearTimeout(launchLeaveTimer)
  clearTimeout(speedTimer)
  downloadSpeed.value = null
  leavingLaunch.value = false
  launching.value = null
}
function finishLaunching() {
  if (!launching.value || leavingLaunch.value) return
  leavingLaunch.value = true
  clearTimeout(speedTimer)
  launchLeaveTimer = setTimeout(stopLaunching, 360)
}
async function launch() {
  if (preparing.value) return
  if (!instance.instance_info) {
    await router.push('/download/game')
    return
  }
  const instanceDirectory = instance.instance_info.directory
  preparing.value = true
  javaCancelled.value = false
  startLaunching()
  try {
    updateLaunching({ stage: 'Minecraft.Launch.Stage.GetJava', progress: 0.05, downloading: false })
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
        stopLaunching()
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
      if (!answer.confirmed || instance.instance_info?.directory !== instanceDirectory) {
        stopLaunching()
        return
      }
      javaProgress.value = { completed: 0, total: 0 }
      await downloadGameJava(instanceDirectory, (progress) => {
        javaProgress.value = {
          completed: Math.max(javaProgress.value?.completed || 0, progress.completed),
          total: progress.total,
        }
        updateLaunching({
          stage: 'Minecraft.Launch.Stage.GetJava',
          progress: progress.total ? 0.05 + (progress.completed / progress.total) * 0.15 : 0.05,
          downloading: true,
        })
        if (!speedTimer) void refreshSpeed()
      })
      if (javaCancelled.value) {
        stopLaunching()
        return
      }
      javaProgress.value = null
      info = await prepareGameJava(instanceDirectory)
      if (!info.runtime) throw new Error(text('Proto.Java.ReadFailed'))
    }
    for (const warning of info.warnings)
      sideTip.show(text(warning.key, [warning.version, warning.min, warning.max]), 'warn')
    const progress = new Channel<LaunchProgress>()
    progress.onmessage = (message) => {
      updateLaunching(message)
      if (message.downloading && !speedTimer) void refreshSpeed()
      if (message.progress === 1) finishLaunching()
    }
    await invoke('launch_game', { instanceDirectory, progress })
  } catch (err) {
    if (!javaCancelled.value) sideTip.show(text('Proto.Error.Launch', [String(err)]), 'warn')
    stopLaunching()
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

onUnmounted(() => {
  clearTimeout(speedTimer)
  clearTimeout(launchLeaveTimer)
})
</script>

<template>
  <aside
    class="launch-left"
    :class="{ 'is-launching': launching && !leavingLaunch, 'is-returning': leavingLaunch }"
  >
    <div class="launch-input">
      <div class="login-panel"><component :is="accountView" @view="profileView = $event" /></div>
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
      </div>
    </div>
    <section v-if="launching" class="launch-status" aria-live="polite">
      <PLoading class="launching-loader" :card="false" />
      <h2>
        {{
          text(
            launching.progress === 1
              ? 'Launch.Status.Title.Launched'
              : 'Launch.Status.Title.Launching',
          )
        }}
      </h2>
      <p class="launch-name">{{ launching.name }}</p>
      <div class="launch-progress" :aria-label="text('Launch.Status.LaunchProgress')">
        <span :style="{ width: `${launching.progress * 100}%` }" />
      </div>
      <dl>
        <dt>{{ text('Launch.Status.CurrentStep') }}</dt>
        <dd>{{ text(launching.stage) }}</dd>
        <dt>{{ text('Launch.Status.LoginMethod') }}</dt>
        <dd>{{ launching.method }}</dd>
        <dt v-if="launching.progress < 1">{{ text('Launch.Status.LaunchProgress') }}</dt>
        <dd v-if="launching.progress < 1">{{ `${(launching.progress * 100).toFixed(2)}%` }}</dd>
        <dt v-if="launching.downloading">{{ text('Launch.Status.DownloadSpeed') }}</dt>
        <dd v-if="launching.downloading">{{ downloadSpeed || '0 B/s' }}</dd>
      </dl>
      <div v-if="launching.progress === 1" class="launch-trivia">
        <span>{{ text('Launch.Status.Trivia') }}</span>
        <p>{{ text('Launch.Status.Trivia.Placeholder') }}</p>
      </div>
      <PButton v-if="javaProgress" @click="cancelJava">{{ text('Common.Action.Cancel') }}</PButton>
    </section>
  </aside>
</template>

<style scoped>
.launch-left {
  position: relative;
  width: 300px;
  flex: 0 0 300px;
  padding: 0 20px 20px;
  overflow: hidden;
}
.launch-input {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  opacity: 1;
  transform: scale(1);
  transition:
    opacity 110ms ease-in 50ms,
    transform 160ms ease-in;
}
.is-launching .launch-input {
  pointer-events: none;
  opacity: 0;
  transform: scale(1.2);
}
.is-returning .launch-input {
  transition:
    opacity 250ms ease-out 50ms,
    transform 300ms cubic-bezier(0.34, 1.56, 0.64, 1) 50ms;
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
  position: absolute;
  inset: 0 20px 20px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 0;
  opacity: 0;
  transform: scale(0.8);
  animation: launch-status-enter 500ms 100ms cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
  text-align: center;
}
.is-returning .launch-status {
  pointer-events: none;
  animation: launch-status-leave 150ms ease-in forwards;
}
.launching-loader :deep(.loading-container) {
  min-height: 0;
  padding: 4px 0 2px;
}
.launching-loader :deep(.loading-text) {
  display: none;
}
.launching-loader :deep(.load-svg) {
  width: 72px;
  height: 72px;
}
.launch-status h2 {
  margin: 10px 15px 0;
  color: var(--color-tint);
  font-size: 20px;
  font-weight: normal;
  transform: skewX(-3deg);
}
.launch-name {
  margin: 5px 40px 0;
  overflow: hidden;
  color: var(--color-tint);
  font-size: 13.5px;
  text-overflow: ellipsis;
  white-space: nowrap;
  transform: skewX(-3deg);
}
.launch-progress {
  height: 4px;
  margin: 12px 10px 27px;
  overflow: hidden;
  background: color-mix(in srgb, var(--color-text-grey) 40%, transparent);
}
.launch-progress > span {
  display: block;
  height: 100%;
  background: linear-gradient(90deg, var(--color-tint-light), var(--color-tint));
  transition: width 180ms ease-out;
}
.launch-status dl {
  display: grid;
  grid-template-columns: auto auto;
  justify-content: center;
  gap: 5px 15px;
  margin: 0;
  font-size: 12.5px;
  text-align: left;
}
.launch-status dt {
  color: var(--color-text-grey);
  text-align: right;
  opacity: 0.8;
}
.launch-status dd {
  margin: 0;
}
.launch-trivia {
  width: 260px;
  box-sizing: border-box;
  margin: 16px auto 2px;
  padding: 0 11px 10px;
  border: 1px solid color-mix(in srgb, var(--color-text-grey) 50%, transparent);
  border-radius: 3px;
  font-size: 12.5px;
}
.launch-trivia > span {
  display: inline-block;
  padding: 0 8px;
  color: var(--color-text-grey);
  background: var(--color-background);
  transform: translateY(-9px);
}
.launch-trivia p {
  margin: -3px 0 0;
  line-height: 1.45;
}
@keyframes launch-status-enter {
  to {
    opacity: 1;
    transform: scale(1);
  }
}
@keyframes launch-status-leave {
  to {
    opacity: 0;
    transform: scale(0.8);
  }
}
</style>
