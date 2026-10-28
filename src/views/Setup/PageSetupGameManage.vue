<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import { usePageSettings } from '@/composables/usePageSettings'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
const { values } = usePageSettings('PageSetupGameManage', {
  ComboDownloadHttpMode: '0',
})

const text = pclText
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 10px 25px; display: flex; flex-direction: column"
        >
          <PCard style="margin: 15px 0px" :title="text('Setup.GameManage.Source.Title')">
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: 28px 7px 28px 7px 27px 7px 27px 27px 27px 27px 27px;
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
                  >{{ text('Setup.GameManage.Source.File') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadSource"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.GameManage.Source.UseMirror') },
                    { key: '1', text: text('Setup.GameManage.Source.PreferOfficialThenMirror') },
                    { key: '2', text: text('Setup.GameManage.Source.UseOfficial') },
                  ]"
                  v-model="values.ComboDownloadSource"
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
                  >{{ text('Setup.GameManage.Source.Version') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadVersion"
                  style="grid-row: 3 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.GameManage.Source.Version.Mirror') },
                    { key: '1', text: text('Setup.GameManage.Source.PreferOfficialThenMirror') },
                    { key: '2', text: text('Setup.GameManage.Source.UseOfficial') },
                  ]"
                  v-model="values.ComboDownloadVersion"
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
                  >{{ text('Setup.GameManage.Download.HttpProtocol') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadHttpMode"
                  style="grid-row: 5 / span 1; grid-column: 2 / span 1"
                  :options="[
                    {
                      key: '0',
                      text: text('Setup.GameManage.Download.HttpProtocol.Auto'),
                      title: text('Setup.GameManage.Download.HttpProtocol.Auto.ToolTip'),
                    },
                    {
                      key: '1',
                      text: text('Setup.GameManage.Download.HttpProtocol.Http11'),
                      title: text('Setup.GameManage.Download.HttpProtocol.Http11.ToolTip'),
                    },
                    {
                      key: '2',
                      text: text('Setup.GameManage.Download.HttpProtocol.Http2'),
                      title: text('Setup.GameManage.Download.HttpProtocol.Http2.ToolTip'),
                    },
                  ]"
                  v-model="values.ComboDownloadHttpMode"
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
                  >{{ text('Setup.GameManage.Download.Threads') }}</span
                >
                <input
                  data-control="SliderDownloadThread"
                  style="grid-row: 7 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.GameManage.Download.Threads.ToolTip')"
                  type="range"
                  min="0"
                  max="255"
                  v-model.number="values['SliderDownloadThread']"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 8 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Download.FileConnections') }}</span
                >
                <input
                  data-control="SliderDownloadFileConnection"
                  style="grid-row: 8 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.GameManage.Download.FileConnections.ToolTip')"
                  type="range"
                  min="0"
                  max="7"
                  v-model.number="values['SliderDownloadFileConnection']"
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
                  >{{ text('Setup.GameManage.Download.SpeedLimit') }}</span
                >
                <input
                  data-control="SliderDownloadSpeed"
                  style="grid-row: 9 / span 1; grid-column: 2 / span 1"
                  :title="text('Setup.GameManage.Download.SpeedLimit.ToolTip')"
                  type="range"
                  min="0"
                  max="42"
                  v-model.number="values['SliderDownloadSpeed']"
                />
                <span
                  style="
                    margin: 5px 25px 5px 0px;
                    grid-row: 10 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: start;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Download.TargetFolder') }}</span
                >
                <span
                  style="
                    margin: 5px 0px;
                    opacity: 0.5;
                    grid-row: 10 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Download.TargetFolder.Hint') }}</span
                >
                <span
                  style="margin: 5px 0px; grid-row: 11 / span 1; grid-column: 1 / span 1"
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Download.InstallBehavior') }}</span
                >
                <div
                  style="
                    grid-row: 11 / span 1;
                    grid-column: 2 / span 1;
                    display: grid;
                    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
                  "
                >
                  <PCheckBox
                    data-control="CheckDownloadAutoSelectVersion"
                    style="grid-row: 1 / span 1; grid-column: 1 / span 1"
                    id="PageSetupGameManage-CheckDownloadAutoSelectVersion"
                    :label="text('Setup.GameManage.Download.AutoSelectInstance')"
                    v-model="values.CheckDownloadAutoSelectVersion"
                  />
                  <PCheckBox
                    data-control="CheckFixAuthlib"
                    style="margin: 0px 0px 0px 0px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                    :title="text('Setup.GameManage.Download.FixAuthlib.ToolTip')"
                    id="PageSetupGameManage-CheckFixAuthlib"
                    :label="text('Setup.GameManage.Download.FixAuthlib')"
                    v-model="values.CheckFixAuthlib"
                  />
                </div>
              </div>
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Setup.GameManage.Community.Title')">
            <div style="margin: 0px 25px 12px 25px; display: flex; flex-direction: column">
              <div
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: 28px 7px 28px 7px 28px 7px 28px 7px auto;
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
                  >{{ text('Setup.GameManage.Community.Source') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadMod"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :options="[
                    { key: '0', text: text('Setup.GameManage.Source.UseMirror') },
                    {
                      key: '1',
                      text: text('Setup.GameManage.Community.Source.PreferOfficialWithFallback'),
                    },
                    { key: '2', text: text('Setup.GameManage.Source.UseOfficial') },
                  ]"
                  v-model="values.ComboDownloadMod"
                />
                <span
                  data-control="TextFilenameFormat"
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Community.FilenameFormat') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadTranslateV2"
                  style="grid-row: 3 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.GameManage.Community.FilenameFormat.ToolTip')"
                  :options="[
                    { key: '0', text: text('Setup.GameManage.Community.FilenameFormat.Style0') },
                    { key: '1', text: text('Setup.GameManage.Community.FilenameFormat.Style1') },
                    { key: '2', text: text('Setup.GameManage.Community.FilenameFormat.Style2') },
                    { key: '3', text: text('Setup.GameManage.Community.FilenameFormat.Style3') },
                    { key: '4', text: text('Setup.GameManage.Community.FilenameFormat.Style4') },
                  ]"
                  v-model="values.ComboDownloadTranslateV2"
                />
                <span
                  data-control="TextModManageStyle"
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 5 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameManage.Community.ModManageStyle') }}</span
                >
                <PComboBox
                  data-control="ComboModLocalNameStyle"
                  style="grid-row: 5 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.GameManage.Community.ModManageStyle.ToolTip')"
                  :options="[
                    {
                      key: '0',
                      text: text('Setup.GameManage.Community.ModManageStyle.TitleTranslation'),
                    },
                    {
                      key: '1',
                      text: text('Setup.GameManage.Community.ModManageStyle.TitleFilename'),
                    },
                  ]"
                  v-model="values.ComboModLocalNameStyle"
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
                  >{{ text('Setup.GameManage.Community.QuickDownload') }}</span
                >
                <PComboBox
                  data-control="ComboDownloadQuickBehavior"
                  style="grid-row: 7 / span 1; grid-column: 2 / span 2"
                  :title="text('Setup.GameManage.Community.QuickDownload.ToolTip')"
                  :options="[
                    { key: '0', text: text('Setup.GameManage.Community.QuickDownload.AlwaysAsk') },
                    {
                      key: '1',
                      text: text('Setup.GameManage.Community.QuickDownload.CurrentInstance'),
                    },
                    {
                      key: '2',
                      text: text('Setup.GameManage.Community.QuickDownload.AskInstance'),
                    },
                    { key: '3', text: text('Setup.GameManage.Community.QuickDownload.AskPath') },
                  ]"
                  v-model="values.ComboDownloadQuickBehavior"
                />
              </div>
              <div
                style="
                  display: grid;
                  grid-template-columns: minmax(0, 1fr);
                  grid-template-rows: auto auto;
                "
              >
                <PCheckBox
                  data-control="CheckDownloadIgnoreQuilt"
                  style="
                    height: 22px;
                    margin: 2px 0px 4px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :title="text('Setup.GameManage.Community.HideQuilt.ToolTip')"
                  id="PageSetupGameManage-CheckDownloadIgnoreQuilt"
                  :label="text('Setup.GameManage.Community.HideQuilt')"
                  v-model="values.CheckDownloadIgnoreQuilt"
                />
                <PCheckBox
                  data-control="CheckDownloadAutoInstallDependencies"
                  style="
                    height: 22px;
                    margin: 2px 0px 4px 0px;
                    grid-row: 2 / span 1;
                    grid-column: 1 / span 1;
                  "
                  :title="text('Setup.GameManage.Community.AutoInstallDependencies.ToolTip')"
                  id="PageSetupGameManage-CheckDownloadAutoInstallDependencies"
                  :label="text('Setup.GameManage.Community.AutoInstallDependencies')"
                  v-model="values.CheckDownloadAutoInstallDependencies"
                />
              </div>
            </div>
          </PCard>
          <PCard
            style="margin: 0px 0px 15px 0px"
            :title="text('Setup.GameManage.Accessibility.Title')"
          >
            <div
              style="
                margin: 0px 25px 17px 25px;
                display: grid;
                grid-template-columns: auto 160px auto;
                grid-template-rows: 22px 8px 22px 8px 22px 8px;
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
                >{{ text('Setup.GameManage.Accessibility.GameUpdateNotice') }}</span
              >
              <PCheckBox
                data-control="CheckUpdateRelease"
                style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                :title="text('Setup.GameManage.Accessibility.ReleaseNotice.ToolTip')"
                id="PageSetupGameManage-CheckUpdateRelease"
                :label="text('Setup.GameManage.Accessibility.ReleaseNotice')"
                v-model="values.CheckUpdateRelease"
              />
              <PCheckBox
                data-control="CheckUpdateSnapshot"
                style="height: 22px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                :title="text('Setup.GameManage.Accessibility.SnapshotNotice.ToolTip')"
                id="PageSetupGameManage-CheckUpdateSnapshot"
                :label="text('Setup.GameManage.Accessibility.SnapshotNotice')"
                v-model="values.CheckUpdateSnapshot"
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
                >{{ text('Setup.GameManage.Accessibility.GameLanguage') }}</span
              >
              <PCheckBox
                data-control="CheckHelpLauncherLanguage"
                style="grid-row: 3 / span 1; grid-column: 2 / span 5"
                id="PageSetupGameManage-CheckHelpLauncherLanguage"
                :label="text('Setup.GameManage.Accessibility.AutoLauncherLanguage')"
                v-model="values.CheckHelpLauncherLanguage"
              />
              <PCheckBox
                data-control="CheckDownloadClipboard"
                style="height: 22px; grid-row: 5 / span 1; grid-column: 1 / span 3"
                :title="text('Setup.GameManage.Accessibility.ReadClipboard.ToolTip')"
                id="PageSetupGameManage-CheckDownloadClipboard"
                :label="text('Setup.GameManage.Accessibility.ReadClipboard')"
                v-model="values.CheckDownloadClipboard"
              />
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
