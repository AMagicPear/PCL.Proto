<script setup lang="ts">
import { showUnavailable } from '@/util/showUnavailable'
import { pclText } from '@/api/pclLocale'
import { useSelectedInstance } from '@/stores/gameLaunch'
import PListItem from '@/components/widget/PListItem.vue'
import { pluginShowText, instanceIcon } from '@/types/gameInfo'
const instance = useSelectedInstance()
import { usePageSettings } from '@/composables/usePageSettings'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import sideTip from '@/composables/sideTip'
const { values } = usePageSettings('PageInstanceOverall', {
  ComboDisplayLogo: '0',
  ComboDisplayType: '0',
})

const text = pclText
async function openFolder(kind: 'instance' | 'saves' | 'mods') {
  const instanceDirectory = instance.instance_info?.directory
  if (!instanceDirectory || !isTauri()) return
  try {
    await invoke('open_instance_folder', { instanceDirectory, kind })
  } catch (error) {
    sideTip.show(pclText('Proto.Error.OpenFolder', [String(error)]), 'warn')
  }
}
</script>
<template>
  <div class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 10px 25px; display: flex; flex-direction: column"
        >
          <PCard style="margin: 15px 0px" hide-title>
            <div data-control="PanDisplayItem" style="margin: 7px 10px">
              <PListItem
                v-if="instance.instance_info"
                :title="instance.instance_info.name"
                :subtitle="instance.instance_info.version"
                :icon="
                  instanceIcon(
                    instance.instance_info.version,
                    instance.instance_info.pluginsVersion || {},
                  )
                "
              />
              <p v-else class="pcl-empty">{{ pclText('Proto.Instance.SelectFirst') }}</p>
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Instance.Overall.Info.Title')">
            <div
              data-control="PanInfo"
              style="margin: 0px 25px 5px 15px; display: flex; flex-direction: column"
            >
              <p v-if="instance.instance_info">Minecraft {{ instance.instance_info.version }}</p>
              <p v-for="(version, loader) in instance.instance_info?.pluginsVersion" :key="loader">
                {{ pluginShowText[loader] || loader }} {{ version }}
              </p>
              <p>{{ instance.instance_info?.directory }}</p>
            </div>
          </PCard>
          <PCard
            data-control="PanDisplay"
            style="margin: 0px 0px 15px 0px"
            :title="text('Instance.Overall.Personalization.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                data-control="PanDisplayIcon"
                style="
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: 28px 9px 28px;
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
                  >{{ text('Instance.Overall.Icon.Title') }}</span
                >
                <PComboBox
                  data-control="ComboDisplayLogo"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :options="[
                    { key: '0', text: text('Common.Option.Auto') },
                    { key: '1', text: text('Common.Option.Customize') },
                    { key: '2', text: text('Instance.Overall.Icon.Cobblestone') },
                    { key: '3', text: text('Instance.Overall.Icon.CommandBlock') },
                    { key: '4', text: text('Instance.Overall.Icon.GoldBlock') },
                    { key: '5', text: text('Instance.Overall.Icon.GrassBlock') },
                    { key: '6', text: text('Instance.Overall.Icon.DirtPath') },
                    { key: '7', text: text('Instance.Overall.Icon.Anvil') },
                    { key: '8', text: text('Instance.Overall.Icon.RedstoneBlock') },
                    { key: '9', text: text('Instance.Overall.Icon.RedstoneLampOn') },
                    { key: '10', text: text('Instance.Overall.Icon.RedstoneLampOff') },
                    { key: '11', text: text('Instance.Overall.Icon.Egg') },
                    { key: '12', text: text('Instance.Overall.Icon.Fabric') },
                    { key: '13', text: text('Instance.Overall.Icon.Quilt') },
                    { key: '14', text: text('Instance.Overall.Icon.NeoForgeFox') },
                    { key: '15', text: text('Instance.Overall.Icon.Cleanroom') },
                  ]"
                  v-model="values.ComboDisplayLogo"
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
                  >{{ text('Instance.Overall.Category.Title') }}</span
                >
                <PComboBox
                  data-control="ComboDisplayType"
                  style="grid-row: 3 / span 1; grid-column: 2 / span 1"
                  :options="[
                    { key: '0', text: text('Common.Option.Auto') },
                    {
                      key: '1',
                      text: text('Instance.Overall.Category.Hidden'),
                      title: text('Instance.Overall.Category.Hidden.ToolTip'),
                    },
                    { key: '2', text: text('Instance.Overall.Category.Modable') },
                    { key: '3', text: text('Instance.Overall.Category.Regular') },
                    { key: '4', text: text('Instance.Overall.Category.LessUsed') },
                    { key: '5', text: text('Instance.Overall.Category.AprilFools') },
                  ]"
                  v-model="values.ComboDisplayType"
                />
              </div>
              <div
                style="
                  height: 35px;
                  margin: 15px 0px 7px 0px;
                  display: grid;
                  grid-template-columns: auto auto auto;
                "
              >
                <PButton
                  data-control="BtnDisplayRename"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Rename') }}</PButton
                >
                <PButton
                  data-control="BtnDisplayDesc"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.EditDescription') }}</PButton
                >
                <PButton
                  data-control="BtnDisplayStar"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 3 / span 1;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Favorite') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="PanFolder"
            style="margin: 0px 0px 15px 0px"
            :title="text('Instance.Overall.Shortcuts.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  height: 35px;
                  margin: 2px 0px 7px 0px;
                  display: grid;
                  grid-template-columns: auto auto auto;
                "
              >
                <PButton
                  data-control="BtnFolderVersion"
                  :disabled="!instance.instance_info || !isTauri()"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                  "
                  @click="openFolder('instance')"
                  >{{ text('Instance.Overall.Shortcuts.InstanceFolder') }}</PButton
                >
                <PButton
                  data-control="BtnFolderSaves"
                  :disabled="!instance.instance_info || !isTauri()"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  @click="openFolder('saves')"
                  >{{ text('Instance.Overall.Shortcuts.SavesFolder') }}</PButton
                >
                <PButton
                  data-control="BtnFolderMods"
                  :disabled="!instance.instance_info || !isTauri()"
                  style="
                    min-width: 140px;
                    margin: 0px 20px 0px 0px;
                    padding: 0px 13px;
                    grid-row: 1 / span 1;
                    grid-column: 3 / span 1;
                  "
                  @click="openFolder('mods')"
                  >{{ text('Instance.Overall.Shortcuts.ModsFolder') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="PanManage"
            style="margin: 0px 0px 15px 0px"
            :title="text('Instance.Overall.Manage.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: -5px -20px 7px 0px;
                  display: flex;
                  flex-direction: row;
                  align-items: center;
                  flex-wrap: wrap;
                "
              >
                <PButton
                  data-control="BtnManageScript"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Manage.ExportLaunchScript') }}</PButton
                >
                <PButton
                  data-control="BtnManageTest"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Manage.TestGame') }}</PButton
                >
                <PButton
                  data-control="BtnManageCheck"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  :title="text('Instance.Overall.Manage.RepairFiles.ToolTip')"
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Manage.RepairFiles') }}</PButton
                >
                <PButton
                  data-control="BtnManageRestore"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  :title="text('Instance.Overall.Manage.Reinstall.ToolTip')"
                  @click="showUnavailable"
                  >{{ text('Common.Action.Reset') }}</PButton
                >
                <PButton
                  data-control="BtnManageDelete"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  @click="showUnavailable"
                  type="warn"
                  >{{ text('Instance.Overall.Manage.Delete') }}</PButton
                >
                <PButton
                  data-control="BtnManagePatch"
                  style="
                    height: 35px;
                    min-width: 140px;
                    margin: 7px 20px 0px 0px;
                    padding: 0px 13px;
                  "
                  @click="showUnavailable"
                  >{{ text('Instance.Overall.Manage.PatchCore') }}</PButton
                >
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
