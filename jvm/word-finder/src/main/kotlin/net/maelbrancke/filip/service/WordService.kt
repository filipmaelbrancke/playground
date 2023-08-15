package net.maelbrancke.filip.service

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import net.maelbrancke.filip.DomainError
import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.WordNotFound
import net.maelbrancke.filip.repo.WordPersistence
import net.maelbrancke.filip.sqldelight.Words

interface WordService {
    suspend fun insertWords(words: List<WordCombination>): Either<DomainError, List<WordCombination>>

    suspend fun checkSolutions(word: String): Either<DomainError, List<WordCombination>>
}

fun wordService(
    wordPersistence: WordPersistence
): WordService =
    object : WordService {
        override suspend fun insertWords(words: List<WordCombination>): Either<DomainError, List<WordCombination>> =
            either {

                wordPersistence.insertAll(words)
                words
            }

        override suspend fun checkSolutions(word: String): Either<DomainError, List<WordCombination>> =
            either {

                val words = wordPersistence.findAllByWord(word)
                ensure(words.isNotEmpty()) { WordNotFound(word) }
                words.map { it.toWordCombination() }
            }

    }

fun Words.toWordCombination(): WordCombination = WordCombination(
    output = word,
    inputWords = parts
)
