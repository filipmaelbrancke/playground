package net.maelbrancke.filip.service

import net.maelbrancke.filip.CannotGenerateSolution
import net.maelbrancke.filip.KotestProject
import net.maelbrancke.filip.NoSolutionsAvailable
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.FreeSpec
import io.kotest.matchers.shouldBe
import net.maelbrancke.filip.WordCombination

class SolverTest: FreeSpec({

    suspend fun solverService(): SolverService = KotestProject.dependencies.get().solver

    "createSolutions" - {
        "input words cannot be empty" {
            val emptyInputs = CreateSolution(emptySet(), 6)
            val expected = CannotGenerateSolution("Unable to generate a solution, invalid inputs")

            val result = solverService().findSolutions(emptyInputs)
            result shouldBeLeft expected
        }
        "inputs without solution" {
            val fubar = CreateSolution(setOf("fubar", "fo", "obar"), 6)
            val expected = NoSolutionsAvailable("Unable to find solutions for [fubar, fo, obar]")

            val result = solverService().findSolutions(fubar)
            result shouldBeLeft expected
        }
        "valid inputs return expected" {
            val foobar = CreateSolution(setOf("foobar", "fo", "obar"), 6)
            val expected = WordCombination("foobar", listOf("fo", "obar"))

            val result = solverService().findSolutions(foobar)
            result.shouldBeRight().first() shouldBe expected
        }
    }
})


