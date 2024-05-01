<script setup lang="ts">
import {ref} from 'vue'
import {isYesterday, addDays} from 'date-fns/esm'
import {message} from "@/utils";

const value = ref(addDays(Date.now(), 1).valueOf())

function handleUpdateValue(
    _: number,
    {year, month, date}: { year: number; month: number; date: number }
) {
  message.success(`${year}-${month}-${date}`)
}

function isDateDisabled(timestamp: number) {
  return isYesterday(timestamp);

}
</script>

<template>
    <n-calendar
        v-model:value="value"
        #="{ year, month, date }"
        :is-date-disabled="isDateDisabled"
        @update:value="handleUpdateValue"
        style="height: 100%;width: 100%"
    >
<!--      {{year + month + date}}-->
    </n-calendar>
</template>

<style scoped>

</style>