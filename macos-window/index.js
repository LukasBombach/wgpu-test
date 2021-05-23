const { hello } = require(".");

const helloValue = hello();

console.log(helloValue);

setTimeout(() => {
  console.log("timeout done");
}, 6000);
