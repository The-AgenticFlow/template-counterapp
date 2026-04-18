const { describe, it } = require('node:test');
const assert = require('node:assert/strict');
const { createCounter } = require('../src/index.js');

describe('createCounter', () => {
  it('should start at 0 by default', () => {
    const counter = createCounter();
    assert.equal(counter.value(), 0);
  });

  it('should accept a custom initial value', () => {
    const counter = createCounter(10);
    assert.equal(counter.value(), 10);
  });

  it('should increment the counter', () => {
    const counter = createCounter();
    assert.equal(counter.increment(), 1);
    assert.equal(counter.value(), 1);
  });

  it('should decrement the counter', () => {
    const counter = createCounter();
    counter.increment();
    assert.equal(counter.decrement(), 0);
    assert.equal(counter.value(), 0);
  });

  it('should reset to the initial value', () => {
    const counter = createCounter(5);
    counter.increment();
    counter.increment();
    assert.equal(counter.value(), 7);
    assert.equal(counter.reset(), 5);
    assert.equal(counter.value(), 5);
  });

  it('should allow negative values', () => {
    const counter = createCounter();
    assert.equal(counter.decrement(), -1);
    assert.equal(counter.value(), -1);
  });
});
