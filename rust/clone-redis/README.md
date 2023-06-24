Implementing a Redis clone in Rust

## Why?

Building a (simplified) version of Redis to learn Rust, and as a general programming exercise.

# Redis

In-memory data structure store.   
To be used as database / cache / message broker.

Traits:     

- fast read/write operations
- flexible: variety of data types like strings, hashes, lists, sets, bitmaps, geospation indexes, ... making Redis useable for features like caching, session storage, messaging systems, leaderboards, analytics, ...
- simple / intuitive command API
- replication/persistence: being an in-memory store, Redis does offer persistence mechanisms. Redis also supports replication in order to offer improved performance and data redundancy
- extensible: Redis supports Lua scripting
- big community & ecosystem

# Resources

- [Redis](https://redis.io/)
- [Implement RougeDB](https://learning.accelerant.dev/implement-rougedb)
- [Build Your Own Redis](https://rohitpaulk.com/articles/redis-0)
- [Redis re-implemented in Rust](https://github.com/seppo0010/rsedis)
- [Writing a Redis clone in Go from scratch](https://mliezun.github.io/2023/04/08/redis-clone.html)
- [Building a (Java) Redis Clone](https://gamlor.info/posts-output/2022-07-04-java-redis-clone/en/)
