# Sherlock Messages

Ultimately Sherlock Messages are json formatted strings in the following format:

```json
{
    "type": ...,
    "data": ...,
    "context": ...,
}
```

The `type` key is a string representing the type of message. (see: [the mycroft docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md) for more details)

The `data` and `context` keys are json values of any variety, their structure and contents are message type specififc. (see: [the mycroft docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md) for more details)

## The Type Key

The `type` key is a string representing the type of message. (see: [the mycroft docs](https://github.com/MycroftAI/documentation/blob/master/docs/mycroft-technologies/mycroft-core/message-types.md) for more details)

These type can be anything from "speak", to "mycroft.internet.connected". The formner instructing Sherlock to speak to the user ussing tts, and the later allerting skills and internal components that Sherlock has connected to the internet.

## The Data Key

TODO: write

## The Context Key

TODO: write
