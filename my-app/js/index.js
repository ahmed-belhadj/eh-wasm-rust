import("../crate/pkg").then(module => {
  module.run();
});

// Access WebAssembly Memory Directly from JavaScript
import("./app").then(() => {
  console.log("loaded...");
});

import("ahmedbelhadj-wasm-lib").then(module => {
  module.greet("World");
});
