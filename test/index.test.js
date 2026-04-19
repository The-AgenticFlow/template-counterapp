const { describe, it } = require('node:test');
const assert = require('node:assert/strict');
const { Counter } = require('../src/index.js');

describe('Counter', () => {
  it('starts at 0 by default', () => {
    const counter = new Counter();
    assert.equal(counter.value, 0);
  });

  it('starts at a custom initial value', () => {
    const counter = new Counter(5);
    assert.equal(counter.value, 5);
  });

  it('increments the value', () => {
    const counter = new Counter();
    assert.equal(counter.increment(), 1);
    assert.equal(counter.value, 1);
  });

  it('decrements the value', () => {
    const counter = new Counter();
    assert.equal(counter.decrement(), -1);
    assert.equal(counter.value, -1);
  });

  it('resets to 0', () => {
    const counter = new Counter(10);
    assert.equal(counter.reset(), 0);
    assert.equal(counter.value, 0);
  });

  it('supports chained operations', () => {
    const counter = new Counter();
    counter.increment();
    counter.increment();
    counter.increment();
    counter.decrement();
    assert.equal(counter.value, 2);
  });
});
