package net.maelbrancke.filip.service

import arrow.core.Either
import arrow.core.raise.either
import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.WordFinderError
import java.io.File

interface WordsLoader {
    suspend fun gatherInputWords(): Either<WordFinderError, List<WordCombination>>
}

fun fileBasedWordsLoader(
    filePath: String,
    solver: SolverService
): WordsLoader =
    object : WordsLoader {

    override suspend fun gatherInputWords(): Either<WordFinderError, List<WordCombination>> =
        either {
            val inputFileLines = File(filePath).readLines()
            val inputs = CreateSolution(inputFileLines)
            val wordCombinations = solver.findSolutions(inputs).bind()
            wordCombinations
        }

}
