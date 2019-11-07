# rustlang first order async/await

Where can I/O take us?

## roadmap and goals

1. get up to speed on current ecosystem

2. convert 0.1 future into 0.3 future

3. convert 0.3 future back into 0.1 future

4. combine all of this into actix-web service/endpoint

5. convert 0.3 stream into Node.js compatible stream

   - [Promise API · Issue #73 · neon-bindings/neon](https://github.com/neon-bindings/neon/issues/73)

6. build a Rust -> NodeJS webserver with NGINX potentially on top

## links and resources

**general**:
- [100 days with Rust, or, a series of brick walls — Brandur Leach](https://brandur.org/fragments/rust-brick-walls)

**what can we do with futures?**:
- [Yoshua Wuyts — Blog](https://blog.yoshuawuyts.com/rust-streams/)

**futures compatibility**:
- [rust-lang-nursery/futures-rs: Zero-cost asynchronous programming in Rust](https://github.com/rust-lang-nursery/futures-rs)
- [std::future - Rust](https://doc.rust-lang.org/std/future/index.html)
- [Compatibility Layer | Futures-rs](https://rust-lang-nursery.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html)
- [Async, futures, and tokio - Rust Crash Course lesson 7](https://www.snoyman.com/blog/2018/12/rust-crash-course-07-async-futures-tokio)

**traits and impls**:
- [Little Orphan Impls](http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/)
- [Derive for remote crate · Serde](https://serde.rs/remote-derive.html)
- [Deriving external traits on external structs - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/deriving-external-traits-on-external-structs/18198/2)
- [Crates should allow private impl of external traits for external structs · Issue #13721 · rust-lang/rust](https://github.com/rust-lang/rust/issues/13721)
- [Deref coercions](https://doc.rust-lang.org/book/deref-coercions.html)
- [vector - How can I wrap another type and add fields and methods to it? - Stack Overflow](https://stackoverflow.com/questions/31497584/how-can-i-wrap-another-type-and-add-fields-and-methods-to-it)
- [Clone - Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/clone.html)
- [Can not impl trait from another crate for Vec<T> with T from this crate · Issue #24745 · rust-lang/rust](https://github.com/rust-lang/rust/issues/24745)
- [Karol Kuczmarski's Blog – Extension traits in Rust](http://xion.io/post/code/rust-extension-traits.html)

**actix and actix-web**:
- [error.rs.html -- source](https://docs.rs/actix-http/0.2.3/src/actix_http/error.rs.html#57-76)
- [actix/examples: Actix web examples](https://github.com/actix/examples)
- [actix/actix: Actor framework for Rust](https://github.com/actix/actix)
- [Examples for integration with tokio · Issue #205 · actix/actix-web](https://github.com/actix/actix-web/issues/205)
- [how to use libs that uses tokio within actix? : actix](https://www.reddit.com/r/actix/comments/8rpbht/how_to_use_libs_that_uses_tokio_within_actix/)

**tokio**:
- [Tokio alpha release with async & await · Tokio](https://tokio.rs/blog/2019-08-alphas/)
- [Making the Tokio scheduler 10x faster · Tokio](https://tokio.rs/blog/2019-10-scheduler/)
- [Release v0.2.0-alpha.5 · tokio-rs/tokio](https://github.com/tokio-rs/tokio/releases/tag/tokio-0.2.0-alpha.5)
- [Getting tokio to match actix-web performance - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/getting-tokio-to-match-actix-web-performance/18659)

