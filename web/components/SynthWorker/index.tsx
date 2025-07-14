"use client";

import type React from "react";
import { useEffect, useRef } from "react";
import type { ReceivedMessage, SentMessage } from "@/components/types";
import type { FmInstrument } from "@/lib/synth/synth";

const secondLength = 44100;

const leftBuffer = new SharedArrayBuffer(
  3 * secondLength * Float32Array.BYTES_PER_ELEMENT
);
const rightBuffer = new SharedArrayBuffer(leftBuffer.byteLength);
// Fill buffers with dummy data for testing.
new Float32Array(leftBuffer).fill(12);
new Float32Array(rightBuffer).fill(12);

/**
 * Float32ArrayデータからWAVファイルのBlobを作成する
 * @param leftChannel 左チャンネルのオーディオデータ
 * @param rightChannel 右チャンネルのオーディオデータ
 * @param sampleRate サンプリングレート
 * @returns WAVファイルのBlob
 */
function createWavBlob(
  leftChannel: Float32Array,
  rightChannel: Float32Array,
  sampleRate: number
): Blob {
  const length = Math.min(leftChannel.length, rightChannel.length);
  const channels = 2;
  const bytesPerSample = 2; // 16ビット
  const byteRate = sampleRate * channels * bytesPerSample;
  const blockAlign = channels * bytesPerSample;
  const dataSize = length * blockAlign;
  const fileSize = 36 + dataSize;

  console.log("WAV file info:", {
    length,
    sampleRate,
    channels,
    bytesPerSample,
    dataSize,
    fileSize
  });

  const arrayBuffer = new ArrayBuffer(44 + dataSize);
  const view = new DataView(arrayBuffer);

  // WAVヘッダーを書き込み
  const writeString = (offset: number, string: string) => {
    for (let i = 0; i < string.length; i++) {
      view.setUint8(offset + i, string.charCodeAt(i));
    }
  };

  // RIFFヘッダー
  writeString(0, 'RIFF');
  view.setUint32(4, fileSize, true);
  writeString(8, 'WAVE');

  // fmtチャンク
  writeString(12, 'fmt ');
  view.setUint32(16, 16, true); // fmtチャンクサイズ
  view.setUint16(20, 1, true); // オーディオフォーマット (PCM)
  view.setUint16(22, channels, true); // チャンネル数
  view.setUint32(24, sampleRate, true); // サンプリングレート
  view.setUint32(28, byteRate, true); // バイトレート
  view.setUint16(32, blockAlign, true); // ブロックアライン
  view.setUint16(34, 16, true); // ビット深度

  // dataチャンク
  writeString(36, 'data');
  view.setUint32(40, dataSize, true);

  // インターリーブしたオーディオデータを書き込み
  let offset = 44;
  for (let i = 0; i < length; i++) {
    // Float32を16ビット整数に変換 (-1.0 ~ 1.0 を -32768 ~ 32767 に変換)
    let leftSample = Math.max(-1, Math.min(1, leftChannel[i] || 0));
    let rightSample = Math.max(-1, Math.min(1, rightChannel[i] || 0));
    
    // より正確な変換
    leftSample = leftSample < 0 ? leftSample * 32768 : leftSample * 32767;
    rightSample = rightSample < 0 ? rightSample * 32768 : rightSample * 32767;
    
    view.setInt16(offset, Math.round(leftSample), true);
    view.setInt16(offset + 2, Math.round(rightSample), true);
    offset += 4;
  }

  return new Blob([arrayBuffer], { type: 'audio/wav' });
}

const instrument: FmInstrument = {
  al: 0x04,
  fb: 0x07,
  op: [
    {
      ar: 0x1f,
      dr: 0x00,
      sr: 0x00,
      rr: 0x00,
      sl: 0x00,
      tl: 28,
      ks: 0x00,
      ml: 0x04,
      dt: 0x00,
      am: false,
      ssg_eg: 0x00,
    },
    {
      ar: 0x1f,
      dr: 10,
      sr: 0x00,
      rr: 0x07,
      sl: 0x01,
      tl: 0,
      ks: 0x00,
      ml: 4,
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
      tl: 21,
      ks: 0x00,
      ml: 4,
      dt: 0x03,
      am: false,
      ssg_eg: 0x00,
    },
    {
      ar: 0x1f,
      dr: 10,
      sr: 0x00,
      rr: 0x07,
      sl: 0x01,
      tl: 0x00,
      ks: 0x00,
      ml: 0x04,
      dt: 0x03,
      am: false,
      ssg_eg: 0x00,
    },
  ],
  lfo_freq: 0,
  ams: 0,
  pms: 0,
};

