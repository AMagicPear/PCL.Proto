<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import PButton from '@/components/widget/PButton.vue'
import PCard from '@/components/widget/PCard.vue'

const openArticle = () =>
  window.open('https://tailscale.com/blog/how-nat-traversal-works', '_blank', 'noopener,noreferrer')
const topics = ['Join', 'Start', 'Server', 'Identity', 'Launchers', 'Nat']
</script>
<template>
  <section class="pcl-page help-page">
    <PCard v-for="topic in topics" :key="topic" :title="pclText(`Proto.Help.${topic}.Title`)">
      <div class="help-content">
        <p>{{ pclText(`Proto.Help.${topic}.Body`) }}</p>
        <PButton v-if="topic === 'Nat'" @click="openArticle">{{
          pclText('Proto.Help.Nat.Article')
        }}</PButton>
      </div>
    </PCard>
  </section>
</template>
<style scoped>
.help-page {
  padding: 25px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}
.help-content {
  padding: 0 25px 15px;
  white-space: pre-line;
}
</style>
