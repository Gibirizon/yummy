// messageStore.js
import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useMessageStore = defineStore('message', () => {
  const text = ref('')
  const type = ref('info')
  const duration = ref(5000)
  const isVisible = ref(false)

  function showMessage(newText, newType = 'info', newDuration = 5000) {
    text.value = newText
    type.value = newType
    duration.value = newDuration
    isVisible.value = true
  }

  function hideMessage() {
    isVisible.value = false
  }

  return {
    text,
    type,
    duration,
    isVisible,
    showMessage,
    hideMessage
  }
})
