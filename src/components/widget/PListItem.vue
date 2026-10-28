<script setup lang="ts">
import PButton from './PButton.vue'

withDefaults(
  defineProps<{
    icon?: string
    title: string
    subtitle?: string
    roundImg?: boolean
    hoverEffect?: boolean
    btn?: {
      text: string
      link: string
    }
    clickable?: boolean
    pixelatedIcon?: boolean
  }>(),
  {
    hoverEffect: true,
    clickable: false,
    roundImg: false,
    pixelatedIcon: false,
  },
)
const emit = defineEmits<{ click: [event: MouseEvent | KeyboardEvent] }>()
</script>

<template lang="pug">
    .gameinfo-container(@click="clickable && emit('click', $event)" :role="clickable ? 'button' : undefined" :tabindex="clickable ? 0 : undefined" @keydown.enter.self="clickable && emit('click', $event)" @keydown.space.self.prevent="clickable && emit('click', $event)" :class="{'hover-effect': hoverEffect, 'round-img': roundImg, 'clickable': clickable }")
        .left
            slot(name="icon")
                img(:src="icon" v-if="icon" :style="{ 'image-rendering': pixelatedIcon ? 'pixelated' : 'auto' }")
            .text
                .title {{ title }}
                .subtitle {{ subtitle }}
        .right(@click.stop)
            slot(name="actions")
            a(v-if="btn" :href="btn.link"): PButton(:inline="true") {{btn.text}}

</template>

<style lang="scss" scoped>
.gameinfo-container {
  display: flex;
  min-height: var(--list-min-height, auto);
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  padding: 6px;
  border-radius: 4px;
  gap: 6px;
  transition: background-color 0.4s ease;

  &.clickable {
    transition:
      scale 0.2s,
      background-color 0.4s ease;
  }

  &.hover-effect:hover {
    background-color: var(--color-tint-lighter);
  }

  &.clickable:active {
    scale: 0.98;
    background-color: var(--half-transparent-blue);
  }

  > .right {
    display: flex;
    align-items: center;
    gap: var(--list-actions-gap, 6px);
    > a {
      > i {
        transition: color 0.4s;
        color: var(--color-tint-light);

        &:hover {
          color: var(--color-tint);
        }
      }

      svg {
        width: 15px;
      }
    }
  }

  > .left {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 6px;
    min-width: 0;

    > .text {
      min-width: 0;
      overflow-wrap: anywhere;
    }

    > img {
      width: var(--list-icon-size, 30px);
      height: var(--list-icon-size, 30px);
      border-radius: var(--list-icon-radius, 0);
      /* fix #3 图标模糊 */
      /* image-rendering: pixelated; */
    }
  }

  &.round-img > .left > img {
    border-radius: 50%;
  }

  &.hover-effect:hover > .right {
    opacity: 1;
  }
}

.title {
  font-size: var(--list-title-size, 13px);
  line-height: 1.1rem;
}

.subtitle {
  font-size: 11px;
  color: var(--color-text-grey);
  line-height: normal;
}
</style>
