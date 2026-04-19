/**
 * Minimal counter module.
 * Provides a Counter class with increment, decrement, and reset operations.
 */

class Counter {
  constructor(initialValue = 0) {
    this._value = initialValue;
  }

  get value() {
    return this._value;
  }

  increment() {
    this._value += 1;
    return this._value;
  }

  decrement() {
    this._value -= 1;
    return this._value;
  }

  reset() {
    this._value = 0;
    return this._value;
  }
}

module.exports = { Counter };
