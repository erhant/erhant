<!--
date: "2026-03-29"
tags: [typescript, puzzles]
title: "Type-Narrowing `expect`"
summary: "How to be more type-safe while writing your tests and assertions."
-->

# Type-Narrowing `expect`

When writing tests in TypeScript, we often want to make assertions about the types of our values. For example, we might want to assert that a function returns a string, or that an object has a certain shape. However, if we use the standard `expect` (e.g. in [Vitest](https://vitest.dev/api/expect.html), [Bun](https://bun.com/reference/bun/test/expect), [Jest](https://jestjs.io/docs/expect)) function from testing libraries like Jest, we might not get the type-narrowing we desire.

> [Type-narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html) is when the compiler knows that if some condition holds, the type must be a subset of the existing type. The simplest example is a boolean:
>
> ```ts
> const b: boolean = true;
> if (b) {
>   // b has type `true` here!
> }
> ```

Even if the object that you are testing is well-typed, `expect` calls will not type-narrow it they way you would want them to. Let's go over some examples.

## Examples

### Example: Non-Nullable

For example, null-checks to not narrow the type to it's [`NonNullable`](https://www.typescriptlang.org/docs/handbook/utility-types.html#nonnullabletype) type:

```ts
const doc: Document | null = fetchDocument();
expect(doc).not.toBeNull();
// doc is still `Document | null` here, not `Document`
```

### Example: Result Types & Status Codes

Some backend frameworks provide wrapper clients with type-safe functions, greatly improving the developer experience for testing and frontend development over an existing backend.

- Elysia has Eden-Treaty
- Hono has RPC client and testClient

These clients often return a result that can do type-narrowing with respect to status code, e.g. if for some repsonse `res` you get `res.status == 200` then the type of `res.json()` is narrowed; this applies to all status codes that the backend defines types for.

Even so, the following code does **not** have type-narrowing:

```ts
const res: ApiResult | null = someApiCall();
expect(res.status).toBe(200);

const body = await res.json();
// body has type `any` or `unknown`
```

## The Solution

Below is a copy-pasteable utility that can do type-narrowing to your `expect` calls for the two common cases of `toBe` and null-checks. It uses `expect` from Bun, but should work with others as well.

````ts
/** Expects a value to be defined.
 *
 * Enforces type narrowing for the value, so that it can be used safely after this assertion.
 *
 * Equivalent to:
 *
 * ```ts
 * expect(val).toBeDefined();
 * if (val === undefined) return; // or throw
 * ```
 *
 * @param val The value to check.
 */
export function expectToBeDefined<T>(val?: T): asserts val is NonNullable<T> {
  expect(val).toBeDefined();
  expect(val).not.toBeNull();
}

/**
 * Expects a value to be equal to a specific value.
 *
 * Enforces type narrowing for the value, so that it can be used safely after this assertion.
 *
 * Equivalent to:
 *
 * ```ts
 * expect(val).toBe(compareVal);
 * if (val !== compareVal) return; // or throw
 * ```
 *
 * @param val The value to check.
 * @param compareVal The value to compare against.
 */
export function expectToBe<T>(val: unknown, compareVal: T): asserts val is T {
  expect(val).toBe(compareVal);
}
````

With these, the examples above can be written as:

```ts
const doc: Document | null = fetchDocument();
expectToBeDefined(doc);
// doc has type `Document`
```

```ts
const res: ApiResult | null = someApiCall();
expectToBe(res.status, 200);

const body = await res.json();
// body has known type w.r.t status 200
```

This saves you a line for each type-narrow that would otherwise require a separate `if` check with `return` / `throw`!
