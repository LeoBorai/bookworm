import init, { worker_entry_point } from "./web-worker/web_worker.js";

console.log("Worker thread: Loading WASM...");

init()
  .then(() => {
    console.log("Worker thread: WASM loaded, starting...");
    worker_entry_point();
  })
  .catch((err) => {
    console.error("Worker thread: Failed to load WASM:", err);
  });
