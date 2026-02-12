<!--
date: "2026-02-11"
tags: [typescript, hono, zod]
title: "Taming hono/zod-openapi Type Performance"
summary: "Using explicit generic type parameters to speed up tsc with @hono/zod-openapi routes."
wip: true
-->

# Taming `hono/zod-openapi` Type Performance with Explicit Generics

If you use `@hono/zod-openapi` in a non-trivial project, you've probably noticed your type-checking getting slower as you add routes. In our case — a backend with 30+ route files — `tsc` was spending the majority of its time resolving `.openapi()` calls. Here's what's happening and how we fixed it.

## The Problem

The `.openapi()` method on `OpenAPIHono` has this signature (simplified):

```typescript
openapi<
  R extends RouteConfig,
  I extends Input = InputTypeParam<R> & InputTypeQuery<R> & InputTypeHeader<R>
                    & InputTypeCookie<R> & InputTypeForm<R> & InputTypeJson<R>,
  P extends string = ConvertPathType<R["path"]>
>(route: R, handler: Handler<..., P, I, RouteConfigToTypedResponse<R>>): ...
```

When you write `.openapi(createRoute({ ... }), handler)`, TypeScript infers `R` from the argument, then computes the defaults for `I` and `P`. This triggers a cascade of conditional and mapped types:

- **`InputTypeParam<R>`**, **`InputTypeQuery<R>`**, **`InputTypeJson<R>`** — conditional types that dig into `R["request"]` to extract Zod input/output types from each parameter location
- **`RouteConfigToTypedResponse<R>`** — a mapped type that iterates over every status code in your `responses` object, extracting content types and building a union of `TypedResponse` types
- **`ConvertPathType<R["path"]>`** — a recursive template literal type that converts `{param}` to `:param` in your path string
- **`RouteMiddlewareParams<R>`** — extracts environment types from middleware arrays

Each `.openapi()` call forces TypeScript to fully instantiate all of these. Multiply that by 30+ routes with Zod schemas containing transforms, discriminated unions, and nested objects, and your type checker slows to a crawl.

## The Fix: Explicit Generic Type Parameters

Instead of letting TypeScript infer `R`, you provide it directly:

```typescript
// Before: TypeScript infers R, then computes I, P, and Response types
app.openapi(
  createRoute({
    method: "get",
    path: "/products/{productId}/reviews",
    middleware: [auth, rateLimit({ max: 100 })] as const,
    request: { params: productIdParams },
    responses: {
      200: {
        description: "List of reviews",
        content: { "application/json": { schema: reviewListResponse } },
      },
      401: { description: "Unauthorized" },
    },
  }),
  async (c) => {
    /* handler */
  },
);
```

```typescript
// After: TypeScript skips inference, uses the explicit type directly
app.openapi<{
  tags: string[];
  method: "get";
  path: "/products/{productId}/reviews";
  middleware: [
    MiddlewareHandler<{ Variables: { user: JwtPayload } }>,
    MiddlewareHandler,
  ];
  security: { bearerAuth: never[] }[];
  request: { params: typeof productIdParams };
  responses: {
    200: {
      description: string;
      content: { "application/json": { schema: typeof reviewListResponse } };
    };
    401: { description: string };
  };
}>(
  createRoute({
    /* identical structure */
  }),
  async (c) => {
    /* handler */
  },
);
```

When you provide `R` explicitly, TypeScript doesn't need to infer it from `createRoute()`'s return type. The conditional types still get evaluated, but they resolve against a simple, pre-flattened object type instead of a deeply nested inferred one. This makes a significant difference in practice.

## Key Rules for the Explicit Type

There are a few things that matter for this to actually help:

**Use literal types for `method` and `path`.** Write `"get"`, not `string`. Write `"/products/{productId}/reviews"`, not `string`. This prevents TypeScript from having to narrow these later.

**Use `typeof` for Zod schemas.** Instead of inlining or re-specifying schema types, reference the schema variable: `typeof productIdParams`, `typeof reviewListResponse`. This reuses the already-computed type rather than creating a new one.

**Spell out middleware `Variables` types.** This is what gives your handler typed `c.get("user")` access. Without it, TypeScript has to infer these from the middleware chain.

**Use `string` for descriptions and tags.** These are runtime-only values that don't affect handler types, so there's no benefit to making them literal.

## Measuring the Impact

You can't improve what you can't measure. TypeScript has a built-in tracing facility that generates per-file and per-type timing data. We keep two scripts in our `package.json`:

```json
{
  "check": "bunx tsc --noEmit --skipLibCheck --strict",
  "check:trace": "rm -f tsconfig.tsbuildinfo && bunx tsc --noEmit --skipLibCheck --strict --generateTrace ./node_modules/.tsctrace",
  "check:analyze": "bunx @typescript/analyze-trace --force-millis 250 ./node_modules/.tsctrace"
}
```

The workflow:

1. **`bun check:trace`** — runs a full type check and writes trace data to `./node_modules/.tsctrace`
2. **`bun check:analyze`** — uses `@typescript/analyze-trace` to parse the trace and surface the slowest types, showing anything over 250ms

The analyze output tells you exactly which type instantiations are expensive. Before the explicit generic pattern, you'll see `RouteConfigToTypedResponse` and `InputTypeJson` dominating the list. After, they drop off or resolve much faster.

You can also load the trace files into Chrome's `chrome://tracing` for a visual flame chart of your type checker's work, which is useful for spotting bottlenecks in non-obvious places like re-exported barrel files or deeply nested Zod transforms.

## Is It Worth the Verbosity?

Yes. The generic type parameter is verbose — it roughly mirrors the route config object. But in a codebase with 30+ routes, each chaining multiple `.openapi()` calls, the cumulative inference cost is substantial. The explicit generic trades a one-time copy-paste for consistent, predictable type-check times as the project grows.

The structure is also highly regular. Every route follows the same pattern: literal `method`, literal `path`, `typeof` for schemas, explicit `MiddlewareHandler<{ Variables: ... }>` for middleware. Once you've written one, the rest are mechanical.

If you're using `@hono/zod-openapi` and your `tsc` times are climbing, try the explicit generic on your most complex route and measure the difference with `--generateTrace`. You might be surprised.
