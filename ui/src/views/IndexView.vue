<script setup lang="ts">
import {
  PersonOutline,
  GridOutline,
  CalendarOutline,
  EyeOutline
} from '@vicons/ionicons5'
import HeaderImg from "@/components/index/HeaderImg.vue";
import {onMounted, ref} from "vue";
import {to_path} from "@/utils";
import {useCounterStore} from "@/stores/counter";
import axios_util from "@/utils/axios_util";
import type {Category} from "@/entity";
const counter = useCounterStore()
onMounted(() => {
  counter.article_all()
  category_all()
})
//查询分类
const category_list = ref<Category[]>([])
async function category_all() {
  await axios_util.get<Category[]>('/category/select/all').then(r => {
    category_list.value = r.data
  })
}
</script>

<template>
  <HeaderImg/>
  <div class="m-box">

    <div class="a-box">
      <div class="a-list-box">
        <n-card style="margin-bottom: 32px;padding: 2px;border-radius: .5rem;"
                v-for="r in counter.article_list"
                :title="r.title"
                :key="r.id"
                @click="to_path('/article/' + r.id)"
                embedded hoverable>
          <n-space>
            <n-icon :component="PersonOutline"/>
            {{ r.nickname }}
            <n-icon :component="GridOutline"/>
            {{ r.category_name }}
            <n-icon :component="CalendarOutline"/>
            {{ r.update_time }}
            <n-icon :component="EyeOutline"/>
            {{ r.views }}
          </n-space>
        </n-card>
        <n-card :bordered="false" size="small" hoverable>
          <n-flex justify="center">
            <n-button
                @click="counter.LoadMoreArticles"
                quaternary
                type="primary"
            >
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
                    :size="68"
                    src="http://q1.qlogo.cn/g?b=qq&nk=2831828656&s=100"
                />
              </n-divider>
            </div>
            <div style="text-align: center;font-size: 1.6em;">Hhzx</div>
            <n-divider/>
            <div>
              Categories
            </div>
            <n-button
                v-for="r in category_list"
                @click="to_path('/category/'+ r.category_name + '/' + r.id)"
                type="primary"
                dashed
            >
              {{ r.category_name }}
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
  width: 278px;
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

@media screen and (max-width: 1024px) {
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