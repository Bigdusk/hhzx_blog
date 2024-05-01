<script setup lang="ts">

</script>

<template>
  <div>
    <n-card
        title="📖 计划"
        embedded
        :bordered="false"
        hoverable
    >
      完善博客系统
    </n-card>
    <n-card title="日历" hoverable>
      <n-timeline>
        <n-timeline-item content="啊" />
        <n-timeline-item
            type="success"
            title="成功"
            content="哪里成功"
            time="2018-04-03 20:46"
        />
        <n-timeline-item type="error" content="哪里错误" time="2018-04-03 20:46" />
        <n-timeline-item
            type="warning"
            title="警告"
            content="哪里警告"
            time="2018-04-03 20:46"
        />
        <n-timeline-item
            type="info"
            title="信息"
            content="是的"
            time="2018-04-03 20:46"
            line-type="dashed"
        />
        <n-timeline-item content="啊" />
      </n-timeline>
    </n-card>
  </div>

</template>

<style scoped>
@media screen and (max-width: 980px){
  div {
    display: none;
  }
}
</style>