<script setup lang="ts">
import PButton from '@/components/widget/PButton.vue'
import PageLoginProfileSkin from './PageLoginProfileSkin.vue'
import PageLoginProfile from './PageLoginProfile.vue'
import PageLoginOffline from './PageLoginOffline.vue'
import PageLoginMs from './PageLoginMs.vue'
import PageLoginAuth from './PageLoginAuth.vue'
import { useSelectedInstance } from '@/stores/gameLaunch'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
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
const route = useRoute()
const router = useRouter()
const instance = useSelectedInstance()
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
const accountView = computed(() =>
  route.path.startsWith('/account')
    ? { offline: PageLoginOffline, microsoft: PageLoginMs, auth: PageLoginAuth }[
        String(route.params.login)
      ] || PageLoginProfile
    : PageLoginProfileSkin,
)
async function launch() {
  if (!instance.instance_info) {
    await router.push('/download/game')
    return
  }
  try {
    await invoke('launch_game')
  } catch (err) {
    sideTip.show(`启动游戏失败：${String(err)}`, 'warn')
  }
}
function settings() {
  if (instance.instance_info) void router.push('/instance_setting')
  else sideTip.show('请先选择一个游戏实例。')
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
      <PButton :click="() => sideTip.show('取消启动尚未接入 PCL.Proto。')">{{
        text('Common.Action.Cancel')
      }}</PButton>
    </div>
    <template v-else
      ><div class="login-panel"><component :is="accountView" /></div>
      <div class="launch-actions">
        <PButton class="launch" type="tint" :click="launch" :disabled="loadingInstances"
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
        ><PButton :click="() => router.push('/instance_select')">{{
          text('Launch.Home.SelectInstance')
        }}</PButton
        ><PButton :click="settings">{{ text('Launch.Home.InstanceSettings') }}</PButton>
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
