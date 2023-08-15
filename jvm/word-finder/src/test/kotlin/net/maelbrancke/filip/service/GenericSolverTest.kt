package net.maelbrancke.filip.service

import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.FreeSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.shouldBe
import net.maelbrancke.filip.NoSolutionsAvailable
import net.maelbrancke.filip.WordCombination

class GenericSolverTest : FreeSpec({

    suspend fun genericSolver(): SolverService = genericSolverService()

    "invalid inputs should have no solutions" - {
        "target length 6 - 2 combinations - no applicable inputs" {
            val fubar = CreateSolution(setOf("fubar", "fo", "obar"), 6)
            val expected = NoSolutionsAvailable("Unable to find solutions for [fubar, fo, obar]")

            val result = genericSolver().findSolutions(fubar)
            result shouldBeLeft expected
        }
    }


    "valid inputs with Solutions" - {
        "target length 6 - 2 combinations" {
            val foobar =
                CreateSolution(words = setOf("foobar", "fo", "obar"), targetLength = 6, numberOfCombinations = 2)
            val expected = WordCombination("foobar", listOf("fo", "obar"))

            val result = genericSolver().findSolutions(foobar)
            assertSoftly {
                result.shouldBeRight().shouldHaveSize(1)
                result.shouldBeRight().first() shouldBe expected
            }
        }
        "target length 6 - 3 combinations" {
            val inputWords = CreateSolution(
                words = setOf(
                    "si",
                    "gn",
                    "al",
                    "shabby",
                    "osine",
                    "fo",
                    "them",
                    "narro",
                    "es",
                    "awler",
                    "plex",
                    "foobar",
                    "qu",
                    "rrow",
                    "iny",
                    "shabb",
                    "rrow",
                    "obar",
                    "well",
                    "zambia",
                    "inks",
                    "nd",
                    "s",
                    "romie",
                    "habb",
                    "osen",
                    "kiing",
                    "mu",
                    "sin",
                    "appeal",
                    "us",
                    "and",
                    "tryf",
                    "d",
                    "t",
                    "y",
                    "tle",
                    "signal"
                ), targetLength = 6, numberOfCombinations = 3
            )
            val expected1 = WordCombination("signal", listOf("si", "gn", "al"))
            val expected2 = WordCombination("shabby", listOf("s", "habb", "y"))

            val result = genericSolver().findSolutions(inputWords)
            assertSoftly {
                result.shouldBeRight().shouldHaveSize(2)
                result.shouldBeRight().first() shouldBe expected1
                result.shouldBeRight().last() shouldBe expected2
            }
        }
    }
})
