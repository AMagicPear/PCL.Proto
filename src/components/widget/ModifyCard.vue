<script setup lang="ts">
import { pclText } from '@/api/pclLocale'
import PCard from '@/components/widget/PCard.vue'
import { pluginShowText, showIconPath, type pluginType, type showIconType } from '@/types/gameInfo'
import PListItem from './PListItem.vue'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useTemplateRef } from 'vue'
import type { McPluginReport } from '@/types/mcPlugin'
// import { useSelectedInstance } from '@/stores/gameLaunch'
// import { computed } from 'vue'
const emit = defineEmits<{
  selectVersion: [type: pluginType, versionId: string]
}>()
const cardRef = useTemplateRef('cardRef')
const props = defineProps<{
  plugin: pluginType
  // versions为undefined时是还没加载，为null时是加载错误，为空列表时是正常得到无可用版本的结果
  versions: McPluginReport[] | undefined | null
  notCompatibleWith?: pluginType
  isLoading: boolean
  iconType: showIconType
}>()

function select(versionId: string) {
  emit('selectVersion', props.plugin, versionId)
  selectedVersionId.value = versionId
  cardRef.value?.SwitchFoldState()
}

const selectedVersionId = ref<string>()
onMounted(() => {
  if (props.plugin == 'vanilla' && props.versions) {
    selectedVersionId.value = props.versions[0]?.version
  }
})
</script>

<template>
  <PCard :default-fold-status="plugin == 'vanilla' ? 'unfoldable' : 'fold'" ref="cardRef">
    <template #title> {{ pluginShowText[plugin] }}</template>
    <template #description>
      <div v-if="notCompatibleWith" class="sub">
        <span>{{
          pclText('Download.Install.Compat.IncompatibleWithLoader', [notCompatibleWith])
        }}</span>
      </div>
      <div v-else-if="isLoading" class="sub">
        <span>{{ pclText('Application.Control.Loading.Default') }}</span>
      </div>
      <div v-else-if="selectedVersionId" class="version">
        <img :src="showIconPath[iconType]" />
        <span>{{ selectedVersionId }} </span>
      </div>
      <div v-else-if="versions" class="sub">
        <span>{{ pclText('Proto.Download.SelectVersion') }}</span>
      </div>
      <div v-else class="sub">
        <span>{{ pclText('Proto.Download.LoadError') }}</span>
      </div>
    </template>
    <template #content v-if="plugin != 'vanilla' && versions">
      <PListItem
        v-for="version in versions"
        :title="version.version"
        :subtitle="
          version.stable == null
            ? undefined
            : version.stable
              ? pclText('Download.Version.Type.Stable')
              : pclText('Proto.Label.Development')
        "
        :click="() => select(version.version)"
        :icon="showIconPath[iconType]"
      />
    </template>
  </PCard>
</template>

<style scoped>
.version {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 6px;
}

span {
  text-align: left;
  font-size: 12px;
}

.sub > span {
  color: var(--color-text-grey);
}
img {
  width: 18px;
  height: 18px;
}
</style>
