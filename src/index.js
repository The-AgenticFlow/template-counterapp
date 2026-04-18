/**
 * Minimal counter module.
 * Creates a counter object with increment, decrement, reset, and value methods.
 *
 * @param {number} [initial=0] - Starting value for the counter.
 * @returns {{ increment: Function, decrement: Function, reset: Function, value: Function }}
 */
function createCounter(initial = 0) {
  let count = initial;

  return {
    increment() {
      count += 1;
      return count;
    },
    decrement() {
      count -= 1;
      return count;
    },
    reset() {
      count = initial;
      return count;
    },
    value() {
      return count;
    },
  };
}

module.exports = { createCounter };
