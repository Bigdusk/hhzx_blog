<script setup lang="ts">
import {darkTheme} from "naive-ui";
import {ref} from "vue";
import {useCounterStore} from "@/stores/counter";
import {
  MoonOutline,
  SunnyOutline,
  Home,
  List,
  PencilSharp,
  GridSharp,
  ChatboxEllipses,
  Person,
  Settings,
  Exit
} from '@vicons/ionicons5'
import {to_path} from "@/utils";

const counter = useCounterStore()
//颜色
const p_color = ref()
</script>

<template>
  <n-space>
    <div class="left-nav">
      <div class="left-nav-m">
        <div class="h-img">
          <n-divider dashed>
            <n-avatar
                :size="48"
                src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
            />
          </n-divider>
        </div>

        <n-button text @click="to_path('/admin')" style="font-size: 24px">
          <n-icon>
            <Home/>
          </n-icon>
        </n-button>

        <n-button text @click="to_path('/admin/articles')" style="font-size: 24px">
          <n-icon>
            <PencilSharp/>
          </n-icon>
        </n-button>

        <n-button text @click="to_path('/admin/category')" style="font-size: 24px">
          <n-icon>
            <GridSharp/>
          </n-icon>
        </n-button>

        <n-button text @click="to_path('/admin/user')" style="font-size: 24px">
          <n-icon>
            <Person/>
          </n-icon>
        </n-button>

        <n-button text @click="to_path('/admin/comment')" style="font-size: 24px">
          <n-icon>
            <ChatboxEllipses/>
          </n-icon>
        </n-button>
      </div>

      <div class="left-nav-m">
        <n-button text style="font-size: 24px">
          <n-icon>
            <Settings/>
          </n-icon>
        </n-button>
        <n-button text style="font-size: 24px">
          <n-icon>
            <Exit/>
          </n-icon>
        </n-button>
        <n-button @click="counter.set_topic" text style="font-size: 34px">
          <n-icon>
            <SunnyOutline v-show="!counter.is_topic_show"/>
            <MoonOutline v-show="counter.is_topic_show"/>
          </n-icon>
        </n-button>
      </div>

    </div>
  </n-space>

  <div class="b-box" :style="{'background-color': p_color}">
    <div class="m-box">
      <RouterView/>
    </div>
  </div>
</template>

<style scoped>
.left-nav {
  width: 100px;
  height: 100%;
  padding: 10px 10px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  z-index: 999;
  transition: ease 1s;

  background-color: rgba(89, 89, 89, 0.05);
  backdrop-filter: blur(30px);
  -webkit-backdrop-filter: blur(30px);
  border-right: 1px solid rgba(255, 255, 255, 0.18);
  box-shadow: rgba(14, 14, 14, 0.19) 0px 6px 15px 0px;
  -webkit-box-shadow: rgba(14, 14, 14, 0.19) 0px 6px 15px 0px;
  border-radius: 0 40px 40px 0;
  -webkit-border-radius: 0 40px 40px 0;
}
.left-nav-m {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.b-box {
  width: 100%;
  height: 100vh;
  padding: 40px 60px 40px 142px;
}
.m-box {
  width: 100%;
  height: 100%;
  padding: 20px;
  background-color: rgba(89, 89, 89, 0.05);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 0px solid rgba(255, 255, 255, 0.18);
  box-shadow: rgba(14, 14, 14, 0.19) 0px 6px 15px 0px;
  -webkit-box-shadow: rgba(14, 14, 14, 0.19) 0px 6px 15px 0px;
  border-radius: 12px;
  -webkit-border-radius: 12px;
}
.h-img {
  display: block;
}

@media screen and (max-width: 1024px) {
  .left-nav {
    width: 100%;
    height: 54px;
    flex-direction: row;
    border-radius: 0 0 20px 20px;
    -webkit-border-radius: 0 0 20px 20px;
  }
  .left-nav-m {
    flex-direction: row;
    gap: 10px;
  }
  .h-img {
    display: none;
  }
  .b-box {
    padding: 64px 0 0 0;
  }
  .m-box {
    padding: 64px 0 0 0;
  }
}
</style>