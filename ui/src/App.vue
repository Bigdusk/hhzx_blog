<script setup lang="ts">
import {RouterView} from 'vue-router'
import HeaderNav from "@/components/index/HeaderNav.vue";
import {useCounterStore} from "@/stores/counter";
import FooterInfo from "@/components/index/FooterInfo.vue";
import { zhCN, dateZhCN } from 'naive-ui'
import {onMounted} from "vue";

const counter = useCounterStore()
//判断当前网站主题
function website_themes_init() {
  counter.theme = null
  counter.is_masks = 0
}
onMounted(() => {
  website_themes_init()
})
</script>

<template>

  <n-config-provider :theme="counter.theme"  :locale="zhCN" :date-locale="dateZhCN">
    <n-layout style="
    width: 100%;
    min-height: 100vh;
    ">
      <RouterView/>
    </n-layout>
  </n-config-provider>
</template>
