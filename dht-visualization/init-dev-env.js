const Dht = require("./components/dht");
const Node = require("./components/helper");
const Post = require("./components/post");
const { 
  MIN_HASH,
  MAX_HASH,
  MIN_HASH_INT,
  MAX_HASH_INT,
  hash, 
  hex2int,
  int2hex,
  rangesHex,
  rangesInt,
} = require("./components/helper");

console.log("\n\n");
console.log(fs.readFileSync('init-dev-env-message.txt', 'utf8'));
console.log("\n\n");

const n = 16;
const min = hex2int("0000000000000000000000000000000000000000000000000000000000000000");
const max = hex2int("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
const range = max - min;
const interval = range / n;
const dictInt = new Array(n).fill(1).map((d, indx) => min + indx*interval);
const dictHex = dictInt.map(d => int2hex(d).padStart(64, "0"));
const precision = Math.ceil(n/16);


// n = 3;
// min = 0;
// max = 6;
// range = max - min;
// interval = range / n;
