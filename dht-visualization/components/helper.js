const hash = input => {
  return crypto.createHash("sha256").update(input).digest("hex");
}
const hex2int = hexStr => {
  return parseInt(hexStr, 16);
}
const int2hex = num => {
  return num.toString(16);
}
class Range {
  constructor(min, max) {
    this.min = min;
    this.max = max;
  }
}
const rangesHex = n => {
  return rangesInt(n).map(range => {
    range.min = int2hex(range.min).padStart(64, "0");
    range.max = int2hex(range.max).padStart(64, "0");
    return range;
  });
}
const rangesInt = n => {
  const range = MAX_HASH_INT - MIN_HASH_INT;
  const interval = range / n;
  const rangesInt = new Array(n).fill(1).map((_, indx) => MIN_HASH_INT + indx*interval);
  const rangesIntFull = rangesInt.concat(MAX_HASH_INT);
  return rangesInt.map((val, i) => new Range(val, rangesIntFull[i+1]));
}
const getInterval = (n) => {
  const range = MAX_HASH_INT - MIN_HASH_INT;
  return range / n;
}
const getNodeIndx = (hash, n) => {
  const interval = getInterval(n); 
  return Math.floor(hex2int(hash) / interval);
}
// CONSTANTS
const MIN_HASH = "0000000000000000000000000000000000000000000000000000000000000000";
const MAX_HASH = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const MIN_HASH_INT = hex2int(MIN_HASH);
const MAX_HASH_INT = hex2int(MAX_HASH);

// EXPORTS
exports.hash = hash;
exports.hex2int = hex2int;
exports.int2hex = int2hex;
exports.rangesHex = rangesHex;
exports.rangesInt = rangesInt;
exports.getNodeIndx = getNodeIndx;
exports.MIN_HASH = MIN_HASH;
exports.MAX_HASH = MAX_HASH;
exports.MIN_HASH_INT = MIN_HASH_INT;
exports.MAX_HASH_INT = MAX_HASH_INT;
