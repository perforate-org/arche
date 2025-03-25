export function matchResult<T, E, R>(
  result: { Ok: T } | { Err: E },
  handlers: { ok: (value: T) => R; err: (error: E) => R },
): R {
  if ("Ok" in result) {
    return handlers.ok(result.Ok);
  }
  return handlers.err(result.Err);
}
