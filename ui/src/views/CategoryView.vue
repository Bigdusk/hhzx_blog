<script setup lang="ts">

import {CalendarOutline, EyeOutline, PersonOutline, PricetagOutline} from "@vicons/ionicons5";
</script>

<template>

  <div class="img-box">
    <div>
      <h1>分类</h1>
      <n-divider vertical />人生如逆旅，我亦是行人。
      <p>子分类</p>
    </div>
  </div>

  <div class="m-box">

    <div class="a-box">

      <div class="a-list-box">
        <n-card style="margin-bottom: 32px;" v-for="i in 10" title="卡片" embedded hoverable>
          <n-space>
            <n-icon :component="PersonOutline" /> 1
            <n-icon :component="PricetagOutline" /> 2
            <n-icon :component="CalendarOutline" /> 3
            <n-icon :component="EyeOutline" /> 4
          </n-space>
        </n-card>
        <n-card :bordered="false" size="small" hoverable>
          <n-flex justify="center">
            <n-button quaternary type="primary">
              加载更多
            </n-button>
          </n-flex>
        </n-card>
      </div>

    </div>
  </div>
</template>

<style scoped>
.img-box {
  filter: opacity(0.9);
  padding: 30px;
  width: 100%;
  height: 500px;
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  transition: ease 1s;
}
.img-box img {
  height: 600px;
}
.a-list-box {
  width: 676px;
  margin: 0 auto;
}
.a-box {
  width: 1024px;
  margin: 0 auto;
  display: flex;
  padding: 0 16px 0 16px;
  justify-content: space-between;
}
.m-box {
  width: 100%;
  min-height: 100vh;
  transition: ease 1s;
}
@media screen and (max-width: 1024px){
  .a-box {
    width: 100%;
  }
  .img-box {
    height: 400px;
  }
}
</style>