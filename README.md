# Rainman

This is an abandoned project. The intention was to have a discord and/or Alexa bot that would let me know when it's raining `Alexa, tell me when it's raining`, however I realised during development that the data would not be reliable enough to be useful.

The code is a proof of concept, it is not production ready.

It would be similar to my other project [Captain Cold](https://github.com/connorjburton/captain-cold).

A short list of TODO items if this was to be continued.

- Integrate [AWS Lambda Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- Docker
- Accept requests to a Discord bot
  - `@rainman let me know when it's going to rain`
- Add configuration for an Alexa skill/routine
- Env vars for longitude/latitude
- Handle errors gracefully
- Discover most efficent serverless setup
  - How to accept Discord/Alexa startup commands without constantly running
  - How to cancel recurring jobs once rain is detected for a day
