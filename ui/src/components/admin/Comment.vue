<script setup lang="ts">
import {h, ref, onMounted} from 'vue'
import {NButton} from 'naive-ui'
import type {DataTableColumns} from 'naive-ui'

onMounted(() => {
  data_all()
})

function data_all() {
  axios_util.get('/comment/select/all').then(r => {
    console.log(r)
    createData.value = r.data
  })
}

const createColumns = ({
                         sendMail
                       }: {
  sendMail: (rowData: Comment) => void
}): DataTableColumns<Comment> => {
  return [
    {
      title: 'id',
      key: 'id'
    },
    {
      title: '父id',
      key: 'parent_id'
    },
    {
      title: '内容',
      key: 'content'
    },
    {
      title: '时间',
      key: 'create_time'
    },
    {
      title: 'Action',
      key: 'actions',
      render(row) {
        return h(
            NButton,
            {
              size: 'small',
              onClick: () => sendMail(row)
            },
            {default: () => '删除'}
        )
      }
    }
  ]
}

const createData = ref<Comment[]>()
const columns = createColumns({
  sendMail(rowData) {
    axios_util.get('/comment/delete/' + rowData.id).then(r => {
      if (r) {
        if (r.data) {
          message.success('删除成功')
          data_all()
        } else {
          message.warning('删除失败')
        }
      }
    })
  }
})

const pagination = {
  pageSize: 10
}

import {createDiscreteApi} from "naive-ui";
import type {Comment} from "@/entity";
import axios_util from "@/utils/axios_util";

const {message} = createDiscreteApi(['message'])
</script>

<template>

  <n-card title="留言管理" hoverable>
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