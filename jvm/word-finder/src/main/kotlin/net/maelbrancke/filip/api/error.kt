package net.maelbrancke.filip.api

import arrow.core.Either
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.ApplicationCall
import io.ktor.server.application.call
import io.ktor.server.request.receive
import io.ktor.server.response.respond
import io.ktor.util.pipeline.PipelineContext
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.MissingFieldException
import kotlinx.serialization.Serializable
import net.maelbrancke.filip.CannotGenerateSolution
import net.maelbrancke.filip.DomainError
import net.maelbrancke.filip.EmptyWordExistenceCheck
import net.maelbrancke.filip.EmptyWords
import net.maelbrancke.filip.IncorrectInput
import net.maelbrancke.filip.IncorrectJson
import net.maelbrancke.filip.NoSolutionsAvailable
import net.maelbrancke.filip.WordNotFound

@Serializable data class GenericErrorModel(val errors: GenericErrorModelErrors)

@Serializable data class GenericErrorModelErrors(val body: List<String>)

fun GenericErrorModel(vararg msg: String): GenericErrorModel =
  GenericErrorModel(GenericErrorModelErrors(msg.toList()))

context(PipelineContext<Unit, ApplicationCall>)

suspend inline fun <reified A : Any> Either<DomainError, A>.respond(status: HttpStatusCode): Unit =
  when (this) {
    is Either.Left -> respond(value)
    is Either.Right -> call.respond(status, value)
  }

@OptIn(ExperimentalSerializationApi::class)
@Suppress("ComplexMethod")
suspend fun PipelineContext<Unit, ApplicationCall>.respond(error: DomainError): Unit =
  when (error) {
    is IncorrectInput ->
      unprocessable(
        error.errors.joinToString { field -> "${field.field}: ${field.errors.joinToString()}" }
      )
    is IncorrectJson ->
      unprocessable("Json is missing fields: ${error.exception.missingFields.joinToString()}")

    is CannotGenerateSolution -> unprocessable(error.description)
    is NoSolutionsAvailable -> notFound(error.description)
    is WordNotFound -> notFound("No solutions found for the word ${error.word}")
    is EmptyWordExistenceCheck -> unprocessable(error.description)
    is EmptyWords -> unprocessable(error.description)
  }

private suspend inline fun PipelineContext<Unit, ApplicationCall>.unprocessable(
  error: String
): Unit = call.respond(HttpStatusCode.UnprocessableEntity, GenericErrorModel(error))

private suspend inline fun PipelineContext<Unit, ApplicationCall>.notFound(
  error: String
): Unit = call.respond(HttpStatusCode.NotFound, GenericErrorModel(error))

@OptIn(ExperimentalSerializationApi::class)
suspend inline fun <reified A : Any> PipelineContext<Unit, ApplicationCall>
        .receiveCatching(): Either<IncorrectJson, A> =
  Either.catchOrThrow<MissingFieldException, A> { call.receive() }.mapLeft { IncorrectJson(it) }
