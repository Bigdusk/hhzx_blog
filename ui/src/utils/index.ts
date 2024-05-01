//导航位置
import router from "@/router";
import {createDiscreteApi} from "naive-ui";

export function to_path(url: string) {
    router.push({path: url}).then(r => {
        console.log('转到' + r?.name)
    })
}

export const {message} = createDiscreteApi(['message'])
