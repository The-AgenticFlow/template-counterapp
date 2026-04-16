class Counter {
  constructor() {
    this.count = 0;
  }

  increment() {
    this.count += 1;
  }

  decrement() {
    this.count -= 1;
  }

  getCount() {
    return this.count;
  }

  reset() {
    this.count = 0;
  }
}

module.exports = Counter;