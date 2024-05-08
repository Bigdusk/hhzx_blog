<script setup lang="ts">
import {h, ref, onMounted} from 'vue'
import {NButton} from 'naive-ui'
import type {DataTableColumns} from 'naive-ui'

onMounted(() => {
  data_all()
})
async function data_all() {
  axios_util.get<Article[]>('/article/select/all').then(r => {
    console.log(r)
    createData.value = r.data
  })
}

const createColumns = ({
                         sendMail
                       }: {
  sendMail: (rowData: Article) => void
}): DataTableColumns<Article> => {
  return [
    {
      title: '标题',
      key: 'title'
    },
    {
      title: '作者',
      key: 'nickname'
    },

    {
      title: '分类',
      key: 'category_name'
    },
    {
      title: '操作',
      key: 'actions',
      render(row) {
        return h(
            NButton,
            {
              size: 'small',
              onClick: () => sendMail(row)
            },
            {default: () => '编辑'}
        )
      }
    }
    ,
    {
      title: '谨慎删除',
      key: 'delete',
      render(row) {
        return h(
            NButton,
            {
              size: 'small',
              onClick: () => {
                date_delete(row.id)
              }
            },
            {default: () => '删除'}
        )
      }
    }
  ]
}

const createData = ref<Article[]>([])
const columns = createColumns({
  sendMail(rowData) {
    //前往编辑界面
    to_path('/admin/article_creation/' + rowData.id)
  }
})

const pagination = {
  pageSize: 10
}

import {createDiscreteApi} from "naive-ui";
import type {Article} from "@/entity";
import axios_util from "@/utils/axios_util";
import {to_path} from "@/utils";

const {message} = createDiscreteApi(['message'])

const showModal = ref(false)

const body = ref<Article>({})

async function date_delete(id?: number) {
  axios_util.get<boolean>('/article/delete/' + id).then(r => {
    if (r.data) {
      message.success('删除成功')
      data_all()
      showModal.value = false
    } else {
      message.warning('删除失败')
    }
  })
}
</script>

<template>

  <n-card title="文章" hoverable>
    <n-data-table
        :bordered="false"
        :single-line="false"
        single-column
        :columns="columns"
        :data="createData"
        :pagination="pagination"
    />
  </n-card>

</template>

<style scoped>

</style>