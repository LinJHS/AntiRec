// 定义登录和注册的工具函数

// 导入Axios库
import axios from "axios";

const baseURL = import.meta.env.VITE_BASE_URL + "/api/login";
const timeout = 10000;
let headers = {
  "Access-Control-Allow-Origin": "*",
  "Content-Type": "application/json",
};

// 创建 Axios 实例
let request = axios.create({
  baseURL: baseURL,
  timeout: timeout,
  headers: headers,
});

// 登录
const login = async (requestData) => {
  let res = await request.post("", requestData);
  return res;
};

// 注册
const register = async (requestData) => {
  let res = await request.put("", requestData);
  return res;
};

// 导出函数
export { login, register };
