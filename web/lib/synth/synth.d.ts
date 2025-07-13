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
	_ma_device_process_pcm_frames_capture__webaudio(
		_0: number,
		_1: number,
		_2: number,
	): void;
	_ma_device_process_pcm_frames_playback__webaudio(
		_0: number,
		_1: number,
		_2: number,
	): void;
};

export type ChipTypeValue<T extends number> = {
	value: T;
};

export type ChipType = ChipTypeValue<0>;

export type NoteNameValue<T extends number> = {
	value: T;
};

export type NoteName =
	| NoteNameValue<0>
	| NoteNameValue<1>
	| NoteNameValue<2>
	| NoteNameValue<3>
	| NoteNameValue<4>
	| NoteNameValue<5>
	| NoteNameValue<6>
	| NoteNameValue<7>
	| NoteNameValue<8>
	| NoteNameValue<9>
	| NoteNameValue<10>
	| NoteNameValue<11>;

export type Note = {
	name: NoteName;
	octave: number;
};

export type FmOperator = {
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
	am: boolean;
};

export type FmOperatorArray = [FmOperator, FmOperator, FmOperator, FmOperator];

export type FmInstrument = {
	al: number;
	fb: number;
	op: FmOperatorArray;
	lfo_freq: number;
	ams: number;
	pms: number;
};

type EmbindModule = {
	ChipType: { Ym2608: ChipTypeValue<0> };
	NoteName: {
		C: NoteNameValue<0>;
		Cs: NoteNameValue<1>;
		D: NoteNameValue<2>;
		Eb: NoteNameValue<3>;
		E: NoteNameValue<4>;
		F: NoteNameValue<5>;
		Fs: NoteNameValue<6>;
		G: NoteNameValue<7>;
		Gs: NoteNameValue<8>;
		A: NoteNameValue<9>;
		Bb: NoteNameValue<10>;
		B: NoteNameValue<11>;
	};
	initialize(): boolean;
	deinitialize(): boolean;
	reset(): void;
	changeChip(type: ChipType): boolean;
	setSamplingRate(rate: number): boolean;
	noteOn(note: Note): void;
	noteOff(note: Note): void;
	setInstrument(instrument: FmInstrument): void;
	generate(
		leftBuffer: BufferView,
		rightBuffer: BufferView,
		numSamples: number,
	): void;
};

type BufferView = SharedArrayBuffer | ArrayBuffer | Float32Array;

export type MainModule = WasmModule & typeof RuntimeExports & EmbindModule;
export default function MainModuleFactory(options?: {
	locateFile?: (fileName: string) => string;
}): Promise<MainModule>;
