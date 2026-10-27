<script setup lang="ts">
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
defineProps<{ java32Bit?: boolean; memoryOverLimit?: boolean }>()
const { values, text, change, act, image } = usePageSettings('PageSetupLaunch', {
  ComboArgumentIndieV2: '1',
  TextArgumentTitle: '',
  TextArgumentInfo: 'PCL',
  ComboArgumentVisibie: '4',
  ComboArgumentPriority: '3',
  ComboArgumentWindowType: '1',
  TextArgumentWindowWidth: '854',
  TextArgumentWindowHeight: '480',
  ComboMsAuthType: '0',
  ComboPreferredIpStack: '0',
  LaunchRamType: '0',
  SliderRamCustom: 13.0,
  SliderRamInitialCustom: 0.0,
  ComboAdvanceRenderer: '0',
  TextAdvanceJvm: '',
  TextAdvanceGame: '',
  TextAdvanceRun: '',
  CheckAdvanceDisableJLW: false,
  CheckAdvanceDisableLF: false,
  CheckAdvanceGraphicCard: false,
  CheckAdvanceNoJavaw: false,
  CheckAdvanceDisableLwjglUnsafeAgent: false,
  CheckAdvanceDisableCrashAnalysis: false,
  CheckAdvanceLockMemory: false,
})
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
              <div
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr) 60px;
                  grid-template-rows: 28px 9px 28px 9px 28px 9px 28px 9px 28px 9px 28px 9px 28px 9px 28px auto;
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
                  >{{ text('Setup.Launch.Options.InstanceIsolation.Label') }}</span
                >
                <PComboBox
                  data-control="ComboArgumentIndieV2"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
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
                  :title="text('Setup.Launch.Options.WindowTitle.ToolTip')"
                  :placeholder="text('Common.Option.Default')"
                  :model-value="String(values['TextArgumentTitle'])"
                  @update:model-value="(value) => change('TextArgumentTitle', value || '')"
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
                  >{{ text('Setup.Launch.Options.CustomInfo.Label') }}</span
                >
                <PTextBox
                  data-control="TextArgumentInfo"
                  style="grid-row: 5 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.Launch.Options.CustomInfo.ToolTip')"
                  :placeholder="text('')"
                  :model-value="String(values['TextArgumentInfo'])"
                  @update:model-value="(value) => change('TextArgumentInfo', value || '')"
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
                  >{{ text('Setup.Launch.Options.Visibility.Label') }}</span
                >
                <PComboBox
                  data-control="ComboArgumentVisibie"
                  style="grid-row: 7 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.Launch.Options.Visibility.CloseImmediately') },
                    { key: '1', text: text('Setup.Launch.Options.Visibility.HideAndCloseOnExit') },
                    { key: '2', text: text('Setup.Launch.Options.Visibility.HideAndReopenOnExit') },
                    { key: '3', text: text('Setup.Launch.Options.Visibility.Minimize') },
                    { key: '4', text: text('Setup.Launch.Options.Visibility.KeepUnchanged') },
                  ]"
                  :model-value="String(values['ComboArgumentVisibie'])"
                  @update:model-value="(value) => change('ComboArgumentVisibie', value || '')"
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
                  >{{ text('Setup.Launch.Options.Priority.Label') }}</span
                >
                <PComboBox
                  data-control="ComboArgumentPriority"
                  style="grid-row: 9 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.Launch.Options.Priority.RealTime') },
                    { key: '1', text: text('Setup.Launch.Options.Priority.High') },
                    { key: '2', text: text('Setup.Launch.Options.Priority.AboveNormal') },
                    { key: '3', text: text('Setup.Launch.Options.Priority.Normal') },
                    { key: '4', text: text('Setup.Launch.Options.Priority.BelowNormal') },
                  ]"
                  :model-value="String(values['ComboArgumentPriority'])"
                  @update:model-value="(value) => change('ComboArgumentPriority', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 11 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Launch.Options.WindowSize.Label') }}</span
                >
                <div
                  data-control="PanArgumentWindow"
                  style="
                    grid-row: 11 / span 1;
                    grid-column: 2 / span 2;
                    display: grid;
                    grid-template-columns: minmax(0, 1fr) auto auto auto;
                  "
                >
                  <PComboBox
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
                    :model-value="String(values['ComboArgumentWindowType'])"
                    @update:model-value="(value) => change('ComboArgumentWindowType', value || '')"
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
                    :model-value="String(values['TextArgumentWindowWidth'])"
                    @update:model-value="(value) => change('TextArgumentWindowWidth', value || '')"
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
                    :model-value="String(values['TextArgumentWindowHeight'])"
                    @update:model-value="(value) => change('TextArgumentWindowHeight', value || '')"
                  />
                </div>
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 13 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  :title="text('Setup.Launch.Options.MsAuth.ToolTip')"
                  class="pcl-text"
                  >{{ text('Setup.Launch.Options.MsAuth.Label') }}</span
                >
                <PComboBox
                  data-control="ComboMsAuthType"
                  style="grid-row: 13 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.Launch.Options.MsAuth.ToolTip')"
                  :options="[
                    { key: '0', text: text('Setup.Launch.Options.MsAuth.WebManager') },
                    { key: '1', text: text('Setup.Launch.Options.MsAuth.DeviceCode') },
                  ]"
                  :model-value="String(values['ComboMsAuthType'])"
                  @update:model-value="(value) => change('ComboMsAuthType', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 15 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Launch.Options.IpStack.Label') }}</span
                >
                <PComboBox
                  data-control="ComboPreferredIpStack"
                  style="grid-row: 15 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.Launch.Options.IpStack.ToolTip')"
                  :options="[
                    { key: '0', text: text('Setup.Launch.Options.IpStack.Ipv4') },
                    { key: '1', text: text('Setup.Launch.Options.IpStack.JavaDefault') },
                    { key: '2', text: text('Setup.Launch.Options.IpStack.Ipv6') },
                  ]"
                  :model-value="String(values['ComboPreferredIpStack'])"
                  @update:model-value="(value) => change('ComboPreferredIpStack', value || '')"
                />
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
            <div
              style="
                margin: 0px 25px 15px 25px;
                display: grid;
                grid-template-columns: auto minmax(0, 1fr);
                grid-template-rows: 28px 9px auto 9px 28px 9px 28px auto auto auto;
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
                  { key: '0', text: text('Setup.Launch.Advanced.Renderer.GameDefault') },
                  { key: '1', text: text('Setup.Launch.Advanced.Renderer.Software') },
                  { key: '2', text: text('Setup.Launch.Advanced.Renderer.DirectX12') },
                  { key: '3', text: text('Setup.Launch.Advanced.Renderer.Vulkan') },
                ]"
                :model-value="String(values['ComboAdvanceRenderer'])"
                @update:model-value="(value) => change('ComboAdvanceRenderer', value || '')"
              />
              <div
                style="
                  margin: 0px 25px 0px 0px;
                  grid-row: 3 / span 1;
                  grid-column: 1 / span 1;
                  justify-self: start;
                  align-self: center;
                  display: flex;
                  flex-direction: row;
                  align-items: center;
                "
              >
                <span class="pcl-text">{{ text('Setup.Launch.Advanced.JvmHead') }}</span>
                <PButton
                  data-control="BtnAdvanceJvmReset"
                  style="height: 23px; margin: 1px 0px 0px 0px"
                  :title="text('Setup.Launch.Advanced.JvmHead.Restore')"
                  :click="() => act('BtnAdvanceJvmReset', 'BtnAdvanceJvmReset_Click', '', '')"
                  inline
                  ><PSvgIcon name="lucide/rotate-ccw" :size="18"
                /></PButton>
              </div>
              <PTextBox
                multiline
                data-control="TextAdvanceJvm"
                style="
                  height: 100px;
                  padding: 5px 0px;
                  grid-row: 3 / span 1;
                  grid-column: 2 / span 2;
                "
                :title="text('Setup.Launch.Advanced.JvmHead.ToolTip')"
                :placeholder="text('')"
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
                :title="text('Setup.Launch.Advanced.GameTail.ToolTip')"
                :placeholder="text('')"
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
                >{{ text('Setup.Launch.Advanced.PreLaunchCommand') }}</span
              >
              <PTextBox
                data-control="TextAdvanceRun"
                style="grid-row: 7 / span 1; grid-column: 2 / span 2"
                :title="text('Setup.Launch.Advanced.PreLaunchCommand.ToolTip')"
                :placeholder="text('')"
                :model-value="String(values['TextAdvanceRun'])"
                @update:model-value="(value) => change('TextAdvanceRun', value || '')"
              />
              <PCheckBox
                v-if="String(values.TextAdvanceRun || '').trim()"
                data-control="CheckAdvanceRunWait"
                style="grid-row: 8; grid-column: 2; margin: 8px 0 7px"
                id="PageSetupLaunch-CheckAdvanceRunWait"
                :label="text('Setup.Launch.Advanced.PreLaunchCommand.Wait')"
                :model-value="Boolean(values.CheckAdvanceRunWait)"
                @update:model-value="(value) => change('CheckAdvanceRunWait', value)"
              />
              <div
                style="
                  margin: 12px 0px 4px 0px;
                  grid-row: 9 / span 1;
                  grid-column: 1 / span 2;
                  justify-self: start;
                  display: flex;
                  flex-direction: column;
                "
              >
                <PCheckBox
                  data-control="CheckAdvanceDisableJLW"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableJlw.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableJLW"
                  :label="text('Setup.Launch.Advanced.DisableJlw')"
                  :model-value="Boolean(values['CheckAdvanceDisableJLW'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableJLW', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLF"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLF.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableLF"
                  :label="text('Setup.Launch.Advanced.DisableLF')"
                  :model-value="Boolean(values['CheckAdvanceDisableLF'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableLF', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceGraphicCard"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.GpuPreference.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceGraphicCard"
                  :label="text('Setup.Launch.Advanced.GpuPreference')"
                  :model-value="Boolean(values['CheckAdvanceGraphicCard'])"
                  @update:model-value="(value) => change('CheckAdvanceGraphicCard', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceNoJavaw"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.UseJavaExe.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceNoJavaw"
                  :label="text('Setup.Launch.Advanced.UseJavaExe')"
                  :model-value="Boolean(values['CheckAdvanceNoJavaw'])"
                  @update:model-value="(value) => change('CheckAdvanceNoJavaw', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableLwjglUnsafeAgent"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableLwjglUnsafeAgent"
                  :label="text('Setup.Launch.Advanced.DisableLwjglUnsafeAgent')"
                  :model-value="Boolean(values['CheckAdvanceDisableLwjglUnsafeAgent'])"
                  @update:model-value="
                    (value) => change('CheckAdvanceDisableLwjglUnsafeAgent', value)
                  "
                />
                <PCheckBox
                  data-control="CheckAdvanceDisableCrashAnalysis"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.DisableCrashAnalysis.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceDisableCrashAnalysis"
                  :label="text('Setup.Launch.Advanced.DisableCrashAnalysis')"
                  :model-value="Boolean(values['CheckAdvanceDisableCrashAnalysis'])"
                  @update:model-value="(value) => change('CheckAdvanceDisableCrashAnalysis', value)"
                />
                <PCheckBox
                  data-control="CheckAdvanceLockMemory"
                  style="height: 28px"
                  :title="text('Setup.Launch.Advanced.LockMemory.ToolTip')"
                  id="PageSetupLaunch-CheckAdvanceLockMemory"
                  :label="text('Setup.Launch.Advanced.LockMemory')"
                  :model-value="Boolean(values['CheckAdvanceLockMemory'])"
                  @update:model-value="(value) => change('CheckAdvanceLockMemory', value)"
                />
              </div>
              <PHint
                style="margin: 5px 0px 10px 0px; grid-row: 10 / span 1; grid-column: 1 / span 2"
                severity="info"
                closable
                >{{ text('Setup.Launch.Advanced.MoreInInstanceSetup') }}</PHint
              >
            </div>
          </PCard>
          <PButton
            data-control="BtnSwitch"
            style="margin: -5px 0px 15px 0px"
            :click="() => act('BtnSwitch', 'BtnSwitch_Click', '', '')"
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
