<script setup lang="ts">
import PSkin from '@/components/widget/PSkin.vue'
import PButton from '@/components/widget/PButton.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useAccountInfo } from '@/stores/account'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { pclText } from '@/api/pclLocale'
import PCard from '@/components/widget/PCard.vue'
import { usePageSettings } from '@/composables/usePageSettings'
const { act } = usePageSettings('PageLoginProfileSkin')
const account = useAccountInfo()
const router = useRouter()
const { locale } = useI18n()
const text = (key: string) => pclText(key, locale.value)
</script>
<template>
  <div class="profile-skin">
    <PSkin /><strong>{{ account.username || 'Player' }}</strong
    ><span>{{ text('Launch.Account.Type.Offline') }}</span>
    <PCard hide-title class="profile-actions-card"
      ><div class="profile-actions">
        <details>
          <summary
            :aria-label="text('Launch.Account.ProfileSkin.SkinsAndCapes')"
            :title="text('Launch.Account.ProfileSkin.SkinsAndCapes')"
          >
            <PSvgIcon name="lucide/shirt" />
          </summary>
          <div class="profile-menu">
            <PButton
              v-for="action in ['ChangeSkin', 'SaveSkin', 'RefreshSkin', 'ChangeCape']"
              :key="action"
              inline
              :click="() => act(`Btn${action}`)"
              >{{ text(`Launch.Account.ProfileSkin.${action}`) }}</PButton
            >
          </div>
        </details>
        <details>
          <summary
            :aria-label="text('Launch.Account.ProfileSkin.EditProfile')"
            :title="text('Launch.Account.ProfileSkin.EditProfile')"
          >
            <PSvgIcon name="lucide/pencil" />
          </summary>
          <div class="profile-menu">
            <PButton inline :click="() => act('BtnEditPassword')">{{
              text('Launch.Account.ProfileSkin.EditPassword')
            }}</PButton
            ><PButton inline :click="() => router.push('/account/offline')">{{
              text('Launch.Account.ProfileSkin.EditUsername')
            }}</PButton>
          </div>
        </details>
        <PButton
          inline
          :click="() => router.push('/account')"
          :title="text('Launch.Account.ProfileSkin.SwitchProfile')"
          ><PSvgIcon name="lucide/arrow-right-left"
        /></PButton></div
    ></PCard>
  </div>
</template>
<style scoped>
.profile-skin {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}
.profile-skin strong {
  font-size: 17px;
  font-weight: normal;
}
.profile-skin > span {
  color: var(--color-text-grey);
  font-size: 12px;
}
.profile-actions {
  display: flex;
  gap: 9px;
  margin-top: 8px;
  padding: 3px 10px;
  position: relative;
}
.profile-actions-card {
  overflow: visible;
}
.profile-actions-card :deep(.mycard-content) {
  overflow: visible;
}
summary {
  cursor: pointer;
  list-style: none;
  display: flex;
  padding: 6px 0;
}
.profile-menu {
  position: absolute;
  z-index: 3;
  left: 0;
  top: 100%;
  min-width: 150px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 6px;
  background: var(--color-background-soft);
  border-radius: 6px;
  box-shadow: var(--box-shadow);
}
</style>
