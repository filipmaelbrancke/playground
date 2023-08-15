package net.maelbrancke.filip.env

import arrow.fx.coroutines.continuations.ResourceScope
import com.sksamuel.cohort.HealthCheckRegistry
import com.sksamuel.cohort.hikari.HikariConnectionsHealthCheck
import kotlinx.coroutines.Dispatchers
import net.maelbrancke.filip.repo.WordPersistence
import net.maelbrancke.filip.repo.wordRepo
import net.maelbrancke.filip.service.ApplicationStarter
import net.maelbrancke.filip.service.SolverService
import net.maelbrancke.filip.service.WordService
import net.maelbrancke.filip.service.fileBasedWordsLoader
import net.maelbrancke.filip.service.genericSolverService
import net.maelbrancke.filip.service.providedDataLoader
import net.maelbrancke.filip.service.solverService
import net.maelbrancke.filip.service.wordService
import kotlin.time.Duration.Companion.seconds

class Dependencies(
    val healthCheck: HealthCheckRegistry,
    val solver: SolverService,
    val genericSolver: SolverService,
    val wordRepo: WordPersistence,
    val applicationStarter: ApplicationStarter,
    val wordService: WordService
)

suspend fun ResourceScope.dependencies(env: Env): Dependencies {
    val hikari = hikari(env.dataSource)
    val sqlDelight = sqlDelight(hikari)
    val solver = solverService()
    val genericSolver = genericSolverService()
    val wordRepo = wordRepo(sqlDelight.wordsQueries)
    val wordService = wordService(wordRepo)
    val wordsLoader = fileBasedWordsLoader(env.wordData.filePath, solver)
    val applicationStarter = providedDataLoader(wordRepo, wordsLoader)

    val checks =
        HealthCheckRegistry(Dispatchers.Default) {
            register(HikariConnectionsHealthCheck(hikari, 1), 5.seconds)
        }

    return Dependencies(
        checks,
        solver,
        genericSolver,
        wordRepo,
        applicationStarter,
        wordService
    )
}
