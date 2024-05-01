<script setup lang="ts">
import {CalendarOutline, EyeOutline, GridOutline, PersonOutline} from "@vicons/ionicons5";
import {message, to_path} from "@/utils";
import axios_util from "@/utils/axios_util";
import {onMounted, ref} from "vue";
import {useRoute} from "vue-router";
onMounted(() => {
  article_all()
})
//查询全部分类文章数据
//文章搜索
interface ArticleInfo {
  category_name?: string
  cover?: string
  id?: number
  likes?: number
  nickname?: string
  publish_time?: string
  status?: number
  title?: string
  update_time?: string
  views?: number
}

const page = ref(1)
const size = ref(5)
const route = useRoute()
//获取全部文章
function article_all() {
  axios_util.get<ArticleInfo[]>('/article/' + page.value + '/' + size.value + '/category/' + route.params.id).then(r => {
    article_list.value = r.data
  })
}

const article_list = ref<ArticleInfo[]>([])

//所有文章下一页
async function LoadMoreArticles() {
  await axios_util.get<ArticleInfo[]>('/article/' + ++page.value + '/' + size.value + '/category/' + route.params.id)
      .then(r => {
        if (r.data.length > 0) {
          r.data.forEach(r => {
            article_list.value.push(r)
          })
        } else {
          message.success('到了尽头...')
        }

      })
}
</script>

<template>

  <div class="img-box">
    <div>
      <h1>{{route.params.category_name}}</h1>
      <n-divider vertical />人生如逆旅，我亦是行人。
    </div>
  </div>

  <div class="m-box">

    <div class="a-box">

      <div class="a-list-box">
        <n-card style="margin-bottom: 32px;padding: 2px;border-radius: .5rem;"
                v-for="r in article_list"
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
                @click="LoadMoreArticles"
                quaternary
                type="primary"
            >
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