// (A) reverse a string
// TESTS:
// reverseString("hello")   // => "olleh"
// reverseString("123i s8") // => "8s i321"
// reverseString("")        // => ""
function reverseString(str) {
  return str.split("").reverse().join("");
}

// (B) find the max of 2 numbers
// TESTS:
// max(-122387,124)    // => 124
// max(-125,-2)        // => -2
// max(142,2)          // => 142
function max(a, b) {
  return a > b ? a : b;
}

// (C) sort a numeric array by values (smallest to highes)
// TESTS:
// sort([8,12,53,1,2,-6,2])    // => [-6,1,2,2,8,12]
// sort([21,5,0,5,22,504])     // => [0,5,5,21,22,504]
function sort(arr) {
  return arr.sort();
}

// (D) remove a given character from a string
// TESTS:
// removeChar("Hello World", "H")            // => "ello World"
// removeChar("Hi, how are you doing?", "i") // => "H, how are you dong?"
function removeChar(str, char) {
  return str.split(char).join();
}

// (E) sum all numbers of an array
// TESTS:
// sum([1,2,3,4,5])      // => 15
// sum([829,-12,758,2])  // => 1577
function sum(arr) {
  return arr.reduce((a,b) => a + b);
}

// (F) chunk an array into an array of arrays
// => what data structur would you use for that in Rust
// => can you even create an Array of Arrays in Rust?
// TESTS:
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 5)
//        => [[1,2,3,4,5],[1,2,3,4,5],[1,2,3]]
// chunkArray([1,2,3,4,5,1,2,3,4,5,1,2,3], 3)
//        => [[1,2,3],[4,5,1],[2,3,4],[5,1,2],[3]]
function chunkArray(arr, size) {
  const chunkedArr = [];
  let index = 0;
  while (index < array.length) {
    chunkedArr.push(array.slice(index, size + index));
    index += size;
  }
  return chunkedArr;
}

// (G) count occurencies all distinct values of an array and return as dictionary
// => what datastructure to use in RUST for such a task?
// TESTS:
// createDict(["a","b","a","a","c","c","a"])
//         => {"a":4, "b":1, "c":2}
function createDict(arr) {
  const dict = {};
  arr.forEach(element => {
    dict[element] = dict[element] ? dict[element] + 1 : 1;
  });
  return dict;
}

// (H) multiply each element of the array with x
// TESTS:
// multiplyByX([1,2,3,4],10); => [10,20,30,40]
// multiplyByX([1,12,33,9],1.5); => [1.5,18,49.5,13.5]
function multiplyByX(arr, x) {
  return arr.map(val => val * x);
}

// (I) flatten an array of an array
// how implement Array of Array in Rust?
// TESTS:
// flatArr([1,2,3],[4,5,6]) // => [1,2,3,4,5,6]
function flatArr(arr) {
	return arr.flat();
}
