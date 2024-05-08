<script setup lang="ts">
import {h, ref, onMounted} from 'vue'
import {NButton} from 'naive-ui'
import type {DataTableColumns} from 'naive-ui'

onMounted(() => {
  data_all()
  roles_all()
})

async function data_all() {
  axios_util.get<User[]>('/user/select/all').then(r => {
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
  ]
}

const createData = ref<User[]>()
const columns = createColumns({
  sendMail(rowData) {
    user_roles_now(rowData.id)
    showModal.value = true
    body.value = rowData
  }
})

const pagination = {
  pageSize: 10
}

import {createDiscreteApi} from "naive-ui";
import type {Roles, User, UserRoles} from "@/entity";
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
  axios_util.post<boolean>('/user/update', body.value).then(r => {
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
  axios_util.get<boolean>('/user/delete/' + body.value.id).then(r => {
    if (r.data) {
      message.success('删除成功')
      data_all()
      showModal.value = false
    } else {
      message.warning('删除失败')
    }
  })
}

//查询所有角色
const user_roles = ref<UserRoles>({})
const roles_list = ref<Roles[]>([])

async function roles_all() {
  axios_util.get<Roles[]>('/roles/select/all').then(r => {
    roles_list.value = r.data
  })
}

//查询用户当前权限角色
async function user_roles_now(user_id?: number) {
  axios_util.get<UserRoles>('/user/roles/select/' + user_id).then(r => {
    user_roles.value = r.data
  })
}
//权限改变回调
async function handle_update_user_roles(roles_id: number) {
  user_roles.value.role_id = roles_id
  axios_util.post('/user/roles/update', user_roles.value).then(r => {
    if (r.data) {
      message.success('改变角色成功')
      user_roles_now(user_roles.value.user_id)
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

      <n-form-item label="昵称">
        <n-input v-model:value="body.nickname"/>
      </n-form-item>

      <n-form-item label="头像">
        <n-input v-model:value="body.avatar"/>
      </n-form-item>
    </n-form>

    <n-radio-group @update:value="handle_update_user_roles" v-model:value="user_roles.role_id" name="radiogroup">
      <n-space>
        <n-radio v-for="song in roles_list" :key="song.id" :value="song.id">
          {{ song.role_name }}
        </n-radio>
      </n-space>
    </n-radio-group>

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