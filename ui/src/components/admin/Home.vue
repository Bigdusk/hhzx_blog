<script setup lang="ts">
import D1 from "@/components/admin/home_box/D1.vue";
import Calendar from "@/components/admin/Calendar.vue";
import D3 from "@/components/admin/home_box/D3.vue";
import {to_path} from "@/utils";
import {onMounted, ref} from "vue";
import type {Comment} from "@/entity";
import axios_util from "@/utils/axios_util";
import QQname from "@/components/diy/QQname.vue";
onMounted(() => {
  data_all()
})
const comment_list = ref<Comment[]>()
async function data_all() {
  axios_util.get('/comment/select/all').then(r => {
    comment_list.value = r.data
  })
}
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

<!--          <n-card hoverable>
            定义界面
          </n-card>-->

          <n-card hoverable>
            待开发
          </n-card>

        </div>

      </n-card>
      <n-card title="日历" hoverable>
        <Calendar/>
      </n-card>
      <n-card title="最新留言" hoverable>
        <n-scrollbar style="max-height: 25vh">
          <n-card v-for="r in comment_list" :bordered="false" @click="to_path('/article/'+ r.article_id)">
            <n-space style="display: flex;align-items: center">
              <n-avatar
                  round
                  :size="38"
                  :src="'http://q1.qlogo.cn/g?b=qq&nk='+r.qq+'&s=100'"
              />
              <div style="font-size: 1.3em">
                <QQname :qq="r.qq"/>
              </div>
              {{ r.create_time }}
            </n-space>

            <n-card>
              {{r.content}}
            </n-card>
          </n-card>
        </n-scrollbar>
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