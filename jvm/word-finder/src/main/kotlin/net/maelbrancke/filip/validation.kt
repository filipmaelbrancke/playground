package net.maelbrancke.filip

import arrow.core.Either
import arrow.core.NonEmptyList
import arrow.core.left
import arrow.core.nonEmptyListOf
import arrow.core.raise.either
import arrow.core.raise.ensure
import arrow.core.raise.ensureNotNull
import arrow.core.raise.zipOrAccumulate
import arrow.core.right
import arrow.core.toNonEmptyListOrNull
import net.maelbrancke.filip.api.SearchWordCombinations

private const val MIN_TARGET_LENGTH = 2

fun SearchWordCombinations.validate(): Either<NonEmptyList<ValidationError>, SearchWordCombinations> =
    either {

        zipOrAccumulate(
            { ensure(words.isNotEmpty()) { EmptyWords("Cannot search for a solution without inputs") } },
            { ensure(targetLength >= MIN_TARGET_LENGTH) { IncorrectInput(InvalidTargetLength(nonEmptyListOf("target length has a minimum of $MIN_TARGET_LENGTH"))) } },
            {
                val validatedInputWords = mapOrAccumulate(words) {
                    it.validateInputWord().bind()
                }
                ensureNotNull(validatedInputWords.toNonEmptyListOrNull()) { EmptyWords("no valid input words") }
            }
        ) { _, _, _ ->
            SearchWordCombinations(words, targetLength)
        }

    }

sealed interface InvalidField {
    val errors: NonEmptyList<String>
    val field: String
}

data class InvalidInputWord(override val errors: NonEmptyList<String>): InvalidField {
    override val field: String = "words"
}

data class InvalidTargetLength(override val errors: NonEmptyList<String>): InvalidField {
    override val field: String = "targetLength"
}

fun String.validateInputWord(): Either<ValidationError, String> =
    if (isNotBlank()) right() else IncorrectInput(InvalidInputWord(nonEmptyListOf("input words should not be blank"))).left()




