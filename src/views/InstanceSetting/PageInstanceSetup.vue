<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { getJavaList, type IJavaRuntimeInfo } from '@/api/javaReq'
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import { useModal } from '@/composables/useModal'
const props = defineProps<{
  loginLocked?: boolean
  memoryOverLimit?: boolean
  canCreateProfile?: boolean
}>()
const { values, text, change, act, image } = usePageSettings('PageInstanceSetup', {
  ComboArgumentIndieV2: '0',
  TextArgumentTitle: '',
  CheckArgumentTitleEmpty: false,
  TextArgumentInfo: '',
  ComboArgumentJava: '0',
  VersionRamType: '2',
  SliderRamCustom: 13.0,
  SliderRamInitialCustom: 0.0,
  ComboServerLoginRequire: '0',
  TextServerAuthServer: '',
  TextServerAuthRegister: '',
  TextServerAuthName: '',
  CheckAdvanceRunWait: false,
  TextServerEnter: '',
  ComboAdvanceRenderer: '0',
  TextAdvanceJvm: '',
  TextAdvanceGame: '',
  TextAdvanceClasspathHead: '',
  TextAdvanceRun: '',
  CheckAdvanceDisableModUpdate: false,
  CheckAdvanceJava: false,
  CheckAdvanceAssetsV2: false,
  CheckAdvanceUseProxyV2: false,
  CheckAdvanceDisableJLW: false,
  CheckAdvanceDisableLF: false,
  CheckUseDebugLog4j2Config: false,
  CheckAdvanceDisableLwjglUnsafeAgent: false,
})
const javaList = ref<IJavaRuntimeInfo[]>([])
const javaLoading = ref(true)
const javaError = ref(false)
let disposed = false
onUnmounted(() => {
  disposed = true
})
onMounted(async () => {
  try {
    const list = await getJavaList()
    if (!disposed) javaList.value = list
  } catch {
    if (!disposed) javaError.value = true
  } finally {
    if (!disposed) javaLoading.value = false
  }
})
const javaOptions = computed(() => [
  { key: '0', text: text('Instance.Setup.Options.Java.AutoSelect') },
  ...javaList.value.map((java) => ({
    key: java.java_exe,
    text: `${java.version} · ${java.architecture} · ${java.directory_path}`,
  })),
])
const authVisible = computed(() => ['2', '3'].includes(String(values.ComboServerLoginRequire)))
const modal = useModal()
async function setLittleSkin() {
  if (props.loginLocked) return
  const server = 'https://littleskin.cn/api/yggdrasil'
  if (values.TextServerAuthServer && values.TextServerAuthServer !== server) {
    const result = await modal.open({
      title: text('Instance.Setup.Server.LittleSkin.Override.Title'),
      content: text('Instance.Setup.Server.LittleSkin.Override.Message'),
      plainText: true,
      buttons: [
        {
          content: text('Instance.Setup.Server.LittleSkin.Override.Continue'),
          operation: () => modal.close(true),
        },
        { content: text('Common.Action.Cancel'), operation: () => modal.close(false) },
      ],
    })
    if (!result.confirmed) return
  }
  values.TextServerAuthServer = server
  values.TextServerAuthRegister = 'https://littleskin.cn/auth/register'
  values.TextServerAuthName = text('Instance.Setup.Server.LittleSkin.Name')
}
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 25px 25px 25px 25px; display: flex; flex-direction: column"
        >
          <PHint style="margin: 0px 0px 15px 0px" severity="info" closable>{{
            text('Instance.Setup.Hint')
          }}</PHint>
          <PCard
            data-control="CardArgument"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Launch.Options.Title')"
          >
            <div style="margin: 0px 25px 21px 25px; display: flex; flex-direction: column">
              <div
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr) 60px;
                  grid-template-rows: 28px 9px 28px 9px auto auto 28px 9px 28px;
                "
              >
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Instance.Setup.Options.InstanceIsolation') }}</span
                >
                <PComboBox
                  data-control="ComboArgumentIndieV2"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :options="[
                    {
                      key: '0',
                      text: text('Instance.Setup.Options.InstanceIsolation.Enabled'),
                      title: text('Instance.Setup.Options.InstanceIsolation.Enabled.ToolTip'),
                    },
                    {
                      key: '1',
                      text: text('Common.Action.Close'),
                      title: text('Instance.Setup.Options.InstanceIsolation.Disabled.ToolTip'),
                    },
                  ]"
                  :model-value="String(values['ComboArgumentIndieV2'])"
                  @update:model-value="(value) => change('ComboArgumentIndieV2', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Launch.Options.WindowTitle.Label') }}</span
                >
                <PTextBox
                  data-control="TextArgumentTitle"
                  :suggestions="[text('Setup.Launch.Options.WindowTitle.SampleFormat')]"
                  style="grid-row: 3 / span 1; grid-column: 2 / span 2"
                  :title="text('Instance.Setup.Options.WindowTitle.ToolTip')"
                  :placeholder="text('')"
                  :model-value="String(values['TextArgumentTitle'])"
                  @update:model-value="(value) => change('TextArgumentTitle', value || '')"
                />
                <PCheckBox
                  data-control="CheckArgumentTitleEmpty"
                  style="margin: 0px 0px 9px 0px; grid-row: 5 / span 1; grid-column: 1 / span 3"
                  :title="text('Instance.Setup.Options.WindowTitle.Default.ToolTip')"
                  id="PageInstanceSetup-CheckArgumentTitleEmpty"
                  :label="text('Instance.Setup.Options.WindowTitle.Default')"
                  :model-value="Boolean(values['CheckArgumentTitleEmpty'])"
                  @update:model-value="(value) => change('CheckArgumentTitleEmpty', value)"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 7 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Launch.Options.CustomInfo.Label') }}</span
                >
                <PTextBox
                  data-control="TextArgumentInfo"
                  style="grid-row: 7 / span 1; grid-column: 2 / span 2"
                  :title="text('Instance.Setup.Options.CustomInfo.ToolTip')"
                  :placeholder="text('Instance.Setup.FollowGlobal')"
                  :model-value="String(values['TextArgumentInfo'])"
                  @update:model-value="(value) => change('TextArgumentInfo', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 9 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Instance.Setup.Options.Java') }}</span
                >
                <PComboBox
                  data-control="ComboArgumentJava"
                  style="grid-row: 9 / span 1; grid-column: 2 / span 2"
                  :title="text('Instance.Setup.Options.Java.ToolTip')"
                  :options="
                    javaLoading
                      ? [{ key: '0', text: text('Instance.Setup.Options.Java.Loading') }]
                      : javaOptions
                  "
                  :disabled="javaLoading"
                  :model-value="String(values['ComboArgumentJava'])"
                  @update:model-value="(value) => change('ComboArgumentJava', value || '')"
                />
              </div>
            </div>
          </PCard>
          <PHint v-if="javaError" severity="warning" style="margin-bottom: 15px"
            >{{ text('Instance.Setup.Options.Java.LoadFailed')
            }}<RouterLink to="/setup/java">{{
              text('Instance.Setup.Options.Java')
            }}</RouterLink></PHint
          >
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Setup.Launch.Memory.Title')">
            <PHint
              v-if="
                javaList.some(
                  (java) => java.java_exe === values.ComboArgumentJava && !java.is_64_bit,
                )
              "
              severity="warning"
              >{{ text('Setup.Launch.Memory.Warning.Java32Bit') }}</PHint
            >
            <PHint v-if="memoryOverLimit" severity="warning">{{
              text('Setup.Launch.Memory.Warning.TooMuch')
            }}</PHint>
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 0px 0px 3px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: auto auto auto auto auto;
                "
              >
                <label
                  data-control="RadioRamType2"
                  style="margin: 0px 20px 9px 0px; grid-row: 1 / span 1; grid-column: 1 / span 2"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['VersionRamType']" value="2" />{{
                    text('Instance.Setup.FollowGlobal')
                  }}</label
                >
                <label
                  data-control="RadioRamType0"
                  style="
                    width: 110px;
                    margin: 0px 20px 9px 0px;
                    grid-row: 2 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                  "
                  :title="text('Setup.Launch.Memory.Auto.ToolTip')"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['VersionRamType']" value="0" />{{
                    text('Setup.Launch.Memory.Auto')
                  }}</label
                >
                <label
                  data-control="RadioRamType1"
                  style="
                    width: 110px;
                    margin: 0px 20px 9px 0px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                  "
                  class="pcl-radio"
                  ><input type="radio" v-model="values['VersionRamType']" value="1" />{{
                    text('Common.Option.Customize')
                  }}</label
                >
                <span
                  style="
                    margin: 0px 20px 9px 0px;
                    grid-row: 4 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Launch.Memory.Max.ToolTip')"
                  class="pcl-text"
                  >{{ text('Setup.Launch.Memory.Max') }}</span
                >
                <input
                  data-control="SliderRamCustom"
                  :disabled="values.VersionRamType !== '1'"
                  style="margin: 0px 12px 9px 0px; grid-row: 4 / span 1; grid-column: 2 / span 1"
                  type="range"
                  min="0"
                  max="49"
                  v-model.number="values['SliderRamCustom']"
                />
                <span
                  style="
                    margin: 0px 20px 0px 0px;
                    grid-row: 5 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Launch.Memory.Initial.ToolTip')"
                  class="pcl-text"
                  >{{ text('Setup.Launch.Memory.Initial') }}</span
                >
                <input
                  data-control="SliderRamInitialCustom"
                  :disabled="values.VersionRamType !== '1'"
                  style="margin: 0px 12px 0px 0px; grid-row: 5 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.Launch.Memory.Initial.ToolTip')"
                  type="range"
                  min="0"
                  max="49"
                  v-model.number="values['SliderRamInitialCustom']"
                />
              </div>
              <div
                data-control="PanRamDisplay"
                style="
                  margin: 11px 2px 0px 0px;
                  display: grid;
                  grid-template-columns: 1fr 1fr 1fr;
                  grid-template-rows: minmax(0, 1fr) 4px minmax(0, 1fr);
                "
              >
                <div
                  data-control="RectRamUsed"
                  style="grid-row: 2 / span 1; grid-column: 1 / span 1"
                  class="pcl-divider"
                />
                <div
                  data-control="RectRamGame"
                  style="opacity: 0.5; grid-row: 2 / span 1; grid-column: 2 / span 1"
                  class="pcl-divider"
                />
                <div
                  data-control="RectRamEmpty"
                  style="opacity: 0.7; grid-row: 2 / span 1; grid-column: 3 / span 1"
                  class="pcl-divider"
                />
                <span
                  data-control="LabRamUsedTitle"
                  style="
                    margin: 0px 0px 5px 2px;
                    font-size: 11px;
                    opacity: 0.7;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                  "
                  class="pcl-text"
                  >{{ text('Instance.Setup.Memory.Used') }}</span
                >
                <span
                  data-control="LabRamGameTitle"
                  style="
                    margin: 0px 0px 5px 2px;
                    font-size: 11px;
                    opacity: 0.7;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Launch.Memory.GameAllocation') }}</span
                >
                <div
                  style="
                    margin: 3px 0px 0px 2px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 3;
                    justify-self: start;
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                  "
                >
                  <span data-control="LabRamUsed" style="font-size: 16px" class="pcl-text">{{
                    text('—')
                  }}</span>
                  <span data-control="LabRamTotal" style="font-size: 16px" class="pcl-text">{{
                    text(' / —')
                  }}</span>
                </div>
                <span
                  data-control="LabRamGame"
                  style="
                    margin: 3px 0px 0px 2px;
                    font-size: 16px;
                    grid-row: 3 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                  "
                  class="pcl-text"
                  >{{ text('—') }}</span
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardServer"
            style="margin: 0px 0px 15px 0px"
            :title="text('Instance.Setup.Server.Title')"
          >
            <div style="margin: 2px 25px 18px 25px; display: flex; flex-direction: column">
              <PHint v-if="loginLocked" severity="warning" style="margin-bottom: 13px">{{
                text('Instance.Setup.Server.LoginLockHint')
              }}</PHint>
              <PHint
                v-if="authVisible && /^https:\/\//i.test(String(values.TextServerAuthServer))"
                severity="info"
                style="margin-bottom: 13px"
                >{{ text('Instance.Setup.Server.SslWarning') }}</PHint
              >
              <PHint
                v-if="authVisible && /^http:\/\//i.test(String(values.TextServerAuthServer))"
                severity="warning"
                style="margin-bottom: 13px"
                >{{ text('Instance.Setup.Server.HttpWarning') }}</PHint
              >
              <div
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: 28px auto auto auto 9px 28px 3px;
                "
              >
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Instance.Setup.Server.RestrictLogin') }}</span
                >
                <PComboBox
                  data-control="ComboServerLoginRequire"
                  :disabled="loginLocked"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Instance.Setup.Server.LoginMethod.Unlimited') },
                    { key: '1', text: text('Instance.Setup.Server.LoginMethod.MicrosoftOnly') },
                    { key: '2', text: text('Instance.Setup.Server.LoginMethod.AuthOnly') },
                    { key: '3', text: text('Instance.Setup.Server.LoginMethod.MicrosoftOrAuth') },
                  ]"
                  :model-value="String(values['ComboServerLoginRequire'])"
                  @update:model-value="(value) => change('ComboServerLoginRequire', value || '')"
                />
                <template v-if="authVisible">
                  <label
                    for="auth-Server"
                    style="grid-row: 2; align-self: center; margin: 8px 25px 0 0"
                    >{{ text('Instance.Setup.Server.AuthServer') }}</label
                  >
                  <PTextBox
                    data-control="TextServerAuthServer"
                    :disabled="loginLocked"
                    input-id="auth-Server"
                    style="grid-row: 2; grid-column: 2 / span 2; margin-top: 8px"
                    :placeholder="text('Instance.Setup.Server.AuthServer.Hint')"
                    v-model="values.TextServerAuthServer as string"
                  />
                </template>
                <template v-if="authVisible">
                  <label
                    for="auth-Register"
                    style="grid-row: 3; align-self: center; margin: 8px 25px 0 0"
                    >{{ text('Instance.Setup.Server.Register') }}</label
                  >
                  <PTextBox
                    data-control="TextServerAuthRegister"
                    :disabled="loginLocked"
                    input-id="auth-Register"
                    style="grid-row: 3; grid-column: 2 / span 2; margin-top: 8px"
                    :placeholder="text('Instance.Setup.Server.Register.Hint')"
                    v-model="values.TextServerAuthRegister as string"
                  />
                </template>
                <template v-if="authVisible">
                  <label
                    for="auth-Name"
                    style="grid-row: 4; align-self: center; margin: 8px 25px 0 0"
                    >{{ text('Instance.Setup.Server.Name') }}</label
                  >
                  <PTextBox
                    data-control="TextServerAuthName"
                    :disabled="loginLocked"
                    input-id="auth-Name"
                    style="grid-row: 4; grid-column: 2 / span 2; margin-top: 8px"
                    :placeholder="text('Instance.Setup.Server.Name.Hint')"
                    v-model="values.TextServerAuthName as string"
                  />
                </template>
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 6 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Instance.Setup.Server.AutoJoin') }}</span
                >
                <PTextBox
                  data-control="TextServerEnter"
                  style="grid-row: 6 / span 1; grid-column: 2 / span 2"
                  :title="text('Instance.Setup.Server.AutoJoin.ToolTip')"
                  :placeholder="text('')"
                  :model-value="String(values['TextServerEnter'])"
                  @update:model-value="(value) => change('TextServerEnter', value || '')"
                />
              </div>
              <div
                v-if="authVisible"
                style="display: flex; gap: 30px; flex-wrap: wrap; margin-top: 12px"
              >
                <PButton
                  data-control="BtnServerAuthLittle"
                  style="height: 35px; min-width: 150px; margin: 0"
                  :click="setLittleSkin"
                  :disabled="loginLocked"
                  type="tint"
                  >{{ text('Instance.Setup.Server.SetLittleSkin') }}</PButton
                >
                <PButton
                  data-control="BtnServerAuthLock"
                  style="height: 35px; min-width: 150px; margin: 0"
                  :click="() => act('BtnServerAuthLock', '', '', '')"
                  disabled
                  >{{ text('Instance.Setup.Server.LockLoginMethod') }}</PButton
                >
                <PButton
                  v-if="canCreateProfile"
                  data-control="BtnServerNewProfile"
                  style="height: 35px; min-width: 150px; margin: 0"
                  :click="() => act('BtnServerNewProfile', '', '', '')"
                  >{{ text('Instance.Setup.Server.CreateProfile') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardAdvance"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Launch.Advanced.Title')"
            default-fold-status="fold"
          >
            <div
              style="
                margin: 0px 25px 15px 25px;
                display: grid;
                grid-template-columns: auto minmax(0, 1fr);
                grid-template-rows: 28px 9px auto 9px 28px 9px 28px 9px 31px auto auto;
              "
            >
              <span
                style="
                  margin: 0px 25px 0px 0px;
                  grid-row: 1 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                "
                class="pcl-text"
                >{{ text('Setup.Launch.Advanced.Renderer.Label') }}</span
              >
              <PComboBox
                data-control="ComboAdvanceRenderer"
                style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                :options="[
                  { key: '0', text: text('Instance.Setup.FollowGlobal') },
                  { key: '1', text: text('Setup.Launch.Advanced.Renderer.GameDefault') },
                  { key: '2', text: text('Setup.Launch.Advanced.Renderer.Software') },
                  { key: '3', text: text('Setup.Launch.Advanced.Renderer.DirectX12') },
                  { key: '4', text: text('Setup.Launch.Advanced.Renderer.Vulkan') },
                ]"
                :model-value="String(values['ComboAdvanceRenderer'])"
                @update:model-value="(value) => change('ComboAdvanceRenderer', value || '')"
              />
              <span
                style="
                  margin: 0px 25px 0px 0px;
                  grid-row: 3 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                "
                class="pcl-text"
                >{{ text('Setup.Launch.Advanced.JvmHead') }}</span
              >
              <PTextBox
                multiline
                data-control="TextAdvanceJvm"
                style="
                  height: 100px;
                  padding: 5px 0px;
                  grid-row: 3 / span 1;
                  grid-column: 2 / span 2;
                "
                :title="text('Instance.Setup.Advanced.JvmArgs.ToolTip')"
                :placeholder="text('Instance.Setup.FollowGlobal')"
                :model-value="String(values['TextAdvanceJvm'])"
                @update:model-value="(value) => change('TextAdvanceJvm', value || '')"
              />
              <span
                style="
                  margin: 0px 25px 0px 0px;
                  grid-row: 5 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                "
                class="pcl-text"
                >{{ text('Setup.Launch.Advanced.GameTail') }}</span
              >
              <PTextBox
                data-control="TextAdvanceGame"
                style="grid-row: 5 / span 1; grid-column: 2 / span 2"
                :title="text('Instance.Setup.Advanced.GameArgs.ToolTip')"
                :placeholder="text('Instance.Setup.FollowGlobal')"
                :model-value="String(values['TextAdvanceGame'])"
                @update:model-value="(value) => change('TextAdvanceGame', value || '')"
              />
              <span
                style="
                  margin: 0px 25px 0px 0px;
                  grid-row: 7 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                "
                class="pcl-text"
                >{{ text('Instance.Setup.Advanced.Classpath') }}</span
              >
              <PTextBox
                data-control="TextAdvanceClasspathHead"
                style="grid-row: 7 / span 1; grid-column: 2 / span 2"
                :title="text('Instance.Setup.Advanced.Classpath.ToolTip')"
                :placeholder="text('')"
                :model-value="String(values['TextAdvanceClasspathHead'])"
                @update:model-value="(value) => change('TextAdvanceClasspathHead', value || '')"
              />
              <span
                style="
                  margin: 0px 25px 3px 0px;
                  grid-row: 9 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                "
                class="pcl-text"
                >{{ text('Setup.Launch.Advanced.PreLaunchCommand') }}</span
              >
              <PTextBox
                data-control="TextAdvanceRun"
                style="margin: 0px 0px 3px 0px; grid-row: 9 / span 1; grid-column: 2 / span 2"
                :title="text('Instance.Setup.Advanced.PreLaunchCommand.ToolTip')"
                :placeholder="text('')"
                :model-value="String(values['TextAdvanceRun'])"
                @update:model-value="(value) => change('TextAdvanceRun', value || '')"
              />
              <div
                style="
                  margin: 8px 0px 4px 0px;
                  grid-row: 10 / span 1;
                  grid-column: 1 / span 2;
                  justify-self: start;
                  display: flex;
                  flex-direction: column;
                "
              >
                <PCheckBox
                  v-if="String(values.TextAdvanceRun || '').trim()"
                  data-control="CheckAdvanceRunWait"
                  style="margin: 0 0 8px"
                  id="PageInstanceSetup-CheckAdvanceRunWait"
                  :label="text('Setup.Launch.Advanced.PreLaunchCommand.Wait')"
                  :model-value="Boolean(values.CheckAdvanceRunWait)"
                  @update:model-value="(value) => change('CheckAdvanceRunWait', value)"
                />

                <PCheckBox
                  data-control="CheckAdvanceDisableModUpdate"
                  style="height: 28px"
                  :title="text('Setup.Ui.FeatureHide.Item.ModUpdate.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceDisableModUpdate"
                  :label="text('Instance.Setup.Advanced.DisableModUpdate')"
                  :model-value="Boolean(values['CheckAdvanceDisableModUpdate'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableModUpdate', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceJava"
                  style="height: 28px"
                  :title="text('Instance.Setup.Advanced.IgnoreJavaCompatibility.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceJava"
                  :label="text('Instance.Setup.Advanced.IgnoreJavaCompatibility')"
                  :model-value="Boolean(values['CheckAdvanceJava'])"
                  @update:model-value="(value) => change('CheckAdvanceJava', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceAssetsV2"
                  style="height: 28px; margin: 0px 60px 0px 0px"
                  :title="text('Instance.Setup.Advanced.DisableFileVerification.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceAssetsV2"
                  :label="text('Instance.Setup.Advanced.DisableFileVerification')"
                  :model-value="Boolean(values['CheckAdvanceAssetsV2'])"
                  @update:model-value="(value) => change('CheckAdvanceAssetsV2', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceUseProxyV2"
                  style="height: 28px"
                  :title="text('Instance.Setup.Advanced.UseGlobalProxy.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceUseProxyV2"
                  :label="text('Instance.Setup.Advanced.UseGlobalProxy')"
                  :model-value="Boolean(values['CheckAdvanceUseProxyV2'])"
                  @update:model-value="(value) => change('CheckAdvanceUseProxyV2', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableJLW"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableJlw.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceDisableJLW"
                  :label="text('Setup.Launch.Advanced.DisableJlw')"
                  :model-value="Boolean(values['CheckAdvanceDisableJLW'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableJLW', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLF"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLF.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceDisableLF"
                  :label="text('Setup.Launch.Advanced.DisableLF')"
                  :model-value="Boolean(values['CheckAdvanceDisableLF'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableLF', value)"
                />
                <PCheckBox
                  data-control="CheckUseDebugLog4j2Config"
                  style="height: 28px"
                  id="PageInstanceSetup-CheckUseDebugLog4j2Config"
                  :label="text('Instance.Setup.Advanced.UseDebugLog4j')"
                  :model-value="Boolean(values['CheckUseDebugLog4j2Config'])"
                  @update:model-value="(value) => change('CheckUseDebugLog4j2Config', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLwjglUnsafeAgent"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent.ToolTip')"
                  id="PageInstanceSetup-CheckAdvanceDisableLwjglUnsafeAgent"
                  :label="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent')"
                  :model-value="Boolean(values['CheckAdvanceDisableLwjglUnsafeAgent'])"
                  @update:model-value="
                    (value) => change('CheckAdvanceDisableLwjglUnsafeAgent', value)
                  "
                />
              </div>
            </div>
          </PCard>
          <PButton
            data-control="BtnSwitch"
            style="margin: -5px 0px 0px 0px"
            :click="() => act('BtnSwitch', '', '', '')"
            inline
            ><PSvgIcon name="lucide/arrow-right" :size="18" />{{
              text('Instance.Setup.OpenGlobalSettings')
            }}</PButton
          >
        </div>
      </div>
    </div>
  </div>
</template>
