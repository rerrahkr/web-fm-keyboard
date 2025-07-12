"use client";

import type React from "react";
import { useEffect, useRef } from "react";

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

    worker.onmessage = (event: MessageEvent) => {
      console.log("Message from worker:", event.data);
    };

    worker.postMessage("Hello from main thread");

    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, []);
  return null;
}
