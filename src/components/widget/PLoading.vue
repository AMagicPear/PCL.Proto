<script lang="ts" setup>
import { useTemplateRef, watchEffect } from 'vue'
import PCard from './PCard.vue'
import shapes from '@/assets/control-geometry.json'
import paths from '@/assets/vector-paths.json'
import { pclText } from '@/api/pclLocale'
const pickaxe = Object.values(paths).find((item) => item.data === shapes['MyLoading.PathPickaxe'])!

export type LoadingState = 'loading' | 'error'
const props = withDefaults(
  defineProps<{ state?: LoadingState; loadingText?: string; card?: boolean }>(),
  {
    state: 'loading',
    loadingText: undefined,
    card: true,
  },
)
const hammerRef = useTemplateRef<SVGGElement>('hammerRef')
const trainglesRef = useTemplateRef<SVGGElement>('trainglesRef')

watchEffect(() => {
  const playState = props.state === 'loading' ? 'running' : 'paused'
  hammerRef.value && (hammerRef.value.style.animationPlayState = playState)
  trainglesRef.value &&
    Array.from(trainglesRef.value?.children).forEach(
      (child) => ((child as SVGPathElement).style.animationPlayState = playState),
    )
})
</script>

<template>
  <component
    :is="card ? PCard : 'div'"
    :hide-title="true"
    :class="{ 'card-loading': card }"
    role="status"
  >
    <div class="loading-container">
      <svg
        width="60"
        height="47"
        viewBox="0 0 60 47"
        class="load-svg"
        :class="state"
        aria-hidden="true"
      >
        <path stroke="currentColor" stroke-width="2" d="M0 46H25" />
        <g class="hammer" ref="hammerRef">
          <svg x="10" y="6" width="35" height="35" :viewBox="pickaxe.viewBox" overflow="visible">
            <path
              :d="pickaxe.data"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              vector-effect="non-scaling-stroke"
            />
          </svg>
        </g>
        <g class="triangles" fill="currentColor" ref="trainglesRef">
          <path :d="shapes['MyLoading.PathLeft']" transform="translate(7 41) rotate(-45 1 2)" />
          <path :d="shapes['MyLoading.PathRight']" transform="translate(14 41) rotate(45 1 2)" />
        </g>
        <path
          class="wrong-x"
          fill="currentColor"
          :d="shapes['MyLoading.PathError'].replace(/^F1\s*/, '')"
          transform="translate(4 23) scale(.85)"
        />
      </svg>
      <p class="loading-text" :class="state">
        {{
          state === 'loading'
            ? loadingText || pclText('Application.Control.Loading.Default')
            : pclText('Application.Control.Loading.Failed')
        }}
      </p>
    </div>
  </component>
</template>

<style lang="css" scoped>
.card-loading {
  width: min(180px, 100%);
  margin: auto;
}

.card-loading .loading-container {
  min-height: 140px;
  padding: 20px 18px;
}

.loading-text {
  text-align: center;
  font-size: 14px;
  line-height: 1.5;
  margin: 0;
  overflow-wrap: anywhere;
}

@keyframes hammerAnim {
  0% {
    transform: rotate(55deg);
  }

  11% {
    transform: rotate(-20deg);
  }

  38% {
    transform: rotate(58deg);
  }

  46% {
    transform: rotate(49deg);
  }

  51% {
    transform: rotate(53deg);
  }

  73% {
    transform: rotate(51deg);
  }

  88% {
    transform: rotate(53deg);
  }

  100% {
    transform: rotate(55deg);
  }
}

@keyframes trianglesR {
  0%,
  10%,
  50%,
  100% {
    opacity: 0;
  }

  16% {
    opacity: 1;
  }

  20% {
    transform: translate(0, 0);
  }

  50% {
    transform: translate(6px, -6px);
  }
}

@keyframes trianglesL {
  0%,
  10%,
  50%,
  100% {
    opacity: 0;
  }

  16% {
    opacity: 1;
  }

  20% {
    transform: translate(0, 0);
  }

  50% {
    transform: translate(-6px, -6px);
  }
}

.hammer {
  transform-origin: 40px 36px;
  animation: hammerAnim 1.6s infinite ease-in;
}

.triangles > :nth-child(1) {
  animation: trianglesL 1.6s infinite ease-out;
}

.triangles > :nth-child(2) {
  animation: trianglesR 1.6s infinite ease-out;
}

.load-svg > .wrong-x {
  opacity: 1;
  transition: opacity 0.4s ease;
}

.load-svg.loading > .wrong-x {
  opacity: 0;
}

.loading-container {
  min-height: 160px;
  padding: 30px 24px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: center;
  justify-content: center;
}

.load-svg {
  flex-shrink: 0;
  overflow: visible;
  transition: color 0.4s ease;
}

.loading {
  color: var(--color-titlebar);
}

.error {
  color: var(--color-warn);
}
</style>