/**
 * Dummy component to control the web worker for synth module.
 * @returns `null`.
 */
export function SynthWorker(): React.JSX.Element {
  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    const worker = new Worker(new URL("./synth-worker.ts", import.meta.url), {
      type: "module",
    });
    workerRef.current = worker;

    worker.onmessage = ({ data: message }: { data: ReceivedMessage }) => {
      if (!message.error) {
        switch (message.type) {
          case "WasmIsReady":
            console.log("Wasm module is ready");
            worker.postMessage({
              type: "InitializeSynth",
            } satisfies SentMessage);
            break;

          case "SynthIsInitialized":
            console.log("Synth module is initialized");

            worker.postMessage({
              type: "ChangeSamplingRate",
              samplingRate: 44100,
            } satisfies SentMessage);

            worker.postMessage({
              type: "ChangeInstrument",
              instrument,
            } satisfies SentMessage);
            break;

          case "FinishGenerateAudio": {
            console.log("Audio generation finished");
            const leftView = new Float32Array(leftBuffer);
            const rightView = new Float32Array(rightBuffer);

            // デバッグ情報を出力
            console.log("Buffer info:", {
              leftBufferSize: leftBuffer.byteLength,
              rightBufferSize: rightBuffer.byteLength,
              leftLength: leftView.length,
              rightLength: rightView.length,
              firstLeftSamples: leftView.slice(0, 10),
              firstRightSamples: rightView.slice(0, 10),
              middleLeftSamples: leftView.slice(Math.floor(leftView.length/2), Math.floor(leftView.length/2) + 10),
              middleRightSamples: rightView.slice(Math.floor(rightView.length/2), Math.floor(rightView.length/2) + 10)
            });

            // 実際にデータがあるかチェック
            const hasData = leftView.some(v => v !== 0) || rightView.some(v => v !== 0);
            console.log("Has audio data:", hasData);

            if (!hasData) {
              console.warn("No audio data found in buffers");
              return;
            }

            // WAVファイルを作成して再生
            const wavBlob = createWavBlob(leftView, rightView, 44100);
            console.log("WAV blob created, size:", wavBlob.size);
            
            const audioUrl = URL.createObjectURL(wavBlob);
            const audio = new Audio(audioUrl);
            
            // オーディオイベントリスナーを追加
            audio.addEventListener('loadstart', () => console.log("Audio load started"));
            audio.addEventListener('loadeddata', () => console.log("Audio data loaded"));
            audio.addEventListener('canplay', () => console.log("Audio can play"));
            audio.addEventListener('playing', () => console.log("Audio is playing"));
            audio.addEventListener('ended', () => {
              console.log("Audio playback ended, revoking URL");
              URL.revokeObjectURL(audioUrl);
            });
            audio.addEventListener('error', (e) => {
              console.error("Audio error:", e);
              URL.revokeObjectURL(audioUrl);
            });

            audio.play().then(() => {
              console.log("Audio playback started");
            }).catch((error) => {
              console.error("Failed to play audio:", error);
              URL.revokeObjectURL(audioUrl);
            });

            break;
          }

          default:
            break;
        }
      } else {
        switch (message.type) {
          case "WasmIsNotLoaded":
            console.error("Wasm module is not loaded");
            break;

          case "FailedToInitializeSynth":
            console.error("Failed to initialize synth module");
            break;

          case "CommonError":
            console.error("Common error:", message.text);
            break;

          default:
            console.error("Unknown error occurred in synth worker");
            break;
        }
      }
    };

    worker.onerror = (error) => {
      console.error("Worker error:", error);
    };

    return () => {
      (async () => {
        await new Promise<void>((resolve) => {
          const onMessage = ({ data: message }: { data: ReceivedMessage }) => {
            if (
              message.type === "SynthIsDeinitialized" ||
              message.type === "FailedToDeinitializeSynth"
            ) {
              worker.removeEventListener("message", onMessage);
              worker.terminate();
              workerRef.current = null;
              resolve();
            }
          };

          worker.addEventListener("message", onMessage);
          worker.postMessage({
            type: "DeinitializeSynth",
          } satisfies SentMessage);
        });
      })();
    };
  }, []);

  return (
    <input
      type="button"
     value={"Generate Audio"}
	  onClick={() => {
		console.log("Generating audio...");
        workerRef.current?.postMessage({
          type: "GenerateAudio",
          leftBuffer,
          rightBuffer,
        } satisfies SentMessage);
      }}
     />
  );
}
