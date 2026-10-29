<script setup lang="ts">
import { showUnavailable } from '@/util/showUnavailable'
import PSkin from '@/components/widget/PSkin.vue'
import PButton from '@/components/widget/PButton.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { useAccountInfo } from '@/stores/account'
import { useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import PCard from '@/components/widget/PCard.vue'

const account = useAccountInfo()
const router = useRouter()
const text = pclText
</script>
<template>
  <div class="profile-skin">
    <PSkin class="profile-avatar" /><strong>{{ account.username || 'Player' }}</strong
    ><span>{{ text('Launch.Account.Type.Offline') }}</span>
    <div class="profile-actions-reveal">
      <PCard hide-title class="profile-actions-card" overflow-visible
        ><div class="profile-actions">
          <details>
            <summary
              class="profile-action"
              :aria-label="text('Launch.Account.ProfileSkin.SkinsAndCapes')"
              :title="text('Launch.Account.ProfileSkin.SkinsAndCapes')"
            >
              <PSvgIcon name="lucide/shirt" :size="14" />
            </summary>
            <div class="profile-menu">
              <PButton
                v-for="action in ['ChangeSkin', 'SaveSkin', 'RefreshSkin', 'ChangeCape']"
                :key="action"
                inline
                @click="showUnavailable"
                >{{ text(`Launch.Account.ProfileSkin.${action}`) }}</PButton
              >
            </div>
          </details>
          <details>
            <summary
              class="profile-action"
              :aria-label="text('Launch.Account.ProfileSkin.EditProfile')"
              :title="text('Launch.Account.ProfileSkin.EditProfile')"
            >
              <PSvgIcon name="lucide/pencil" :size="14" />
            </summary>
            <div class="profile-menu">
              <PButton inline @click="showUnavailable">{{
                text('Launch.Account.ProfileSkin.EditPassword')
              }}</PButton
              ><PButton inline @click="() => router.push('/account/offline')">{{
                text('Launch.Account.ProfileSkin.EditUsername')
              }}</PButton>
            </div>
          </details>
          <button
            type="button"
            class="profile-action"
            @click="router.push('/account')"
            :aria-label="text('Launch.Account.ProfileSkin.SwitchProfile')"
            :title="text('Launch.Account.ProfileSkin.SwitchProfile')"
          >
            <PSvgIcon name="lucide/arrow-right-left" :size="14" />
          </button></div
      ></PCard>
    </div>
  </div>
</template>
<style scoped>
.profile-skin {
  position: relative;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  transform: translateY(10px);
}
.profile-skin::before {
  content: '';
  position: absolute;
  inset: -16px 0 0;
}
.profile-avatar {
  margin: 0;
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
  align-items: center;
  gap: 9px;
  padding: 3px 9px 3px 10px;
  position: relative;
}
.profile-actions-reveal {
  position: absolute;
  top: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  padding-top: 8px;
  opacity: 0;
  transition: opacity 0.15s;
}
.profile-skin:hover .profile-actions-reveal,
.profile-skin:focus-within .profile-actions-reveal,
.profile-skin:has(details[open]) .profile-actions-reveal {
  opacity: 1;
}
.profile-actions-card {
  border-radius: 5px;
}
.profile-action {
  width: 24px;
  height: 24px;
  padding: 5px;
  border: 0;
  border-radius: 50%;
  background: transparent;
  color: var(--color-tint-light, var(--color-tint));
  cursor: pointer;
  list-style: none;
  display: flex;
  align-items: center;
  justify-content: center;
  transition:
    color 0.15s,
    transform 0.25s;
}
.profile-action::-webkit-details-marker {
  display: none;
}
.profile-action:hover,
.profile-action:focus-visible {
  color: var(--color-tint);
}
.profile-action:active {
  transform: scale(0.8);
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
