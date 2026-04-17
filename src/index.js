// Counter App - Simple counter functionality

let counter = 0;

function increment() {
  counter += 1;
  return counter;
}

function decrement() {
  counter -= 1;
  return counter;
}

function getValue() {
  return counter;
}

function reset() {
  counter = 0;
  return counter;
}

module.exports = {
  increment,
  decrement,
  getValue,
  reset
};