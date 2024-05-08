<script setup lang="ts">
import {onMounted, ref} from "vue";
import type {Timeline} from "@/entity";
import axios_util from "@/utils/axios_util";
import {message} from "@/utils";

const showModal = ref(false)
onMounted(() => {
  timeline_select_all()
})
//添加行程
const timeline = ref<Timeline>({id: -1})
const timeline_list = ref<Timeline[]>()
async function timeline_insert() {
  axios_util.post('/timeline/insert', timeline.value).then( r => {
    if (r.data) {
      message.success('添加成功')
      timeline_select_all()
    }
  })
}

async function timeline_select_all() {
  axios_util.get('/timeline/select/all').then( r => {
    timeline_list.value = r.data
    showModal.value = false
  })
}
//单选
const songs = [
  {
    value: "success",
    label: "成功"
  },  {
    value: "warning",
    label: "警告"
  },  {
    value: "error",
    label: "错误"
  },  {
    value: "info",
    label: "信息"
  },
].map((s) => {
  return s
})
</script>

<template>
  <div>
    <n-card
        title="📖 计划"
        :bordered="false"
        @click="showModal = true"
        hoverable
    >
      点击添加
    </n-card>
    <n-card hoverable>
      <n-scrollbar style="max-height: 75vh">
        <n-timeline>
          <n-timeline-item
              :type="r.type"
              :title="r.title"
              :content="r.content"
              :time="r.time"
              line-type="dashed"
              v-for="r in timeline_list"
              :key="r.id"
          />
        </n-timeline>
      </n-scrollbar>

    </n-card>
  </div>


  <n-modal v-model:show="showModal">
    <n-card
        style="width: 600px"
        title="计划"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <template #header-extra>
        <n-button @click="timeline_insert">添加</n-button>
      </template>
      <n-form>
        <n-form-item label="图标">
          <n-radio-group v-model:value="timeline.type" name="radiogroup">
            <n-space>
              <n-radio v-for="song in songs" :key="song.value" :value="song.value">
                {{ song.label }}
              </n-radio>
            </n-space>
          </n-radio-group>
        </n-form-item>

        <n-form-item label="标题">
          <n-input v-model:value="timeline.title"/>
        </n-form-item>

        <n-form-item label="内容">
          <n-input v-model:value="timeline.content"/>
        </n-form-item>
        <n-date-picker
            v-model:formatted-value="timeline.time"
            value-format="yyyy-MM-dd'T'HH:mm:ss'Z'"
            type="datetime" clearable
        />
      </n-form>
    </n-card>
  </n-modal>

</template>

<style scoped>
@media screen and (max-width: 980px){
  div {
    display: none;
  }
}
</style>