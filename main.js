async function translate(text, _from, _to, options) {
    const { utils } = options;
    const { Database } = utils;

    const id = "plugin.com.pot-app.ecdict";

    const db = await Database.load(`sqlite:plugins/translate/${id}/stardict.db`);
    let res = await db.select('SELECT * FROM stardict WHERE word = $1', [text]);

    if (res.length > 0) {
        let result = res[0];
        let phonetic = result.phonetic;
        let translation = result.translation;
        let tag = result.tag;
        let exchange = result.exchange;
        let translation_list = translation.split("\n");
        let explanations = [];
        let associations = [];

        for (const line of translation_list) {
            let temp = line.split(".");
            let trait_name = "";
            let explains = [];

            if (temp.length > 1) {
                trait_name = temp[0];
                explains = temp[1].split(",");
            } else {
                trait_name = "";
                explains = temp[0].split(",");
            }
            let explain_list = [];
            for (const explain of explains) {
                explain_list.push(explain.trim());
            }
            explanations.push({
                "trait": trait_name,
                "explains": explain_list
            });
        }
        if (exchange !== null && exchange !== "") {
            for (const item of exchange.split("/")) {
                let temp = item.split(":");
                let word = temp[1];
                switch (temp[0]) {
                    case "p": associations.push(`过去式: ${word}`); break;
                    case "d": associations.push(`过去分词: ${word}`); break;
                    case "i": associations.push(`现在分词: ${word}`); break;
                    case "3": associations.push(`第三人称单数: ${word}`); break;
                    case "r": associations.push(`比较级: ${word}`); break;
                    case "t": associations.push(`最高级: ${word}`); break;
                    case "s": associations.push(`复数: ${word}`); break;
                    case "0": associations.push(`Lemma: ${word}`); break;
                    case "1": associations.push(`Lemma: ${word}`); break;
                }
            }
        }

        if (tag !== "") {
            associations.push("");
            associations.push(tag);
        }
        let target = {
            explanations
        };
        if (phonetic !== "") {
            target.pronunciations = [{ symbol: `/${phonetic}/` }];
        }
        if (associations.length > 0) {
            target.associations = associations;
        }
        return target;
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}
