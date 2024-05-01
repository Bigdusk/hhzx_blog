<script setup lang="ts">
import D1 from "@/components/admin/home_box/D1.vue";
import {onMounted} from "vue";
import Calendar from "@/components/admin/Calendar.vue";
import D3 from "@/components/admin/home_box/D3.vue";
import {to_path} from "@/utils";

</script>

<template>
  <div class="h-box">
    <div class="d1">
      <D1/>
    </div>

    <div class="d2">
      <n-card title="操作" hoverable>
        <div
            style="
        display: flex;
        flex-direction: row;
      "
        >
          <n-card @click="to_path('/admin/article_creation')" hoverable>
            书写文章
          </n-card>

          <n-card hoverable>
            行程安排
          </n-card>

          <n-card hoverable>
            定义界面
          </n-card>
        </div>

      </n-card>
      <n-card title="日历" hoverable>
        <Calendar/>
      </n-card>
      <n-card title="最新文章" hoverable>
        <n-card v-for="i in 3" hoverable>
          文章
        </n-card>
      </n-card>
    </div>

    <div class="d3">
      <D3/>
    </div>
  </div>
</template>

<style scoped>
.h-box {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 10px;
}

.h-box div {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.d1 {
  flex-grow: 1;
  gap: 10px;
  display: block;
}

.d2 {
  flex-grow: 2;
  display: block;
}

.d3 {
  flex-grow: 1;
  display: block;
}
@media screen and (max-width: 980px){

}
</style>