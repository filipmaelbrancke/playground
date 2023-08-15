package net.maelbrancke.filip.repo

import net.maelbrancke.filip.WordCombination
import net.maelbrancke.filip.logger
import net.maelbrancke.filip.sqldelight.Words
import net.maelbrancke.filip.sqldelight.WordsQueries

@JvmInline
value class WordId(val serial: Long)

interface WordPersistence {

    suspend fun insert(
        word: String,
        parts: List<String>
    ): WordId

    suspend fun insertAll(wordCombinations: List<WordCombination>)

    suspend fun findAllByWord(word: String): List<Words>

    suspend fun exists(word: String): Boolean

    suspend fun deleteAll()

    suspend fun findAll(): List<Words>

    suspend fun count(): Long
}

fun wordRepo(wordsQueries: WordsQueries) = object : WordPersistence {

    val logger = logger()

    override suspend fun insert(word: String, parts: List<String>): WordId =
        wordsQueries.transactionWithResult {
            val wordId =
                wordsQueries
                    .insertAndGetId(word, parts)
                    .executeAsOne()
            wordId
        }

    override suspend fun insertAll(wordCombinations: List<WordCombination>) =
        wordsQueries.transaction {
            wordCombinations.forEach { wordCombination ->
                wordsQueries.insert(
                    word = wordCombination.output,
                    parts = wordCombination.inputWords
                )
            }
        }

    override suspend fun findAllByWord(word: String): List<Words> =
        wordsQueries.selectPartsForWord(word).executeAsList()

    override suspend fun exists(word: String): Boolean =
        wordsQueries.wordExists(word).executeAsOne()

    override suspend fun deleteAll() = wordsQueries.deleteAll()

    override suspend fun findAll() =
        wordsQueries.selectAll().executeAsList()

    override suspend fun count() = wordsQueries.countWords().executeAsOne()

}
