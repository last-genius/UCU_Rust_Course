A repository for [Linux_Club@UCU](https://github.com/ucu-cs/UCU_Linux_Club) Rust course

## Contents
1. [Schedule](#Schedule)
1. [Resources](#Resources)
	1. [Cool Rust Apps](#Cool-Rust-Apps)
	1. [Books](#Books)
	1. [Practice](#Practice)
	1. [Other resources](#Other-resources)
1. [Vim integration guide](#Vim-Integration-Guide)

## Schedule

[*Latex sources are available here*](https://github.com/LastGenius-edu/latex_templates)

[![](images/week1.png)](rust_1/lecture.pdf)
[![](images/week2.png)](rust_2/lecture.pdf)
[![](images/week3.png)](rust_3/lecture.pdf)
[![](images/projects.png)](./PROJECTS.md)
[![](images/week4.png)]()

## Resources

### Cool Rust apps

* [`nushell`](https://github.com/nushell/nushell) - An interesting new type of shell
* [`dust`](https://github.com/bootandy/dust) - A more intuitive replacement for `du`
* [`ytop`](https://github.com/cjbassi/ytop) - A super-fast `top` replacement
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
* [Async Book](https://rust-lang.github.io/async-book/) - Rust introduced stable `async/.await` syntax in 2019, which
is now used in many libraries (`tokio`, `rayon`), and allows for fearless and beautiful concurrency.
* [Roguelike Book](https://bfnightly.bracketproductions.com/rustbook/) - A nice intro to Rust through a simple game (also uses ECS!)
* [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) - A head-on Rust book that does not
fear difficult Rust memory stuff and implements a bunch of linked lists (which are their own thing with Rust's ownership system).
* [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html) - A nice little collection of practical lessons on Rust macros.

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

* [rustlings](https://github.com/rust-lang/rustlings/) - An interactive basic local course.
* [Tour of Rust](https://tourofrust.com/) - Basic Web Intro to Rust.
* [Exercism Rust Track](https://exercism.io/tracks/rust) - Interactive Rust exercises. To get Rust embedded in your muscle memory.
* [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/) - A bunch of Rust examples ranked by topic and difficulty.
* [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - Super useful and smart ways of achieving basic things in Rust.
* [Rust Playground](https://play.rust-lang.org/) - Rust IDE and compiler online.

---
### Other resources

* [Rust Anthology](https://github.com/brson/rust-anthology/blob/master/master-list.md) - A semi-abandoned (I think) but nevertheless impressive collection of great Rust blog posts.
* [Rust Patterns](https://github.com/rust-unofficial/patterns) - A collection of useful (and idiomatic) Rust patterns for the most popular use cases.
* [Idiomatic Rust](https://github.com/mre/idiomatic-rust) - A collection of idiomatic and beautiful Rust. Worth checking out.
* [Rust Learning](https://github.com/ctjhoa/rust-learning) - A huge collection of various Rust learning resources. Too overwhelming though.
* [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - One of those 'Awesome language' repos, just a collection of Rust projects.
* [Rust Using Simple English](https://github.com/Dhghomon/easy_rust) - Rust is difficult enough on its own,
there is no need to complicate it with difficult English, and this guide solves this!
* [This Week In Rust](https://this-week-in-rust.org/) - Weekly newsletter on all things Rust: new blog posts, videos, conferences, features etc. 
* [This Month in OSDev](https://rust-osdev.com/) - Monthly newsletter on Operating Systems development in Rust
* [This Month in GameDev](https://rust-gamedev.github.io/) - Monthly newsletter on gamedevelopment in Rust
* [pretzelhammer's blog](https://github.com/pretzelhammer/rust-blog/) - What follows is a bunch of blogs that I found nice, by no 
means an exhaustive list. pretzelhammer writes super cool deep dive articles on Rust-specific stuff which makes people's heads boil.
* [phil-op blog](https://os.phil-opp.com/) - Philipp Oppermann's x86 Operating System Blog. The next best thing in the Internet after this repository web page.
* [fasterthanlime blog](https://fasterthanli.me/) - A great blog on all things Rust. 
* [Depth-First blog](https://depth-first.com/) - A few interesting things here. Not that many though.
* [Armin Ronacher's blog](https://lucumr.pocoo.org/about/) - This guy created Flask and Jinja. And now he writes about Rust. What more do you need.
* [Stephen Marz OS blog](https://osblog.stephenmarz.com/) - RISC-V OS Blog.
* [Carlos Galdino's blog](https://blog.carlosgaldino.com/) - This blog has an interesting practical Rust file system implementation.
* [Shesh's blog](http://www.sheshbabu.com/) - A nice blog going over some of the basic Rust things and teaching Rust for JavaScript devs.
* [Gankra's blog](https://gankra.github.io/) - An awesome intermediate/advanced blog on the intricacies of Rust's (often unsafe) groundings from the author of the Rustonomicon.
* [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/) - One of the many relatively short intro guides to Rust. This is one of the better ones.
* [Rust101](https://www.ralfj.de/projects/rust-101/) - One of the many tutorials I think has something going for it (and the blog is awesome too!)
* [A creative coding framework](https://nannou.cc/) - Just something I hacked with for a while and it's super nice.
* [RustStarterKit 2020](https://wiki.alopex.li/RustStarterKit2020) - An interesting guide on things you might wanna
consider using if you work with Rust in 2020+. Since Rust has a minimal standard library, it suggests a few popular
math crates, utility crates that work with different file formats, devices etc., as well as a lot of algorithms/data
structures implemented efficiently, networking/web stuff, asynchronous crates.
* [Are We Web Yet?](https://www.arewewebyet.org/) - What follows are several websites tracking Rust progress on the way of
becoming a usable language in a specific domain. This requires having a bunch of often difficult libraries and vibrant
communities to support them. Makes it a little easier to know whether Rust is okay to use in a web/game/ml/gui context.
* [Are We Game Yet?](https://arewegameyet.rs/)
* [Are We Learning Yet?](http://www.arewelearningyet.com/)
* [Are We IDE Yet?](https://areweideyet.com/)
* [Are We GUI Yet?](https://www.areweguiyet.com/)
* [Are We Async Yet?](https://areweasyncyet.rs/)

## Vim Integration Guide

**My dotfiles with detailed instructions are available [over here](https://github.com/LastGenius-edu/my_dotfiles).**

You don't have to use Vim of course, Rust has nice integrations for most
of the popular IDEs, including VSCode, JetBrains, Emacs etc.
