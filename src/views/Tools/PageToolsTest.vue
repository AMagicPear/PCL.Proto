<script setup lang="ts">
import { ref, defineAsyncComponent } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { usePageSettings } from '@/composables/usePageSettings'
import { useModal } from '@/composables/useModal'
import sideTip from '@/composables/sideTip'
import getSkinUrl from '@/api/skinGetter'
import MinecraftServerCard from '@/components/widget/MinecraftServerCard.vue'
const SkinViewer = defineAsyncComponent(() => import('@/components/widget/SkinViewer.vue'))
const skin = ref('')
import PTextBox from '@/components/widget/PTextBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PCard from '@/components/widget/PCard.vue'
import PButton from '@/components/widget/PButton.vue'
const {
  values,
  text,
  change,
  act: fallback,
  image,
} = usePageSettings('PageToolsTest', {
  TextDownloadUrl: '',
  TextDownloadFolder: '',
  TextDownloadName: '',
  AchievementBlockTextBox: '',
  AchievementTitleTextBox: '',
  AchievementString1TextBox: '',
  AchievementString2TextBox: '',
  CmbHeadSize: '0',
})
async function act(id: string, event = '', type = '', data = '') {
  if (event === 'BtnLuck_Click') {
    try {
      const luck = await invoke<number>('get_lucky_today')
      await useModal().open({
        title: text('Tools.Test.Luck.Title'),
        content: `你今天的人品值是：${luck}`,
        buttons: [{ type: 'tint', content: '确定' }],
      })
    } catch (err) {
      sideTip.show(`未能获取今日人品：${String(err)}`, 'warn')
    }
    return
  }
  if (id === 'BtnSkinSave') {
    const name = String(values.TextSkinID || '').trim()
    if (!/^[A-Za-z0-9_]{3,16}$/.test(name)) {
      sideTip.show('请输入有效的正版玩家名。', 'warn')
      return
    }
    try {
      await getSkinUrl(
        name,
        (url) => {
          skin.value = url
          const link = document.createElement('a')
          link.href = url
          link.download = `${name}.png`
          link.click()
        },
        'username',
      )
    } catch (err) {
      sideTip.show(`未能获取玩家皮肤：${String(err)}`, 'warn')
    }
    return
  }
  await fallback(id, event, type, data)
}
</script>
<template>
  <section class="pcl-page">
    <div>
      <div data-control="PanBack">
        <div
          data-control="PanMain"
          style="margin: 10px 25px; display: flex; flex-direction: column"
        >
          <PCard style="margin: 15px 0px" :title="text('Tools.Test.Title')">
            <div
              style="
                margin: 0px 21px 5px 25px;
                display: flex;
                flex-direction: row;
                align-items: center;
                flex-wrap: wrap;
              "
            >
              <PButton
                data-control="BtnClear"
                style="min-width: 120px; margin: 0px 15px 15px 0px"
                :title="text('Tools.Test.Clean.ToolTip')"
                :click="() => act('BtnClear', 'BtnClear_Click', '', '')"
                >{{ text('Tools.Test.Clean.Title') }}</PButton
              >
              <PButton
                style="min-width: 100px; margin: 0px 15px 15px 0px"
                :click="() => act('BtnLuck_Click', 'BtnLuck_Click', '', '')"
                >{{ text('Tools.Test.Luck.Title') }}</PButton
              >
              <PButton
                style="min-width: 120px; margin: 0px 15px 15px 0px"
                :title="text('Tools.Test.Shortcut.ToolTip')"
                :click="() => act('BtnCreateShortcut_Click', 'BtnCreateShortcut_Click', '', '')"
                >{{ text('Tools.Test.Shortcut.Title') }}</PButton
              >
              <PButton
                style="min-width: 120px; margin: 0px 15px 15px 0px"
                :click="() => act('BtnLaunchCount_Click', 'BtnLaunchCount_Click', '', '')"
                >{{ text('Tools.Test.LaunchCount.Title') }}</PButton
              >
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Tools.Test.CustomDownload.Title')">
            <div style="margin: 0px 25px 20px 25px; display: flex; flex-direction: column">
              <span class="pcl-text">{{ text('Tools.Test.CustomDownload.Description') }}</span>
              <div
                style="
                  margin: 10px 0px 0px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr) auto;
                  grid-template-rows: auto auto auto minmax(0, 1fr);
                "
              >
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.CustomDownload.Url') }}</span
                >
                <PTextBox
                  data-control="TextDownloadUrl"
                  style="grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['TextDownloadUrl'])"
                  @update:model-value="(value) => change('TextDownloadUrl', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.CustomDownload.SaveTo') }}</span
                >
                <PTextBox
                  data-control="TextDownloadFolder"
                  style="margin: 10px 0px; grid-row: 3 / span 1; grid-column: 2 / span 1"
                  :placeholder="text('')"
                  :model-value="String(values['TextDownloadFolder'])"
                  @update:model-value="(value) => change('TextDownloadFolder', value || '')"
                />
                <PButton
                  style="
                    margin: 0px 5px 0px 10px;
                    grid-row: 3 / span 1;
                    grid-column: 3 / span 1;
                    align-self: center;
                  "
                  :click="() => act('MyTextButton_Click', 'MyTextButton_Click', '', '')"
                  inline
                  >{{ text('Tools.Test.CustomDownload.Select') }}</PButton
                >
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 4 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.CustomDownload.FileName') }}</span
                >
                <PTextBox
                  data-control="TextDownloadName"
                  style="grid-row: 4 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['TextDownloadName'])"
                  @update:model-value="(value) => change('TextDownloadName', value || '')"
                />
              </div>
              <div
                style="
                  margin: 15px 0px 0px 0px;
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
                  grid-template-rows: 35px;
                "
              >
                <PButton
                  data-control="BtnDownloadStart"
                  style="
                    width: 140px;
                    margin: 0px 15px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: end;
                  "
                  :click="() => act('BtnDownloadStart', 'BtnDownloadStart_Click', '', '')"
                  type="tint"
                  >{{ text('Tools.Test.CustomDownload.Start') }}</PButton
                >
                <PButton
                  data-control="BtnDownloadOpen"
                  style="
                    width: 140px;
                    margin: 0px 15px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                  "
                  :click="() => act('BtnDownloadOpen', 'BtnDownloadOpen_Click', '', '')"
                  >{{ text('Common.Action.OpenFolder') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            data-control="CardSkin"
            style="margin: 0px 0px 15px 0px"
            :title="text('Tools.Test.Skin.Title')"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 14px 0px 2px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                  grid-template-rows: auto auto;
                "
              >
                <div class="pcl-form-row">
                  <label>{{ text('Tools.Test.Skin.PlayerName') }}</label
                  ><PTextBox
                    :model-value="String(values.TextSkinID || '')"
                    @update:model-value="(value) => change('TextSkinID', value || '')"
                  />
                </div>
                <PButton
                  data-control="BtnSkinSave"
                  style="
                    min-width: 140px;
                    justify-self: start;
                    grid-row: 2;
                    grid-column: 1 / -1;
                    margin-top: 14px;
                  "
                  :click="() => act('BtnSkinSave')"
                  >{{ text('Tools.Test.Skin.Save') }}</PButton
                ><SkinViewer v-if="skin" :skin-url="skin" />
              </div>
            </div>
          </PCard>
          <PCard style="margin: 0px 0px 15px 0px" :title="text('Tools.Test.ServerQuery.Title')">
            <div style="margin: 0px 15px 15px 15px"><MinecraftServerCard embedded /></div>
          </PCard>
          <PCard
            style="margin: 0px 0px 15px 0px"
            :title="text('Tools.Test.Achievement.Title')"
            default-fold-status="fold"
          >
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 4px 0px 2px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr) auto;
                  grid-template-rows: auto auto auto auto minmax(0, 1fr);
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
                  >{{ text('Tools.Test.Achievement.ItemId') }}</span
                >
                <PTextBox
                  data-control="AchievementBlockTextBox"
                  style="margin: 10px 0px; grid-row: 1 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['AchievementBlockTextBox'])"
                  @update:model-value="(value) => change('AchievementBlockTextBox', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 2 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.Achievement.Name') }}</span
                >
                <PTextBox
                  data-control="AchievementTitleTextBox"
                  style="grid-row: 2 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['AchievementTitleTextBox'])"
                  @update:model-value="(value) => change('AchievementTitleTextBox', value || '')"
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
                  >{{ text('Tools.Test.Achievement.Line1') }}</span
                >
                <PTextBox
                  data-control="AchievementString1TextBox"
                  style="margin: 10px 0px; grid-row: 3 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['AchievementString1TextBox'])"
                  @update:model-value="(value) => change('AchievementString1TextBox', value || '')"
                />
                <span
                  style="
                    margin: 0px 25px 0px 0px;
                    grid-row: 4 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.Achievement.Line2') }}</span
                >
                <PTextBox
                  data-control="AchievementString2TextBox"
                  style="grid-row: 4 / span 1; grid-column: 2 / span 2"
                  :placeholder="text('')"
                  :model-value="String(values['AchievementString2TextBox'])"
                  @update:model-value="(value) => change('AchievementString2TextBox', value || '')"
                />
              </div>
              <div
                style="
                  margin: 15px 0px 10px 0px;
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
                  grid-template-rows: 35px;
                "
              >
                <PButton
                  data-control="BtnAchievementPreview"
                  style="
                    width: 140px;
                    margin: 0px 15px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: end;
                  "
                  :click="() => act('BtnAchievementPreview', 'BtnAchievementPreview_Click', '', '')"
                  type="tint"
                  >{{ text('Tools.Test.Achievement.Preview') }}</PButton
                >
                <PButton
                  data-control="BtnAchievementSave"
                  style="
                    width: 140px;
                    margin: 0px 15px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                    justify-self: start;
                  "
                  :click="() => act('BtnAchievementSave', 'BtnAchievementSave_Click', '', '')"
                  >{{ text('Tools.Test.Achievement.Save') }}</PButton
                >
              </div>
            </div>
          </PCard>
          <PCard
            style="margin: 0px 0px 15px 0px"
            :title="text('Tools.Test.Avatar.Title')"
            default-fold-status="fold"
          >
            <div style="margin: 0px 25px 20px 25px; display: flex; flex-direction: column">
              <div
                style="
                  margin: 0px 0px 10px 0px;
                  display: grid;
                  grid-template-columns: auto minmax(0, 1fr);
                "
              >
                <span
                  style="
                    margin: 0px 10px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 1 / span 1;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Tools.Test.Avatar.Size') }}</span
                >
                <PComboBox
                  data-control="CmbHeadSize"
                  style="height: 26px; grid-row: 1 / span 1; grid-column: 2 / span 1"
                  :options="
                    ['64x64', '96x96', '128x128'].map((size, index) => ({
                      key: String(index),
                      text: size,
                    }))
                  "
                  :model-value="String(values['CmbHeadSize'])"
                  @update:model-value="(value) => change('CmbHeadSize', value || '')"
                />
              </div>
              <div
                style="
                  display: grid;
                  grid-template-columns: minmax(0, 1fr) auto auto minmax(0, 1fr);
                "
              >
                <PButton
                  data-control="BtnSelectSkin"
                  style="
                    width: 140px;
                    height: 35px;
                    margin: 0px 10px 0px 0px;
                    grid-row: 1 / span 1;
                    grid-column: 2 / span 1;
                  "
                  :click="() => act('BtnSelectSkin', '', '', '')"
                  >{{ text('Tools.Test.Avatar.SelectSkin') }}</PButton
                >
                <PButton
                  style="width: 140px; height: 35px; grid-row: 1 / span 1; grid-column: 3 / span 1"
                  :click="() => act('BtnSaveHead_Click', 'BtnSaveHead_Click', '', '')"
                  >{{ text('Tools.Test.Avatar.Save') }}</PButton
                >
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </section>
</template>
