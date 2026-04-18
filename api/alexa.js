import Alexa from 'ask-sdk-core';

// ── Handlers ──────────────────────────────────────────────────────────────────

const LaunchRequestHandler = {
canHandle(input) {
return Alexa.getRequestType(input.requestEnvelope) === 'LaunchRequest';
},
handle(input) {
const speech = 'Welcome to Chyren. How can I help you?';
return input.responseBuilder
.speak(speech)
.reprompt('What would you like to do?')
.getResponse();
},
};

const HelpIntentHandler = {
canHandle(input) {
return (
Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.HelpIntent'
);
},
handle(input) {
return input.responseBuilder
.speak('You can ask me anything. What would you like to know?')
.reprompt('What would you like to know?')
.getResponse();
},
};

const CancelAndStopIntentHandler = {
canHandle(input) {
return (
Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
(Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.CancelIntent' ||
Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.StopIntent')
);
},
handle(input) {
return input.responseBuilder.speak('Goodbye!').getResponse();
},
};

const SessionEndedRequestHandler = {
canHandle(input) {
return Alexa.getRequestType(input.requestEnvelope) === 'SessionEndedRequest';
},
handle(input) {
console.log('Session ended:', JSON.stringify(input.requestEnvelope.request.reason));
return input.responseBuilder.getResponse();
},
};

const ErrorHandler = {
canHandle() {
return true;
},
handle(input, error) {
console.error('Error:', error.message);
return input.responseBuilder
.speak('Sorry, I had trouble with that. Please try again.')
.reprompt('Please try again.')
.getResponse();
},
};

// ── Skill instance ─────────────────────────────────────────────────────────────

const skill = Alexa.SkillBuilders.custom()
.addRequestHandlers(
LaunchRequestHandler,
HelpIntentHandler,
CancelAndStopIntentHandler,
SessionEndedRequestHandler,
)
.addErrorHandlers(ErrorHandler)
.create();

// ── Vercel handler ─────────────────────────────────────────────────────────────

export default async function handler(req, res) {
if (req.method !== 'POST') {
return res.status(405).json({ error: 'Method not allowed' });
}
try {
const response = await skill.invoke(req.body, {});
return res.status(200).json(response);
} catch (err) {
console.error('Skill invocation error:', err);
return res.status(500).json({ error: 'Internal server error' });
}
}