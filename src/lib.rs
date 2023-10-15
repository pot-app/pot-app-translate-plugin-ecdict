use dirs::config_dir;
use futures::TryStreamExt;
use serde_json::{json, Value};
use sqlx::{Connection, Row, SqliteConnection};
use std::collections::HashMap;
use std::error::Error;
use tokio::runtime::Runtime;

#[no_mangle]
pub fn translate(
    text: &str,
    _from: &str,
    _to: &str,
    _detect: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let rt = Runtime::new()?;
    return Ok(rt.block_on(do_work(text))?);
}
async fn do_work(text: &str) -> Result<Value, Box<dyn Error>> {
    let config_dir_path = config_dir().unwrap();

    let db_path = config_dir_path
        .join("com.pot-app.desktop")
        .join("plugins")
        .join("translate")
        .join("[plugin].com.pot-app.ecdict")
        .join("stardict.db");

    let db_path = db_path.to_str().unwrap();

    let mut conn = SqliteConnection::connect(&format!("sqlite:{db_path}")).await?;

    let mut rows = sqlx::query("SELECT * FROM stardict WHERE word = ?")
        .bind(text)
        .fetch(&mut conn);

    while let Some(row) = rows.try_next().await? {
        let phonetic: String = row.try_get("phonetic")?;
        let translation: String = row.try_get("translation")?;
        let tag: String = row.try_get("tag")?;
        let exchange: String = row.try_get("exchange")?;
        let translation_list = translation.split("\n").collect::<Vec<&str>>();
        let mut explanations: Vec<Value> = Vec::new();
        let mut associations: Vec<String> = Vec::new();

        for line in translation_list {
            let temp = line.split(".").collect::<Vec<&str>>();
            let mut trait_name = "";
            let mut explains = Vec::new();

            if temp.len() > 1 {
                trait_name = temp[0];
                explains = temp[1].split(",").collect::<Vec<&str>>();
            } else {
                trait_name = "";
                explains = temp[0].split(",").collect::<Vec<&str>>();
            }
            let mut explain_list: Vec<Value> = Vec::new();
            for explain in explains {
                explain_list.push(Value::String(explain.to_string()));
            }
            explanations.push(json!({
                "trait": trait_name,
                "explains": explain_list
            }));
        }
        if !exchange.is_empty() {
            for item in exchange.split("/") {
                println!("{item}");
                let temp = item.split(":").collect::<Vec<&str>>();

                let word = temp[1];
                match temp[0] {
                    "p" => associations.push(format!("过去式: {word}")),
                    "d" => associations.push(format!("过去分词: {word}")),
                    "i" => associations.push(format!("现在分词: {word}")),
                    "3" => associations.push(format!("第三人称单数: {word}")),
                    "r" => associations.push(format!("比较级: {word}")),
                    "t" => associations.push(format!("最高级: {word}")),
                    "s" => associations.push(format!("复数: {word}")),
                    "0" => associations.push(format!("Lemma: {word}")),
                    "1" => associations.push(format!("Lemma: {word}")),
                    _ => {}
                }
            }
        }

        if !tag.is_empty() {
            associations.push("".to_string());
            associations.push(tag);
        }
        let mut result = json!({
          "explanations": explanations
        });
        if !phonetic.is_empty() {
            result.as_object_mut().unwrap().insert(
                "pronunciations".to_string(),
                json!([
                  {
                    "symbol": format!("/{phonetic}/")
                  }
                ]),
            );
        }
        if !associations.is_empty() {
            result
                .as_object_mut()
                .unwrap()
                .insert("associations".to_string(), associations.into());
        }

        return Ok(result);
    }
    Err("Not found".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("hello", "auto", "zh_cn", "en", needs).unwrap();
        println!("{result}");
    }
}
