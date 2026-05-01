const https = require('https');

exports.handler = async (event) => {
    const tunnelUrl = "";
    const userQuery = event.request.intent.slots.Query.value;

    const chyrenRequest = JSON.stringify({
        task: userQuery,
        provider: "local_ollama"
    });

    const options = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
    };

    return new Promise((resolve, reject) => {
        const req = https.request(tunnelUrl + '/api/v1/task', options, (res) => {
            let data = '';
            res.on('data', (chunk) => data += chunk);
            res.on('end', () => {
                const response = JSON.parse(data);
                resolve({
                    version: "1.0",
                    response: {
                        outputSpeech: { type: "PlainText", text: response.text },
                        shouldEndSession: false
                    }
                });
            });
        });
        req.write(chyrenRequest);
        req.end();
    });
};
