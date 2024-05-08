import axios from "axios";
import {message, to_path} from "@/utils/index";
const axios_util = axios.create({
    baseURL: "https://127.0.0.1:8888",
    timeout: 60000
});

interface ApiResponse<T> {
    code: number,
    message: string,
    data: T
}

//请求拦截器
axios_util.interceptors.request.use(
    config => {
        config.headers.Authorization = localStorage.getItem('authorization')
        config.headers.fingerprint = localStorage.getItem('fingerprint')
        return config;
    },
    error => {
        return Promise.reject(error);
    }
)

axios_util.interceptors.response.use(
    response => {
        console.log("请求")
        console.log(response)
        if (response.data.code === 10010) {
            message.warning(response.data.message)
            to_path('/login')
        }
        if (response.data.code === 500) {
            message.warning(response.data.message)
        }
        return response.data;
    },
    error => {
        console.log("错误")
        console.log(error)
        if (error.response.status === 401) {
            message.warning('请重新登录')
            //to_path('/login')
        }
        if (error.response.status === 422) {
            message.warning('请填写完整参数')
        }
        if (error.response.status === 405) {
            message.warning('请求方法不对')
        }
        return Promise.reject(error);
    }
)


//要求实现参数提示
//封装get方法
export async function get<T>(
    url: string,
    params?: any
): Promise<T> {
    return await axios_util.get<ApiResponse<T>>(url, { params })
        .then(response => {
            return response.data
        })
        .catch(error => {
            return error
        })
}

//封装post请求
export async function post<T>(
    url: string,
    data: any,
    params?: any
): Promise<T> {
    return await axios_util.post<ApiResponse<T>>(url, data, { params })
        .then(response => {
            return response.data
        })
        .catch(error => {
            return error
        })
}

export default axios_util;