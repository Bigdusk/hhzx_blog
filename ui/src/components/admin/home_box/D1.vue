<script setup lang="ts">
import {onMounted, ref} from "vue";
import axios_util from "@/utils/axios_util";

onMounted(() => {
  all_count()
})
//查询数据数量
const count_data = ref([])
async function all_count() {
  axios_util.get('/info_count').then(r => {
    count_data.value = r.data
  })
}
</script>

<template>
  <div>
    <n-card title="文章总数" hoverable>
      <n-statistic tabular-nums>
        <n-number-animation ref="numberAnimationInstRef" :from="0" :to="count_data[0]" />
      </n-statistic>
    </n-card>
    <n-card title="分类总数" hoverable>
      <n-statistic tabular-nums>
        <n-number-animation ref="numberAnimationInstRef" :from="0" :to="count_data[1]" />
      </n-statistic>
    </n-card>
    <n-card title="标签总数" hoverable>
      <n-statistic tabular-nums>
        <n-number-animation ref="numberAnimationInstRef" :from="0" :to="count_data[2]" />
      </n-statistic>
    </n-card>
  </div>
</template>

<style scoped>
@media screen and (max-width: 980px){
  div {
    display: none;
  }
}
</style>