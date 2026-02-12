<!--
date: "2026-02-12"
tags: [typescript]
title: "Zod input & infer as Model & View"
summary: "Using Zod's infer and input with transforms to derive both database and API types from a single schema."
-->

# Zod `input` and `infer` as Model & View

If you use Zod with a database like MongoDB with Mongoose in between, you've probably written separate types for your database documents (e.g. Model) and your API responses (e.g. View). The document has `ObjectId`s and `Date`s; the response has `string`s and `number`s. Keeping these in sync is tedious and error-prone.

Zod's `.transform()` gives you a better way. You define **one** schema, and derive **both** types from it.

## The Key Insight

When a Zod schema has transforms, it has two type signatures:

- **`z.input<typeof schema>`** — the type that goes _into_ `.parse()`
- **`z.infer<typeof schema>`** — the type that comes _out_ of `.parse()`

A schema without transforms has the same type for both. But add a transform, and they diverge, which is exactly what we want.

## Building Transform Primitives

Start by defining small reusable transforms for the types that differ between your database and API layers:

```typescript
import { Types } from "mongoose";
import { z } from "zod";

// MongoDB ObjectId → string
const zObjectId = z.instanceof(Types.ObjectId).transform((id) => id.toString());

// Date → Unix timestamp (ms)
const zDate = z.date().transform((date) => date.getTime());

// Nullable variants
const zObjectIdNullish = z
  .instanceof(Types.ObjectId)
  .nullish()
  .transform((id) => id?.toString() ?? null);

const zDateNullish = z
  .date()
  .nullish()
  .transform((date) => date?.getTime());
```

That's the entire foundation. Four primitives.

## Defining a Model

Use these primitives in your schema, then extract both types:

```typescript
// DTO = data-transfer object
const BookDTO = z.object({
  _id: zObjectId,
  authorId: zObjectId,
  title: z.string(),
  genre: z.enum(["fiction", "non_fiction", "science", "history"]),
  synopsis: z.string().nullish(),
  publishedAt: zDateNullish,
  createdAt: zDate,
  updatedAt: zDate,
});

// Database representation (what Mongoose returns)
type IBook = z.input<typeof BookDTO>;
// {
//   _id: Types.ObjectId;
//   authorId: Types.ObjectId;
//   genre: "fiction" | "non_fiction" | "science" | "history";
//   publishedAt: Date | null | undefined;
//   createdAt: Date;
//   ...
// }

// API representation (what clients receive)
type BookDTO = z.infer<typeof BookDTO>;
// {
//   _id: string;
//   authorId: string;
//   genre: "fiction" | "non_fiction" | "science" | "history";
//   publishedAt: number | null | undefined;
//   createdAt: number;
//   ...
// }
```

Note that the type and value share the same name `BookDTO`. This is valid TypeScript where values and types occupy separate namespaces.

The way we name `FooDTO` model as `IFoo` is to adhere with how Mongoose names their interface in their [typescript guide](https://mongoosejs.com/docs/typescript.html#using-generics).

## Using the Types

The `z.input` type goes to Mongoose for schema and query typing:

```typescript
const bookSchema = new Schema<IBook>({
  authorId: { type: Schema.Types.ObjectId, ref: "authors", required: true },
  title: { type: String, required: true },
  genre: { type: String, enum: GENRES, required: true },
  publishedAt: { type: Date },
  createdAt: { type: Date },
  // ...
});

const BookModel = model("books", bookSchema);
```

The `z.infer` type is your API response. The `.parse()` call is the bridge:

```typescript
// Query returns IBook (z.input) — ObjectIds, Dates
const books = await BookModel.find({ authorId }).lean<IBook[]>();

// .parse() transforms to BookDTO (z.infer) — strings, numbers
return books.map((b) => BookDTO.parse(b));
```

If the object is not retrieved with [lean](https://mongoosejs.com/docs/tutorials/lean.html) you need an additional `toObject` call beforehand:

```typescript
const books = await BookModel.find<IBook[]>({ authorId });

// notice `b.toObject()` here ----------vvvvvvvv
return books.map((b) => BookDTO.parse(b.toObject()));
```

## It Scales to Complex Shapes

This pattern handles discriminated unions, nested objects, and arrays without any extra machinery. Here's a "library" model with a discriminated catalog field:

```typescript
const LibraryDTO = z.object({
  _id: zObjectId,
  name: z.string(),
  catalog: z.discriminatedUnion("kind", [
    z.object({
      kind: z.literal("physical"),
      address: z.string(),
      capacity: z.number(),
    }),
    z.object({
      kind: z.literal("digital"),
      url: z.string(),
      books: z.array(
        z.object({
          bookId: zObjectId,
          addedAt: zDate, // Date in DB, number in API
        }),
      ),
    }),
  ]),
  createdAt: zDate,
});

type ILibrary = z.input<typeof LibraryDTO>;
type LibraryDTO = z.infer<typeof LibraryDTO>;
```

The `z.input` type correctly infers `addedAt: Date` nested inside the `digital` variant, while `z.infer` gives you `addedAt: number`. No manual type wrangling needed.

You can also reference nested types directly for sub-schemas:

```typescript
const catalogSchema = new Schema<ILibrary["catalog"]>({
  // ...
});
```

Building on top of the existing DTO objects, you can use Zod [Omit](https://zod.dev/api#omit) and [Pick](https://zod.dev/api#pick), [Extend](https://zod.dev/api#extend) etc. to re-use existing schemas for special purpose views.

## Why This Works Well

**Single source of truth.** One schema defines the shape, the transforms, the database type, the API type, and (if you use something like `@hono/zod-openapi`) the OpenAPI documentation.

**Type-safe parse boundary.** The `.parse()` call is both a runtime validation step and the compile-time boundary between your DB and API types. If the data doesn't match, you get a Zod error instead of a silent wrong-type-at-runtime bug.

**Zero drift.** Add a field to the schema, and both types update. Rename a field, and the compiler catches every usage on both sides.

The tradeoff is that `z.input` and `z.infer` are less obvious than explicitly written types. But once your team learns the convention, it eliminates an entire class of bugs.
