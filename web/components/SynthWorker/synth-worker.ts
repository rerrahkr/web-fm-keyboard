import createSynthModule, { type MainModule } from "@/lib/synth/synth";

let module: MainModule;

createSynthModule({
  locateFile: (fileName: string) => `/wasm/${fileName}`,
})
  .then((mod) => {
    module = mod;
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

self.onmessage = (event: MessageEvent) => {
  // self.postMessage(`SYN ${event.data}`);
};
