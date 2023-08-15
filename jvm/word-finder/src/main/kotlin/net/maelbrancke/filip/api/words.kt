package net.maelbrancke.filip.api

import arrow.core.raise.either
import arrow.core.raise.ensure
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.routing.get
import io.ktor.server.routing.post
import io.ktor.server.routing.route
import io.ktor.server.routing.routing
import kotlinx.serialization.Serializable
import net.maelbrancke.filip.EmptyWordExistenceCheck
import net.maelbrancke.filip.logger
import net.maelbrancke.filip.service.CreateSolution
import net.maelbrancke.filip.service.SolverService
import net.maelbrancke.filip.service.WordService

@Serializable
data class SearchWordCombinations(
    val words: Set<String>,
    val targetLength: Int,
    val numberOfCombinations: Int? = null
)

@Serializable
data class Solutions(
    val wordCombinations: List<String>
)

fun Application.wordRoutes(
    solverService: SolverService,
    wordService: WordService
) = routing {
    val logger = logger()

    route("/api/words") {
        post {
            either {
                val (words, targetLength, numberOfCombinations) = receiveCatching<SearchWordCombinations>().bind()
                logger.info(
                    """Incoming API call: words = $words / 
                    target length = $targetLength / 
                    number of combinations = $numberOfCombinations"""
                )
                val solutions = solverService.findSolutions(CreateSolution(words, targetLength, numberOfCombinations ?: 2)).bind()
                Solutions(solutions.map { it.display() })
            }
                .respond(HttpStatusCode.OK)
        }
        get("/exists") {
            either {
                val check = call.request.queryParameters["word"]
                ensure(!check.isNullOrBlank()) { EmptyWordExistenceCheck("Unable to check for an empty word") }
                val solutions = wordService.checkSolutions(check).bind()
                Solutions(solutions.map { it.display() })
            }
                .respond(HttpStatusCode.OK)
        }
    }
}

