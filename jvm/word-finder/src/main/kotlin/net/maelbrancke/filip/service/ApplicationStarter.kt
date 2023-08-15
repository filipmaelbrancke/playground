package net.maelbrancke.filip.service

import arrow.core.Either
import arrow.core.raise.either
import net.maelbrancke.filip.DomainError
import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.logger
import net.maelbrancke.filip.repo.WordPersistence

interface ApplicationStarter {
    suspend fun insertProvidedData(): Either<DomainError, List<WordCombination>>
}

fun providedDataLoader(
    wordPersistence: WordPersistence,
    wordsLoader: WordsLoader
): ApplicationStarter =
    object : ApplicationStarter {

        val logger = logger()

        override suspend fun insertProvidedData(): Either<DomainError, List<WordCombination>> =
            either {
                logger.info("Loading sample data")

                val wordCombinations = wordsLoader.gatherInputWords().bind()

                wordPersistence.deleteAll()
                wordPersistence.insertAll(wordCombinations)
                wordCombinations
            }

    }
