export function maxLogLines(value: number): number {
  return value <= 5
    ? value * 10 + 50
    : value <= 13
      ? value * 50 - 150
      : value <= 28
        ? value * 100 - 800
        : Infinity
}
