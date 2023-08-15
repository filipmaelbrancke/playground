package net.maelbrancke.filip.service

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import arrow.core.raise.ensure
import net.maelbrancke.filip.CannotGenerateSolution
import net.maelbrancke.filip.NoSolutionsAvailable
import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.WordFinderError
import net.maelbrancke.filip.logger

const val DEFAULT_TARGET_LENGTH = 6
const val DEFAULT_NUMBER_OF_COMBINATIONS = 2

data class CreateSolution(
    val words: Set<String>,
    val targetLength: Int,
    val numberOfCombinations: Int = DEFAULT_NUMBER_OF_COMBINATIONS
) {
    companion object {
        operator fun invoke(inputWords: List<String>): CreateSolution {
            return CreateSolution(inputWords.toSet(), DEFAULT_TARGET_LENGTH)
        }
    }
}

interface SolverService {
    suspend fun findSolutions(input: CreateSolution): Either<WordFinderError, List<WordCombination>>
}

fun solverService(): SolverService = object : SolverService {

    private val logger = logger()

    private tailrec suspend fun Raise<CannotGenerateSolution>.recursiveSolutionGeneration(
        words: List<String>,
        index: Int,
        carryOn: List<WordCombination>
    ): List<WordCombination> {
        ensure(words.isNotEmpty()) {
            CannotGenerateSolution("Unable to generate a solution from too small an inpput collection $words")
        }

        if (index > words.lastIndex) {
            return carryOn
        }
        val otherElements = words.subList(0, index) + words.subList(index + 1, words.size)
        val element = words[index]
        return recursiveSolutionGeneration(
            words,
            index + 1,
            carryOn + otherElements.map { WordCombination(element + it, listOf(element, it)) })
    }

    override suspend fun findSolutions(input: CreateSolution): Either<WordFinderError, List<WordCombination>> = either {
        val targetWords = input.words.extractWordsOfLength(input.targetLength)
        val inputWords = input.words.filter { it.length < input.targetLength }           //.toList()
        logger.info("The target words are: $targetWords")
        ensure(inputWords.isNotEmpty()) { CannotGenerateSolution("Unable to generate a solution, invalid inputs") }
        val solutions = recursiveSolutionGeneration(inputWords, 0, emptyList())
            .filter { it.output.length == input.targetLength }
            .filter { targetWords.contains(it.output) }
        ensure(solutions.isNotEmpty()) { NoSolutionsAvailable("Unable to find solutions for ${input.words}") }
        solutions
    }
}

fun genericSolverService(): SolverService = object : SolverService {

    private val logger = logger()

    private suspend fun generateCombinationsUntil(
        words: List<String>,
        combinationSize: Int,
        remainingLength: Int,
        start: Int,
        currentCombo: MutableList<String>,
        result: MutableList<WordCombination>
    ) {
        if (combinationSize == 0 && remainingLength == 0) {
            val combined = currentCombo.joinToString(separator = "")
            if (words.contains(combined)) {
                result.add(WordCombination(combined, currentCombo.toList()))
            }
        }

        for (i in start until words.size) {
            val word = words[i]
            if (word.length <= remainingLength) {
                currentCombo.add(word)
                generateCombinationsUntil(
                    words,
                    combinationSize - 1,
                    remainingLength - word.length,
                    i,
                    currentCombo,
                    result
                )
                currentCombo.removeAt(currentCombo.size - 1)
            }
        }
    }

    override suspend fun findSolutions(input: CreateSolution): Either<WordFinderError, List<WordCombination>> = either {
        val result: MutableList<WordCombination> = mutableListOf()
        generateCombinationsUntil(input.words.toList(),  input.numberOfCombinations, input.targetLength, 0, mutableListOf(), result)

        ensure(result.isNotEmpty()) { NoSolutionsAvailable("Unable to find solutions for ${input.words}") }
        result.toList()
    }

}

fun Set<String>.extractWordsOfLength(targetWordLength: Int) = this.filter { it.length == targetWordLength }
