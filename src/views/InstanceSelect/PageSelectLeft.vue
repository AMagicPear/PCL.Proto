<script setup lang="ts">
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import sideTip from '@/composables/sideTip'
import SideNavLayout from '@/layout/SideNavLayout.vue'
import { useRepositoriesStore } from '@/stores/repositories'
import { computed, h, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { pclText } from '@/api/pclLocale'
import { isTauri } from '@tauri-apps/api/core'
const repo = useRepositoriesStore()
const router = useRouter()
const route = useRoute()
const text = pclText
async function add() {
  try {
    await repo.addNew()
  } catch (err) {
    sideTip.show(pclText('Proto.Error.AddFolder', [String(err)]), 'warn')
  }
}
const groups = computed(() => [
  {
    title: text('Select.Folder.ListTitle'),
    content:
      repo.repositories?.map((item, index) => ({
        text: item.name,
        linkto: `/instance_select/instance_select_sub/${index}`,
      })) || [],
  },
  {
    title: text('Select.Folder.AddOrImport'),
    content: [
      {
        text: text('Select.Folder.AddExisting.Title'),
        icon: () => h(PSvgIcon, { name: 'lucide/folder-plus' }),
        clickCallback: add,
      },
      {
        text: text('Select.Folder.ImportModpack.Title'),
        icon: () => h(PSvgIcon, { name: 'lucide/package' }),
        clickCallback: () => sideTip.show(pclText('Proto.NotImplemented.Action')),
      },
    ],
  },
])
onMounted(async () => {
  try {
    if (!isTauri()) return
    if (!repo.repositories) await repo.fetchFromBackend()
    if (repo.repositories?.length && !route.params.repository)
      await router.replace('/instance_select/instance_select_sub/0')
  } catch (err) {
    sideTip.show(pclText('Proto.Error.Folders', [String(err)]), 'warn')
  }
})
</script>
<template><SideNavLayout :side-nav-groups="groups" /></template>
