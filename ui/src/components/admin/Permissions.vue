<script setup lang="ts">
import {onMounted, ref} from "vue";
import axios_util from "@/utils/axios_util";
import type {Category, Permissions, RolePermissions, Roles} from "@/entity";
import {message} from "@/utils";
import type {SelectOption} from "naive-ui";
onMounted(() => {
  roles_all()
})
const value = ref(null)

const show_permissions_modal = ref(false)
//权限
const permissions = ref<Permissions>({id: 0})
const permissions_list = ref<any>([])

async function permissions_delete(id: any) {
  axios_util.get<boolean>('/permissions/delete/' + id).then(r => {
    if (r.data) {
      message.success('删除成功')
      permissions_all(roles_permissions.value.roles_id)
    }
  })
}

//添加接口
async function permissions_insert() {
  permissions.value.id = 0
  axios_util.post<boolean>('/permissions/insert', permissions.value).then(r => {
    if (r.data) {
      message.success('添加成功')
      permissions.value = {}
      show_permissions_modal.value = false
      permissions_all(roles_permissions.value.roles_id)
    }
  })
}
async function permissions_all(roles_id?: number) {
  axios_util.get<[[Permissions,boolean]]>('/permissions/select/roles/' + roles_id).then(r => {
    permissions_list.value = r.data
  })
}

const show_roles_modal = ref(false)
//权限
const roles = ref<Roles>({id: 0})
const roles_list = ref<any>([])

async function roles_delete(id: any) {
  axios_util.get<boolean>('/roles/delete/' + id).then(r => {
    if (r.data) {
      message.success('删除成功')
      roles_all()
    }
  })
}

//添加接口
async function roles_insert() {
  roles.value.id = 0
  axios_util.post<boolean>('/roles/insert', roles.value).then(r => {
    if (r.data) {
      message.success('添加成功')
      roles.value = {}
      show_roles_modal.value = false
      roles_all()
    }
  })
}
async function roles_all() {
  roles_list.value.length = 0
  axios_util.get<Roles[]>('/roles/select/all').then(r => {
    console.log(r)
    r.data.forEach( r => {
      roles_list.value.push({
        value: r.id,
        label: r.role_name
      })
    })
  })
}

function handleUpdateValue (value: number) {
  roles_permissions.value.roles_id = value
  permissions_all(value)
}

//关联权限
const roles_permissions = ref<RolePermissions>({id:0})
function roles_permissions_insert(permissions_id?: number, is_delete?: boolean) {
  roles_permissions.value.permissions_id = permissions_id
  if (is_delete) {
    axios_util.post<boolean>('/rp/delete', roles_permissions.value).then(r => {
      if (r.data) {
        message.success('取消勾线')
        permissions_all(roles_permissions.value.roles_id)
      }
    })
  }else {
    axios_util.post<boolean>('/rp/insert', roles_permissions.value).then(r => {
      if (r.data) {
        message.success('勾线')
        permissions_all(roles_permissions.value.roles_id)
      }
    })
  }

}
</script>

<template>
  <div class="p-box">
    <div>
      <n-card title="权限编辑" hoverable>
        <n-button @click="show_roles_modal = true">添加角色</n-button>
        <n-button @click="show_permissions_modal = true">创建权限</n-button>
        <n-select
            v-model:value="value"
            :options="roles_list"
            @update:value="handleUpdateValue"
        />
      </n-card>
      <n-card title="全部角色" hoverable>
        <n-card v-for="r in roles_list" :key="r.value" hoverable>
          {{r.label}}
          <n-button @click="roles_delete(r.value)">
            删除
          </n-button>
        </n-card>
      </n-card>
    </div>
    <div>

      <n-card title="权限选择" hoverable>
        <n-space>
          <n-card  v-for="r in permissions_list"
                   :key="r[0].id"
                   hoverable>
            <n-button
                @click="roles_permissions_insert(r[0].id, r[1])"
                :type="r[1]? 'success':'error'"
            >
              {{r[0].permission_name}}|{{r[0].description}}
            </n-button>
            -
            <n-button @click="permissions_delete(r[0].id)" size="small" type="error" dashed>删除</n-button>
          </n-card>

        </n-space>
      </n-card>

    </div>
  </div>

  <n-modal v-model:show="show_permissions_modal">
    <n-card
        style="width: 600px"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
        title="权限"
    >
      <template #header-extra>
        <n-button @click="permissions_insert">提交</n-button>
      </template>
      <n-flex vertical>
        <n-input v-model:value="permissions.permission_name" placeholder="名称"/>
        <n-input v-model:value="permissions.description" placeholder="地址"/>
      </n-flex>
    </n-card>
  </n-modal>


  <n-modal v-model:show="show_roles_modal">
    <n-card
        style="width: 600px"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
        title="角色"
    >
      <template #header-extra>
        <n-button @click="roles_insert">提交</n-button>
      </template>
      <n-flex vertical>
        <n-input v-model:value="roles.role_name" placeholder="角色名称"/>
        <n-input v-model:value="roles.description" placeholder="描述"/>
      </n-flex>
    </n-card>
  </n-modal>
</template>

<style scoped>
.p-box {
  display: flex;

}
</style>