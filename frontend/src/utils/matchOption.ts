type Optional<T> = [T] | [];

export function fromOption<T>(opt: Optional<T>): T | null {
  return opt.length > 0 ? (opt[0] as T) : null;
}

export function toOption<T>(value: T | null | undefined): Optional<T> {
  return value != null ? [value] : [];
}

export function mapOption<T, U>(
  opt: Optional<T>,
  fn: (value: T) => U,
): Optional<U> {
  return opt.length > 0 ? [fn(opt[0] as T)] : [];
}

export function andThen<T, U>(
  opt: Optional<T>,
  fn: (value: T) => Optional<U>,
): Optional<U> {
  return opt.length > 0 ? fn(opt[0] as T) : [];
}
