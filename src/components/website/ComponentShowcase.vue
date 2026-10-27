<script setup lang="ts">
import { ref, useId } from 'vue'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PCheckBox from '@/components/widget/PCheckBox.vue'
import PComboBox from '@/components/widget/PComboBox.vue'
import PHint from '@/components/widget/PHint.vue'
import PLoading, { type LoadingState } from '@/components/widget/PLoading.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import sideTip from '@/composables/sideTip'
import { useModal } from '@/composables/useModal'

const id = useId()
const count = ref(0)
const loading = ref<LoadingState>('loading')
const playerName = ref('Player')
const editable = ref(true)
const edition = ref('release')
const editions = [
  { key: 'release', text: '正式版' },
  { key: 'snapshot', text: '快照版' },
  { key: 'old', text: '远古版' },
]
const modal = useModal()

async function showDialog(withInput: boolean) {
  const result = await modal.open({
    title: withInput ? '输入对话框' : '确认对话框',
    content: withInput
      ? '输入一个名字，试试确认后的提示。'
      : '这是一个演示操作，可以放心确认或取消。',
    plainText: true,
    showInput: withInput,
    defaultInputText: playerName.value,
    buttons: [
      { type: 'tint', content: '确认', operation: () => modal.close(true) },
      { content: '取消', operation: () => modal.close(false) },
    ],
  })
  sideTip.show(
    result.confirmed
      ? withInput
        ? `你好，${result.input || 'Player'}！`
        : '已确认这次操作。'
      : '已取消这次操作。',
    result.confirmed ? 'success' : 'default',
  )
}
</script>

<template>
  <div class="component-showcase" v-card-drop-children-animate>
    <PButton type="tint" :click="() => sideTip.show(`耶 ✌️ 你已经点击了 ${++count} 次！`)"
      >点击这个按钮会有提示哦 😬</PButton
    >
    <PCard title="按钮与侧边提示" default-fold-status="unfold">
      <div class="demo-content">
        <p>试试悬停、按下和键盘聚焦，看看按钮的颜色与缩放变化。</p>
        <div class="demo-row">
          <PButton :click="() => sideTip.show('这是一条普通提示。')">普通按钮</PButton>
          <PButton type="tint" :click="() => sideTip.show('这是一条成功提示。', 'success')"
            >高亮按钮</PButton
          >
          <PButton type="warn" :click="() => sideTip.show('这是一条警告提示。', 'warn')"
            >警告按钮</PButton
          >
          <PButton disabled>禁用按钮</PButton>
          <PButton
            inline
            tooltip="这是按钮的悬停提示"
            :click="() => sideTip.show('图标与文字也可以组合。')"
            ><PSvgIcon name="lucide/heart" :size="16" /> 图标按钮</PButton
          >
        </div>
      </div>
    </PCard>
    <PCard title="输入与选择" default-fold-status="unfold">
      <div class="demo-content">
        <label :for="`${id}-name`">玩家名称</label>
        <PTextBox
          :input-id="`${id}-name`"
          v-model="playerName"
          :disabled="!editable"
          placeholder="输入一个喜欢的名字"
        />
        <PComboBox v-model="edition" :options="editions" aria-label="演示版本类型" />
        <PCheckBox :id="`${id}-editable`" v-model="editable" label="启用名称输入框" />
        <PCheckBox :id="`${id}-disabled`" :model-value="true" disabled label="禁用的勾选项" />
        <p class="demo-value">
          {{ playerName || 'Player' }} · {{ editions.find((item) => item.key === edition)?.text }}
        </p>
      </div>
    </PCard>
    <PCard title="加载动画" default-fold-status="unfold">
      <div class="demo-content">
        <PLoading :state="loading" :card="false" loading-text="正在展示加载动画……" />
        <div class="demo-row">
          <PButton type="tint" :click="() => (loading = 'loading')">设置为 loading</PButton>
          <PButton type="warn" :click="() => (loading = 'error')">设置为 error</PButton>
        </div>
      </div>
    </PCard>
    <PCard title="模态框" default-fold-status="unfold">
      <div class="demo-content">
        <p>确认、取消和输入结果都会通过侧边提示反馈。</p>
        <div class="demo-row">
          <PButton :click="() => showDialog(false)">确认对话框</PButton>
          <PButton type="tint" :click="() => showDialog(true)">输入对话框</PButton>
        </div>
      </div>
    </PCard>
    <PCard title="折叠卡片与行内提示 · 点击展开" default-fold-status="fold">
      <div class="demo-content">
        <PHint severity="info" closable>这是一条信息提示，可以点击右侧关闭。</PHint>
        <PHint severity="warning">这是一条警告提示。</PHint>
        <PHint severity="error">这是一条错误提示。</PHint>
      </div>
    </PCard>
    <PCard hide-title>
      <div class="demo-content demo-untitled">卡片也可以没有标题，用来承载一小段内容。</div>
    </PCard>
  </div>
</template>

<style scoped>
.component-showcase,
.demo-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.demo-content {
  padding: 0 20px 18px;
}
.demo-untitled {
  padding-top: 18px;
}
.demo-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
.demo-value {
  color: var(--color-tint);
}
</style>
