package net.maelbrancke.filip

import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.FreeSpec
import net.maelbrancke.filip.api.SearchWordCombinations

class ValidationTest: FreeSpec({

    "validation is ok" - {
        "valid inputs" {
            val valid = SearchWordCombinations(words = setOf("foo", "bar", "foobar"), targetLength = 6, numberOfCombinations = 2)
            val result = valid.validate()

            result.shouldBeRight()
        }
    }

    "validation fails" - {
        "invalid target length" {
            val invalid = SearchWordCombinations(words = setOf("foo", "bar", "foobar"), targetLength = 1, numberOfCombinations = 2)
            val result = invalid.validate()

            result.shouldBeLeft()
        }
        "invalid input word" {
            val invalid = SearchWordCombinations(words = setOf("foo", "bar", ""), targetLength = 6, numberOfCombinations = 2)
            val result = invalid.validate()

            result.shouldBeLeft()
        }
        "invalid input word and invalid target length" {
            val invalid = SearchWordCombinations(words = setOf("foo", "bar", ""), targetLength = 1, numberOfCombinations = 2)
            val result = invalid.validate()

            result.shouldBeLeft()
        }
    }
})
