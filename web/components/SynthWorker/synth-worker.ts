import type { ReceivedMessage, SentMessage } from "@/components/types";
import createSynthModule, {
	type FmInstrument,
	type MainModule,
} from "@/lib/synth/synth";

let module: MainModule;

// Helper function to use type inference for creating messages.
function postCustomMessage(message: ReceivedMessage) {
	postMessage(message);
}

createSynthModule({
	locateFile: (fileName: string) => `/wasm/${fileName}`,
})
	.then((mod) => {
		module = mod;
		postCustomMessage({
			type: "WasmIsReady",
			error: false,
		});
	})
	.catch((error: unknown) => {
		if (error instanceof Error) {
			throw new Error(`Failed to create synth module: ${error.message}`);
		} else {
			throw new Error(
				"An unknown error occurred while creating the synth module.",
			);
		}
	});

// biome-ignore lint/suspicious/noGlobalAssign: Web worker.
onmessage = ({ data: message }: { data: SentMessage }) => {
	if (!module) {
		postCustomMessage({
			type: "WasmIsNotLoaded",
			error: true,
		});
		return;
	}

	switch (message.type) {
		case "InitializeSynth":
			try {
				const result = module.initialize();
				if (result) {
					console.log("Synth initialized successfully");
					postCustomMessage({
						type: "SynthIsInitialized",
						error: false,
					});
				} else {
					console.error("Synth initialization returned false");
					postCustomMessage({
						type: "FailedToInitializeSynth",
						error: true,
					});
				}
			} catch (error) {
				console.error("Error during synth initialization:", error);
				postCustomMessage({
					type: "FailedToInitializeSynth",
					error: true,
				});
			}
			break;

		case "ChangeSamplingRate":
			module.setSamplingRate(message.samplingRate);
			console.log("Sampling rate changed to", message.samplingRate);
			break;

		case "ChangeInstrument":
			try {
				module.setInstrument(message.instrument);
				console.log("Instrument changed successfully:", message.instrument);
			} catch (error) {
				console.error("Error setting instrument:", error);
				postCustomMessage({
					type: "CommonError",
					error: true,
					text: `Error setting instrument: ${error}`,
				});
			}
			break;

		case "GenerateAudio":
			console.log("received GenerateAudio message");
			generateAudio(message.leftBuffer, message.rightBuffer);
			postCustomMessage({
				type: "FinishGenerateAudio",
				error: false,
			});
			break;

		case "DeinitializeSynth":
			if (module.deinitialize()) {
				postCustomMessage({
					type: "SynthIsDeinitialized",
					error: false,
				});
			} else {
				postCustomMessage({
					type: "FailedToDeinitializeSynth",
					error: true,
				});
			}
			break;

		default:
			break;
	}
};

