const Counter = require('../src/counter.js');

describe('Counter', () => {
  let counter;

  beforeEach(() => {
    counter = new Counter();
  });

  test('initializes with count of 0', () => {
    expect(counter.getCount()).toBe(0);
  });

  test('increment increases count by 1', () => {
    counter.increment();
    expect(counter.getCount()).toBe(1);
  });

  test('decrement decreases count by 1', () => {
    counter.decrement();
    expect(counter.getCount()).toBe(-1);
  });

  test('reset returns count to 0', () => {
    counter.increment();
    counter.increment();
    counter.reset();
    expect(counter.getCount()).toBe(0);
  });

  test('multiple operations work correctly', () => {
    counter.increment();
    counter.increment();
    counter.increment();
    expect(counter.getCount()).toBe(3);
    
    counter.decrement();
    expect(counter.getCount()).toBe(2);
    
    counter.decrement();
    counter.decrement();
    counter.decrement();
    expect(counter.getCount()).toBe(-1);
    
    counter.reset();
    expect(counter.getCount()).toBe(0);
  });

  test('can increment multiple times', () => {
    for (let i = 0; i < 5; i++) {
      counter.increment();
    }
    expect(counter.getCount()).toBe(5);
  });

  test('can decrement multiple times', () => {
    for (let i = 0; i < 3; i++) {
      counter.decrement();
    }
    expect(counter.getCount()).toBe(-3);
  });

  test('reset works even when count is already 0', () => {
    counter.reset();
    expect(counter.getCount()).toBe(0);
  });
});