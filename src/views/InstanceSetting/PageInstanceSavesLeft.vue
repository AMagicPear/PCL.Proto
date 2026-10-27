<script setup lang="ts">
import SideNavLayout from '@/layout/SideNavLayout.vue'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import { computed, h } from 'vue'
import { pclText } from '@/api/pclLocale'
import sideTip from '@/composables/sideTip'
const text = pclText
const groups = computed(() => [
  {
    content: [
      {
        text: text('Instance.Saves.Left.Info'),
        linkto: '/instance_save/info',
        icon: () => h(PSvgIcon, { name: 'lucide/info' }),
      },
      {
        text: text('Instance.Saves.Left.Datapack'),
        linkto: '/instance_save/datapack',
        icon: () => h(PSvgIcon, { name: 'lucide/file-archive' }),
      },
    ],
  },
  {
    title: text('Instance.Saves.Left.QuickActions'),
    content: [
      {
        text: text('Common.Action.OpenFolder'),
        icon: () => h(PSvgIcon, { name: 'lucide/folder-open' }),
        clickCallback: () => sideTip.show(pclText('Proto.NotImplemented.Action')),
      },
    ],
  },
])
</script>
<template><SideNavLayout :side-nav-groups="groups" /></template>
