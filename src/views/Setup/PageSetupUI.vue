<script setup lang="ts">
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
withDefaults(defineProps<{ hasBackground?: boolean; hasMusic?: boolean }>(), {
  hasBackground: false,
  hasMusic: false,
})
const { values, text, change, act, image } = usePageSettings('PageSetupUI', {
  ComboUiFont: 'system-ui',
  ComboUiMotdFont: 'system-ui',
  CheckLogoLeft: false,
  ComboBackgroundSuit: '0',
  SliderBackgroundOpacity: 1000,
  SliderBackgroundBlur: 0,
  SliderMusicVolume: 1000,
  CheckMusicRandom: false,
  CheckMusicAuto: false,
  CheckMusicStart: false,
  CheckMusicStop: false,
  CheckMusicSMTC: false,
  SliderLauncherOpacity: 0.0,
  ComboDarkMode: '0',
  ComboLightColor: '0',
  ComboDarkColor: '0',
  CheckLauncherLogo: false,
  CheckLockWindowSize: false,
  CheckShowLaunchingHint: false,
  CheckBlur: false,
  SliderBlurValue: 0.0,
  SliderBlurSamplingRate: 0.0,
  ComboBlurType: '0',
  CheckAutoPauseVideo: false,
  CheckBackgroundColorful: false,
  UiLogoType: '0',
  TextLogoText: '',
  UiCustomType: '0',
  TextCustomNet: '',
  ComboCustomPreset: '0',
  CheckHiddenPageDownload: false,
  CheckHiddenPageSetup: false,
  CheckHiddenPageTools: false,
  CheckHiddenSetupLaunch: false,
  CheckHiddenSetupJava: false,
  CheckHiddenSetupGameManage: false,
  CheckHiddenSetupGameLink: false,
  CheckHiddenSetupUI: false,
  CheckHiddenSetupLauncherLanguage: false,
  CheckHiddenLauncherMisc: false,
  CheckHiddenSetupUpdate: false,
  CheckHiddenSetupAbout: false,
  CheckHiddenSetupFeedback: false,
  CheckHiddenSetupLog: false,
  CheckHiddenToolsGameLink: false,
  CheckHiddenToolsTest: false,
  CheckHiddenVersionEdit: false,
  CheckHiddenVersionExport: false,
  CheckHiddenVersionSave: false,
  CheckHiddenVersionScreenshot: false,
  CheckHiddenVersionMod: false,
  CheckHiddenVersionResourcePack: false,
  CheckHiddenVersionShader: false,
  CheckHiddenVersionSchematic: false,
  CheckHiddenVersionServer: false,
  CheckHiddenFunctionSelect: false,
  CheckHiddenFunctionModUpdate: false,
  CheckHiddenFunctionHidden: false,
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
          <PCard
            data-control="CardLauncher"
            style="margin: 15px 0px"
            :title="text('Setup.Ui.Basic.Title')"
          >
            <div style="margin: 0px 25px 10px 25px; display: grid">
              <div
                style="
                  grid-row: 1 / span 1;
                  grid-column: 1 / span 1;
                  display: flex;
                  flex-direction: column;
                "
              >
                <div
                  style="
                    margin: 0px 0px 6px 0px;
                    display: grid;
                    grid-template-columns: auto minmax(0, 1fr) 30px auto minmax(0, 1fr);
                    grid-template-rows: auto auto auto;
                  "
                >
                  <span
                    style="
                      margin: 0px 20px 0px 0px;
                      grid-row: 1 / span 1;
                      grid-column: 1 / span 1;
                      justify-self: start;
                      align-self: center;
                    "
                    class="pcl-text"
                    >{{ text('Setup.Ui.Basic.Opacity') }}</span
                  >
                  <input
                    data-control="SliderLauncherOpacity"
                    style="grid-row: 1 / span 1; grid-column: 2 / span 5"
                    type="range"
                    min="0"
                    max="600"
                    v-model.number="values['SliderLauncherOpacity']"
                  />
                </div>
                <div
                  style="
                    height: 28px;
                    margin: 0px 0px 7px 0px;
                    display: grid;
                    grid-template-columns: auto minmax(0, 1fr) 92px minmax(0, 1fr) minmax(
                        0,
                        1fr
                      ) minmax(0, 1fr);
                  "
                >
                  <span
                    style="
                      grid-row: 1 / span 1;
                      grid-column: 1 / span 1;
                      justify-self: start;
                      align-self: center;
                    "
                    class="pcl-text"
                    >{{ text('Setup.Ui.Theme.Title') }}</span
                  >
                  <PComboBox
                    data-control="ComboDarkMode"
                    style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                    :options="[
                      { key: '0', text: text('Setup.Ui.Theme.Light') },
                      { key: '1', text: text('Setup.Ui.Theme.Dark') },
                      { key: '2', text: text('Setup.Ui.Theme.FollowSystem') },
                    ]"
                    :model-value="String(values['ComboDarkMode'])"
                    @update:model-value="(value) => change('ComboDarkMode', value || '')"
                  />
                  <span
                    style="
                      grid-row: 1 / span 1;
                      grid-column: 3 / span 1;
                      justify-self: center;
                      align-self: center;
                    "
                    class="pcl-text"
                    >{{ text('Setup.Ui.Theme.LightColor') }}</span
                  >
                  <PComboBox
                    data-control="ComboLightColor"
                    style="grid-row: 1 / span 1; grid-column: 4 / span 1"
                    :options="
                      ['SkyBlue', 'CatBlue', 'CrashBlue'].map((color, index) => ({
                        key: String(index),
                        text: text(`Setup.Ui.Theme.Color.${color}`),
                      }))
                    "
                    :model-value="String(values['ComboLightColor'])"
                    @update:model-value="(value) => change('ComboLightColor', value || '')"
                  />
                  <span
                    style="
                      grid-row: 1 / span 1;
                      grid-column: 5 / span 1;
                      justify-self: center;
                      align-self: center;
                    "
                    class="pcl-text"
                    >{{ text('Setup.Ui.Theme.DarkColor') }}</span
                  >
                  <PComboBox
                    data-control="ComboDarkColor"
                    style="grid-row: 1 / span 1; grid-column: 6 / span 1"
                    :options="
                      ['SkyBlue', 'CatBlue', 'CrashBlue'].map((color, index) => ({
                        key: String(index),
                        text: text(`Setup.Ui.Theme.Color.${color}`),
                      }))
                    "
                    :model-value="String(values['ComboDarkColor'])"
                    @update:model-value="(value) => change('ComboDarkColor', value || '')"
                  />
                </div>
                <PCheckBox
                  data-control="CheckLauncherLogo"
                  style="height: 22px; margin: 0px 0px 7px -1px"
                  id="PageSetupUI-CheckLauncherLogo"
                  :label="text('Setup.Ui.Basic.ShowLogo')"
                  :model-value="Boolean(values['CheckLauncherLogo'])"
                  @update:model-value="(value) => change('CheckLauncherLogo', value)"
                />
                <PCheckBox
                  data-control="CheckLockWindowSize"
                  style="height: 22px; margin: 0px 0px 7px -1px"
                  id="PageSetupUI-CheckLockWindowSize"
                  :label="text('Setup.Ui.Basic.LockWindow')"
                  :model-value="Boolean(values['CheckLockWindowSize'])"
                  @update:model-value="(value) => change('CheckLockWindowSize', value)"
                />
                <PCheckBox
                  data-control="CheckShowLaunchingHint"
                  style="height: 22px; margin: 0px 0px 7px -1px"
                  id="PageSetupUI-CheckShowLaunchingHint"
                  :label="text('Setup.Ui.Basic.ShowTriviaHint')"
                  :model-value="Boolean(values['CheckShowLaunchingHint'])"
                  @update:model-value="(value) => change('CheckShowLaunchingHint', value)"
                />
                <div
                  data-control="PanelBlur"
                  style="margin: 0px 0px 10px 0px; display: flex; flex-direction: column"
                >
                  <PCheckBox
                    data-control="CheckBlur"
                    style="height: 22px"
                    :title="text('Setup.Ui.Basic.AdvancedMaterial.ToolTip')"
                    id="PageSetupUI-CheckBlur"
                    :label="text('Setup.Ui.Basic.AdvancedMaterial')"
                    :model-value="Boolean(values['CheckBlur'])"
                    @update:model-value="(value) => change('CheckBlur', value)"
                  />
                  <div
                    v-if="values.CheckBlur"
                    data-control="PanBlurValue"
                    style="
                      margin: 10px 0px 0px 0px;
                      display: grid;
                      grid-template-columns: auto auto auto minmax(0, 1fr);
                      grid-template-rows: minmax(0, 1fr) 10px minmax(0, 1fr);
                    "
                  >
                    <span
                      style="
                        grid-row: 1 / span 1;
                        grid-column: 1 / span 1;
                        justify-self: start;
                        align-self: center;
                      "
                      class="pcl-text"
                      >{{ text('Setup.Ui.Basic.BlurRadius') }}</span
                    >
                    <input
                      data-control="SliderBlurValue"
                      style="grid-row: 1 / span 1; grid-column: 2 / span 3"
                      :title="text('Setup.Ui.Basic.BlurRadius.ToolTip')"
                      type="range"
                      min="0"
                      max="40"
                      v-model.number="values['SliderBlurValue']"
                    />
                    <span
                      style="
                        grid-row: 3 / span 1;
                        grid-column: 3 / span 1;
                        justify-self: center;
                        align-self: center;
                      "
                      class="pcl-text"
                      >{{ text('Setup.Ui.Basic.BlurSamplingRate') }}</span
                    >
                    <input
                      data-control="SliderBlurSamplingRate"
                      style="grid-row: 3 / span 1; grid-column: 4 / span 1"
                      :title="text('Setup.Ui.Basic.BlurSamplingRate.ToolTip')"
                      type="range"
                      min="0"
                      max="100"
                      v-model.number="values['SliderBlurSamplingRate']"
                    />
                    <span
                      style="
                        grid-row: 3 / span 1;
                        grid-column: 1 / span 1;
                        justify-self: start;
                        align-self: center;
                      "
                      class="pcl-text"
                      >{{ text('Setup.Ui.Basic.BlurMethod') }}</span
                    >
                    <PComboBox
                      data-control="ComboBlurType"
                      style="
                        height: 28px;
                        min-width: 100px;
                        grid-row: 3 / span 1;
                        grid-column: 2 / span 1;
                      "
                      :title="text('Setup.Ui.Basic.BlurMethod.ToolTip')"
                      :options="[
                        { key: '0', text: text('Setup.Ui.Basic.BlurMethod.Gaussian'), title: text('Setup.Ui.Basic.BlurMethod.Gaussian.ToolTip') },
                        { key: '1', text: text('Setup.Ui.Basic.BlurMethod.Box'), title: text('Setup.Ui.Basic.BlurMethod.Box.ToolTip') },
                      ]"
                      :model-value="String(values['ComboBlurType'])"
                      @update:model-value="(value) => change('ComboBlurType', value || '')"
                    />
                  </div>
                </div>
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardFont"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.Font.Title')"
          >
            <div
              style="
                margin: 0px 25px 15px 25px;
                display: grid;
                grid-template-columns: auto 25px minmax(0, 1fr);
                grid-template-rows: 30px 10px minmax(0, 1fr);
              "
            >
              <span
                style="grid-row: 1 / span 1; grid-column: 1 / span 1; align-self: center"
                class="pcl-text"
                >{{ text('Setup.Ui.Font.Global') }}</span
              >
              <PComboBox
                data-control="ComboUiFont"
                style="grid-row: 1 / span 1; grid-column: 3 / span 1"
                :options="[
                  { key: 'system-ui', text: '跟随系统' },
                  { key: 'Microsoft YaHei UI', text: 'Microsoft YaHei UI' },
                  { key: 'PingFang SC', text: 'PingFang SC' },
                ]"
                :model-value="String(values.ComboUiFont)"
                @update:model-value="(value) => change('ComboUiFont', value || 'system-ui')"
              />
              <span
                style="grid-row: 3 / span 1; grid-column: 1 / span 1; align-self: center"
                class="pcl-text"
                >{{ text('Setup.Ui.Font.Motd') }}</span
              >
              <PComboBox
                data-control="ComboUiMotdFont"
                style="grid-row: 3 / span 1; grid-column: 3 / span 1"
                :options="[
                  { key: 'system-ui', text: '跟随系统' },
                  { key: 'Microsoft YaHei UI', text: 'Microsoft YaHei UI' },
                  { key: 'PingFang SC', text: 'PingFang SC' },
                ]"
                :model-value="String(values.ComboUiMotdFont)"
                @update:model-value="(value) => change('ComboUiMotdFont', value || 'system-ui')"
              />
            </div>
          </PCard>
          <PCard
            data-control="CardBackground"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.Background.TitleDefault')"
          >
            <div style="margin: 0px 25px 20px 25px; display: flex; flex-direction: column">
              <template v-if="hasBackground">
                <div data-control="PanBackgroundSuit" class="pcl-form-row">
                  <label>{{ text('Setup.Ui.Background.ContentAdapt') }}</label
                  ><PComboBox
                    data-control="ComboBackgroundSuit"
                    :options="
                      [
                        'Smart',
                        'Center',
                        'Fit',
                        'Stretch',
                        'Tile',
                        'TopLeft',
                        'TopRight',
                        'BottomLeft',
                        'BottomRight',
                      ].map((key, index) => ({
                        key: String(index),
                        text: text(`Setup.Ui.Background.Suit.${key}`),
                      }))
                    "
                    :model-value="String(values.ComboBackgroundSuit)"
                    @update:model-value="(value) => change('ComboBackgroundSuit', value || '0')"
                  />
                </div>
                <div data-control="PanBackgroundOpacity" class="pcl-form-row">
                  <label for="background-opacity">{{ text('Setup.Ui.Basic.Opacity2') }}</label
                  ><input
                    id="background-opacity"
                    type="range"
                    min="0"
                    max="1000"
                    v-model.number="values.SliderBackgroundOpacity"
                  />
                </div>
                <div data-control="PanBackgroundBlur" class="pcl-form-row">
                  <label for="background-blur">{{ text('Setup.Ui.Background.Blur') }}</label
                  ><input
                    id="background-blur"
                    type="range"
                    min="0"
                    max="40"
                    v-model.number="values.SliderBackgroundBlur"
                  />
                </div>
              </template>
              <PCheckBox
                data-control="CheckAutoPauseVideo"
                style="height: 22px; margin: 0px 0px 8px -1px"
                :title="text('Setup.Ui.Background.PauseVideo.ToolTip')"
                id="PageSetupUI-CheckAutoPauseVideo"
                :label="text('Setup.Ui.Background.PauseVideo')"
                :model-value="Boolean(values['CheckAutoPauseVideo'])"
                @update:model-value="(value) => change('CheckAutoPauseVideo', value)"
              />
              <PCheckBox
                data-control="CheckBackgroundColorful"
                style="height: 22px; margin: 0px 0px 8px -1px"
                id="PageSetupUI-CheckBackgroundColorful"
                :label="text('Setup.Ui.Background.ColorOverlay')"
                :model-value="Boolean(values['CheckBackgroundColorful'])"
                @update:model-value="(value) => change('CheckBackgroundColorful', value)"
              />
              <div
                style="
                  height: 35px;
                  margin: 2px 0px 0px 0px;
                  display: grid;
                  grid-template-columns: auto auto auto;
                "
              >
                <PButton
                  data-control="BtnBackgroundOpen"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :title="text('Setup.Ui.Background.OpenFolder.ToolTip')"
                  :click="() => act('BtnBackgroundOpen', 'BtnUIBgOpen_Click', '', '')"
                  >{{ text('Common.Action.OpenFolder') }}</PButton
                >
                <PButton
                  data-control="BtnBackgroundRefresh"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  :title="text('Setup.Ui.Background.Refresh.ToolTip')"
                  :click="() => act('BtnBackgroundRefresh', 'BtnBackgroundRefresh_Click', '', '')"
                  >{{ text('Setup.Ui.Background.Refresh') }}</PButton
                ><PButton
                  v-if="hasBackground"
                  data-control="BtnBackgroundClear"
                  type="warn"
                  :click="() => act('BtnBackgroundClear')"
                  >{{ text('Setup.Ui.Background.Clear') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardMusic"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.Music.Title')"
          >
            <div style="margin: 2px 25px 20px 25px; display: flex; flex-direction: column">
              <template v-if="hasMusic">
                <div data-control="PanMusicVolume" class="pcl-form-row">
                  <label for="music-volume">{{ text('Setup.Ui.Music.Volume') }}</label
                  ><input
                    id="music-volume"
                    type="range"
                    min="0"
                    max="1000"
                    v-model.number="values.SliderMusicVolume"
                  />
                </div>
                <div
                  data-control="PanMusicDetail"
                  style="display: flex; flex-direction: column; gap: 5px; margin-bottom: 8px"
                >
                  <PCheckBox
                    v-for="setting in [
                      { id: 'CheckMusicRandom', key: 'Shuffle' },
                      { id: 'CheckMusicAuto', key: 'AutoStart' },
                      { id: 'CheckMusicStart', key: 'StartInGame' },
                      { id: 'CheckMusicStop', key: 'StopInGame' },
                      { id: 'CheckMusicSMTC', key: 'Smtc' },
                    ]"
                    :key="setting.id"
                    :id="`PageSetupUI-${setting.id}`"
                    :label="text(`Setup.Ui.Music.${setting.key}`)"
                    :model-value="Boolean(values[setting.id])"
                    @update:model-value="(value) => change(setting.id, value)"
                  />
                </div>
              </template>
              <div style="height: 35px; display: grid; grid-template-columns: auto auto auto">
                <PButton
                  data-control="BtnMusicOpen"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :title="text('Setup.Ui.Music.OpenFolder.ToolTip')"
                  :click="() => act('BtnMusicOpen', 'BtnMusicOpen_Click', '', '')"
                  >{{ text('Common.Action.OpenFolder') }}</PButton
                >
                <PButton
                  data-control="BtnMusicRefresh"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  :title="text('Setup.Ui.Music.Refresh.ToolTip')"
                  :click="() => act('BtnMusicRefresh', 'BtnMusicRefresh_Click', '', '')"
                  >{{ text('Setup.Ui.Music.Refresh') }}</PButton
                ><PButton
                  v-if="hasMusic"
                  data-control="BtnMusicClear"
                  type="warn"
                  :click="() => act('BtnMusicClear')"
                  >{{ text('Setup.Ui.Music.Clear') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardLogo"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.Logo.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 0px 0px 0px -1px;
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(
                      0,
                      1fr
                    ) minmax(0, 0.2fr);
                "
              >
                <label
                  data-control="RadioLogoType0"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 1 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiLogoType']" value="0" />{{
                    text('Setup.Ui.Logo.None')
                  }}</label
                >
                <label
                  data-control="RadioLogoType1"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiLogoType']" value="1" />{{
                    text('Common.Option.Default')
                  }}</label
                >
                <label
                  data-control="RadioLogoType2"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiLogoType']" value="2" />{{
                    text('Setup.Ui.Logo.Text')
                  }}</label
                >
                <label
                  data-control="RadioLogoType3"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 4 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiLogoType']" value="3" />{{
                    text('Setup.Ui.Logo.Image')
                  }}</label
                >
              </div>
              <PCheckBox
                v-if="values.UiLogoType === '0'"
                data-control="CheckLogoLeft"
                style="margin: 8px 0 3px"
                id="PageSetupUI-CheckLogoLeft"
                :label="text('Setup.Ui.Logo.LeftAlign')"
                :model-value="Boolean(values.CheckLogoLeft)"
                @update:model-value="(value) => change('CheckLogoLeft', value)"
              />
              <div
                data-control="PanLogoText"
                style="
                  margin: 14px 0px 5px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                "
                v-if="values.UiLogoType === '2'"
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
                  >{{ text('Setup.Ui.Logo.TextLabel') }}</span
                >
                <PTextBox
                  data-control="TextLogoText"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :placeholder="text('')"
                  :model-value="String(values['TextLogoText'])"
                  @update:model-value="(value) => change('TextLogoText', value || '')"
                />
              </div>
              <div
                data-control="PanLogoChange"
                style="
                  height: 35px;
                  margin: 14px 0px 5px 0px;
                  display: grid;
                  grid-template-columns: auto auto;
                "
                v-if="values.UiLogoType === '3'"
              >
                <PButton
                  data-control="BtnLogoChange"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :click="() => act('BtnLogoChange', 'BtnLogoChange_Click', '', '')"
                  >{{ text('Setup.Ui.Logo.ChangeImage') }}</PButton
                >
                <PButton
                  data-control="BtnLogoDelete"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  :click="() => act('BtnLogoDelete', 'BtnLogoDelete_Click', '', '')"
                  type="warn"
                  >{{ text('Setup.Ui.Logo.ClearImage') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardCustom"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.Homepage.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 0px 0px 0px -1px;
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1fr) minmax(
                      0,
                      1fr
                    ) minmax(0, 0.2fr);
                "
              >
                <label
                  data-control="RadioCustomType0"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 1 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiCustomType']" value="0" />{{
                    text('Setup.Ui.Homepage.Blank')
                  }}</label
                >
                <label
                  data-control="RadioCustomType3"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiCustomType']" value="3" />{{
                    text('Setup.Ui.Homepage.Preset')
                  }}</label
                >
                <label
                  data-control="RadioCustomType1"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiCustomType']" value="1" />{{
                    text('Setup.Ui.Homepage.LocalFile')
                  }}</label
                >
                <label
                  data-control="RadioCustomType2"
                  style="height: 22px; grid-row: 1 / span 1; grid-column: 4 / span 1"
                  class="pcl-radio"
                  ><input type="radio" v-model="values['UiCustomType']" value="2" />{{
                    text('Setup.Ui.Homepage.NetUpdate')
                  }}</label
                >
              </div>
              <PHint data-control="HintCustom" style="margin: 15px 0px -4px 0px" severity="info">{{
                text('Setup.Ui.Homepage.HintReferToModSetup')
              }}</PHint>
              <PHint
                v-if="values.UiCustomType === '2'"
                data-control="HintCustomWarn"
                style="margin: 15px 0px 6px 0px"
                severity="warning"
                closable
                >{{ text('Setup.Ui.Homepage.NetWarning') }}</PHint
              >
              <div
                data-control="PanCustomLocal"
                style="
                  height: 35px;
                  margin: 10px 0px 5px 0px;
                  display: grid;
                  grid-template-columns: auto auto auto;
                "
                v-if="values.UiCustomType === '1'"
              >
                <PButton
                  data-control="BtnCustomRefresh"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                  "
                  :click="() => act('BtnCustomRefresh', 'BtnCustomRefresh_Click', '', '')"
                  type="tint"
                  >{{ text('Setup.Ui.Homepage.Refresh') }}</PButton
                >
                <PButton
                  data-control="BtnCustomTutorial"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                  "
                  :click="() => act('BtnCustomTutorial', 'BtnCustomTutorial_Click', '', '')"
                  >{{ text('Setup.Ui.Homepage.Tutorial') }}</PButton
                >
              </div>
              <div
                data-control="PanCustomNet"
                style="
                  margin: 10px 0px 5px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                "
                v-if="values.UiCustomType === '2'"
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
                  >{{ text('Setup.Ui.Homepage.DownloadUrl') }}</span
                >
                <PTextBox
                  data-control="TextCustomNet"
                  style="max-height: 28px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :placeholder="text('')"
                  :model-value="String(values['TextCustomNet'])"
                  @update:model-value="(value) => change('TextCustomNet', value || '')"
                />
              </div>
              <div
                data-control="PanCustomPreset"
                style="
                  height: 28px;
                  margin: 12px 10px 5px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                "
                v-if="values.UiCustomType === '3'"
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
                  >{{ text('Setup.Ui.Homepage.PresetLabel') }}</span
                >
                <PComboBox
                  data-control="ComboCustomPreset"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.Ui.Homepage.Preset.Trivia') },
                    { key: '1', text: text('Setup.Ui.Homepage.Preset.McNews') },
                    { key: '2', text: text('Setup.Ui.Homepage.Preset.DailyModpack') },
                    { key: '3', text: text('Setup.Ui.Homepage.Preset.McSkin') },
                    { key: '4', text: text('Setup.Ui.Homepage.Preset.OpenBmclapi') },
                    { key: '5', text: text('Setup.Ui.Homepage.Preset.PclManual') },
                    { key: '6', text: text('Setup.Ui.Homepage.Preset.Magazine') },
                    { key: '7', text: text('Setup.Ui.Homepage.Preset.GitHub') },
                    { key: '8', text: text('Setup.Ui.Homepage.Preset.McUpdateSummary') },
                    { key: '9', text: text('Setup.Ui.Homepage.Preset.NewsToday') },
                    { key: '10', text: text('Setup.Ui.Homepage.Preset.MinecraftKnowledge') },
                    { key: '11', text: text('Setup.Ui.Homepage.Preset.ModpackRecommendation') },
                    { key: '12', text: text('Setup.Ui.Homepage.Preset.Bangumi') },
                    { key: '13', text: text('Setup.Ui.Homepage.Preset.PclAnnouncement') },
                    { key: '14', text: text('Setup.Ui.Homepage.Preset.McOfficialFeed') },
                  ]"
                  :model-value="String(values['ComboCustomPreset'])"
                  @update:model-value="(value) => change('ComboCustomPreset', value || '')"
                />
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardSwitch"
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.Ui.FeatureHide.Title')"
          >
            <div style="margin: 0px 15px 15px 25px; display: flex; flex-direction: column">
              <span style="margin: 1px 0px 1px 0px" class="pcl-text">{{
                text('Setup.Ui.FeatureHide.Description')
              }}</span>
              <div
                style="
                  display: grid;
                  grid-template-columns: auto 18px minmax(0, 0.8fr) minmax(0, 0.9fr) minmax(
                      0,
                      0.8fr
                    ) minmax(0, 0.8fr) minmax(0, 1fr) minmax(0, 1fr);
                  grid-template-rows: 4px 30px 30px 30px 30px 30px 30px 30px auto;
                "
              >
                <span
                  style="
                    grid-row: 2 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Ui.FeatureHide.MainPage') }}</span
                >
                <PCheckBox
                  data-control="CheckHiddenPageDownload"
                  style="
                    height: 22px;
                    grid-row: 2 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenPageDownload"
                  :label="text('Setup.Ui.FeatureHide.Item.Download')"
                  :model-value="Boolean(values['CheckHiddenPageDownload'])"
                  @update:model-value="(value) => change('CheckHiddenPageDownload', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenPageSetup"
                  style="
                    height: 22px;
                    grid-row: 2 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenPageSetup"
                  :label="text('Setup.Ui.FeatureHide.Item.Settings')"
                  :model-value="Boolean(values['CheckHiddenPageSetup'])"
                  @update:model-value="(value) => change('CheckHiddenPageSetup', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenPageTools"
                  style="
                    height: 22px;
                    grid-row: 2 / span 1;
                    grid-column: 5 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenPageTools"
                  :label="text('Setup.Ui.FeatureHide.Item.Tools')"
                  :model-value="Boolean(values['CheckHiddenPageTools'])"
                  @update:model-value="(value) => change('CheckHiddenPageTools', value)"
                />
                <span
                  style="
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Ui.FeatureHide.SubSetup') }}</span
                >
                <PCheckBox
                  data-control="CheckHiddenSetupLaunch"
                  style="
                    height: 22px;
                    grid-row: 3 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupLaunch"
                  :label="text('Setup.Ui.FeatureHide.Item.Launch')"
                  :model-value="Boolean(values['CheckHiddenSetupLaunch'])"
                  @update:model-value="(value) => change('CheckHiddenSetupLaunch', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupJava"
                  style="
                    height: 22px;
                    grid-row: 3 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupJava"
                  :label="text('Java')"
                  :model-value="Boolean(values['CheckHiddenSetupJava'])"
                  @update:model-value="(value) => change('CheckHiddenSetupJava', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupGameManage"
                  style="
                    height: 22px;
                    grid-row: 3 / span 1;
                    grid-column: 5 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupGameManage"
                  :label="text('Setup.Ui.FeatureHide.Item.GameManage')"
                  :model-value="Boolean(values['CheckHiddenSetupGameManage'])"
                  @update:model-value="(value) => change('CheckHiddenSetupGameManage', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupGameLink"
                  style="
                    height: 22px;
                    grid-row: 3 / span 1;
                    grid-column: 6 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupGameLink"
                  :label="text('Setup.Ui.FeatureHide.Item.GameLink')"
                  :model-value="Boolean(values['CheckHiddenSetupGameLink'])"
                  @update:model-value="(value) => change('CheckHiddenSetupGameLink', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupUI"
                  style="
                    height: 22px;
                    grid-row: 3 / span 1;
                    grid-column: 7 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupUI"
                  :label="text('Setup.Ui.FeatureHide.Item.Ui')"
                  :model-value="Boolean(values['CheckHiddenSetupUI'])"
                  @update:model-value="(value) => change('CheckHiddenSetupUI', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupLauncherLanguage"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupLauncherLanguage"
                  :label="text('Setup.Ui.FeatureHide.Item.Language')"
                  :model-value="Boolean(values['CheckHiddenSetupLauncherLanguage'])"
                  @update:model-value="(value) => change('CheckHiddenSetupLauncherLanguage', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenLauncherMisc"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenLauncherMisc"
                  :label="text('Setup.Ui.FeatureHide.Item.Misc')"
                  :model-value="Boolean(values['CheckHiddenLauncherMisc'])"
                  @update:model-value="(value) => change('CheckHiddenLauncherMisc', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupUpdate"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 5 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupUpdate"
                  :label="text('Setup.Ui.FeatureHide.Item.Update')"
                  :model-value="Boolean(values['CheckHiddenSetupUpdate'])"
                  @update:model-value="(value) => change('CheckHiddenSetupUpdate', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupAbout"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 6 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupAbout"
                  :label="text('Setup.Ui.FeatureHide.Item.About')"
                  :model-value="Boolean(values['CheckHiddenSetupAbout'])"
                  @update:model-value="(value) => change('CheckHiddenSetupAbout', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupFeedback"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 7 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupFeedback"
                  :label="text('Setup.Ui.FeatureHide.Item.Feedback')"
                  :model-value="Boolean(values['CheckHiddenSetupFeedback'])"
                  @update:model-value="(value) => change('CheckHiddenSetupFeedback', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenSetupLog"
                  style="
                    height: 22px;
                    grid-row: 4 / span 1;
                    grid-column: 8 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenSetupLog"
                  :label="text('Setup.Ui.FeatureHide.Item.Log')"
                  :model-value="Boolean(values['CheckHiddenSetupLog'])"
                  @update:model-value="(value) => change('CheckHiddenSetupLog', value)"
                />
                <span
                  style="
                    grid-row: 5 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Ui.FeatureHide.SubTools') }}</span
                >
                <PCheckBox
                  data-control="CheckHiddenToolsGameLink"
                  style="
                    height: 22px;
                    grid-row: 5 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenToolsGameLink"
                  :label="text('Setup.Ui.FeatureHide.Item.Lobby')"
                  :model-value="Boolean(values['CheckHiddenToolsGameLink'])"
                  @update:model-value="(value) => change('CheckHiddenToolsGameLink', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenToolsTest"
                  style="
                    height: 22px;
                    grid-row: 5 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenToolsTest"
                  :label="text('Setup.Ui.FeatureHide.Item.Toolbox')"
                  :model-value="Boolean(values['CheckHiddenToolsTest'])"
                  @update:model-value="(value) => change('CheckHiddenToolsTest', value)"
                />
                <span
                  style="
                    grid-row: 6 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Ui.FeatureHide.SubInstance') }}</span
                >
                <PCheckBox
                  data-control="CheckHiddenVersionEdit"
                  style="
                    height: 22px;
                    grid-row: 6 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionEdit"
                  :label="text('Setup.Ui.FeatureHide.Item.Edit')"
                  :model-value="Boolean(values['CheckHiddenVersionEdit'])"
                  @update:model-value="(value) => change('CheckHiddenVersionEdit', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionExport"
                  style="
                    height: 22px;
                    grid-row: 6 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionExport"
                  :label="text('Setup.Ui.FeatureHide.Item.Export')"
                  :model-value="Boolean(values['CheckHiddenVersionExport'])"
                  @update:model-value="(value) => change('CheckHiddenVersionExport', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionSave"
                  style="
                    height: 22px;
                    grid-row: 6 / span 1;
                    grid-column: 5 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionSave"
                  :label="text('Setup.Ui.FeatureHide.Item.Save')"
                  :model-value="Boolean(values['CheckHiddenVersionSave'])"
                  @update:model-value="(value) => change('CheckHiddenVersionSave', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionScreenshot"
                  style="
                    height: 22px;
                    grid-row: 6 / span 1;
                    grid-column: 6 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionScreenshot"
                  :label="text('Setup.Ui.FeatureHide.Item.Screenshot')"
                  :model-value="Boolean(values['CheckHiddenVersionScreenshot'])"
                  @update:model-value="(value) => change('CheckHiddenVersionScreenshot', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionMod"
                  style="
                    height: 22px;
                    grid-row: 6 / span 1;
                    grid-column: 7 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionMod"
                  :label="text('Mod')"
                  :model-value="Boolean(values['CheckHiddenVersionMod'])"
                  @update:model-value="(value) => change('CheckHiddenVersionMod', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionResourcePack"
                  style="
                    height: 22px;
                    grid-row: 7 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionResourcePack"
                  :label="text('Setup.Ui.FeatureHide.Item.ResourcePack')"
                  :model-value="Boolean(values['CheckHiddenVersionResourcePack'])"
                  @update:model-value="(value) => change('CheckHiddenVersionResourcePack', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionShader"
                  style="
                    height: 22px;
                    grid-row: 7 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionShader"
                  :label="text('Setup.Ui.FeatureHide.Item.Shader')"
                  :model-value="Boolean(values['CheckHiddenVersionShader'])"
                  @update:model-value="(value) => change('CheckHiddenVersionShader', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionSchematic"
                  style="
                    height: 22px;
                    grid-row: 7 / span 1;
                    grid-column: 5 / span 2;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionSchematic"
                  :label="text('Setup.Ui.FeatureHide.Item.Schematic')"
                  :model-value="Boolean(values['CheckHiddenVersionSchematic'])"
                  @update:model-value="(value) => change('CheckHiddenVersionSchematic', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenVersionServer"
                  style="
                    height: 22px;
                    grid-row: 7 / span 1;
                    grid-column: 7 / span 1;
                    align-self: center;
                  "
                  id="PageSetupUI-CheckHiddenVersionServer"
                  :label="text('Setup.Ui.FeatureHide.Item.Server')"
                  :model-value="Boolean(values['CheckHiddenVersionServer'])"
                  @update:model-value="(value) => change('CheckHiddenVersionServer', value)"
                />
                <span
                  style="
                    grid-row: 8 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.Ui.FeatureHide.SpecificFeature') }}</span
                >
                <PCheckBox
                  data-control="CheckHiddenFunctionSelect"
                  style="
                    height: 22px;
                    grid-row: 8 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Ui.FeatureHide.Item.InstanceSelect.ToolTip')"
                  id="PageSetupUI-CheckHiddenFunctionSelect"
                  :label="text('Setup.Ui.FeatureHide.Item.InstanceSelect')"
                  :model-value="Boolean(values['CheckHiddenFunctionSelect'])"
                  @update:model-value="(value) => change('CheckHiddenFunctionSelect', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenFunctionModUpdate"
                  style="
                    height: 22px;
                    grid-row: 8 / span 1;
                    grid-column: 4 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Ui.FeatureHide.Item.ModUpdate.ToolTip')"
                  id="PageSetupUI-CheckHiddenFunctionModUpdate"
                  :label="text('Setup.Ui.FeatureHide.Item.ModUpdate')"
                  :model-value="Boolean(values['CheckHiddenFunctionModUpdate'])"
                  @update:model-value="(value) => change('CheckHiddenFunctionModUpdate', value)"
                />
                <PCheckBox
                  data-control="CheckHiddenFunctionHidden"
                  style="
                    height: 22px;
                    grid-row: 8 / span 1;
                    grid-column: 5 / span 1;
                    align-self: center;
                  "
                  :title="text('Setup.Ui.FeatureHide.Item.FeatureHide.ToolTip')"
                  id="PageSetupUI-CheckHiddenFunctionHidden"
                  :label="text('Setup.Ui.FeatureHide.Item.FeatureHide')"
                  :model-value="Boolean(values['CheckHiddenFunctionHidden'])"
                  @update:model-value="(value) => change('CheckHiddenFunctionHidden', value)"
                />
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
