<script setup lang="ts">
import {RouterView} from "vue-router";
import HeaderNav from "@/components/index/HeaderNav.vue";
import FooterInfo from "@/components/index/FooterInfo.vue";
</script>
<template>
  <HeaderNav/>
  <RouterView/>
  <FooterInfo/>
</template>
