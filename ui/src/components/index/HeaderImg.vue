<script setup lang="ts">

</script>

<template>
  <div class="img-box">
    <img src="@/assets/img/illustration/与猫贴贴.png" alt="怅然">
    <div class="img-box-text">
      <div>
        <h1>Hello world</h1>
        <p>无根树，花正幽，</p>
        <p>贪恋荣华谁肯休。</p>
        <p>浮生事，苦海舟，</p>
        <p>荡去飘来不自由。</p>
      </div>
    </div>
    <img src="@/assets/img/illustration/女孩抱猫.png" alt="若失">
  </div>
</template>

<style scoped>
.img-box {
  filter: opacity(0.9);
  padding: 30px;
  position: fixed;
  width: 100%;
  min-width: 380px;
  height: 600px;
  display: flex;
  justify-content: space-between;
  transition: ease 1s;
}
.img-box img {
  width: auto;
  height: auto;
}
.img-box-text {
  width: 100%;
  display: none;
  text-align: center;
}
@media screen and (max-width: 1024px) {
  .img-box {
    position: static;
    padding-left: 0;
    padding-right: 0;
    margin: 20px 0 20px 0;
  }
  .img-box-text {
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
@media screen and (max-width: 880px) {
  .img-box {
    padding-top: 60px;
    height: 500px;
  }
}
@media screen and (max-width: 780px) {
  .img-box {
    height: 400px;
  }
}
@media screen and (max-width: 680px) {
  .img-box {
    height: 300px;
  }
}
</style>