# Athena

Athena is a math crate for game engines, rendering, and physics simulation, built
around **Projective Geometric Algebra (PGA)** instead of the usual pile of
vectors, quaternions, and matrices bolted together.

In most engines, translation, rotation, reflection, and their compositions
each get their own type, their own edge cases, and their own conversion
functions between them. PGA replaces all of it with one algebra: points,
lines, and planes are all first-class objects, and a single object — a
**motor** — represents any rigid transformation (rotation + translation,
screw motions included) as one thing you can compose, interpolate, and
apply uniformly. No gimbal lock, no separate quaternion/translation pair to
keep in sync, no ad-hoc "convert to matrix to combine, then decompose back."

## Why PGA

- **One representation for rigid motion.** A `Motor` *is* a
  rotation-and-translation. Composing two motors composes the motions;
  there's no separate rotation/translation bookkeeping.
- **Points, lines, and planes are peers.** Meeting two lines gives a point;
  joining two points gives a line — the same algebra runs in both
  directions (`meet` / `join`), instead of hand-rolled geometry per shape
  pair.
- **Reflections are the primitive.** Rotations and translations are both
  built from reflections, matching how the algebra actually composes them,
  rather than treating reflection as a special case bolted onto rotation.
- **No hidden singularities.** No gimbal lock, no quaternion/matrix drift to
  re-normalize away.

## What's in the crate

- **2D and 3D PGA**: `Point2`/`Point3`, `Line2`/`Line3`, `Plane3`, and
  `Motor2`/`Motor3`, plus the underlying graded elements (scalars, vectors,
  bivectors, trivectors, pseudoscalars) with the geometric product, wedge
  (`meet`), and regressive (`join`) operations that drive them.
- **Generic linear algebra**: an `N`-dimensional `Vector<T, N>` with
  swizzling (`v.xy()`, `v.xyz()`, ...) and a column-major `Matrix<T, N, M>`,
  independent of the PGA layer, for the parts of an engine that still want
  plain vectors and matrices.
- **Numeric-backend agnostic**: everything is generic over a `Num` trait
  implemented for `f32` and `f64` — pick your precision per call site.
- **`no_std`-friendly**: the `std` feature is on by default and can be
  disabled.
- **Optional `serde` support** for the vector and matrix types.
- **Safe by default**: `#![deny(unsafe_code)]` at the crate root — the few
  places that need `unsafe` (array layout reinterpretation for zero-cost
  vector/matrix construction) opt back in locally and narrowly, everything
  else is safe Rust.

## Example

```rust
use athena::{Line2, Motor2, Point2};

// Two lines, given as ax + by + c = 0.
let a = Line2::from_abc(1.0, 0.0, 0.0);   // the y axis
let b = Line2::from_abc(0.0, 1.0, -1.0);  // y = 1

// The motor that takes `a` to `b` — a rotation about their intersection.
let motor = Motor2::line_line(a, b);

let p = Point2::at(0.0, 0.0);
let moved = motor.move_point(p);

assert_eq!(moved.coords(), (0.0, 2.0));
```

## Status

Athena is early (`0.1.0`) and its API is still settling as the PGA layer
gets exercised against real engine use cases. If you're evaluating it for a
project, expect some rough edges and pin a version.

## License

Licensed under either of MIT or Apache-2.0 at your option.
