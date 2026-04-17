const { increment, decrement, getValue, reset } = require('./index');

describe('Counter App', () => {
  beforeEach(() => {
    reset();
  });

  test('initial value should be 0', () => {
    expect(getValue()).toBe(0);
  });

  test('increment increases value by 1', () => {
    expect(increment()).toBe(1);
    expect(getValue()).toBe(1);
  });

  test('increment multiple times', () => {
    increment();
    increment();
    increment();
    expect(getValue()).toBe(3);
  });

  test('decrement decreases value by 1', () => {
    expect(decrement()).toBe(-1);
    expect(getValue()).toBe(-1);
  });

  test('decrement multiple times', () => {
    decrement();
    decrement();
    expect(getValue()).toBe(-2);
  });

  test('increment and decrement combined', () => {
    increment();
    increment();
    decrement();
    expect(getValue()).toBe(1);
  });

  test('reset returns value to 0', () => {
    increment();
    increment();
    expect(reset()).toBe(0);
    expect(getValue()).toBe(0);
  });
});