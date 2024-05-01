<script setup lang="ts">
import axios from "axios";
import {onMounted, ref} from "vue";
onMounted(() => {
  QQ_info(props.qq)
})
let props = defineProps(['qq'])
interface QqInfo {
  "user_id"?: number,
  "nickname"?: string,
  "sex"?: string,
  "age"?: number,
  "area"?: string
}
async function QQ_info(qq?: string) {
  await axios.get('https://api.oioweb.cn/api/qq/info?qq=' + qq).then( r => {
    info.value = r.data.result
  })
}
const info = ref<QqInfo>({})
</script>

<template>
{{info.nickname}}
</template>

<style scoped>

</style>