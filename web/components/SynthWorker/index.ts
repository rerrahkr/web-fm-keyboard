"use client";

import type React from "react";
import { useEffect, useRef } from "react";
import type { ReceivedMessage, SentMessage } from "@/components/types";

/**
 * Dummy component to control the web worker for synth module.
 * @returns `null`.
 */
export function SynthWorker(): React.JSX.Element | null {
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
            worker.postMessage({ type: "InitializeSynth" } satisfies SentMessage);
            break;

          case "SynthIsInitialized":
            console.log("Synth module is initialized");
            break;

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
          worker.postMessage({ type: "DeinitializeSynth" } satisfies SentMessage);
        });
      })();
    };
  }, []);

  return null;
}
