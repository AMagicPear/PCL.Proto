import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  getJavaList,
  getJavaSettings,
  refreshJavaList,
  setGlobalJava,
  setJavaEnabled,
  setInstanceJavaSettings,
  type IJavaRuntimeInfo,
  type InstanceJavaSettings,
} from '@/api/javaReq'

export const useJavaStore = defineStore('java', () => {
  const runtimes = ref<IJavaRuntimeInfo[]>([])
  const globalJava = ref<string | null>(null)
  const globalIs64Bit = ref(true)
  const instanceIs64Bit = ref<Record<string, boolean>>({})
  const instances = ref<Record<string, InstanceJavaSettings>>({})
  let request = 0
  async function load(directory?: string, refresh = false) {
    const current = ++request
    let list = await (refresh ? refreshJavaList() : getJavaList())
    if (!list.length && !refresh) list = await refreshJavaList()
    if (current !== request) return
    const settings = await getJavaSettings(directory)
    if (current !== request) return
    runtimes.value = list
    globalJava.value = settings.global_java
    globalIs64Bit.value = settings.global_is_64_bit
    if (directory) {
      instances.value[directory] = settings.instance
      instanceIs64Bit.value[directory] = settings.instance_is_64_bit
    }
  }
  async function selectGlobal(path: string | null) {
    await setGlobalJava(path)
    request++
    globalJava.value = path
    globalIs64Bit.value = (await getJavaSettings()).global_is_64_bit
  }
  async function setEnabled(runtime: IJavaRuntimeInfo) {
    await setJavaEnabled(runtime.java_exe, !runtime.is_enabled)
    request++
    runtime.is_enabled = !runtime.is_enabled
  }
  async function saveInstance(directory: string, settings: InstanceJavaSettings) {
    await setInstanceJavaSettings(directory, settings)
    request++
    instances.value[directory] = settings
    instanceIs64Bit.value[directory] = (await getJavaSettings(directory)).instance_is_64_bit
  }
  return {
    runtimes,
    globalJava,
    globalIs64Bit,
    instances,
    instanceIs64Bit,
    load,
    selectGlobal,
    setEnabled,
    saveInstance,
  }
})
