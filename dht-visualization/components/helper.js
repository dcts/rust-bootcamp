
const hash = input => {
  return crypto.createHash("sha256").update(input).digest("hex");
}

const hex2int = hexStr => {
  return parseInt(hexStr, 16);
}

const int2hex = num => {
  return num.toString(16);
}

const computeRanges = n => {
  const min = hex2int("0000000000000000000000000000000000000000000000000000000000000000");
  const max = hex2int("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
  const range = max - min;
  const interval = range / n;
  const dictInt = new Array(n).fill(1).map((d, indx) => min + indx*interval);
  const dictHex = dictInt.map(d => int2hex(d));
  const precision = Math.ceil(n/16);
  return dictHex.map(d => d.slice(0, precision).padEnd(precision, "0"));
}

exports.hash = hash;
exports.hex2int = hex2int;
exports.int2hex = int2hex;
exports.computeRanges = computeRanges;
