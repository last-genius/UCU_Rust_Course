A WIP repository for Linux_Club@UCU Rust course

(The course itself is expected to start in March 2021)

I am adding notes and code snippets as I go on creating and testing them,
so they might contain errors, and will definitely change

## Contents
1. [Schedule](#Schedule)
1. [Resources](#Resources)
	1. [Cool Rust Apps](#Cool-Rust-Apps)
	1. [Books](#Books)
	1. [Practice](#Practice)
	1. [Other resources](#Other-resources)
1. [(TODO!!!!) Vim integration guide]()

##### TODO:
* Finish up the second lecture and clean up first two classes' homeworks
* Start working on the third and fourth lectures
* Improve formatting in this doc a bit (maybe just split it between several files?)
* Add some basic commentary to all resources, add more apps
* Start thinking of project topics and organizing that

## Schedule

[![](images/week6.png)](rust_1/lecture.pdf)
[![](images/week7.png)](rust_2/lecture.pdf)
[![](images/week9.png)]()
[![](images/week10.png)]()

## Resources

### Cool Rust apps

* [`broot`](https://github.com/Canop/broot) - Cool new `tree` (a lot more usable)
* [`bat`](https://github.com/sharkdp/bat) -  Updated `cat` (with git integration and a lot more)
* [`exa`](https://github.com/ogham/exa) - Modern replacement for `ls` 
* [`fd`](https://github.com/sharkdp/fd) - `find` alternative 
* [`alacritty`](https://github.com/alacritty/alacritty) - GPU-accelerated terminal emulator
* [`MdBook`](https://crates.io/crates/mdbook) - Book generator from Markdown docs
* [`Rust Godot bindings`](https://godot-rust.github.io/) - Super fast replacement for GDScript
* [`i3status-rust`](https://github.com/greshake/i3status-rust) - Fast i3status replacement
* [`zola`](https://github.com/getzola/zola) - A fast static site generator in a single binary with everything built-in

---
### Books

#### Beginner level

* [The Rust Programming Language](https://doc.rust-lang.org/book/) - A must-read that goes through almost everything
you will ever need to learn about Rust. This is also its main downside, as it might get a little too boring and
difficult, it's better to combine it with separate practice and exercises, and just take breaks from it. Is also
often referred to by the community as 'the Book', due to its importance and quality.

#### Intermediate level

* [Rust In Action (WIP)](https://livebook.manning.com/book/rust-in-action/welcome/v-15/) - A semi-official intermediate
book that's still in progress (although in its finishing stages).
* [Command Line Book](https://rust-cli.github.io/book/index.html) - What follows are official domain-specific books on main topics you might be interested in.
Better to read them when you actually need to work on respective topics rather than as your first intermediate books.
* [WebAssembly Book](https://rustwasm.github.io/docs/book/)
* [Embedded Book](https://doc.rust-lang.org/embedded-book)
* [The Rust Standard Library](https://doc.rust-lang.org/std/index.html) - Some of the best programming language
documentation ever. A logical continuation of the Book, both stylistically and practically. Don't read all of it
though! You might just need to explore a couple of topics to understand how basic Rust documentation works, 
and then just read up on something when you need it/when you are having problems with it. Once again, Rust documentation
is awesome, it allows you to jump to the source code immediately, since recently it allows you to nicely link to stuff,
allows to quickly look up types etc.

#### Intermediate References

* [Rust Reference](https://doc.rust-lang.org/reference/index.html) - THE Reference, describing everything humanity knows about Rust. Almost.
Use it as a table book, as an encyclopedia, not as a regular book!
* [Rust Language Cheat Sheet](https://cheats.rs/) - THE Cheat Sheet to quickly remind yourself how an `if-else` looks like. And everything else too.
* [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/index.html) - Rust has a 6-week update cycle, but every now and then Rust
releases a new edition which brings together features over the last few years and introduces breaking changes. This is a guide that helps
to understand when certain things were introduced in Rust. This isn't the most useful thing to you right now, but might help occasionally.
* [Cargo Book](https://doc.rust-lang.org/cargo/index.html) - What follows are little books on Rust toolchain, consult them if you want to
understand Rust ecosystem better or if you want to contribute to it.
* [RustDoc Book](https://doc.rust-lang.org/rustdoc/index.html)
* [Rustc Book](https://doc.rust-lang.org/rustc/index.html)
* [Compiler Error Index](https://doc.rust-lang.org/error-index.html) - ALL of the possible errors the Rust compiler might throw at you. There
are legends of people who have memorized all of it and have obtained the power of the whole galaxy (or just have never encountered another Rust
compiler error, and who is to say which is better).
* [Rust and WebAssembly](https://rustwasm.github.io/book/) - A little book on how to marry Rust and WASM together. They love each other.

#### Advanced level

* [Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) - I am going to steal the official description: "
This book digs into all the awful details that are necessary to 
understand in order to write correct Unsafe Rust programs. Due 
to the nature of this problem, it may lead to unleashing untold horrors 
that shatter your psyche into a billion infinitesimal fragments of despair."
* O'Reilly's Programming Rust [2017](https://1lib.eu/book/3400043/791885) or [2021(WIP)](https://1lib.eu/book/11015337/796e6b) - O'Reilly
books are awesome as always, but you should probably wait till 2021 edition comes out, it's in the final stages.
* [Mastering Rust](https://1lib.eu/book/4991763/d99ef4) - One of the strongest general advanced Rust books.
* [Rust Performance Book](https://nnethercote.github.io/perf-book/) - An interesting relatively low-level dive into performant Rust.
* [The Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html) - Often in Rust you'll have to resort to using
unstable features, which are included with various flags. Recently a lot of these have been merged into the stable Rust, but
this book still provides many useful ones with detailed descriptions.

---
### Practice

* [rustlings](https://github.com/rust-lang/rustlings/) - An interactive basic local course
* [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
* [Tour of Rust](https://tourofrust.com/)
* [Rust Playground](https://play.rust-lang.org/)

---
### Other resources

* [Rust Anthology](https://github.com/brson/rust-anthology)
* [Rust Patterns](https://github.com/rust-unofficial/patterns)
* [Idiomatic Rust](https://github.com/mre/idiomatic-rust)
* [Rust Learning](https://github.com/ctjhoa/rust-learning)
* [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
* [Rust Using Simple English](https://github.com/Dhghomon/easy_rust)
* [This Week In Rust](https://this-week-in-rust.org/) - Weekly newsletter on all things Rust: new blog posts, videos, conferences, features etc. 
* [This Month in OSDev](https://rust-osdev.com/) - Monthly newsletter on Operating Systems development in Rust
* [This Month in GameDev](https://rust-gamedev.github.io/) - Monthly newsletter on gamedevelopment in Rust
* [pretzelhammer's blog](https://github.com/pretzelhammer/rust-blog/)
* [phil-op blog](https://os.phil-opp.com/)
* [fasterthanlime blog](https://fasterthanli.me/)
* [Depth-First blog](https://depth-first.com/)
* [Armin Ronacher's blog](https://lucumr.pocoo.org/about/)
* [Stephen Marz OS blog](https://osblog.stephenmarz.com/)
* [Carlos Galdino's blog](https://blog.carlosgaldino.com/)
* [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/)
* [Rust101](https://www.ralfj.de/projects/rust-101/)
* [A creative coding framework](https://nannou.cc/)
* [RustStarterKit 2020](https://wiki.alopex.li/RustStarterKit2020)
* [Are We Web Yet?](https://www.arewewebyet.org/)
* [Are We Game Yet?](https://arewegameyet.rs/)
* [Are We Learning Yet?](http://www.arewelearningyet.com/)
* [Are We IDE Yet?](https://areweideyet.com/)
* [Are We GUI Yet?](https://www.areweguiyet.com/)

