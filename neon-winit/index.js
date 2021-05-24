const { open_window } = require(".");

console.log("opening window");

open_window();

console.log("window should be open and I should show up");

console.log("starting setTimeout for 60 seconds");

setTimeout(() => {
  console.log("Timeout done");
}, 60000);
