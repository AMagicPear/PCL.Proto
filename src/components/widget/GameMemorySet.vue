<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import { useModal } from '@/composables/useModal'
import { ref, watch } from 'vue'

const picked = ref('auto')
const checked = ref(false)

watch(checked, async (value) => {
  if (value == true) {
    let result = await useModal().open({
      title: pclText('Setup.Launch.Options.Visibility.CloseImmediately.Warning.Title'),
      content: pclText('Proto.Memory.OptimizeNotice'),
    })
    if (result.confirmed == false) {
      checked.value = false
    }
  }
})
</script>

<template>
  <div class="option">
    <input class="checkable" type="radio" id="one" value="auto" v-model="picked" />
    <label for="one">{{ pclText('Setup.Launch.Memory.Auto') }}</label>
  </div>
  <div class="option">
    <input class="checkable" type="radio" id="two" value="custom" v-model="picked" />
    <label for="two">{{ pclText('Common.Option.Customize') }}</label>
  </div>
  <div class="option">
    <input type="checkbox" id="checkbox" class="checkable" v-model="checked" />
    <label for="checkbox">{{ pclText('Proto.Memory.OptimizeBeforeLaunch') }}</label>
  </div>
</template>
