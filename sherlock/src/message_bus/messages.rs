use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SherlockMessage {
    #[serde(rename = "type")]
    pub msg_type: SherlockMessageType,
    #[serde(default = "empty_map")]
    pub data: Value,
    #[serde(default = "empty_map")]
    pub context: Value,
}

pub fn empty_map() -> Value {
    // HashMap::new()
    json!({})
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SherlockMessageType {
    /// Speak message data as audio, emitted by the TTS Node.
    #[serde(rename = "spoken")]
    Spoken,
    /// Request to speak utterance
    #[serde(rename = "speak")]
    Speak,
    /// Internet connection is now available (only generated on initial connection)
    #[serde(rename = "mycroft.internet.connected")]
    MycroftInternetConnected,
    /// Sent by start-up sequence when everything is ready for user interaction
    #[serde(rename = "mycroft.ready")]
    MycroftReady,
    /// Stop command (e.g. button pressed)
    #[serde(rename = "mycroft.stop")]
    MycroftStop,
    /// Start the pairing process when this event is emitted.
    #[serde(rename = "mycroft.not.paired")]
    MycroftNotPaired,
    /// Pairing has completed
    #[serde(rename = "mycroft.paired")]
    MycroftPaired,
    /// Has come out of sleep mode
    #[serde(rename = "mycroft.awoken")]
    MycroftAwoken,
    /// log level can be: "CRITICAL" "ERROR" "WARNING" "INFO" "DEBUG" These correspond to the Python logging object. The "bus" parameter allows turning the logging of all bus messages on/off.
    #[serde(rename = "mycroft.debug.log")]
    MycroftDebugLog,
    /// Intent processing failed
    #[serde(rename = "complete_intent_failure")]
    CompleteIntentFailure,
    /// Notification to services that the configuration has changed and needs reloaded
    #[serde(rename = "configuration.updated")]
    ConfigurationUpdated,
    /// Wakeword was heard
    #[serde(rename = "recognizer_loop:wakeword")]
    RecognizerLoopWakeword,
    /// Recording has started
    #[serde(rename = "recognizer_loop:record_begin")]
    RecognizerLoopRecordBegin,
    /// Recording has ended
    #[serde(rename = "recognizer_loop:record_end")]
    RecognizerLoopRecordEnd,
    /// STT has detected the given text or text was injected as an utterance via the CLI.
    #[serde(rename = "recognizer_loop:utterance")]
    RecognizerLoopUtterance,
    /// Text output (TTS) has begun
    #[serde(rename = "recognizer_loop:audio_output_start")]
    RecognizerLoopAudioOutputStart,
    /// Text output (TTS) has ended
    #[serde(rename = "recognizer_loop:audio_output_end")]
    RecognizerLoopAudioOutputEnd,
    /// Go into "sleep" mode. Everything except "Hey Mycroft, wake up" will be ignored.
    #[serde(rename = "recognizer_loop:sleep")]
    RecognizerLoopSleep,
    /// Come out of "sleep" mode.
    #[serde(rename = "recognizer_loop:wake_up")]
    RecognizerLoopWakeUp,
    /// Detected a connection error during STT
    #[serde(rename = "enclosure.notify.no_internet")]
    EnclosureNotifyNoInternet,
    /// start: timestamp for audio starts (unix epoch) END_TIME: time in seconds from "start" until the end of the viseme CODE can be 0 = shape for sounds like 'y' or 'aa' 1 = shape for sounds like 'aw' 2 = shape for sounds like 'uh' or 'r' 3 = shape for sounds like 'th' or 'sh' 4 = neutral shape for no sound 5 = shape for sounds like 'f' or 'v' 6 = shape for sounds like 'oy' or 'ao'
    #[serde(rename = "enclosure.mouth.viseme_list")]
    EnclosureMouthVisemeList,
    /// Change eyes to default color
    #[serde(rename = "mycroft.eyes.default")]
    MycroftEyesDefault,
    /// Begin recording for STT processing
    #[serde(rename = "mycroft.mic.listen")]
    MycroftMicListen,
    /// Turn off the mic (no wakeword or STT processing)
    #[serde(rename = "mycroft.mic.mute")]
    MycroftMicMute,
    /// Turn on the mic (enable wakeword and STT processing)
    #[serde(rename = "mycroft.mic.unmute")]
    MycroftMicUnmute,
    /// Start playback of tracklist
    #[serde(rename = "mycroft.audio.service.play")]
    MycroftAudioServicePlay,
    /// Stop playback
    #[serde(rename = "mycroft.audio.service.stop")]
    MycroftAudioServiceStop,
    /// Pause playback (if supported)
    #[serde(rename = "mycroft.audio.service.pause")]
    MycroftAudioServicePause,
    /// Resume playback (if supported by backend)
    #[serde(rename = "mycroft.audio.service.resume")]
    MycroftAudioServiceResume,
    /// Skip to next track
    #[serde(rename = "mycroft.audio.service.next")]
    MycroftAudioServiceNext,
    /// Skip to previous track
    #[serde(rename = "mycroft.audio.service.prev")]
    MycroftAudioServicePrev,
    /// Request track info from audio service
    #[serde(rename = "mycroft.audio.service.track_info")]
    MycroftAudioServiceTrackInfo,
    /// Reply to track info request
    #[serde(rename = "mycroft.audio.service.track_info_reply")]
    MycroftAudioServiceTrackInfoReply,
    /// Returns list of available backends.
    #[serde(rename = "mycroft.audio.service.list_backends")]
    MycroftAudioServiceListBackends,
    /// Enclosure Volume up
    #[serde(rename = "mycroft.volume.increase")]
    MycroftVolumeIncrease,
    /// Enclosure Volume down
    #[serde(rename = "mycroft.volume.decrease")]
    MycroftVolumeDecrease,
    /// Enclosure Volume muted
    #[serde(rename = "mycroft.volume.mute")]
    MycroftVolumeMute,
    /// Enclosure Volume unmuted
    #[serde(rename = "mycroft.volume.unmute")]
    MycroftVolumeUnmute,
    /// Set enclosure volume (0.0 = no output, 1.0 = loudest possible)
    #[serde(rename = "mycroft.volume.set")]
    MycroftVolumeSet,
    /// Request volume level
    #[serde(rename = "mycroft.volume.get")]
    MycroftVolumeGet,
    ///
    #[serde(rename = "mycroft.volume.get.response")]
    MycroftVolumeGetResponse,
    /// Reduce the volume level temporarily
    #[serde(rename = "mycroft.volume.duck")]
    MycroftVolumeDuck,
    /// Restore the volume level
    #[serde(rename = "mycroft.volume.unduck")]
    MycroftVolumeUnduck,
    ///
    #[serde(rename = "mycroft.skill.handler.start")]
    MycroftSkillHandlerStart,
    ///
    #[serde(rename = "mycroft.skill.handler.complete")]
    MycroftSkillHandlerComplete,
    /// Enable disabled intent
    #[serde(rename = "mycroft.skill.enable_intent")]
    MycroftSkillEnableIntent,
    /// Disable intent
    #[serde(rename = "mycroft.skill.disable_intent")]
    MycroftSkillDisableIntent,
    /// A Skill has been loaded
    #[serde(rename = "mycroft.skills.loaded")]
    MycroftSkillsLoaded,
    /// A Skill has failed to load
    #[serde(rename = "mycroft.skills.loading_failure")]
    MycroftSkillsLoadingFailure,
    /// A Skill has shutdown
    #[serde(rename = "mycroft.skills.shutdown")]
    MycroftSkillsShutdown,
    /// Upon startup, all skills have been loaded
    #[serde(rename = "mycroft.skills.initialized")]
    MycroftSkillsInitialized,
    /// List of loaded skills (response to 'skillmanager.list')
    #[serde(rename = "mycroft.skills.list")]
    MycroftSkillsList,
    /// Pull new skill settings from the server
    #[serde(rename = "mycroft.skills.settings.update")]
    MycroftSkillsSettingsUpdate,
    /// MSM install has begun
    #[serde(rename = "msm.updating")]
    MsmUpdating,
    /// MSM update has begun
    #[serde(rename = "msm.installing")]
    MsmInstalling,
    /// MSM install succeeded for given skill
    #[serde(rename = "msm.install.succeeded")]
    MsmInstallSucceeded,
    /// MSM install failed for given skill
    #[serde(rename = "msm.install.failed")]
    MsmInstallFailed,
    /// MSM install is complete
    #[serde(rename = "msm.installed")]
    MsmInstalled,
    /// MSM update is complete
    #[serde(rename = "msm.updated")]
    MsmUpdated,
    /// MSM remove has begun
    #[serde(rename = "msm.removing")]
    MsmRemoving,
    /// MSM remove succeeded for given skill
    #[serde(rename = "msm.remove.succeeded")]
    MsmRemoveSucceeded,
    /// MSM remove failed for given skill
    #[serde(rename = "msm.remove.failed")]
    MsmRemoveFailed,
    /// MSM remove is complete
    #[serde(rename = "msm.removed")]
    MsmRemoved,
    /// Deactivate a skill. Activate by typing ":deactivate " in the CLI
    #[serde(rename = "skillmanager.deactivate")]
    SkillmanagerDeactivate,
    /// List installed skills. Activate by typing ":list" in the CLI
    #[serde(rename = "skillmanager.list")]
    SkillmanagerList,
    /// Request immediate update of all skills
    #[serde(rename = "skillmanager.update")]
    SkillmanagerUpdate,
    /// websocket connection has closed
    #[serde(rename = "open")]
    Open,
    /// websocket connection was lost, reconnecting
    #[serde(rename = "close")]
    Close,
    /// websocket connection has opened
    #[serde(rename = "reconnecting")]
    Reconnecting,
    /// Kick off a a wifi-setup session
    #[serde(rename = "system.wifi.setup")]
    SystemWifiSetup,
    /// Clear the saved wifi settings
    #[serde(rename = "system.wifi.reset")]
    SystemWifiReset,
    /// Force the system clock to synchronize with NTP servers
    #[serde(rename = "system.ntp.sync")]
    SystemNtpSync,
    /// Configure system to allow SSH connections
    #[serde(rename = "system.ssh.enable")]
    SystemSshEnable,
    /// Configure system to block SSH connections
    #[serde(rename = "system.ssh.disable")]
    SystemSshDisable,
    /// Force a Linux reboot
    #[serde(rename = "system.reboot")]
    SystemReboot,
    /// Force a Linux shutdown
    #[serde(rename = "system.shutdown")]
    SystemShutdown,
    /// Force an apt-get update on 'mycroft-mark-1' or 'mycroft-picroft' package (as appropriate)
    #[serde(rename = "system.update")]
    SystemUpdate,
    ///
    #[serde(rename = "play:query")]
    PlayQuery,
    /// There are three responses to a play:query. These are not intended to be consumed directly by a Skill, see the methods available in the CommonPlaySkill Class.
    #[serde(rename = "play:query.response")]
    PlayQueryResponse,
    ///
    #[serde(rename = "play:start")]
    PlayStart,
    ///
    #[serde(rename = "question:query")]
    QuestionQuery,
    ///
    #[serde(rename = "question:query.response")]
    QuestionQueryResponse,
    ///
    #[serde(rename = "question:action")]
    QuestionAction,
    /// Count of running alarms (0 == no alarms)
    #[serde(rename = "private.mycroftai.has_alarm")]
    PrivateMycroftaiHasAlarm,
}
