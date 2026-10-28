<script setup lang="ts">
import SetupItem from '@/components/widget/SetupItem.vue'
import { useSetup } from '@/stores/setup'
import { watch } from 'vue'
import { useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
const setup = useSetup()
defineProps<{ java32Bit?: boolean; memoryOverLimit?: boolean }>()
const { values, reset } = usePageSettings('PageSetupLaunch', {
  TextArgumentTitle: setup.launch.gameWindowTitle,
  TextArgumentInfo: setup.launch.customInfo,
  ComboArgumentVisibie: '4',
  ComboArgumentPriority: '3',
  ComboArgumentWindowType: '1',
  ComboPreferredIpStack: '0',
})

watch(
  () => [values.TextArgumentTitle, values.TextArgumentInfo],
  ([title, info]) => {
    setup.launch.gameWindowTitle = title || ''
    setup.launch.customInfo = info || ''
  },
  { immediate: true },
)
const router = useRouter()
const text = pclText
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 25px 25px 10px 25px; display: flex; flex-direction: column"
        >
          <PCard
            data-control="CardArgument"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Launch.Options.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div class="settings-grid">
                <SetupItem
                  :label="text('Setup.Launch.Options.InstanceIsolation.Label')"
                  v-slot="{ id }"
                  ><PComboBox
                    :input-id="id"
                    data-control="ComboArgumentIndieV2"
                    :title="text('Setup.Launch.Options.InstanceIsolation.ToolTip')"
                    :options="[
                      {
                        key: '0',
                        text: text('Common.Action.Close'),
                        title: text('Setup.Launch.Options.InstanceIsolation.None.ToolTip'),
                      },
                      {
                        key: '1',
                        text: text('Setup.Launch.Options.InstanceIsolation.ModdedOnly'),
                        title: text('Setup.Launch.Options.InstanceIsolation.ModdedOnly.ToolTip'),
                      },
                      {
                        key: '2',
                        text: text('Setup.Launch.Options.InstanceIsolation.NonRelease'),
                        title: text('Setup.Launch.Options.InstanceIsolation.NonRelease.ToolTip'),
                      },
                      {
                        key: '3',
                        text: text('Setup.Launch.Options.InstanceIsolation.ModdedAndNonRelease'),
                      },
                      {
                        key: '4',
                        text: text('Setup.Launch.Options.InstanceIsolation.All'),
                        title: text('Setup.Launch.Options.InstanceIsolation.All.ToolTip'),
                      },
                    ]"
                    v-model="values.ComboArgumentIndieV2"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.WindowTitle.Label')" v-slot="{ id }"
                  ><PTextBox
                    :input-id="id"
                    data-control="TextArgumentTitle"
                    :suggestions="[text('Setup.Launch.Options.WindowTitle.SampleFormat')]"
                    :title="text('Setup.Launch.Options.WindowTitle.ToolTip')"
                    :placeholder="text('Common.Option.Default')"
                    v-model="values.TextArgumentTitle"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.CustomInfo.Label')" v-slot="{ id }"
                  ><PTextBox
                    :input-id="id"
                    data-control="TextArgumentInfo"
                    :title="text('Setup.Launch.Options.CustomInfo.ToolTip')"
                    :placeholder="text('')"
                    v-model="values.TextArgumentInfo"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.Visibility.Label')" v-slot="{ id }"
                  ><PComboBox
                    :input-id="id"
                    data-control="ComboArgumentVisibie"
                    :options="[
                      { key: '0', text: text('Setup.Launch.Options.Visibility.CloseImmediately') },
                      {
                        key: '1',
                        text: text('Setup.Launch.Options.Visibility.HideAndCloseOnExit'),
                      },
                      {
                        key: '2',
                        text: text('Setup.Launch.Options.Visibility.HideAndReopenOnExit'),
                      },
                      { key: '3', text: text('Setup.Launch.Options.Visibility.Minimize') },
                      { key: '4', text: text('Setup.Launch.Options.Visibility.KeepUnchanged') },
                    ]"
                    v-model="values.ComboArgumentVisibie"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.Priority.Label')" v-slot="{ id }"
                  ><PComboBox
                    :input-id="id"
                    data-control="ComboArgumentPriority"
                    :options="[
                      { key: '0', text: text('Setup.Launch.Options.Priority.RealTime') },
                      { key: '1', text: text('Setup.Launch.Options.Priority.High') },
                      { key: '2', text: text('Setup.Launch.Options.Priority.AboveNormal') },
                      { key: '3', text: text('Setup.Launch.Options.Priority.Normal') },
                      { key: '4', text: text('Setup.Launch.Options.Priority.BelowNormal') },
                    ]"
                    v-model="values.ComboArgumentPriority"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.WindowSize.Label')" v-slot="{ id }"
                  ><div
                    data-control="PanArgumentWindow"
                    style="display: grid; grid-template-columns: minmax(0, 1fr) auto auto auto"
                  >
                    <PComboBox
                      :input-id="id"
                      data-control="ComboArgumentWindowType"
                      style="grid-row: 1 / span 1; grid-column: 1 / span 1"
                      :title="text('Setup.Launch.Options.WindowSize.ModernUiWarning')"
                      :options="[
                        { key: '0', text: text('Setup.Launch.Options.WindowSize.Fullscreen') },
                        { key: '1', text: text('Common.Option.Default') },
                        { key: '2', text: text('Setup.Launch.Options.WindowSize.SameAsLauncher') },
                        {
                          key: '3',
                          text: text('Setup.Launch.Options.WindowSize.Custom'),
                          title: text('Setup.Launch.Options.WindowSize.CustomHeightBug.ToolTip'),
                        },
                        { key: '4', text: text('Setup.Launch.Options.WindowSize.Maximized') },
                      ]"
                      v-model="values.ComboArgumentWindowType"
                    />
                    <PTextBox
                      data-control="TextArgumentWindowWidth"
                      style="
                        width: 60px;
                        margin: 0px 0px 0px 50px;
                        padding: 0px 5px 0px 0px;
                        grid-row: 1 / span 1;
                        grid-column: 2 / span 1;
                      "
                      v-if="values.ComboArgumentWindowType === '3'"
                      :placeholder="text('')"
                      v-model="values.TextArgumentWindowWidth"
                    />
                    <span
                      data-control="LabArgumentWindowMiddle"
                      style="
                        font-size: 18px;
                        grid-row: 1 / span 1;
                        grid-column: 3 / span 1;
                        align-self: center;
                      "
                      v-if="values.ComboArgumentWindowType === '3'"
                      class="pcl-text"
                      >{{ text(' × ') }}</span
                    >
                    <PTextBox
                      data-control="TextArgumentWindowHeight"
                      style="
                        width: 60px;
                        padding: 0px 5px 0px 0px;
                        grid-row: 1 / span 1;
                        grid-column: 4 / span 1;
                      "
                      v-if="values.ComboArgumentWindowType === '3'"
                      :title="text('Setup.Launch.Options.WindowSize.CustomHeightBug.ToolTip')"
                      :placeholder="text('')"
                      v-model="values.TextArgumentWindowHeight"
                    /></div
                ></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.MsAuth.Label')" v-slot="{ id }"
                  ><PComboBox
                    :input-id="id"
                    data-control="ComboMsAuthType"
                    :title="text('Setup.Launch.Options.MsAuth.ToolTip')"
                    :options="[
                      { key: '0', text: text('Setup.Launch.Options.MsAuth.WebManager') },
                      { key: '1', text: text('Setup.Launch.Options.MsAuth.DeviceCode') },
                    ]"
                    v-model="values.ComboMsAuthType"
                /></SetupItem>
                <SetupItem :label="text('Setup.Launch.Options.IpStack.Label')" v-slot="{ id }"
                  ><PComboBox
                    :input-id="id"
                    data-control="ComboPreferredIpStack"
                    :title="text('Setup.Launch.Options.IpStack.ToolTip')"
                    :options="[
                      { key: '0', text: text('Setup.Launch.Options.IpStack.Ipv4') },
                      { key: '1', text: text('Setup.Launch.Options.IpStack.JavaDefault') },
                      { key: '2', text: text('Setup.Launch.Options.IpStack.Ipv6') },
                    ]"
                    v-model="values.ComboPreferredIpStack"
                /></SetupItem>
              </div>
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Setup.Launch.Memory.Title')">
            <PHint v-if="java32Bit" severity="warning">{{
              text('Setup.Launch.Memory.Warning.Java32Bit')
            }}</PHint>
            <PHint v-if="memoryOverLimit" severity="warning">{{
              text('Setup.Launch.Memory.Warning.TooMuch')
            }}</PHint>
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 0px 0px 3px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: auto auto auto auto;
                "
              >
                <label
                  data-control="RadioRamType0"
                  style="
                    width: 110px;
                    margin: 0px 20px 9px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                  "
                  :title="text('Setup.Launch.Memory.Auto.ToolTip')"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['LaunchRamType']" value="0" />{{
                    text('Setup.Launch.Memory.Auto')
                  }}</label
                >
                <label
                  data-control="RadioRamType1"
                  style="margin: 0px 20px 9px 0px; grid-row: 2 / span 1; grid-column: 1 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['LaunchRamType']" value="1" />{{
                    text('Common.Option.Customize')
                  }}</label
                >
                <span
                  style="
                    margin: 0px 20px 9px 0px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Launch.Memory.Max.ToolTip')"
                  class="pcl-text"
                  >{{ text('Setup.Launch.Memory.Max') }}</span
                >
                <input
                  data-control="SliderRamCustom"
                  :disabled="values.LaunchRamType !== '1'"
                  style="margin: 0px 12px 9px 0px; grid-row: 3 / span 1; grid-column: 2 / span 1"
                  type="range"
                  min="0"
                  max="49"
                  v-model.number="values['SliderRamCustom']"
                />
                <span
                  style="
                    margin: 0px 20px 0px 0px;
                    grid-row: 4 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Launch.Memory.Initial.ToolTip')"
                  class="pcl-text"
                  >{{ text('Setup.Launch.Memory.Initial') }}</span
                >
                <input
                  data-control="SliderRamInitialCustom"
                  :disabled="values.LaunchRamType !== '1'"
                  style="grid-row: 4 / span 1; grid-column: 2 / span 1"
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
                  >{{ text('Setup.Launch.Memory.UsedTotal') }}</span
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
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Launch.Advanced.Title')"
            default-fold-status="fold"
          >
            <div class="settings-grid" style="margin: 0px 25px 15px 25px">
              <SetupItem :label="text('Setup.Launch.Advanced.Renderer.Label')" v-slot="{ id }"
                ><PComboBox
                  :input-id="id"
                  data-control="ComboAdvanceRenderer"
                  :options="[
                    { key: '0', text: text('Setup.Launch.Advanced.Renderer.GameDefault') },
                    { key: '1', text: text('Setup.Launch.Advanced.Renderer.Software') },
                    { key: '2', text: text('Setup.Launch.Advanced.Renderer.DirectX12') },
                    { key: '3', text: text('Setup.Launch.Advanced.Renderer.Vulkan') },
                  ]"
                  v-model="values.ComboAdvanceRenderer"
              /></SetupItem>
              <SetupItem
                ><template #label="{ id }"
                  ><div style="display: flex; flex-direction: row; align-items: center">
                    <label :for="id" class="pcl-text">{{
                      text('Setup.Launch.Advanced.JvmHead')
                    }}</label>
                    <PButton
                      data-control="BtnAdvanceJvmReset"
                      style="height: 23px; margin: 1px 0px 0px 0px"
                      :title="text('Setup.Launch.Advanced.JvmHead.Restore')"
                      @click="() => reset('TextAdvanceJvm')"
                      inline
                      ><PSvgIcon name="lucide/rotate-ccw" :size="18"
                    /></PButton></div></template
                ><template #default="{ id }"
                  ><PTextBox
                    :input-id="id"
                    multiline
                    data-control="TextAdvanceJvm"
                    style="height: 100px; padding: 5px 0px"
                    :title="text('Setup.Launch.Advanced.JvmHead.ToolTip')"
                    :placeholder="text('')"
                    v-model="values.TextAdvanceJvm" /></template
              ></SetupItem>
              <SetupItem :label="text('Setup.Launch.Advanced.GameTail')" v-slot="{ id }"
                ><PTextBox
                  :input-id="id"
                  data-control="TextAdvanceGame"
                  :title="text('Setup.Launch.Advanced.GameTail.ToolTip')"
                  :placeholder="text('')"
                  v-model="values.TextAdvanceGame"
              /></SetupItem>
              <SetupItem :label="text('Setup.Launch.Advanced.PreLaunchCommand')" v-slot="{ id }"
                ><PTextBox
                  :input-id="id"
                  data-control="TextAdvanceRun"
                  :title="text('Setup.Launch.Advanced.PreLaunchCommand.ToolTip')"
                  :placeholder="text('')"
                  v-model="values.TextAdvanceRun"
              /></SetupItem>
              <PCheckBox
                v-if="String(values.TextAdvanceRun || '').trim()"
                data-control="CheckAdvanceRunWait"
                style="margin: 8px 0 7px; grid-column: 2"
                id="PageSetupLaunch-CheckAdvanceRunWait"
                :label="text('Setup.Launch.Advanced.PreLaunchCommand.Wait')"
                v-model="values.CheckAdvanceRunWait"
              />
              <div
                style="
                  margin: 12px 0px 4px 0px;
                  display: flex;
                  flex-direction: column;
                  grid-column: 1 / -1;
                "
              >
                <PCheckBox
                  data-control="CheckAdvanceDisableJLW"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableJlw.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableJLW"
                  :label="text('Setup.Launch.Advanced.DisableJlw')"
                  v-model="values.CheckAdvanceDisableJLW"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLF"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLF.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableLF"
                  :label="text('Setup.Launch.Advanced.DisableLF')"
                  v-model="values.CheckAdvanceDisableLF"
                />
                <PCheckBox
                  data-control="CheckAdvanceGraphicCard"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.GpuPreference.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceGraphicCard"
                  :label="text('Setup.Launch.Advanced.GpuPreference')"
                  v-model="values.CheckAdvanceGraphicCard"
                />
                <PCheckBox
                  data-control="CheckAdvanceNoJavaw"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.UseJavaExe.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceNoJavaw"
                  :label="text('Setup.Launch.Advanced.UseJavaExe')"
                  v-model="values.CheckAdvanceNoJavaw"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLwjglUnsafeAgent"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableLwjglUnsafeAgent"
                  :label="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent')"
                  v-model="values.CheckAdvanceDisableLwjglUnsafeAgent"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableCrashAnalysis"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableCrashAnalysis.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableCrashAnalysis"
                  :label="text('Setup.Launch.Advanced.DisableCrashAnalysis')"
                  v-model="values.CheckAdvanceDisableCrashAnalysis"
                />
                <PCheckBox
                  data-control="CheckAdvanceLockMemory"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.LockMemory.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceLockMemory"
                  :label="text('Setup.Launch.Advanced.LockMemory')"
                  v-model="values.CheckAdvanceLockMemory"
                />
              </div>
              <PHint
                style="margin: 5px 0px 10px 0px; grid-column: 1 / -1"
                severity="info"
                closable
                >{{ text('Setup.Launch.Advanced.MoreInInstanceSetup') }}</PHint
              >
            </div>
          </PCard>
          <PButton
            data-control="BtnSwitch"
            style="margin: -5px 0px 15px 0px"
            @click="() => router.push('/instance_setting/setting')"
            inline
            ><PSvgIcon name="lucide/arrow-right" :size="18" />{{
              text('Setup.Launch.SwitchToInstanceSetup')
            }}</PButton
          >
        </div>
      </div>
    </div>
  </div>
</template>