function generateAudio(
	leftBuffer: SharedArrayBuffer,
	rightBuffer: SharedArrayBuffer,
) {
	if (!module) {
		postCustomMessage({
			type: "WasmIsNotLoaded",
			error: true,
		});
		return;
	}

	console.log("start generate audio");
	console.log("Buffer info:", {
		leftBufferSize: leftBuffer.byteLength,
		rightBufferSize: rightBuffer.byteLength,
	});

	const secondLength = 44100;

	const leftView = new Float32Array(leftBuffer);
	const rightView = new Float32Array(rightBuffer);

	console.log("Float32Array views created:", {
		leftLength: leftView.length,
		rightLength: rightView.length,
	});

	// バッファをクリア
	leftView.fill(0);
	rightView.fill(0);

	try {
		// WASM関数の呼び出しをtry-catchで囲む
		console.log("Testing WASM module functions...");

		// モジュールの状態をチェック
		console.log("Module methods available:", {
			hasInitialize: typeof module.initialize === "function",
			hasGenerate: typeof module.generate === "function",
			hasNoteOn: typeof module.noteOn === "function",
			hasNoteOff: typeof module.noteOff === "function",
			hasSetInstrument: typeof module.setInstrument === "function",
			hasSetSamplingRate: typeof module.setSamplingRate === "function",
			hasNoteName: !!module.NoteName,
		});

		// ここで明示的にサンプリングレートとインストゥルメントを再設定
		console.log("Re-setting sampling rate and instrument...");
		try {
			module.setSamplingRate(44100);
			console.log("Sampling rate re-set to 44100");
		} catch (error) {
			console.error("Error re-setting sampling rate:", error);
		}

		// テスト用のシンプルなインストゥルメント
		const testInstrument: FmInstrument = {
			al: 0x04,
			fb: 0x07,
			op: [
				{
					ar: 0x1f,
					dr: 0x00,
					sr: 0x00,
					rr: 0x00,
					sl: 0x00,
					tl: 0,
					ks: 0x00,
					ml: 0x01,
					dt: 0x00,
					am: false,
					ssg_eg: 0x00,
				},
				{
					ar: 0x1f,
					dr: 0x00,
					sr: 0x00,
					rr: 0x07,
					sl: 0x00,
					tl: 0,
					ks: 0x00,
					ml: 0x01,
					dt: 0x00,
					am: false,
					ssg_eg: 0x00,
				},
				{
					ar: 0x1f,
					dr: 0x00,
					sr: 0x00,
					rr: 0x00,
					sl: 0x00,
					tl: 63,
					ks: 0x00,
					ml: 0x01,
					dt: 0x00,
					am: false,
					ssg_eg: 0x00,
				},
				{
					ar: 0x1f,
					dr: 0x00,
					sr: 0x00,
					rr: 0x07,
					sl: 0x00,
					tl: 63,
					ks: 0x00,
					ml: 0x01,
					dt: 0x00,
					am: false,
					ssg_eg: 0x00,
				},
			] as const,
			lfo_freq: 0,
			ams: 0,
			pms: 0,
		};

		try {
			module.setInstrument(testInstrument);
			console.log("Test instrument set successfully");
		} catch (error) {
			console.error("Error setting test instrument:", error);
		}

		const tmpSize = 1024;
		let remainLength = leftView.length;

		// まず小さなテストを行う
		console.log("Testing noteOn and generate with small buffer...");
		const testBuffer = new Float32Array(1024);
		const testBufferR = new Float32Array(1024);

		try {
			module.noteOn({ name: module.NoteName.C, octave: 4 });
			console.log("Test noteOn successful");

			module.generate(testBuffer, testBufferR, 1024);
			console.log("Test generate successful");

			const hasTestData =
				testBuffer.some((v) => v !== 0) || testBufferR.some((v) => v !== 0);
			console.log(
				"Test data generated:",
				hasTestData,
				"Sample values:",
				testBuffer.slice(0, 5),
			);

			module.noteOff({ name: module.NoteName.C, octave: 4 });
			console.log("Test noteOff successful");

			if (!hasTestData) {
				console.warn("Test generation failed - no audio data produced");
				return;
			}
		} catch (error) {
			console.error("Test generation failed:", error);
			return;
		}

		// テストが成功した場合、実際の生成を開始
		console.log("Starting actual audio generation...");

		// 最初の無音期間（0.5秒）
		let repeatLength = secondLength / 2;
		while (repeatLength > 0) {
			const length = Math.min(tmpSize, repeatLength);
			const offset = leftView.length - remainLength;

			try {
				module.generate(
					leftView.subarray(offset, offset + length),
					rightView.subarray(offset, offset + length),
					length,
				);
			} catch (error) {
				console.error("Error during generate (silence):", error);
				throw error;
			}

			repeatLength -= length;
			remainLength -= length;
		}

		console.log("note on C5");

		try {
			module.noteOn({ name: module.NoteName.C, octave: 5 });
		} catch (error) {
			console.error("Error during noteOn:", error);
			throw error;
		}

		// 音が鳴る期間（2秒）
		repeatLength = secondLength * 2;
		while (repeatLength > 0) {
			const length = Math.min(tmpSize, repeatLength);
			const offset = leftView.length - remainLength;

			try {
				module.generate(
					leftView.subarray(offset, offset + length),
					rightView.subarray(offset, offset + length),
					length,
				);

				// 一定間隔でデータをチェック
				if (offset % 44100 === 0) {
					const currentSample = leftView[offset];
					console.log(`Sample at offset ${offset}: ${currentSample}`);
				}
			} catch (error) {
				console.error("Error during generate (audio):", error);
				throw error;
			}

			repeatLength -= length;
			remainLength -= length;
		}

		console.log("note off C5");

		try {
			module.noteOff({ name: module.NoteName.C, octave: 5 });
		} catch (error) {
			console.error("Error during noteOff:", error);
			throw error;
		}

		// 残りの期間（リリース）
		while (remainLength > 0) {
			const length = Math.min(tmpSize, remainLength);
			const offset = leftView.length - remainLength;

			try {
				module.generate(
					leftView.subarray(offset, offset + length),
					rightView.subarray(offset, offset + length),
					length,
				);
			} catch (error) {
				console.error("Error during generate (release):", error);
				throw error;
			}

			remainLength -= length;
		}

		// 生成後のデータをチェック
		const hasLeftData = leftView.some((v) => v !== 0);
		const hasRightData = rightView.some((v) => v !== 0);

		// 最大値・最小値を安全に計算
		let maxLeftValue = -Infinity;
		let minLeftValue = Infinity;
		let maxRightValue = -Infinity;
		let minRightValue = Infinity;

		for (let i = 0; i < leftView.length; i++) {
			if (leftView[i] > maxLeftValue) maxLeftValue = leftView[i];
			if (leftView[i] < minLeftValue) minLeftValue = leftView[i];
			if (rightView[i] > maxRightValue) maxRightValue = rightView[i];
			if (rightView[i] < minRightValue) minRightValue = rightView[i];
		}

		console.log("Generated audio data check:", {
			hasLeftData,
			hasRightData,
			leftSample: leftView.slice(44100, 44110), // 1秒後のサンプル
			rightSample: rightView.slice(44100, 44110),
			maxLeftValue,
			minLeftValue,
			maxRightValue,
			minRightValue,
		});

		console.log("finished generating audio");
	} catch (error) {
		console.error("WASM error during audio generation:", error);
		postCustomMessage({
			type: "CommonError",
			error: true,
			text: `WASM error during audio generation: ${error}`,
		});
	}
}
