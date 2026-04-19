/**
 * Minimal counter module.
 * Provides a simple counter with increment, decrement, and reset operations.
 */

function createCounter(initialValue = 0) {
  let value = initialValue;

  return {
    increment() {
      value += 1;
      return value;
    },
    decrement() {
      value -= 1;
      return value;
    },
    reset() {
      value = initialValue;
      return value;
    },
    value() {
      return value;
    }
  };
}

module.exports = { createCounter };
