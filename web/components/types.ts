export type SentMessage = {};

export type ReceivedMessage = WasmIsReadyMessage;

export type WasmIsReadyMessage = {
  type: "WasmIsReady";
};
