package net.maelbrancke.filip.api

import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.collections.shouldContainExactlyInAnyOrder
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.collections.shouldNotBeEmpty
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.ktor.client.call.body
import io.ktor.client.request.get
import io.ktor.client.request.post
import io.ktor.client.request.setBody
import io.ktor.http.ContentType
import io.ktor.http.HttpStatusCode
import io.ktor.http.contentType
import net.maelbrancke.filip.KotestProject
import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.service.WordService
import net.maelbrancke.filip.withService

class WordsApiTest : StringSpec({

    val validInput = setOf("foo", "bar", "foobar")

    val testData = listOf(
        WordCombination(output = "signal", inputWords = listOf("s", "ignal")),
        WordCombination(output = "signal", inputWords = listOf("signa", "l")),
        WordCombination(output = "signal", inputWords = listOf("si", "gnal")),
        WordCombination(output = "signal", inputWords = listOf("sig", "nal")),
        WordCombination(output = "signal", inputWords = listOf("sign", "al")),
    )

    suspend fun wordService(): WordService = KotestProject.dependencies.get().wordService

    beforeTest {
        // add some test data to the database
        wordService().insertWords(testData)
    }

    "Solver API should have a result for valid inputs" {
        withService {
            val response =
                post("/api/words") {
                    contentType(ContentType.Application.Json)
                    setBody(SearchWordCombinations(validInput, 6, 2))
                }

            response.status shouldBe HttpStatusCode.OK
            assertSoftly {
                val solutions = response.body<Solutions>().wordCombinations
                solutions.shouldNotBeEmpty()
                solutions.first() shouldContain "foobar"
            }

        }
    }

    "Solver API should fail for invalid inputs" {
        withService {
            val invalidInputs = setOf("s", "hower", "skiing")
            val response =
                post("/api/words") {
                    contentType(ContentType.Application.Json)
                    setBody(SearchWordCombinations(invalidInputs, 6, 2))
                }

            response.status shouldBe  HttpStatusCode.NotFound
            response.body<GenericErrorModel>().errors.body shouldBe
                    listOf("Unable to find solutions for " + invalidInputs.joinToString(separator = ", ", prefix = "[", postfix = "]"))
        }
    }

    "Existence check API should give solution for the word signal" {
        withService {
            val response =
                get("/api/words/exists?word=signal") {
                    contentType(ContentType.Application.Json)
                }

            response.status shouldBe  HttpStatusCode.OK
            val solutions = response.body<Solutions>().wordCombinations
            assertSoftly {
                solutions.shouldHaveSize(5)
                solutions shouldContainExactlyInAnyOrder testData.map { it.display() }
            }
        }
    }

    "Existence check API should fail for invalid input" {
        withService {
            val response =
                get("/api/words/exists?word=") {
                    contentType(ContentType.Application.Json)
                }

            response.status shouldBe  HttpStatusCode.UnprocessableEntity
            response.body<GenericErrorModel>().errors.body shouldBe
                    listOf("Unable to check for an empty word")
        }
    }

    "Existence check API should give not found for unknown word" {
        withService {
            val response =
                get("/api/words/exists?word=abcdef") {
                    contentType(ContentType.Application.Json)
                }

            response.status shouldBe  HttpStatusCode.NotFound
            response.body<GenericErrorModel>().errors.body shouldBe
                    listOf("No solutions found for the word abcdef")
        }
    }

})
