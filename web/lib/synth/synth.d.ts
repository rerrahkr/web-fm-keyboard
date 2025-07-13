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

type WasmModule = {
  _ma_device__on_notification_unlocked(_0: number): void;
  _ma_malloc_emscripten(_0: number, _1: number): number;
  _ma_free_emscripten(_0: number, _1: number): void;
  _ma_device_process_pcm_frames_capture__webaudio(_0: number, _1: number, _2: number): void;
  _ma_device_process_pcm_frames_playback__webaudio(_0: number, _1: number, _2: number): void;
}

type EmbindModule = {
  // Enums.
  ChipType: {
    Ym2608: 0;
  };
  
  NoteName: {
    C: 0;
    Cs: 1;
    D: 2;
    Eb: 3;
    E: 4;
    F: 5;
    Fs: 6;
    G: 7;
    Gs: 8;
    A: 9;
    Bb: 10;
    B: 11;
  };

  // Types for value objects.
  Note: typeof Note;
  FmInstrument: typeof FmInstrument;
  FmOperator: typeof FmOperator;

  // Functions.
  initialize(): boolean;
  deinitialize(): boolean;
  reset(): void;
  changeChip(type: ChipTypeValue): boolean;
  setSamplingRate(rate: number): boolean;
  noteOn(note: Note): void;
  noteOff(note: Note): void;
  setInstrument(instrument: FmInstrument): void;
  generate(leftBuffer: BufferView, rightBuffer: BufferView, numSamples: number): void;
}

type BufferView = SharedArrayBuffer | ArrayBuffer |  Float32Array;
type ChipTypeValue = EmbindModule["ChipType"][keyof EmbindModule["ChipType"]];
type NoteNameValue = EmbindModule["NoteName"][keyof EmbindModule["NoteName"]];

// Value object definitions.
class Note {
  name: NoteNameValue;
  octave: number;

  constructor();
}

class FmOperator {
  ar: number;
  dr: number;
  sr: number;
  rr: number;
  sl: number;
  tl: number;
  ks: number;
  ml: number;
  dt: number;
  ssg_eg: number;
  am: number;

  constructor();
}

class FmInstrument {
  al: number;
  fb: number;
  op: [FmOperator, FmOperator, FmOperator, FmOperator];
  lfo_freq: number;
  ams: number;
  pms: number;

  constructor();
}

export type MainModule = WasmModule & typeof RuntimeExports & EmbindModule;
export default function MainModuleFactory (options?: {
  locateFile: (fileName: string) => string;
}): Promise<MainModule>;
