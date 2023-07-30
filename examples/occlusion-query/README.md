# occlusion-query

This example shows how to use occlusion queries to query the number of fragment samples that pass per-framgent tests like scissor, sample mask, alpha to coverage, stencil and depth tests.

It does so by drawing overlapping rectangles at different depth and then querying how many of their samples were visible at the time they're rendered.


## To Run

```
cargo run --bin occlusion-query
```

## Screenshots

TODO: Screenshot