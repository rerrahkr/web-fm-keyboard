// TypeScript bindings for emscripten-generated code.  Automatically generated at compile time.
declare namespace RuntimeExports {
    let HEAPF32: any;
    let HEAPF64: any;
    let HEAP_DATA_VIEW: any;
    let HEAP8: any;
    let HEAPU8: any;
    let HEAP16: any;
    let HEAPU16: any;
    let HEAP32: any;
    let HEAPU32: any;
    let HEAP64: any;
    let HEAPU64: any;
}
interface WasmModule {
  _ma_device__on_notification_unlocked(_0: number): void;
  _ma_malloc_emscripten(_0: number, _1: number): number;
  _ma_free_emscripten(_0: number, _1: number): void;
  _ma_device_process_pcm_frames_capture__webaudio(_0: number, _1: number, _2: number): void;
  _ma_device_process_pcm_frames_playback__webaudio(_0: number, _1: number, _2: number): void;
}

interface EmbindModule {
}

export type MainModule = WasmModule & typeof RuntimeExports & EmbindModule;
export default function MainModuleFactory (options?: unknown): Promise<MainModule>;
