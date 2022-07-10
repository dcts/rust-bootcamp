const Dht = require("./components/dht");
const Node = require("./components/helper");
const Post = require("./components/post");
const { 
  hash, 
  hex2int,
  int2hex,
  computeRanges,
} = require("./components/helper");

console.log("\n\n");
console.log(fs.readFileSync('init-dev-env-message.txt', 'utf8'));
console.log("\n\n");
