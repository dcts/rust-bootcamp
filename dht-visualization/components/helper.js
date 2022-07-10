const hash = input => {
  return crypto.createHash("sha256").update(input).digest("hex");
}

const hex2int = hexStr => {
  return parseInt(hexStr, 16);
}

const int2hex = num => {
  return num.toString(16);
}

const rangesHex = n => {
  const range = MAX_HASH_INT - MIN_HASH_INT;
  const interval = range / n;
  const ranges = new Array(n).fill(1).map((_, indx) => MAX_HASH_INT + indx*interval);
  return ranges.map(d => int2hex(d).padStart(64, "0"));
}

const rangesInt = n => {
  return rangesHex(n).map(hex => hex2int(hex));
}

// CONSTANTS
const MIN_HASH = "0000000000000000000000000000000000000000000000000000000000000000";
const MAX_HASH = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const MIN_HASH_INT = hex2int(MIN_HASH);
const MAX_HASH_INT = hex2int(MAX_HASH);

// const rangeDict = n => {
//   const ranges = computeRanges(n);
//   const precision = Math.ceil(n/16);
//   const dictChar = ranges.map(d => d.slice(0, precision).padEnd(precision, "0"));
//   const dict = {};
//   dictChar.forEach((ch, indx) => dict[ch] = indx);
//   return dict;
// }

const getNodeIndx = (hash, n) => {
  return hex2int(hash) / 2;
}

exports.hash = hash;
exports.hex2int = hex2int;
exports.int2hex = int2hex;
exports.rangesHex = rangesHex;
exports.rangesInt = rangesInt;
exports.MIN_HASH = MIN_HASH;
exports.MAX_HASH = MAX_HASH;
exports.MIN_HASH_INT = MIN_HASH_INT;
exports.MAX_HASH_INT = MAX_HASH_INT;

/*
0 START
1 
2 
3 
4 
5 
6 END

1.543 


*/