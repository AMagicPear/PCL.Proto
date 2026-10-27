<script setup lang="ts">
import { useAccountInfo } from '@/stores/account'
import PListItem from '@/components/widget/PListItem.vue'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'
import PHint from '@/components/widget/PHint.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'
const account = useAccountInfo()
const router = useRouter()
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
defineProps<{ createAndSelect?: boolean; requireMicrosoft?: boolean }>()
</script>
<template>
  <section class="account-list">
    <PHint severity="info" :closable="!createAndSelect">{{
      text(
        createAndSelect
          ? 'Launch.Account.Profile.CreateAndSelectHint'
          : 'Launch.Account.Profile.SelectHint',
      )
    }}</PHint>
    <PHint v-if="requireMicrosoft" severity="warning">{{
      text('Launch.Account.Profile.RequireMicrosoftHint')
    }}</PHint>
    <PListItem
      v-if="account.username"
      :title="account.username"
      :subtitle="text('Launch.Account.Type.Offline')"
      :click="() => router.push('/home')"
    />
    <PCard hide-title class="create-card">
      <details>
        <summary
          :title="text('Launch.Account.Profile.Create')"
          :aria-label="text('Launch.Account.Profile.Create')"
        >
          <PSvgIcon name="lucide/user-plus" />
        </summary>
        <div class="account-types">
          <p>{{ text('Launch.Account.Profile.Create.SelectAuthType.Title') }}</p>
          <PButton v-if="!requireMicrosoft" inline :click="() => router.push('/account/offline')">{{
            text('Launch.Account.Type.Offline')
          }}</PButton
          ><PButton inline :click="() => router.push('/account/microsoft')">{{
            text('Launch.Account.Type.Microsoft')
          }}</PButton
          ><PButton v-if="!requireMicrosoft" inline :click="() => router.push('/account/auth')">{{
            text('Launch.Account.Type.ThirdParty')
          }}</PButton>
        </div>
      </details>
    </PCard>
  </section>
</template>
<style scoped>
.account-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}
.account-types {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  width: 230px;
  background: var(--color-background);
  box-shadow: var(--box-shadow);
  border-radius: 5px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
  gap: 9px;
}
.create-card {
  align-self: center;
  margin-top: auto;
  overflow: visible;
}
.create-card :deep(.mycard-content) {
  overflow: visible;
}
details {
  position: relative;
}
summary {
  cursor: pointer;
  list-style: none;
  padding: 8px 12px;
}
</style>
