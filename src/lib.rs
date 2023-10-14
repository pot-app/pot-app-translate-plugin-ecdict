use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let mut url = match needs.get("requestPath") {
        Some(url) => url.to_string(),
        None => return Err("requestPath not found".into()),
    };

    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }

    let plain_text = text.replace("/", "@@");
    let encode_text = encode(plain_text.as_str());

    let res: Value = client
        .get(format!("{url}/api/v1/{from}/{to}/{encode_text}"))
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res.as_object()?.get("translation")?.as_str()?.to_string();

        Some(result.replace("@@", "/"))
    }
    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("requestPath".to_string(), "lingva.pot-app.com".to_string());
        let result = translate("你好 世界！", "auto", "en", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
