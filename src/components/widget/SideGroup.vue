<script lang="ts" setup>
defineProps<INavItemGroup>()
import SideNavItem from './SideNavItem.vue'
import { type INavItemGroup } from '@/types/naviOptions'
</script>

<template lang="pug">
    div
        //- 添加 sidenav-line 类是为了方便外部做动画的
        p.side-group-title.sidenav-line(v-if="title") {{title}}
        .side-group-content
            SideNavItem(
                v-for="item in content" 
                v-bind="item")
</template>

<style lang="css" scoped>
.side-group-content {
  display: flex;
  flex-direction: column;
}

.side-group-title {
  margin: 6px 5px 4px 13px;
  font-size: 12px;
  color: var(--color-text-grey);
}
</style>
