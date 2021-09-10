# Tips

* Avoid `unwrap()` in your task functions -- Goose generates a lot of load, and this tends to trigger errors. Embrace Rust's warnings and properly handle all possible errors, this will save you time debugging later.
* When running your load test, use the cargo `--release` flag to generate optimized code. This can generate considerably more load test traffic. Learn more about this and other optimizations in ["The golden Goose egg, a compile-time adventure"](https://www.tag1consulting.com/blog/golden-goose-egg-compile-time-adventure).
