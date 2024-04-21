use std::fmt::Debug;
use std::str::FromStr;

///配置文件解析
pub fn env_var<T: FromStr>(key: &str) -> T where <T as FromStr>::Err: Debug{
    dotenv::var(key)
        .expect(format!("配置文件字段读取失败:{key}").as_str())
        .parse::<T>()
        .expect(format!("配置文件字段解析失败:{key}").as_str())
}