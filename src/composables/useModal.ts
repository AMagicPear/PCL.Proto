import { computed, ref, type Ref } from 'vue'
import { type ModalApi, type ModalOptions, type ModalResult } from '@/types/modal'

const state = ref<{ isOpen: boolean; options: ModalOptions }>({ isOpen: false, options: {} })

const resolvePromise: Ref<((value: ModalResult) => void) | null> = ref(null)
const inputValue = ref<string>()
const selectedValue = ref<string>()

export function useModal(): ModalApi {
  const open = (options: ModalOptions): Promise<ModalResult> => {
    state.value.options = options
    state.value.isOpen = true
    inputValue.value = options.defaultInputText
    selectedValue.value = undefined

    return new Promise((resolve) => {
      resolvePromise.value = resolve
    })
  }

  const close = (confirmed: boolean = false): void => {
    state.value.isOpen = false
    if (resolvePromise.value) {
      resolvePromise.value({
        confirmed,
        input: inputValue.value,
        selected: selectedValue.value,
      })
      resolvePromise.value = null
    }
  }

  return {
    isOpen: computed(() => state.value.isOpen),
    options: computed(() => state.value.options),
    inputValue,
    selectedValue,
    open,
    close,
  }
}
