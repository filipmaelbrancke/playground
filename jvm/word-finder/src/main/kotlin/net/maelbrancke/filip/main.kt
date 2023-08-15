package net.maelbrancke.filip

import arrow.continuations.SuspendApp
import arrow.continuations.ktor.server
import arrow.fx.coroutines.resourceScope
import io.ktor.server.application.Application
import io.ktor.server.netty.Netty
import kotlinx.coroutines.awaitCancellation
import net.maelbrancke.filip.api.wordRoutes
import net.maelbrancke.filip.env.Dependencies
import net.maelbrancke.filip.env.Env
import net.maelbrancke.filip.env.configure
import net.maelbrancke.filip.env.dependencies
import net.maelbrancke.filip.api.health

fun main(): Unit = SuspendApp {
  val env = Env()
  resourceScope {
    val dependencies = dependencies(env)
    server(Netty, host = env.http.host, port = env.http.port) { app(dependencies) }
    dependencies.applicationStarter.insertProvidedData()
    awaitCancellation()
  }
}

fun Application.app(module: Dependencies) {
  configure()
  wordRoutes(module.genericSolver, module.wordService)
  health(module.healthCheck)
}
