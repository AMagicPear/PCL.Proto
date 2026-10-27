<script setup lang="ts">
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
const { values, text, change, act, image } = usePageSettings('PageSetupLauncherMisc', {
  ComboSystemActivity: '0',
  SliderAniFPS: 59.0,
  SliderMaxLog: 13.0,
  CheckSystemDisableHardwareAcceleration: false,
  CheckSystemTelemetry: false,
  CheckNetDohEnable: false,
  SystemHttpProxyType: '0',
  TextSystemHttpProxy: '',
  TextSystemHttpProxyCustomUsername: '',
  TextSystemHttpProxyCustomPassword: '',
  SliderDebugAnim: 0.0,
  CheckDebugSkipCopy: false,
  CheckDebugMode: false,
  CheckDebugDelay: false,
})
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 10px 25px; display: flex; flex-direction: column"
        >
          <PCard style="margin: 15px 0px" :title="text('Setup.Misc.System.Title')">
            <div
              style="
                margin: 0px 25px 16px 25px;
                display: grid;
                grid-template-columns: auto minmax(0, 1fr);
                grid-template-rows: 28px 9px 28px 5px 28px 5px 20px 8px auto;
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
                >{{ text('Setup.Misc.System.Announcement') }}</span
              >
              <PComboBox
                data-control="ComboSystemActivity"
                style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                :options="[
                  { key: '0', text: text('Setup.Misc.System.Announcement.All'), title: text('Setup.Misc.System.Announcement.All.ToolTip') },
                  { key: '1', text: text('Setup.Misc.System.Announcement.ImportantOnly') },
                  { key: '2', text: text('Setup.Misc.System.Announcement.Disabled') },
                ]"
                :model-value="String(values['ComboSystemActivity'])"
                @update:model-value="(value) => change('ComboSystemActivity', value || '')"
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
                >{{ text('Setup.Misc.System.MaxFps') }}</span
              >
              <input
                data-control="SliderAniFPS"
                style="grid-row: 3 / span 1; grid-column: 2 / span 1"
                :title="text('Setup.Misc.System.MaxFps.ToolTip')"
                type="range"
                min="0"
                max="59"
                v-model.number="values['SliderAniFPS']"
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
                >{{ text('Setup.Misc.System.MaxLogLines') }}</span
              >
              <input
                data-control="SliderMaxLog"
                style="grid-row: 5 / span 1; grid-column: 2 / span 1"
                :title="text('Setup.Misc.System.MaxLogLines.ToolTip')"
                type="range"
                min="0"
                max="29"
                v-model.number="values['SliderMaxLog']"
              />
              <div
                style="
                  grid-row: 7 / span 1;
                  grid-column: 1 / span 5;
                  display: grid;
                  grid-template-columns: auto auto auto auto;
                  grid-template-rows: 20px;
                "
              >
                <PCheckBox
                  data-control="CheckSystemDisableHardwareAcceleration"
                  style="grid-row: 1 / span 1; grid-column: 1 / span 1"
                  id="PageSetupLauncherMisc-CheckSystemDisableHardwareAcceleration"
                  :label="text('Setup.Misc.System.DisableHardwareAcceleration')"
                  :model-value="Boolean(values['CheckSystemDisableHardwareAcceleration'])"
                  @update:model-value="
                    (value) => change('CheckSystemDisableHardwareAcceleration', value)
                  "
                />
                <PCheckBox
                  data-control="CheckSystemTelemetry"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.Misc.System.Telemetry.ToolTip')"
                  id="PageSetupLauncherMisc-CheckSystemTelemetry"
                  :label="text('Setup.Misc.System.Telemetry')"
                  :model-value="Boolean(values['CheckSystemTelemetry'])"
                  @update:model-value="(value) => change('CheckSystemTelemetry', value)"
                />
              </div>
              <div
                style="
                  grid-row: 9 / span 1;
                  grid-column: 1 / span 5;
                  display: grid;
                  grid-template-columns: auto auto auto auto;
                  grid-template-rows: 35px;
                "
              >
                <PButton
                  data-control="BtnSystemSettingExp"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :click="() => act('BtnSystemSettingExp', 'BtnSystemSettingExp_Click', '', '')"
                  >{{ text('Setup.Misc.System.ExportSettings') }}</PButton
                >
                <PButton
                  data-control="BtnSystemSettingImp"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  :click="() => act('BtnSystemSettingImp', 'BtnSystemSettingImp_Click', '', '')"
                  >{{ text('Setup.Misc.System.ImportSettings') }}</PButton
                >
                <PButton
                  data-control="BtnSystemStopUsingPclCe"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 3 / span 1;
                  "
                  :click="
                    () => act('BtnSystemStopUsingPclCe', 'BtnSystemStopUsingPclCe_Click', '', '')
                  "
                  type="warn"
                  >{{ text('Setup.Misc.System.StopUsingPclCe') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Setup.Misc.Network.Title')">
            <div
              style="
                margin: 0px 25px 15px 25px;
                display: grid;
                grid-template-columns: auto minmax(0, 1fr);
                grid-template-rows: 28px 4px 28px 4px auto;
              "
            >
              <PCheckBox
                data-control="CheckNetDohEnable"
                style="grid-row: 1 / span 1; grid-column: 1 / span 2"
                :title="text('Setup.Misc.Network.DoH.ToolTip')"
                id="PageSetupLauncherMisc-CheckNetDohEnable"
                :label="text('Setup.Misc.Network.DoH')"
                :model-value="Boolean(values['CheckNetDohEnable'])"
                @update:model-value="(value) => change('CheckNetDohEnable', value)"
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
                >{{ text('Setup.Misc.Network.HttpProxy') }}</span
              >
              <div
                style="
                  grid-row: 3 / span 1;
                  grid-column: 2 / span 1;
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(
                      0,
                      0.2fr
                    );
                "
              >
                <label
                  data-control="RadioHttpProxyType0"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 1 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['SystemHttpProxyType']" value="0" />{{
                    text('Setup.Misc.Network.Proxy.None')
                  }}</label
                >
                <label
                  data-control="RadioHttpProxyType1"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['SystemHttpProxyType']" value="1" />{{
                    text('Setup.Misc.Network.Proxy.System')
                  }}</label
                >
                <label
                  data-control="RadioHttpProxyType2"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['SystemHttpProxyType']" value="2" />{{
                    text('Setup.Misc.Network.Proxy.Custom')
                  }}</label
                >
              </div>
              <div
                data-control="HttpProxyCustom"
                style="
                  grid-row: 5 / span 1;
                  grid-column: 2 / span 1;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr) auto minmax(0, 1fr) auto;
                  grid-template-rows: 28px 9px 28px 9px 28px;
                "
              >
                <PHint style="grid-row: 1 / span 1; grid-column: 1 / span 5" severity="warning">{{
                  text('Setup.Misc.Network.Proxy.UntrustedWarning')
                }}</PHint>
                <PTextBox
                  data-control="TextSystemHttpProxy"
                  style="grid-row: 3 / span 1; grid-column: 1 / span 5"
                  :title="text('Setup.Misc.Network.Proxy.InvalidWarning')"
                  :placeholder="text('比如 http://127.0.0.1:1080/')"
                  :model-value="String(values['TextSystemHttpProxy'])"
                  @update:model-value="(value) => change('TextSystemHttpProxy', value || '')"
                />
                <span
                  style="
                    margin: 0px 5px 0px 0px;
                    grid-row: 5 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Misc.Network.Proxy.Username') }}</span
                >
                <PTextBox
                  data-control="TextSystemHttpProxyCustomUsername"
                  style="margin: 0px 5px 0px 0px; grid-row: 5 / span 1; grid-column: 2 / span 1"
                  :placeholder="text('Setup.Misc.Network.Proxy.Optional')"
                  :model-value="String(values['TextSystemHttpProxyCustomUsername'])"
                  @update:model-value="
                    (value) => change('TextSystemHttpProxyCustomUsername', value || '')
                  "
                />
                <span
                  style="
                    margin: 0px 5px 0px 0px;
                    grid-row: 5 / span 1;
                    grid-column: 3 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Misc.Network.Proxy.Password') }}</span
                >
                <PTextBox
                  type="password"
                  data-control="TextSystemHttpProxyCustomPassword"
                  style="margin: 0px 5px 0px 0px; grid-row: 5 / span 1; grid-column: 4 / span 1"
                  :placeholder="text('Setup.Misc.Network.Proxy.Optional')"
                  :model-value="String(values['TextSystemHttpProxyCustomPassword'])"
                  @update:model-value="
                    (value) => change('TextSystemHttpProxyCustomPassword', value || '')
                  "
                />
                <PButton
                  data-control="BtnApplyHttpProxy"
                  style="grid-row: 5 / span 1; grid-column: 5 / span 1"
                  :click="() => act('BtnApplyHttpProxy', 'ApplyHttpProxyBtn_OnClicked', '', '')"
                  >{{ text('Setup.Misc.Network.Proxy.Apply') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardDebug"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Misc.Debug.Title')"
            default-fold-status="fold"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                data-control="PanDebugAnim"
                style="
                  height: 22px;
                  margin: 0px 0px 10px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
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
                  >{{ text('Setup.Misc.Debug.AnimationSpeed') }}</span
                >
                <input
                  data-control="SliderDebugAnim"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                  type="range"
                  min="0"
                  max="30"
                  v-model.number="values['SliderDebugAnim']"
                />
              </div>
              <div
                style="
                  display: grid;
                  grid-template-columns: minmax(0, 1.5fr) minmax(0, 1fr) minmax(0, 1fr);
                "
              >
                <PCheckBox
                  data-control="CheckDebugSkipCopy"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 1 / span 1"
                  :title="text('Setup.Misc.Debug.SkipCopy.ToolTip')"
                  id="PageSetupLauncherMisc-CheckDebugSkipCopy"
                  :label="text('Setup.Misc.Debug.SkipCopy')"
                  :model-value="Boolean(values['CheckDebugSkipCopy'])"
                  @update:model-value="(value) => change('CheckDebugSkipCopy', value)"
                />
                <PCheckBox
                  data-control="CheckDebugMode"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.Misc.Debug.Mode.ToolTip')"
                  id="PageSetupLauncherMisc-CheckDebugMode"
                  :label="text('Setup.Misc.Debug.Mode')"
                  :model-value="Boolean(values['CheckDebugMode'])"
                  @update:model-value="(value) => change('CheckDebugMode', value)"
                />
                <PCheckBox
                  data-control="CheckDebugDelay"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                  :title="text('Setup.Misc.Debug.Delay.ToolTip')"
                  id="PageSetupLauncherMisc-CheckDebugDelay"
                  :label="text('Setup.Misc.Debug.Delay')"
                  :model-value="Boolean(values['CheckDebugDelay'])"
                  @update:model-value="(value) => change('CheckDebugDelay', value)"
                />
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
