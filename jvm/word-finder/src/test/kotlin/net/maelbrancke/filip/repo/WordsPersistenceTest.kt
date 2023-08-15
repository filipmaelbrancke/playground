package net.maelbrancke.filip.repo

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.longs.shouldBeGreaterThan
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import net.maelbrancke.filip.KotestProject
import net.maelbrancke.filip.WordCombination

class WordsPersistenceTest : FunSpec({

    suspend fun wordRepo(): WordPersistence = KotestProject.dependencies.get().wordRepo

    val testWord = "foobar"
    val testParts = listOf("fo", "obar")

    /*
    * Integration tests using TestContainers
    * The test dependencies hava a test listener registered
    * that empties the database table between every test run
    */

    test("insert word into db") {
        val result = wordRepo().insert(testWord, testParts)

        result.serial.shouldBeGreaterThan(0)
    }

    test("insert word and check db existence") {
        wordRepo().insert(testWord, testParts)
        val result = wordRepo().exists(testWord)

        result.shouldBeTrue()
    }

    test("word should have a uniqueness constraint") {
        wordRepo().insert(testWord, testParts)

        val exception = shouldThrow<Exception> {
            wordRepo().insert(testWord, testParts)
        }
        exception.message shouldContain "duplicate"
    }

    test("insert multiple wordCombinations") {
        wordRepo().insertAll(listOf(
            WordCombination("foobar", listOf("fo", "obar")),
            WordCombination("search", listOf("s", "earch"))
        ))

        val numberOfWordsInDb = wordRepo().count()

        numberOfWordsInDb.shouldBe(2)
    }
})
