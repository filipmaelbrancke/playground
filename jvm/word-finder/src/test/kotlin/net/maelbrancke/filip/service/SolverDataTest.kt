package net.maelbrancke.filip.service

import net.maelbrancke.filip.KotestProject
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.FunSpec
import io.kotest.datatest.WithDataTestName
import io.kotest.datatest.withData
import io.kotest.matchers.collections.shouldContainExactlyInAnyOrder
import net.maelbrancke.filip.WordCombination

class SolverDataTest : FunSpec({

    suspend fun solverService(): SolverService = KotestProject.dependencies.get().solver


    context("test inputs that should have valid combinations") {
        withData(
            TestCombination(
                input = TestInput("foobar", "foob", "ar"),
                solutions = TestSolutions(WordCombination(output = "foobar", inputWords = listOf("foob", "ar")))
            ),
            TestCombination(
                input = TestInput("narrow", "narro", "w"),
                solutions = TestSolutions(WordCombination(output = "narrow", inputWords = listOf("narro", "w")))
            ),
            TestCombination(
                input = TestInput("eens", "qu", "w", "qu", "queens"),
                solutions = TestSolutions(WordCombination(output = "queens", inputWords = listOf("qu", "eens")))
            ),
            TestCombination(
                input = TestInput("s", "ignal", "shabby", "habby", "signal"),
                solutions = TestSolutions(
                    WordCombination(output = "shabby", inputWords = listOf("s", "habby")),
                    WordCombination(output = "signal", inputWords = listOf("s", "ignal"))
                )
            ),
            TestCombination(
                input = TestInput("s", "skiing", "hower", "qu", "eens", "shower", "w", "teamy",
                    "iver", "kiing", "quiver"),
                solutions = TestSolutions(
                    WordCombination(output = "skiing", inputWords = listOf("s", "kiing")),
                    WordCombination(output = "shower", inputWords = listOf("s", "hower")),
                    WordCombination(output = "quiver", inputWords = listOf("qu", "iver"))
                )
            )
        ) {
            val result = solverService().findSolutions(CreateSolution(it.input.words))
            result.shouldBeRight().shouldContainExactlyInAnyOrder(it.solutions.wordCombinations)
        }
    }

    context("test inputs that should have no solution") {
        withData(
            TestInput("foo", "bar"),
            TestInput("osine", "them", "narro", "es", "awler", "plex", "qu", "rrow", "iny")
        ) {
            val result = solverService().findSolutions(CreateSolution(it.words))
            result.shouldBeLeft()
        }
    }

})

data class TestInput(val words: List<String>) : WithDataTestName {
    companion object {
        operator fun invoke(vararg inputWords: String): TestInput {
            return TestInput(inputWords.asList())
        }
    }

    override fun dataTestName() = "testing $words"
}

data class TestSolutions(val wordCombinations: List<WordCombination>) {
    companion object {
        operator fun invoke(vararg solutions: WordCombination): TestSolutions {
            return TestSolutions(solutions.asList())
        }
    }
}

data class TestCombination(val input: TestInput, val solutions: TestSolutions) : WithDataTestName {
    override fun dataTestName() = "solving for ${solutions.wordCombinations.joinToString { it.output } }"
}
