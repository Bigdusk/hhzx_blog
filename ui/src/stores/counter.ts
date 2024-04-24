import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import type {GlobalTheme} from "naive-ui";

export const useCounterStore = defineStore('counter', () => {

  const theme = ref<GlobalTheme | null>()

  const nav_is_show = ref(true)

  return { theme }
})
