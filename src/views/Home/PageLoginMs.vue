<script setup lang="ts">
import { PMsgLogin } from '@/components/widget/PMsgLogin'
import PButton from '@/components/widget/PButton.vue'
import PPath from '@/components/widget/PPath.vue'
import paths from '@/assets/vector-paths.json'
const loginShape = paths['05803c336117c503'].data
import type { ProfileView } from '@/types/profile'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
import { openExternal } from '@/util/externalLink'
const emit = defineEmits<{ view: [view: ProfileView] }>()
const text = pclText
defineExpose({ showLoginDialog: PMsgLogin })
function login() {
  sideTip.show(pclText('Proto.NotImplemented.Microsoft'))
}
</script>
<template>
  <section class="ms-login">
    <PPath :data="loginShape" class="login-avatar" /><PButton type="tint" @click="login">{{
      text('Launch.Account.Microsoft.Start')
    }}</PButton
    ><PButton inline @click="() => openExternal('https://www.minecraft.net')">{{
      text('Launch.Account.Microsoft.Website')
    }}</PButton
    ><PButton
      inline
      @click="
        () => openExternal('https://www.minecraft.net/store/minecraft-java-bedrock-edition-pc')
      "
      >{{ text('Launch.Account.Microsoft.Purchase') }}</PButton
    ><PButton inline @click="() => emit('view', 'profiles')">{{
      text('Launch.Account.Back')
    }}</PButton>
  </section>
</template>
<style scoped>
.ms-login {
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: center;
  width: 100%;
}
.ms-login > button {
  width: 100%;
}
.login-avatar {
  width: 40px;
  height: 40px;
  opacity: 0.7;
}
</style>
