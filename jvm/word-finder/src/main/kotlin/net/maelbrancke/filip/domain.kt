package net.maelbrancke.filip

import arrow.core.NonEmptyList
import arrow.core.nonEmptyListOf
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.MissingFieldException

data class WordCombination(val output: String, val inputWords: List<String>) {
    fun display(): String {
        return inputWords.joinToString(separator = "+", postfix = "=${this.output}")
    }
}


sealed interface DomainError

sealed interface ValidationError : DomainError

@OptIn(ExperimentalSerializationApi::class)
data class IncorrectJson(val exception: MissingFieldException) : ValidationError

data class IncorrectInput(val errors: NonEmptyList<InvalidField>) : ValidationError {
    constructor(head: InvalidField) : this(nonEmptyListOf(head))
}

data class EmptyWords(val description: String) : ValidationError

data class EmptyWordExistenceCheck(val description: String) : ValidationError

sealed interface WordFinderError : DomainError

data class CannotGenerateSolution(val description: String) : WordFinderError
data class NoSolutionsAvailable(val description: String) : WordFinderError

sealed interface WordError : DomainError

data class WordNotFound(val word: String) : WordError
