<script setup lang="ts">
import {h, ref, onMounted} from 'vue'
import {NButton} from 'naive-ui'
import type {DataTableColumns} from 'naive-ui'

onMounted(() => {
  data_all()
})

async function data_all() {
  await axios_util.get<User[]>('/user/select/all').then(r => {
    console.log(r)
    createData.value = r.data
  })
}

const createColumns = ({
                         sendMail
                       }: {
  sendMail: (rowData: User) => void
}): DataTableColumns<User> => {
  return [
    {
      title: 'id',
      key: 'id'
    },
    {
      title: '账号',
      key: 'username'
    },
    {
      title: '邮箱',
      key: 'email'
    },
    {
      title: '编辑',
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
  ]
}

const createData = ref<User[]>()
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
import type {User} from "@/entity";
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

const body = ref<User>({})

async function update() {
  await axios_util.post<boolean>('/user/update', body.value).then(r => {
    if (r.data) {
      message.success('保存成功')
      data_all()
      showModal.value = false
    } else {
      message.warning('保存失败')
    }
  })
}

async function date_delete() {
  await axios_util.get<boolean>('/user/delete/' + body.value.id).then(r => {
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

  <n-card title="用户管理" hoverable>
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
      title="信息"
      size="huge"
      :bordered="false"
      :segmented="segmented"
  >
    <template #header-extra>
      噢!
    </template>
    <n-form>
      <n-form-item label="账号">
        <n-input v-model:value="body.username"/>
      </n-form-item>

      <n-form-item label="密码">
        <n-input v-model:value="body.password"/>
      </n-form-item>

      <n-form-item label="邮箱">
        <n-input v-model:value="body.email"/>
      </n-form-item>

      <n-form-item label="头像">
        <n-input v-model:value="body.avatar"/>
      </n-form-item>
    </n-form>
    {{ body }}
    <template #footer>
      <n-flex justify="space-between">
        <n-button @click="date_delete" type="error">删除</n-button>
        <n-button @click="update" type="success">保存</n-button>
      </n-flex>
    </template>
  </n-modal>

</template>

<style scoped>

</style>