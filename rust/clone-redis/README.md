Implementing a Redis clone in Rust

## Why?

Building a (simplified) version of Redis to learn Rust, and as a general programming exercise.

# Redis

In-memory data structure store.   
To be used as database / cache / message broker.

## Traits:     

- fast read/write operations
- flexible: variety of data types like strings, hashes, lists, sets, bitmaps, geospation indexes, ... making Redis useable for features like caching, session storage, messaging systems, leaderboards, analytics, ...
- simple / intuitive command API
- replication/persistence: being an in-memory store, Redis does offer persistence mechanisms. Redis also supports replication in order to offer improved performance and data redundancy
- extensible: Redis supports Lua scripting
- big community & ecosystem

## Commands

Obviously not going to implement the full [list of Redis commands](https://redis.io/commands/), only doing the most used ones, in order to practise some Rust and learn a bit more on the [tokio](https://tokio.rs/) ecosystem.

Redis commands consist of an operation and arguments (for instance: `SET key value`), commands get send to the Redis server using the Redis Serialization Protocol (RESP).

## Redis Serialization Protocol

RESP:    

- serialize
- transmit over TCP
- text-based -> encoded data structures into sequences of bytes
- uses set of rules to encode data structures into sequences of bytes to be sent over the network and interpreted by the receiver
- protocol uses prefixes to indicate the type of the following data:
	- simple Strings have lines that start with "+".
	- errors have lines that start with "-".
	- integers: Lines that start with a ":".
	- Bulk Strings have lines that start with a "$".
	- arrays: Lines that start with a "*".

A Redis command in RESP format starts with an asterisk ("*"), followed by the number of arguments in the command, and then each argument is represented on its own line.    
For example, the `SET welcome "Hello, RDB!"` command would be sent over the wire as:

```
*3
$3
SET
$7
welcome
$11
Hello, RDB!
```

Explanation:

- the *3 means that there are three arguments in the command (SET, welcome, Hello, RDB!)
- the "$3", "$7", and "$11" represent the lengths of each of these arguments
- between each line are two bytes, carriage return and line feed (CRLF)


### Redis Bulk Strings

https://redis.io/docs/reference/protocol-spec/#resp-bulk-strings

# Resources

- [Redis](https://redis.io/)
- [Redis protocol spec](https://redis.io/docs/reference/protocol-spec/)
- [Implement RougeDB](https://learning.accelerant.dev/implement-rougedb)
- [Build Your Own Redis](https://rohitpaulk.com/articles/redis-0)
- [Redis re-implemented in Rust](https://github.com/seppo0010/rsedis)
- [Writing a Redis clone in Go from scratch](https://mliezun.github.io/2023/04/08/redis-clone.html)
- [Building a (Java) Redis Clone](https://gamlor.info/posts-output/2022-07-04-java-redis-clone/en/)
- [Build Your Own Redis with C/C++](https://build-your-own.org/redis/)
