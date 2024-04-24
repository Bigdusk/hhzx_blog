<script setup lang="ts">
import {ref} from 'vue';
import {MdPreview, MdCatalog} from 'md-editor-v3';
// preview.css相比style.css少了编辑器那部分样式
import 'md-editor-v3/lib/preview.css';
import {CalendarOutline} from "@vicons/ionicons5";

const id = 'preview-only';
const text = ref('# Hello Editor');
const scrollElement = document.documentElement;
//qq接口
//头像
//http://q1.qlogo.cn/g?b=qq&nk=QQ号码&s=100
//名称
//http://users.qzone.qq.com/fcg-bin/cgi_get_portrait.fcg?uins=QQ号码

//回复弹窗
const showModal = ref(false)
</script>

<template>
  <div class="m-box">

    <div class="a-box">
      <div class="a-list-box">
        <n-card :bordered="false" embedded hoverable>
          <h1>标题</h1>
          <MdPreview :editorId="id" :modelValue="text"/>

          <n-card class="l-box" :bordered="false" embedded hoverable>
            <n-button>
              点赞
            </n-button>
          </n-card>
        </n-card>

        <n-divider style="margin: 40px 0; line-height: 19px" title-placement="right">
          <n-icon size="20" :component="CalendarOutline" />-Last Updated 2024/4/14 16:54:05
        </n-divider>

        <!--        评论-->

        <n-card title="留言" :bordered="false" embedded hoverable>
          <n-input
              type="textarea"
              placeholder="我欲留言叮嘱付。"
              :autosize="{
                minRows: 3
              }"
          />
          <div style="display: flex">
            <n-input-group>
              <n-input-group-label>企鹅</n-input-group-label>
              <n-input placeholder="QQ"/>
            </n-input-group>
            <n-input-group>
              <n-input-group-label>网址</n-input-group-label>
              <n-input placeholder="http://blog.hhzx.top"/>
            </n-input-group>
            <n-button type="primary">
              留言
            </n-button>
          </div>
        </n-card>

        <n-divider dashed>
          留言
        </n-divider>

        <n-card :bordered="false" embedded hoverable>
          <n-space style="display: flex;align-items: center">
            <n-avatar
                round
                :size="38"
                src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
            />
            <div style="font-size: 1.3em">
              hhzx
            </div>
            <n-button @click="showModal = true">回复</n-button>
          </n-space>

          <n-card>
            如果你年轻的时候不 996，你什么时候可以 996？你一辈子没有
            996，你觉得你就很骄傲了？这个世界上，我们每一个人都希望成功，都希望美好生活，都希望被尊重，我请问大家，你不付出超越别人的努力和时间，你怎么能够实现你想要的成功？
          </n-card>

          <n-card :bordered="false" embedded hoverable>
            <n-space style="display: flex;align-items: center">
              <n-avatar
                  round
                  :size="38"
                  src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg"
              />
              <div style="font-size: 1.3em">
                hhzx
              </div>
              <div>
                回复 hhzx 在 2024-04-09T18:02:34
              </div>
            </n-space>

            <n-card>
              如果你年轻的时候不 996，你什么时候可以 996？你一辈子没有
              996，你觉得你就很骄傲了？这个世界上，我们每一个人都希望成功，都希望美好生活，都希望被尊重，我请问大家，你不付出超越别人的努力和时间，你怎么能够实现你想要的成功？
            </n-card>
          </n-card>

        </n-card>
      </div>

      <div class="u-info-box">
        <n-card title="目录" class="u-info-box-card" :bordered="false" embedded hoverable>

          <MdCatalog :editorId="id" :scrollElement="scrollElement"/>

        </n-card>
      </div>
    </div>

  </div>


  <n-modal v-model:show="showModal">
    <n-card
        style="width: 600px"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >

      <n-input
          type="textarea"
          placeholder="我欲留言叮嘱付。"
          :autosize="{
                minRows: 3
              }"
      />
      <div style="display: flex">
        <n-input-group>
          <n-input-group-label>企鹅</n-input-group-label>
          <n-input placeholder="QQ"/>
        </n-input-group>
        <n-input-group>
          <n-input-group-label>网址</n-input-group-label>
          <n-input placeholder="http://blog.hhzx.top"/>
        </n-input-group>
        <n-button type="primary">
          留言
        </n-button>
      </div>

    </n-card>
  </n-modal>
</template>

<style scoped>
.l-box {
  display: flex;
  align-items: center;
}

.md-editor {
  --md-bk-color: transparent;
}

.a-list-box {
  width: 826px;
}

.u-info-box {
  width: 148px;
  transition: ease 1s;
}

.u-info-box-card {
  position: fixed;
  width: 148px;
}

.a-box {
  width: 1024px;
  margin: 0 auto;
  display: flex;
  padding: 0 16px 0 16px;
  justify-content: space-between;
}

.m-box {
  width: 100%;
  min-height: 100vh;
  padding: 80px 0 100px 0;
  transition: ease 1s;
}

@media screen and (max-width: 1024px) {
  .u-info-box {
    display: none;
  }

  .a-list-box {
    margin: 0 auto;
  }

  .a-box {
    width: 100%;
  }
}
</style>