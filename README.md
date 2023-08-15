# playground

![API build](https://github.com/filipmaelbrancke/playground/actions/workflows/api-build.yml/badge.svg) ![Cmd-line apps build](https://github.com/filipmaelbrancke/playground/actions/workflows/cmd-apps-build.yml/badge.svg)

> Trying to program a bit during the evenings when not-programming at the job...

Ongoing:

- [playing around with backend development in Rust](rust/api/) I have quite a bit of experience 'doing backends' on the JVM, but it's interesting to find out what building a cloud-native application could look like in Rust.    
There are many competing options in the Rust web framework ecosystem (axtix, axum, poem, rocket, ...), here I'm choosing to explore  `actix-web` which seems to be the one with the most extensive usage, largest community and plugin ecosystem, and runs on tokio, what makes this an interesting exercise to learn a bit more on that async runtime.
- [building a (simplified) clone of Redis in Rust](rust/clone-redis/)

One-offs:

- small Rust exercises implementing the basics of well-known command-line applications:
	- [echo command in Rust](rust/recho/)
	- [cat command in Rust](rust/rcat/)
	- [head command in Rust](rust/rhead/)
	- [wc command in Rust](rust/rwc/)
	- [uniq command in Rust](rust/runiq/)
	- [find command in Rust](rust/rfind/)
	- [cut command in Rust](rust/rcut/)
	- [grep command in Rust](rust/rgrep/)
	- [comm command in Rust](rust/rcomm/)
	- [tail command in Rust](rust/rtail/)
	- [fortune command in Rust](rust/rfortune/)
	- [cal command in Rust](rust/rcal/)
	- [ls command in Rust](rust/rls/)
