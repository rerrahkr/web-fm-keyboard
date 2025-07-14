import type { FmInstrument } from "@/lib/synth/synth";

// Sent.
export type SentMessage =
  | InitializeSynthMessage
  | DeinitializeSynthMessage
  | ChangeSamplingRateMessage
  | ChangeInstrumentMessage
  | GenerateAudioMessage;

export type InitializeSynthMessage = {
  type: "InitializeSynth";
};

export type DeinitializeSynthMessage = {
  type: "DeinitializeSynth";
};

export type ChangeSamplingRateMessage = {
  type: "ChangeSamplingRate";
  samplingRate: number;
};

export type ChangeInstrumentMessage = {
  type: "ChangeInstrument";
  instrument: FmInstrument;
};

export type GenerateAudioMessage = {
  type: "GenerateAudio";
  leftBuffer: SharedArrayBuffer;
  rightBuffer: SharedArrayBuffer;
};

// Received.
export type ReceivedMessage = SuccessMessage | ErrorMessage;
export type SuccessMessage =
  | WasmIsReadyMessage
  | SynthIsInitializedMessage
  | SynthIsDeinitializedMessage
  | FinishGenerateAudioMessage;
export type ErrorMessage =
  | CommonErrorMessage
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

export type CommonErrorMessage = {
  type: "CommonError";
  error: true;
  text?: string;
};

export type FinishGenerateAudioMessage = {
  type: "FinishGenerateAudio";
  error: false;
};
