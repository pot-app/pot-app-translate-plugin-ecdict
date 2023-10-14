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
        let translation_list = translation.split("\n").collect::<Vec<&str>>();
        let mut explanations: Vec<Value> = Vec::new();

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

        return Ok(json!({
          "pronunciations": [
            {
              "symbol": phonetic
            }
          ],
          "explanations": explanations,
          "associations": tag.split(",").collect::<Vec<&str>>(),
        }));
    }
    Err("Not found".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("hello", "auto", "zh_cn", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
