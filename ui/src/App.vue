<script setup lang="ts">
import {RouterView} from 'vue-router'
import HeaderNav from "@/components/index/HeaderNav.vue";
import {useCounterStore} from "@/stores/counter";
import FooterInfo from "@/components/index/FooterInfo.vue";
import HeaderImg from "@/components/index/HeaderImg.vue";

const counter = useCounterStore()
</script>

<template>
  <n-config-provider :theme="counter.theme">
    <n-layout style="
    width: 100%;
    min-height: 100vh;
    ">
      <HeaderNav/>
      <RouterView/>
      <FooterInfo/>
    </n-layout>
  </n-config-provider>
</template>
