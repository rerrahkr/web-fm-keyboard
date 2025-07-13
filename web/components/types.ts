// Sent.
export type SentMessage = InitializeSynthMessage | DeinitializeSynthMessage;

export type InitializeSynthMessage = {
  type: "InitializeSynth";
};

export type DeinitializeSynthMessage = {
  type: "DeinitializeSynth";
};

// Received.
export type ReceivedMessage = SuccessMessage | ErrorMessage;
export type SuccessMessage =
  | WasmIsReadyMessage
  | SynthIsInitializedMessage
  | SynthIsDeinitializedMessage;
export type ErrorMessage =
  | WasmIsNotLoadedMessage
  | FailedToInitializeSynthMessage
  | FailedToDeinitializeSynthMessage;

export type WasmIsReadyMessage = {
  type: "WasmIsReady";
  error: false;
};

export type WasmIsNotLoadedMessage = {
  type: "WasmIsNotLoaded";
  error: true;
};

export type SynthIsInitializedMessage = {
  type: "SynthIsInitialized";
  error: false;
};

export type FailedToInitializeSynthMessage = {
  type: "FailedToInitializeSynth";
  error: true;
};

export type SynthIsDeinitializedMessage = {
  type: "SynthIsDeinitialized";
  error: false;
};

export type FailedToDeinitializeSynthMessage = {
  type: "FailedToDeinitializeSynth";
  error: true;
};
