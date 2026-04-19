const { describe, it } = require('node:test');
const assert = require('node:assert/strict');
const { createCounter } = require('../src/index.js');

describe('createCounter', () => {
  it('should start with the default value of 0', () => {
    const counter = createCounter();
    assert.equal(counter.value(), 0);
  });

  it('should start with a custom initial value', () => {
    const counter = createCounter(10);
    assert.equal(counter.value(), 10);
  });

  it('should increment the counter', () => {
    const counter = createCounter();
    assert.equal(counter.increment(), 1);
    assert.equal(counter.value(), 1);
  });

  it('should decrement the counter', () => {
    const counter = createCounter(5);
    assert.equal(counter.decrement(), 4);
    assert.equal(counter.value(), 4);
  });

  it('should reset the counter to its initial value', () => {
    const counter = createCounter(3);
    counter.increment();
    counter.increment();
    assert.equal(counter.value(), 5);
    assert.equal(counter.reset(), 3);
    assert.equal(counter.value(), 3);
  });

  it('should allow negative values', () => {
    const counter = createCounter(0);
    counter.decrement();
    assert.equal(counter.value(), -1);
  });
});
