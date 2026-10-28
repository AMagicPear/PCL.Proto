<script lang="ts" setup>
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue'
import IconUnfold from '../icons/control/IconUnfold.vue'
import { motion } from 'motion-v'

export type FoldStatus = 'unfold' | 'fold' | 'unfoldable'

const props = withDefaults(
  defineProps<{
    hideTitle?: boolean
    defaultFoldStatus?: FoldStatus
    overflowVisible?: boolean
    title?: string
  }>(),
  {
    hideTitle: false,
    defaultFoldStatus: 'unfoldable',
    overflowVisible: false,
  },
)

const foldState = ref<FoldStatus>(props.defaultFoldStatus)
const cardHeight = ref<number>(40)
const mycardInnerRef = useTemplateRef<HTMLElement>('mycardInner')
const shouldSpring = ref<boolean>(true)

let observer: ResizeObserver | null = null

function toggleFold() {
  switch (foldState.value) {
    case 'unfold':
      foldState.value = 'fold'
      // 卡片折叠状态固定为高度40
      // 在此处赋值是因为卡片内容有过渡时间
      // 切换时就赋值可以让卡片高度和内容的过渡同时进行
      shouldSpring.value = cardHeight.value < 500
      cardHeight.value = 40
      break
    case 'fold':
      foldState.value = 'unfold'
      break
  }
}

onMounted(() => {
  observer = new ResizeObserver(() => {
    // 若非折叠状态需要根据内容调整高度
    // 为了方便使高度动画具有回弹动效，使用observer侦测内容来赋值而非让其自动撑开
    if (foldState.value != 'fold') {
      shouldSpring.value = mycardInnerRef.value!.offsetHeight < 500
      cardHeight.value = mycardInnerRef.value!.offsetHeight
    }
  })
  observer.observe(mycardInnerRef.value!)
})

onUnmounted(() => observer?.disconnect())
</script>

<template>
  <motion.div
    :class="['mycard-container', foldState, { 'overflow-visible': overflowVisible }]"
    :animate="{ height: cardHeight }"
    :transition="{ type: shouldSpring ? 'spring' : 'tween', stiffness: 330, damping: 25, mass: 1 }"
  >
    <div class="mycard" ref="mycardInner" :class="{ 'hide-title': hideTitle }">
      <header
        class="mycard-title"
        v-if="!hideTitle"
        :role="foldState !== 'unfoldable' ? 'button' : undefined"
        :tabindex="foldState !== 'unfoldable' ? 0 : undefined"
        :aria-expanded="foldState !== 'fold'"
        @keydown.enter="toggleFold"
        @keydown.space.prevent="toggleFold"
        @click="toggleFold"
      >
        <!-- 此处为兼容性设计：title插槽被设置时显示插槽内容，否则默认显示props中的title属性 -->
        <p>
          <slot name="title">{{ title }}</slot>
        </p>
        <div class="description" v-if="$slots.description">
          <slot name="description"></slot>
        </div>
        <i><IconUnfold /></i>
      </header>
      <Transition name="card-content">
        <section
          class="mycard-content"
          v-if="$slots.content || $slots.default"
          v-show="foldState == 'unfold' || foldState == 'unfoldable'"
        >
          <!-- 此处为兼容性设计：content被设置时显示content内容，否则显示默认插槽内容 -->
          <slot name="content"><slot></slot></slot>
        </section>
      </Transition>
    </div>
  </motion.div>
</template>

<style>
.card-content-enter-active,
.card-content-leave-active {
  transition: opacity 0.2s;
}

.card-content-enter-from,
.card-content-leave-to {
  opacity: 0;
}

.mycard-container {
  flex-shrink: 0;
  transition: box-shadow 0.4s;
  border-radius: 6px;
  background: var(--color-background-soft);
  box-shadow: 0px 0px 6px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  will-change: height;
}

.mycard-container.overflow-visible,
.mycard-container.overflow-visible > .mycard > .mycard-content {
  overflow: visible;
}

.mycard-container:hover {
  box-shadow: 0px 0px 6px var(--color-tint-shadow);
}

.mycard-container:not(.unfoldable) .mycard-title {
  cursor: pointer;
}

.mycard-container.unfoldable .mycard-title > i {
  visibility: hidden;
}

.mycard {
  padding: 0;
}

.mycard.hide-title {
  /* 不知道为什么必须有上下的padding */
  padding: 0;
}

.mycard > .mycard-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  color: var(--color-text);
  min-height: 40px;
  padding: 0 15px;
}

.mycard > .mycard-title > i {
  color: var(--color-text);
  transition: transform 0.4s ease;
  flex: 0;
}

.mycard-container.unfold .mycard-title > i {
  transform: rotate(180deg);
}

.mycard > .mycard-title > p *,
.mycard > .mycard-title > p {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0px;
  line-height: 14.06px;
  text-align: left;
  vertical-align: top;
  transition: color 0.4s;
  flex: 1;
}

.mycard-container:hover > .mycard > .mycard-title {
  color: var(--color-tint);
}

.mycard > .mycard-content {
  font-size: 12px;
  margin: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.description {
  flex: 3;
}

.pcl-page .mycard-title {
  font-size: 13px;
}

.pcl-page .mycard-content {
  font-size: 13px;
}
</style>
