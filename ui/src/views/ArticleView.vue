<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {CalendarOutline} from "@vicons/ionicons5";
import axios_util from "@/utils/axios_util";
import type {Article, Comment} from "@/entity";

import 'vditor/dist/index.css';


onMounted(() => {
  article_by_id()
  comment_select_all()
});

//qq接口
//头像
//http://q1.qlogo.cn/g?b=qq&nk=QQ号码&s=100
//名称
//https://api.oioweb.cn/api/qq/info?qq=qq号码
//获取qq信息
//回复弹窗
const showModal = ref(false)

const route = useRoute()
//获取文章信息
async function article_by_id() {
  axios_util.get<Article>('/article/select/' + route.params.id).then(r => {
    article.value = r.data
  })
}

const article = ref<Article>({})

import {MdPreview, MdCatalog} from 'md-editor-v3';
// preview.css相比style.css少了编辑器那部分样式
import 'md-editor-v3/lib/preview.css';
import {useCounterStore} from "@/stores/counter";
import {useRoute} from "vue-router";
import {message} from "@/utils";
import QQname from "@/components/diy/QQname.vue";

const id = 'preview-only';
const scrollElement = document.documentElement;
const counter = useCounterStore()

//添加评论
const comment = ref<Comment>(
    {
      id: 0,
      article_id: Number(route.params.id)
    }
)

const comment_list = ref<{
  0: Comment,
  1: Comment[]
}[]>([])

//添加评论
async function comment_insert() {
  comment.value.comment_id = undefined
  axios_util.post('/comment/insert', comment.value).then(r => {
    if (r.data) {
      message.success('发布成功')
      article.value.content = ''
      comment_select_all()
    }
  })
}

//回复评论
async function reply_comment_insert(comment_id?: number) {
  comment.value.comment_id = comment_id
  axios_util.post('/comment/insert', comment.value).then(r => {
    comment.value.comment_id = undefined
    if (r.data) {
      message.success('发布成功')
      article.value.content = ''
      showModal.value = false
      comment_select_all()
    }
  })
}

//查询评论
async function comment_select_all() {
  axios_util.get('/comment/select/article/' + route.params.id).then(r => {
    comment_list.value = r.data
  })
}
</script>

<template>
  <div class="m-box">

    <div class="a-box">
      <div class="a-list-box">
        <n-card :bordered="false" embedded hoverable>
          <h1>标题</h1>

          <MdPreview :theme="counter.is_topic_show? 'light':'dark'" :editorId="id" :modelValue="article.markdown"/>

          <n-card class="l-box" :bordered="false" embedded hoverable>
            <n-button>
              未经作者授权，禁止转载
            </n-button>
          </n-card>
        </n-card>

        <n-divider style="margin: 40px 0; line-height: 19px" title-placement="right">
          <n-icon size="20" :component="CalendarOutline"/>
          -Last Updated {{ article.update_time }}
        </n-divider>

        <!--        评论-->

        <n-card title="留言" :bordered="false" embedded hoverable>
          <n-input
              v-model:value="comment.content"
              type="textarea"
              placeholder="我欲留言叮嘱付。"
              :autosize="{
                minRows: 3
              }"
          />
          <div style="display: flex">
            <n-input-group>
              <n-input-group-label>企鹅</n-input-group-label>
              <n-input v-model:value="comment.qq" placeholder="QQ"/>
            </n-input-group>
            <n-input-group>
              <n-input-group-label>网址</n-input-group-label>
              <n-input v-model:value="comment.web_url" placeholder="http://blog.hhzx.top"/>
            </n-input-group>
            <n-button @click="comment_insert" type="primary">
              留言
            </n-button>
          </div>
        </n-card>

        <n-divider dashed>
          留言
        </n-divider>

        <n-card v-for="r in comment_list" :bordered="false" embedded hoverable>
          <n-space style="display: flex;align-items: center">
            <n-avatar
                round
                :size="38"
                :src="'http://q1.qlogo.cn/g?b=qq&nk='+r[0].qq+'&s=100'"
            />
            <div style="font-size: 1.3em">
              <QQname :qq="r[0].qq"/>
            </div>
            {{ r[0].create_time }}
            <n-button @click="showModal = true">回复</n-button>

            <n-modal v-model:show="showModal">
              <n-card
                  style="width: 600px"
                  :bordered="false"
                  size="huge"
                  role="dialog"
                  aria-modal="true"
              >

                <n-input
                    v-model:value="comment.content"
                    type="textarea"
                    placeholder="我欲留言叮嘱付。"
                    :autosize="{
                minRows: 3
              }"
                />
                <div style="display: flex">
                  <n-input-group>
                    <n-input-group-label>企鹅</n-input-group-label>
                    <n-input v-model:value="comment.qq" placeholder="QQ"/>
                  </n-input-group>
                  <n-input-group>
                    <n-input-group-label>网址</n-input-group-label>
                    <n-input v-model:value="comment.web_url" placeholder="http://blog.hhzx.top"/>
                  </n-input-group>
                  <n-button @click="reply_comment_insert(r[0].id)" type="primary">
                    留言
                  </n-button>
                </div>

              </n-card>
            </n-modal>

          </n-space>

          <n-card>
            {{r[0].content}}
          </n-card>

          <n-card v-for="r2 in r[1]" :bordered="false" embedded hoverable>
            <n-space style="display: flex;align-items: center">
              <n-avatar
                  round
                  :size="38"
                  :src="'http://q1.qlogo.cn/g?b=qq&nk='+r2.qq+'&s=100'"
              />
              <div style="font-size: 1.3em">
                <QQname :qq="r2.qq"/>
              </div>
              <div>
                回复 <QQname :qq="r[0].qq"/> 在 {{ r2.create_time }}
              </div>
            </n-space>

            <n-card>
              {{r2.content}}
            </n-card>
          </n-card>

        </n-card>


      </div>


      <div class="u-info-box">
        <n-card title="目录" class="u-info-box-card" :bordered="false" embedded hoverable>

          <MdCatalog :editorId="id" :scrollElement="scrollElement"/>

        </n-card>
      </div>
    </div>

  </div>
</template>

<style scoped>
.l-box {
  display: flex;
  align-items: center;
}


.a-list-box {
  width: 826px;
}

.u-info-box {
  width: 148px;
  transition: ease 1s;
}

.u-info-box-card {
  position: fixed;
  width: 148px;
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
}
</style>