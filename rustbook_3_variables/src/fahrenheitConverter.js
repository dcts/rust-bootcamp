function convertToFahrenheit(celsius) {
  let fahrenheit = celsius * (9/5) - 32;
  return fahrenheit;
}

console.log(convertToFahrenheit(-30)); // => -22
console.log(convertToFahrenheit(-10)); // => 14
console.log(convertToFahrenheit(0));   // => 32
console.log(convertToFahrenheit(20));  // => 68
console.log(convertToFahrenheit(30));  // => 86
