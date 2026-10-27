import { pclText } from '@/api/pclLocale'
import { FloatButtonType, useFloatButton } from '@/composables/useFloatButton'
import { Channel, invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export enum TaskStatus {
  Pending = 0,
  Running = 1,
  Completed = 2,
  Failed = 3,
}

export class Task {
  id: number
  versionId: string
  status: TaskStatus
  speed: number
  items: TaskItem[]
  progress: number

  get name() {
    return pclText('Proto.Task.Download', [this.versionId])
  }

  constructor(id: number, version_id: string) {
    this.id = id
    this.versionId = version_id
    this.status = TaskStatus.Pending
    this.speed = 0
    this.items = [
      new TaskItem(0, 'Proto.Task.Manifest', id, 1),
      new TaskItem(1, 'Proto.Task.Client', id, 1),
      new TaskItem(2, 'Proto.Task.Libraries', id),
      new TaskItem(3, 'Minecraft.Download.Stage.DownloadAssets', id),
    ]
    this.progress = 0
  }
}

export interface ITaskItem {
  id: number
  task_id: number
  name: string
  status: TaskStatus
  speed: number | null
  progress: number
  remaining: number | undefined
}

class TaskItem implements ITaskItem {
  id: number
  task_id: number
  nameKey: string
  status: TaskStatus
  speed: number | null
  progress: number
  remaining: number | undefined

  get name() {
    return pclText(this.nameKey)
  }

  constructor(id: number, name: string, task_id: number, files_num?: number) {
    this.id = id
    this.nameKey = name
    this.task_id = task_id
    this.status = TaskStatus.Pending
    this.progress = 0
    this.remaining = files_num
    this.speed = null
  }

  update(event: TaskItemReport) {
    this.status = event.status
    this.progress = event.progress
    this.remaining = event.files_remaining
    this.speed = event.speed
  }
}

interface TaskItemReport {
  item_id: number
  task_id: number
  files_remaining: number
  progress: number
  status: TaskStatus
  speed: number | null
}

export const useTaskManager = defineStore('task-manager', () => {
  const tasks = ref<Task[]>([])
  let current_taskid = -1
  const totalProgress = computed(() => {
    return (
      (tasks.value.reduce((acc, task) => {
        return acc + task.progress
      }, 0) /
        (tasks.value.length || 1)) *
      100
    ).toFixed(2)
  })
  const totalSpeed = computed(() => {
    return tasks.value.reduce((acc, task) => {
      return acc + task.speed
    }, 0)
  })
  const totalRemaining = computed(() => {
    return tasks.value.reduce((acc, task) => {
      return acc + task.items.reduce((sum, item) => sum + (item.remaining || 0), 0)
    }, 0)
  })
  const activeTaskCount = computed(() => {
    return tasks.value.filter(
      (task) => task.status !== TaskStatus.Completed && task.status !== TaskStatus.Failed,
    ).length
  })
  const floatButton = useFloatButton()

  async function startDownloadMCVersion(version_id: string, instance_name: string) {
    let newTask = new Task(++current_taskid, version_id)
    tasks.value.push(newTask)
    const onEvent = new Channel<TaskItemReport>()
    onEvent.onmessage = (message) => {
      const taskToUpdate = tasks.value[message.task_id]
      if (taskToUpdate) {
        const itemToUpdate = taskToUpdate.items[message.item_id]
        if (itemToUpdate) {
          itemToUpdate.update(message)
          if (taskToUpdate.items.every((item) => item.status === TaskStatus.Completed)) {
            taskToUpdate.status = TaskStatus.Completed
          }
          taskToUpdate.progress =
            taskToUpdate.items.reduce((total, item) => total + item.progress, 0) /
            taskToUpdate.items.length
          taskToUpdate.speed = taskToUpdate.items.reduce(
            (total, item) => total + (item.speed || 0),
            0,
          )
        }
      }
    }
    try {
      await invoke('download_minecraft_version', {
        on_event: onEvent,
        version_id,
        task_id: current_taskid,
        instance_name,
      })
    } catch (err) {
      const failedTask = tasks.value.find((task) => task.id === newTask.id)!
      failedTask.status = TaskStatus.Failed
      failedTask.items
        .filter((item) => item.status !== TaskStatus.Completed)
        .forEach((item) => (item.status = TaskStatus.Failed))
      throw err
    }
    console.log('download task completed', tasks.value)
    if (
      activeTaskCount.value <= 0 ||
      floatButton.floatButtonState.type === FloatButtonType.TaskManage
    ) {
      floatButton.floatButtonState.visible = false
    }
  }

  return {
    tasks,
    totalProgress,
    totalSpeed,
    totalRemaining,
    startDownloadMCVersion,
    activeTaskCount,
  }
})
