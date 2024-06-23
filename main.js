async function translate(text, from, to, options) {
    const { utils } = options;
    const { tauriFetch: fetch } = utils;

    const res = await fetch(`https://pot-app.com/api/dict`, {
        method: 'POST',
        body: {
            type: "Json",
            payload: { text }
        }
    });

    if (res.ok) {
        let result = res.data;
        return result;
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}