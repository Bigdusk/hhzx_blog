<script setup lang="ts">
//登录
import {ref} from "vue";
import type {User} from "@/entity";
import axios_util from "@/utils/axios_util";
import {message, to_path} from "@/utils";

const user = ref<User>({id: -1})

function login() {
  axios_util.post<string>('/login', user.value).then(r => {
    if (r.data) {
      message.success('登录成功')
      localStorage.setItem('authorization', r.data)
      to_path('/admin')
    }
  })
}

function registration() {
  axios_util.post<string>('/registration', user.value).then(r => {
    if (r.data) {
      message.success('注册成功')
      localStorage.setItem('authorization', r.data)
    }
  })
}
</script>

<template>
  <div class="m-box">
    <div class="login-box">
      <n-tabs
          class="card-tabs"
          default-value="signin"
          size="large"
          animated
          pane-wrapper-style="margin: 0 -4px"
          pane-style="padding-left: 4px; padding-right: 4px; box-sizing: border-box;"
      >
        <n-tab-pane name="signin" tab="登录">
          <n-form>
            <n-form-item-row label="账号">
              <n-input v-model:value="user.username"/>
            </n-form-item-row>
            <n-form-item-row label="密码">
              <n-input v-model:value="user.password"/>
            </n-form-item-row>
          </n-form>
          <n-button @click="login" type="primary" block secondary strong>
            登录
          </n-button>
        </n-tab-pane>
        <n-tab-pane name="signup" tab="注册">
          <n-form>
            <n-form-item-row label="账号">
              <n-input v-model:value="user.username"/>
            </n-form-item-row>
            <n-form-item-row label="邮箱">
              <n-input v-model:value="user.email"/>
            </n-form-item-row>
            <n-form-item-row label="密码">
              <n-input v-model:value="user.password"/>
            </n-form-item-row>
          </n-form>
          <n-button @click="registration" type="primary" block secondary strong>
            注册
          </n-button>
        </n-tab-pane>
      </n-tabs>
    </div>
  </div>
</template>

<style scoped>
.m-box {
  width: 100%;
  height: 100%;
  padding: 130px 0;
}

.login-box {
  width: 500px;
  margin: 0 auto;
  padding: 20px;
  background-color: rgba(89, 89, 89, 0.05);
  backdrop-filter: blur(30px);
  -webkit-backdrop-filter: blur(30px);
  border-right: 1px solid rgba(255, 255, 255, 0.18);
  box-shadow: rgba(14, 14, 14, 0.19) 0 6px 15px 0;
  -webkit-box-shadow: rgba(14, 14, 14, 0.19) 0 6px 15px 0;
  border-radius: 20px;
  -webkit-border-radius: 20px;
  transition: ease 1s;
}

@media screen and (max-width: 600px) {
  .login-box {
    width: 400px;
  }
}
</style>