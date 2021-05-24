const { open_window } = require(".");

console.log("opening window");

open_window();

console.log("window should be open and I should show up");

console.log("starting setTimeout for 5 seconds");

setTimeout(() => {
  console.log("Timeout done");
}, 5000);
