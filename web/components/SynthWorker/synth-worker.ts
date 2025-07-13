import type { ReceivedMessage, SentMessage } from "@/components/types";
import createSynthModule, { type MainModule } from "@/lib/synth/synth";

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
        "An unknown error occurred while creating the synth module."
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
      if (module.initialize()) {
        postCustomMessage({
          type: "SynthIsInitialized",
          error: false,
        });
      } else {
        postCustomMessage({
          type: "FailedToInitializeSynth",
          error: true,
        });
      }
      break;

    default:
      break;
  }
};
