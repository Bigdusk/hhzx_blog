<script setup lang="ts">
import {
  PersonOutline,
  PricetagOutline,
  CalendarOutline,
  EyeOutline
} from '@vicons/ionicons5'
import HeaderImg from "@/components/index/HeaderImg.vue";
</script>

<template>
  <HeaderImg/>
  <div class="m-box">

    <div class="a-box">
      <div class="a-list-box">
        <n-card style="margin-bottom: 32px;padding: 2px;border-radius: .5rem;" v-for="i in 10" title="卡片" embedded hoverable>
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

      <div class="u-info-box">
        <n-card class="u-info-box-card" embedded hoverable>
          <n-flex vertical>
            <div>
              <n-divider dashed>
                <n-avatar
                    round
                    :size="48"
                    src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
                />
              </n-divider>
            </div>
            <div style="text-align: center;font-size: 1.6em;">Hhzx</div>
            <n-divider/>
            <div>
              Categories
            </div>
            <n-button v-for="i in 5" type="primary" dashed>
              分类
            </n-button>
            <div>
              Tags
            </div>
            <n-space>
              <n-tag v-for="i in 10" type="success">
                标签
              </n-tag>
            </n-space>
          </n-flex>

        </n-card>
      </div>
    </div>
  </div>
</template>

<style scoped>
.a-list-box {
  width: 676px;
}
.u-info-box {
  width: 298px;
  transition: ease 1s;
}
.u-info-box-card {
  position: fixed;
  width: 298px;
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
  padding: 80px 0 100px 0;
  transition: ease 1s;
}
@media screen and (max-width: 1024px){
  .u-info-box {
    display: none;
  }
  .a-list-box {
    margin: 0 auto;
  }
  .a-box {
    width: 100%;
  }
  .m-box {
    padding-top: 0;
  }
}
</style>