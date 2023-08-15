package net.maelbrancke.filip.env

import io.ktor.serialization.kotlinx.json.json
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.plugins.contentnegotiation.ContentNegotiation
import io.ktor.server.plugins.defaultheaders.DefaultHeaders
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.SerializersModule

val kotlinXSerializersModule = SerializersModule {
}

fun Application.configure() {
  install(DefaultHeaders)
  install(ContentNegotiation) {
    json(
      Json {
        serializersModule = kotlinXSerializersModule
        isLenient = true
        ignoreUnknownKeys = true
      }
    )
  }
}
