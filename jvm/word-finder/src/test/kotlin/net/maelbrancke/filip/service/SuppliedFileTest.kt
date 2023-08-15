package net.maelbrancke.filip.service

import net.maelbrancke.filip.KotestProject
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.collections.shouldNotBeEmpty
import java.io.File

class SuppliedFileTest: FunSpec({

    suspend fun solverService(): SolverService = KotestProject.dependencies.get().solver

    test("The supplied input file should be solvable") {
        val allInputs = File("src/main/resources/input.txt").readLines().toSet()
        val tryToSolveAllInputs = CreateSolution(allInputs, 6)

        val result = solverService().findSolutions(tryToSolveAllInputs)
        result.shouldBeRight().shouldNotBeEmpty()
    }

})
