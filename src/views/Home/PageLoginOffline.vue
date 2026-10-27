<script setup lang="ts">
import { ref } from 'vue'
import PTextBox from '@/components/widget/PTextBox.vue'
import PButton from '@/components/widget/PButton.vue'
import { useAccountInfo } from '@/stores/account'
import { useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
const router = useRouter()
const text = pclText
const username = ref(useAccountInfo().username || '')
const uuidType = ref('standard')
const uuid = ref('')
function create() {
  if (!/^[A-Za-z0-9_]{3,16}$/.test(username.value)) {
    sideTip.show(text('Launch.Account.Profile.EditPlayerId.NotAllowed'), 'warn')
    return
  }
  if (uuidType.value === 'custom' && !/^[a-f0-9]{32}$/i.test(uuid.value)) {
    sideTip.show(text('Launch.Account.Offline.Uuid.Custom.Input.ToolTip'), 'warn')
    return
  }
  sideTip.show(pclText('Proto.NotImplemented.Profile'))
}
</script>
<template>
  <form class="offline-form" @submit.prevent="create">
    <label>{{ text('Launch.Account.Offline.PlayerId') }}<PTextBox v-model="username" /></label
    ><strong>{{ text('Launch.Account.Offline.UuidStandard') }}</strong>
    <div class="uuid-types">
      <label v-for="kind in ['standard', 'legacy', 'custom']" :key="kind"
        ><input type="radio" v-model="uuidType" :value="kind" />{{
          text(
            kind === 'custom'
              ? 'Common.Option.Customize'
              : `Launch.Account.Offline.Uuid.${kind === 'standard' ? 'Standard' : 'Legacy'}`,
          )
        }}</label
      >
    </div>
    <label v-if="uuidType === 'custom'">UUID<PTextBox v-model="uuid" /></label>
    <div class="form-actions">
      <PButton :click="() => router.push('/account')">{{
        text('Launch.Account.Offline.Back')
      }}</PButton
      ><PButton type="tint" :click="create">{{ text('Launch.Account.Offline.Create') }}</PButton>
    </div>
  </form>
</template>
<style scoped>
.offline-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}
.offline-form > label {
  display: flex;
  gap: 10px;
  align-items: center;
}
.offline-form > label > :last-child {
  flex: 1;
}
.offline-form > strong {
  text-align: center;
}
.uuid-types,
.form-actions {
  display: flex;
  justify-content: center;
  gap: 12px;
}
</style>
