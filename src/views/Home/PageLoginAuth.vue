<script setup lang="ts">
import { ref } from 'vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import PButton from '@/components/widget/PButton.vue'
import { useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
import PPath from '@/components/widget/PPath.vue'
import paths from '@/assets/vector-paths.json'
const loginShape = paths.a734eac303dcf223.data
const props = defineProps<{ website?: string; registerUrl?: string }>()
const oauthMode = ref(false)
function open(url: string | undefined) {
  if (url && /^https?:\/\//i.test(url)) window.open(url, '_blank', 'noopener,noreferrer')
}
const router = useRouter()
const text = pclText
const server = ref('')
const username = ref('')
const password = ref('')
function login() {
  if (!/^https?:\/\//i.test(server.value)) {
    sideTip.show(text('Launch.Account.Auth.InvalidServer'), 'warn')
    return
  }
  if (!oauthMode.value && (!username.value || !password.value)) {
    sideTip.show(text('Launch.Account.Auth.EmptyFields'), 'warn')
    return
  }
  sideTip.show(pclText('Proto.NotImplemented.Auth'))
}
</script>
<template>
  <form class="auth-form" @submit.prevent="login">
    <label
      >{{ text('Launch.Account.Auth.SelectServer')
      }}<PTextBox v-model="server" placeholder="https://" /></label
    ><template v-if="!oauthMode"
      ><label>{{ text('Launch.Account.Auth.Email') }}<PTextBox v-model="username" /></label
      ><label
        >{{ text('Launch.Account.Auth.Password')
        }}<input type="password" v-model="password" autocomplete="off"
      /></label>
      <PButton v-if="registerUrl" inline :click="() => open(registerUrl)">{{
        text('Launch.Account.Auth.Register')
      }}</PButton>
      <PButton inline :click="() => (oauthMode = true)">{{
        text('Launch.Account.Auth.UseOAuth')
      }}</PButton>
    </template>
    <div v-else class="oauth-login">
      <PPath :data="loginShape" /><PButton type="tint" :click="login">{{
        text('Launch.Account.Auth.OAuthLogin')
      }}</PButton
      ><PButton inline :click="() => (oauthMode = false)">{{
        text('Launch.Account.Auth.UsePassword')
      }}</PButton>
    </div>
    <PButton v-if="website" inline :click="() => open(website)">{{
      text('Launch.Account.Microsoft.Website')
    }}</PButton>
    <div>
      <PButton :click="() => router.push('/account')">{{
        text('Launch.Account.Auth.Back')
      }}</PButton
      ><PButton v-if="!oauthMode" type="tint" :click="login">{{
        text('Launch.Account.Auth.Login')
      }}</PButton>
    </div>
  </form>
</template>
<style scoped>
.auth-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}
.auth-form label {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.auth-form > div {
  display: flex;
  justify-content: center;
  gap: 12px;
}
.auth-form > .oauth-login {
  flex-direction: column;
  align-items: center;
  gap: 15px;
}
.oauth-login > svg {
  width: 40px;
  height: 40px;
  opacity: 0.7;
}
.auth-form input {
  padding: 6px 8px;
  border: 1px solid var(--color-tint-lightist);
  border-radius: 4px;
  color: inherit;
  background: transparent;
}
</style>
