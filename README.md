# Chyren — Alexa Skill Backend on Vercel

This repo hosts the Alexa skill backend as a Vercel Serverless Function.

## Architecture

```
User → Alexa → POST https://chyren.vercel.app/api/alexa → Vercel Function → Response
```

## Alexa Console Setup

1. Open [Alexa Developer Console](https://developer.amazon.com/alexa/console/ask)
2. Select your skill → **Build** tab → **Endpoint**
3. Choose **HTTPS**
4. Set endpoint URL to: `https://chyren.vercel.app/api/alexa`
5. SSL certificate type: **"My development endpoint is a sub-domain of a domain that has a wildcard certificate from a certificate authority"**
6. Click **Save Endpoints** → **Build Model**

## GitHub → Vercel CI/CD Setup

Add these three secrets to your GitHub repo (`Settings → Secrets → Actions`):

| Secret | Where to find it |
|---|---|
| `VERCEL_TOKEN` | [vercel.com/account/tokens](https://vercel.com/account/tokens) |
| `VERCEL_ORG_ID` | `.vercel/project.json` after first deploy (orgId) |
| `VERCEL_PROJECT_ID` | `.vercel/project.json` after first deploy (projectId) |

### First-time deploy (run locally once)

```bash
npm install
npx vercel login
npx vercel --yes   # links project, creates .vercel/project.json
npx vercel --prod  # production deploy
```

After this, every `git push` to `main` auto-deploys to production.

## Local Development

```bash
npm install
npx vercel dev
# Skill available at http://localhost:3000/api/alexa
```

## Adding Intents

Edit `api/alexa.js` — add a new handler object and register it in `SkillBuilders.custom().addRequestHandlers(...)`.

```js
const MyCustomIntentHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
      Alexa.getIntentName(input.requestEnvelope) === 'MyCustomIntent'
    );
  },
  handle(input) {
    return input.responseBuilder
      .speak('You triggered my custom intent!')
      .getResponse();
  },
};
```