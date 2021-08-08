function reverseString(str) {
  return str.split("").reverse().join("");
}

function max(a, b) {
  return a > b ? a : b;
}

function sort(arr) {
  return arr.sort();
}

function removeChar(str, char) {
  return str.split(char).join();
}

function sum(arr) {
  return arr.reduce((a,b) => a + b);
}

function chunkArray(arr, size) {
  const chunkedArr = [];
  let index = 0;
  while (index < array.length) {
    chunkedArr.push(array.slice(index, size + index));
    index += size;
  }
  return chunkedArr;
}

function createDict(arr) {
  const dict = {};
  arr.forEach(element => {
    dict[element] = dict[element] ? dict[element] + 1 : 1;
  });
  return dict;
}

function multiplyByX(arr, x) {
  return arr.map(val => val * x);
}

function findValue(arr, x) {
  return arr.find(val => val === x);
}
