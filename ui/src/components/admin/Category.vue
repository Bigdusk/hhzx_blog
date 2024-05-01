<script setup lang="ts">
import {h, ref, onMounted} from 'vue'
import {NButton} from 'naive-ui'
import type {DataTableColumns} from 'naive-ui'
import type {Category} from "@/entity";
onMounted(() => {
  data_all()
})
//查询所有分类
async function data_all() {
  await axios_util.get<Category[]>('/category/select/all').then(r => {
    console.log(r)
    createData.value = r.data
  })
}

const createColumns = ({
                         sendMail
                       }: {
  sendMail: (rowData: Category) => void
}): DataTableColumns<Category> => {
  return [
    {
      title: 'id',
      key: 'id'
    },
    {
      title: '图标',
      key: 'ioc'
    },
    {
      title: '名称',
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
    },
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

const createData = ref<Category[]>()
const columns = createColumns({
  sendMail(rowData) {
    showModal.value = true
    body.value = rowData
  }
})

const pagination = {
  pageSize: 10
}

import {createDiscreteApi} from "naive-ui";
import axios_util from "@/utils/axios_util";

const {message} = createDiscreteApi(['message'])

const bodyStyle = {
  width: '600px'
}
const segmented = {
  content: 'soft',
  footer: 'soft'
} as const
const showModal = ref(false)
const showModal2 = ref(false)

const body = ref<Category>({})

async function update() {
  await axios_util.post<boolean>('/category/update', body.value).then(r => {
    if (r.data) {
      message.success('保存成功')
      data_all()
      body.value = {}
      showModal.value = false
    }
  })
}

async function date_delete(id: any) {
  await axios_util.get<boolean>('/category/delete/' + id).then(r => {
    if (r.data) {
      message.success('删除成功')
      data_all()
      showModal.value = false
    }
  })
}

//添加接口
async function data_insert() {
  body.value.id = 0
  await axios_util.post<boolean>('/category/insert', body.value).then(r => {
    if (r.data) {
      message.success('添加成功')
      data_all()
      body.value = {}
      showModal2.value = false
    }
  })
}
</script>

<template>

  <n-card title="分类管理" hoverable>
    <n-button @click="showModal2 = true" strong secondary type="primary">
      创建分类
    </n-button>
    <n-data-table
        :bordered="false"
        :single-line="false"
        single-column
        :columns="columns"
        :data="createData"
        :pagination="pagination"
    />
  </n-card>


  <n-modal
      v-model:show="showModal"
      class="custom-card"
      preset="card"
      :style="bodyStyle"
      title="编辑"
      size="huge"
      :bordered="false"
      :segmented="segmented"
  >
    <template #header-extra>
      噢!
    </template>
    <n-form>
      <n-form-item label="名称">
        <n-input v-model:value="body.category_name"/>
      </n-form-item>
      <n-form-item label="图标">
        <n-input v-model:value="body.ioc"/>
      </n-form-item>
    </n-form>
    {{ body }}
    <template #footer>
      <n-flex justify="space-between">
        <n-button @click="update" type="success">保存</n-button>
      </n-flex>
    </template>
  </n-modal>


  <n-modal
      v-model:show="showModal2"
      class="custom-card"
      preset="card"
      :style="bodyStyle"
      title="创建"
      size="huge"
      :bordered="false"
      :segmented="segmented"
  >
    <template #header-extra>
      噢!
    </template>
    <n-form>
      <n-form-item label="名称">
        <n-input v-model:value="body.category_name"/>
      </n-form-item>
      <n-form-item label="图标">
        <n-input v-model:value="body.ioc"/>
      </n-form-item>
    </n-form>
    {{ body }}
    <template #footer>
      <n-flex justify="space-between">
        <n-button @click="data_insert" type="success">保存</n-button>
      </n-flex>
    </template>
  </n-modal>
</template>

<style scoped>

</style>