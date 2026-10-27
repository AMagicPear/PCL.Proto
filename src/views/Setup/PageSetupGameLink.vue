<script setup lang="ts">
import { usePageSettings } from '@/composables/usePageSettings'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
const { values, text, change, act, image } = usePageSettings('PageSetupGameLink', {
  TextLinkUsername: '',
  ComboPreferProtocol: '0',
  CheckLatencyFirstMode: false,
  CheckTryPunchSym: false,
  CheckEnableIPv6: false,
  CheckEnableCliOutput: false,
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
          <PCard style="margin: 15px 0px" :title="text('EasyTier')">
            <div style="margin: 0px 25px 15px 25px; display: flex; flex-direction: column">
              <PHint style="margin: 0px 0px 10px 0px" severity="warning">{{
                text('Setup.GameLink.RestartRequired')
              }}</PHint>
              <div
                style="
                  display: grid;
                  grid-template-columns: auto 8px minmax(0, 1fr);
                  grid-template-rows: 28px 8px 28px 4px 28px 28px 28px 28px;
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
                  >{{ text('Setup.GameLink.Username') }}</span
                >
                <PTextBox
                  data-control="TextLinkUsername"
                  style="grid-row: 1 / span 1; grid-column: 3 / span 2"
                  :title="text('Setup.GameLink.Username.ToolTip')"
                  :placeholder="text('')"
                  :model-value="String(values['TextLinkUsername'])"
                  @update:model-value="(value) => change('TextLinkUsername', value || '')"
                />
                <span
                  style="
                    grid-row: 3 / span 1;
                    grid-column: 1 / span 1;
                    justify-self: start;
                    align-self: center;
                  "
                  class="pcl-text"
                  >{{ text('Setup.GameLink.Protocol.Preference') }}</span
                >
                <PComboBox
                  data-control="ComboPreferProtocol"
                  style="grid-row: 3 / span 1; grid-column: 3 / span 1"
                  :options="[
                    { key: '0', text: text('Setup.GameLink.Protocol.Tcp'), title: text('Setup.GameLink.Protocol.Tcp.ToolTip') },
                    { key: '1', text: text('Setup.GameLink.Protocol.Udp'), title: text('Setup.GameLink.Protocol.Udp.ToolTip') },
                  ]"
                  :model-value="String(values['ComboPreferProtocol'])"
                  @update:model-value="(value) => change('ComboPreferProtocol', value || '')"
                />
                <PCheckBox
                  data-control="CheckLatencyFirstMode"
                  style="grid-row: 5 / span 1; grid-column: 1 / span 3"
                  :title="text('Setup.GameLink.LatencyFirstMode.ToolTip')"
                  id="PageSetupGameLink-CheckLatencyFirstMode"
                  :label="text('Setup.GameLink.LatencyFirstMode')"
                  :model-value="Boolean(values['CheckLatencyFirstMode'])"
                  @update:model-value="(value) => change('CheckLatencyFirstMode', value)"
                />
                <PCheckBox
                  data-control="CheckTryPunchSym"
                  style="grid-row: 6 / span 1; grid-column: 1 / span 3"
                  :title="text('Setup.GameLink.PortGuessNat.ToolTip')"
                  id="PageSetupGameLink-CheckTryPunchSym"
                  :label="text('Setup.GameLink.PortGuessNat')"
                  :model-value="Boolean(values['CheckTryPunchSym'])"
                  @update:model-value="(value) => change('CheckTryPunchSym', value)"
                />
                <PCheckBox
                  data-control="CheckEnableIPv6"
                  style="grid-row: 7 / span 1; grid-column: 1 / span 3"
                  :title="text('Setup.GameLink.EnableIPv6.ToolTip')"
                  id="PageSetupGameLink-CheckEnableIPv6"
                  :label="text('Setup.GameLink.EnableIPv6')"
                  :model-value="Boolean(values['CheckEnableIPv6'])"
                  @update:model-value="(value) => change('CheckEnableIPv6', value)"
                />
                <PCheckBox
                  data-control="CheckEnableCliOutput"
                  style="grid-row: 8 / span 1; grid-column: 1 / span 3"
                  :title="text('Setup.GameLink.CliDebugOutput.ToolTip')"
                  id="PageSetupGameLink-CheckEnableCliOutput"
                  :label="text('Setup.GameLink.CliDebugOutput')"
                  :model-value="Boolean(values['CheckEnableCliOutput'])"
                  @update:model-value="(value) => change('CheckEnableCliOutput', value)"
                />
              </div>
            </div>
          </PCard>
        </div>
      </div>
    </div>
  </div>
</template>
