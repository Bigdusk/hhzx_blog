use axum::extract::{Multipart, Path};
use axum::Json;
use nanoid::nanoid;
use serde_json::Value;
use crate::utils::env_var;
use crate::utils::result::ResultUtil;

///文件上传
pub async fn file_upload(
    multipart: Multipart
) -> Json<Value>{
    let r = upload(multipart).await;
    match r {
        Ok(r) => {
            let mut  file_path: String = env_var("FILE_API_ADDRESS");
            file_path.push_str(r.as_str());
            ResultUtil::success(file_path)
        }
        Err(r) => {
            ResultUtil::<&str>::error(r.to_string().as_str())
        }
    }
}
async fn upload(mut multipart: Multipart) -> Result<String, Box<dyn std::error::Error>>{
    let r = multipart.next_field().await?;

    match r {
        None => Err(Box::from("没有上传")),
        Some(file) => {
            //文件类型
            let  content_type = file.content_type();
            let content_type = match content_type {
                None => {return Err(Box::from("查询文件类型失败"))}
                Some(r) => {r}
            };
            let file_type = content_type.replace("/", ".");

            //根据文件类型生成随机文件名(出于安全考虑)
            let nanoid = nanoid!();

            //上传文件名称
            let file_path = format!("./public/article_img/{}-{}", &nanoid, &file_type);

            //上传文件内容
            let file_data = file.bytes().await?;

            //保存上传
            tokio::fs::write(&file_path, &file_data)
                .await
                .map_err(|error| error.to_string())?;

            Ok(format!("{}-{}", &nanoid, &file_type))
        }
    }
}

pub async fn file_download(
    Path(file_name): Path<String>
) -> Result<Vec<u8>, Json<Value>>{
    let r = download(file_name).await;

    match r {
        Ok(r) => {
            Ok(r)
        }
        Err(r) => {
            Err(ResultUtil::<&str>::error(r.to_string().as_str()))
        }
    }
}

async fn download(file_name: String) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    //路径
    let file_path = format!("./public/article_img/{}", file_name);
    //读取文件
    let file = tokio::fs::read(file_path)
        .await
        .map_err(|error| error.to_string())?;

    Ok(file)
}