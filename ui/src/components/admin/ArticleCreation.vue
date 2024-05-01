<script setup lang="ts">
import {onMounted, ref} from 'vue';
import axios_util from "@/utils/axios_util";
import type {Article, Category} from "@/entity";
import {message, to_path} from "@/utils";
import {MdEditor, type ToolbarNames} from 'md-editor-v3';
import 'md-editor-v3/lib/style.css';
import {useCounterStore} from "@/stores/counter";
import {type UploadFileInfo} from "naive-ui";
import { type DrawerPlacement } from 'naive-ui';

onMounted(() => {
  data_all()
})

//查询所有分类
interface category_view {
  label?: string,
  value?: number
}

//分类
const category_all = ref<category_view[]>([])

async function data_all() {
  await axios_util.get<Category[]>('/category/select/all').then(r => {
    console.log(r)
    r.data.forEach(r => {
      category_all.value.push({
        label: r.category_name,
        value: r.id
      })
    })
  })
}

//文章添加
const article = ref<Article>({
  id: 0,
  status: 0
})

function article_insert() {
  //将标签赋值
  article.value.article_tags = tags.value.toString()
  axios_util.post<boolean>('/article/insert', article.value).then(r => {
    if (r.data) {
      message.success('添加成功')
      to_path('/admin')
    }
  })
}

//tags
const tags = ref([])

//单选
const songs = [
  {
    value: 0,
    label: "发布"
  },
  {
    value: 1,
    label: '草稿'
  }
].map((s) => {
  return s
})

const toolbars: ToolbarNames[] = [
  'bold',
  'underline',
  'italic',
  '-',
  'title',
  'strikeThrough',
  'sub',
  'sup',
  'quote',
  'unorderedList',
  'orderedList',
  'task',
  '-',
  'codeRow',
  'code',
  'link',
  'image',
  'table',
  'mermaid',
  'katex',
  '=',
  'fullscreen',
  'preview',
  'htmlPreview',
  'catalog',
  'save',
];
const counter = useCounterStore()

//文件上传
const onUploadImg = async (files: any, callback: any) => {
  const res = await Promise.all(
      files.map((file: any) => {
        return new Promise((rev, rej) => {
          const form = new FormData();
          form.append('file', file);

          axios_util
              .post<string>('/file/upload', form, {
                headers: {
                  'Content-Type': 'multipart/form-data'
                }
              })
              .then((res) => rev(res))
              .catch((error) => rej(error));
        });
      })
  );
  console.log(res)
  callback(res.map((item) => item.data));
}

//封面上传
const handleFinish = ({
                        file,
                        event
                      }: {
  file: UploadFileInfo
  event?: ProgressEvent
}) => {
  const r = JSON.parse((event?.target as XMLHttpRequest).response)
  message.success(r.message)
  const ext = r.data.split('/')
  file.name = ext[ext.length - 1]
  file.url = r.data
  return file
}
//弹出
const active = ref(false)
const placement = ref<DrawerPlacement>('right')
const activate = (place: DrawerPlacement) => {
  active.value = true
  placement.value = place
}
</script>

<template>
  <div class="article-header-box" id="drawer-target">
    <div class="article-header-box-left">
      <n-input v-model:value="article.title" type="text" placeholder="标题"/>
      -
      <n-select v-model:value="article.category_id" :options="category_all" placeholder="分类" style="width: 200px"/>
    </div>

    <div class="article-header-box-right">
      <n-button @click="activate('right')">
        设置
      </n-button>
    </div>

  </div>

  <n-divider vertical/>

  <n-dynamic-tags
      v-model:value="tags"

  />

  <MdEditor
      style="
      width: 100%;
      height: 80vh;
      "
      :theme="counter.is_topic_show? 'light':'dark'"
      v-model="article.markdown"
      :toolbars="toolbars"
      :preview="false"
      @onSave="article_insert"
      @onUploadImg="onUploadImg"
  />


  <n-drawer
      v-model:show="active"
      :width="200"
      :height="200"
      :placement="placement"
      :trap-focus="false"
      :block-scroll="false"
      to="#drawer-target"
  >
    <n-drawer-content title="设置">
      <n-radio-group v-model:value="article.status" name="radiogroup">
        <n-space>
          <n-radio v-for="song in songs" :key="song.value" :value="song.value">
            {{ song.label }}
          </n-radio>
        </n-space>
      </n-radio-group>
      <n-upload
          action="https://127.0.0.1:8888/file/upload"
          @finish="handleFinish"
          :max="1"
      >
        <n-button>上传封面</n-button>
      </n-upload>
    </n-drawer-content>
  </n-drawer>
</template>

<style scoped>
.article-header-box {
  display: flex;
  justify-content: space-between;
}

.article-header-box-left {
  display: flex;
  gap: 10px;
}

.article-header-box-right {
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
}
</style>