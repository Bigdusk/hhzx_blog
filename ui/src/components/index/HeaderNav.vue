<script setup lang="ts">
import {useCounterStore} from "@/stores/counter";
import {darkTheme, type DrawerPlacement,} from "naive-ui";
import {MenuOutline, MoonOutline, SunnyOutline} from '@vicons/ionicons5'
import {onMounted, ref} from "vue";
import router from "@/router";
import {to_path} from "@/utils";

onMounted(() => {
  set_page_show()
})
function set_page_show() {
  let path = window.location.pathname
  page_show.value = !path.includes('/admin');
}
const page_show = ref(true)

const counter = useCounterStore()

const activeRef = ref(false)
const placementRef = ref<DrawerPlacement>('right')
const activate = (place: DrawerPlacement) => {
  activeRef.value = true
  placementRef.value = place
}

const active = activeRef
const placement = placementRef
//搜索
const search_value = ref("")
function search() {
  activeRef.value = false
  counter.search_value_set(search_value.value)
  counter.article_all()
  to_path('/')
}
</script>

<template>

  <div class="b-masks" :style="{width: counter.is_masks + '%'}"></div>

  <div v-show="page_show" class="box">
    <n-button class="menu-box" @click="activate('left')" text style="font-size: 34px">
      <n-icon>
        <MenuOutline/>
      </n-icon>
    </n-button>
    <n-text>
      <p @click="router.push({path: '/'})" style="font-size: 25px;font-weight: 600;cursor:pointer">
        Hhzx`Blog
      </p>
    </n-text>
      <n-space>
        <div class="nav-box">
          <n-space>
            <n-input-group>
              <n-input v-model:value="search_value" placeholder="关键字"/>
              <n-button @click="search" type="primary" ghost>
                搜索
              </n-button>
            </n-input-group>
            <n-button v-for="i in 3">
              导航
            </n-button>
          </n-space>
        </div>
        <n-button @click="counter.set_topic" text style="font-size: 34px">
          <n-icon>
            <SunnyOutline v-show="!counter.is_topic_show"/>
            <MoonOutline v-show="counter.is_topic_show"/>
          </n-icon>
        </n-button>
      </n-space>
  </div>

  <n-drawer
      v-model:show="active"
      :default-width="280"
      :placement="placement"
      resizable
  >
    <n-drawer-content title="hhzx" closable>

      <n-flex vertical>
        <div>
          <n-divider dashed>
            <n-avatar
                round
                :size="68"
                src="http://q1.qlogo.cn/g?b=qq&nk=2831828656&s=100"
            />
          </n-divider>
        </div>
        <n-input-group>
          <n-input v-model:value="search_value" placeholder="关键字"/>
          <n-button @click="search" type="primary" ghost>
            搜索
          </n-button>
        </n-input-group>
        <n-divider/>
        <div>
          Nav
        </div>
        <n-button v-for="i in 3" type="primary" dashed>
          导航
        </n-button>
        <div>
          Categories
        </div>
        <n-button v-for="i in 5" type="primary" dashed>
          分类
        </n-button>
      </n-flex>

    </n-drawer-content>
  </n-drawer>
</template>

<style scoped>
.menu-box {
  display: none;
}
.nav-box {
  display: block;
}
.box {
  z-index: 999;
  background-color: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(30px);
  -webkit-backdrop-filter: blur(30px);
  border: 0px solid rgba(255, 255, 255, 0.18);
  border-radius: 0px;
  -webkit-border-radius: 0px;
  color: rgba(255, 255, 255, 0.15);
  min-width: 375px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 40px;
  position: fixed;
  width: 100%;
  transition: ease 1s;
}

@media screen and (max-width: 1024px) {
  .menu-box {
    display: block;
  }
  .nav-box {
    display: none;
  }
  .box {
    padding-left: 10px;
    padding-right: 10px;
    background: transparent;
  }
}

.b-masks {
  z-index: 0;
  width: 100%;
  min-height: 100vh;
  background-image: url("@/assets/img/h.png");
  background-size: cover;
  background-attachment:fixed;
  background-repeat: no-repeat;
  background-position: center;
  position: fixed;
  transition: ease 2s;
}
</style>