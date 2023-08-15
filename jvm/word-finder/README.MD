# Word finding exercise

> Example small testing exercise


*Word finder rules*

A file is supplied, input.txt, that contains words of varying lengths (1 to 6 characters).

The objective is to show all combinations of those words that together form a word of 6 characters. That combination must also be present in input.txt  
E.g.:
``` 
foobar  
fo  
obar
```
should result in the output:
```
fo+obar=foobar
```

You can start by only supporting combinations of two words and improve the algorithm at the end of the exercise to support any combinations.

Extras to do: create an API that can be called with a request containing a list of words and then returning the output as described above.
Add persistency.....


## How?

This implementation of the exercise is used as an opportunity to use & learn a bit more about ::    
- [Kotlin](https://kotlinlang.org/docs/home.html) 
- [Ktor](https://ktor.io/) as asynchronous web framework
- [Arrow](https://arrow-kt.io/) for functional programming features
- [SqlDelight](https://cashapp.github.io/sqldelight/) for the persistence layer
- [Kotest](https://kotest.io/) for testing

## Running the project

Needs Docker

```shell
docker compose up
./gradlew run
curl -i localhost:8080/readiness
```

Sample HTTP requests in `api.http`
