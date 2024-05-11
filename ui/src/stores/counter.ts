import {ref} from 'vue'
import {defineStore} from 'pinia'
import type {GlobalTheme} from "naive-ui";
import {darkTheme} from "naive-ui";
import axios_util from "@/utils/axios_util";
import {message} from "@/utils";

export const useCounterStore = defineStore('counter', () => {

    const theme = ref<GlobalTheme | null>()

    const is_topic_show = ref(true)
    const is_masks = ref(100)

    //主题切换
    async function set_topic() {
        is_topic_show.value = !is_topic_show.value
        if (is_topic_show.value) {
            theme.value = null
            is_masks.value = 0
        } else {
            theme.value = darkTheme
            is_masks.value = 100
        }
    }


    //文章搜索
    interface ArticleInfo {
        category_name?: string
        cover?: string
        id?: number
        likes?: number
        nickname?: string
        publish_time?: string
        status?: number
        title?: string
        update_time?: string
        views?: number
    }

    const page = ref(1)
    const size = ref(5)
    const search_value = ref<string | undefined>('')
//设置搜索信息
    async function search_value_set(value: string) {
        search_value.value = value
    }
//获取全部文章
    async function article_all() {
        show_spin.value = true
        if (search_value.value === '') {
            search_value.value = 'undefined'
        }
        axios_util.get<ArticleInfo[]>('/article/select/' + page.value + '/' + size.value + '/title/' + search_value.value).then(r => {
            article_list.value = r.data
            show_spin.value = false
        })
    }

    const article_list = ref<ArticleInfo[]>([])

//所有文章下一页
    async function LoadMoreArticles() {
        show_spin.value = true
        if (search_value.value === '') {
            search_value.value = 'undefined'
        }
        axios_util.get<ArticleInfo[]>('/article/select/' + ++page.value + '/' + size.value + '/title/' + search_value.value)
            .then(r => {
                if (r.data.length > 0) {
                    r.data.forEach(r => {
                        article_list.value.push(r)
                    })
                } else {
                    message.success('到了尽头...')
                }
                show_spin.value = false
            })
    }

    //加载蒙版
    const show_spin = ref(false)
    function set_spin(b: boolean) {
        show_spin.value = b
    }

    return {
        theme,
        is_masks,
        set_topic,
        is_topic_show,
        page,
        size,
        search_value,
        article_all,
        article_list,
        LoadMoreArticles,
        search_value_set,
        show_spin,
        set_spin
    }
})
