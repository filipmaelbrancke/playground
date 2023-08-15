@file:Suppress("DEPRECATION")

package net.maelbrancke.filip

import arrow.fx.coroutines.continuations.resource
import net.maelbrancke.filip.env.Env
import net.maelbrancke.filip.env.dependencies
import net.maelbrancke.filip.env.hikari
import io.kotest.assertions.arrow.fx.coroutines.ProjectResource
import io.kotest.core.config.AbstractProjectConfig
import io.kotest.core.extensions.Extension
import io.kotest.core.listeners.TestListener
import io.kotest.core.test.TestCase
import io.kotest.core.test.TestResult
import io.kotest.extensions.testcontainers.StartablePerProjectListener
import org.testcontainers.containers.PostgreSQLContainer
import org.testcontainers.containers.wait.strategy.Wait

private class PostgreSQL : PostgreSQLContainer<PostgreSQL>("postgres:latest") {
    init {
        waitingFor(Wait.forListeningPort())
    }
}

/**
 * Configuration for Kotest 
 * Configures TestContainers for all tests
 */
object KotestProject : AbstractProjectConfig() {
    private val postgres = StartablePerProjectListener(PostgreSQL(), "postgres")

    private val dataSource: Env.DataSource by lazy {
        Env.DataSource(
            postgres.startable.jdbcUrl,
            postgres.startable.username,
            postgres.startable.password,
            postgres.startable.driverClassName
        )
    }

    private val env: Env by lazy { Env().copy(dataSource = dataSource) }

    val dependencies = ProjectResource(resource { dependencies(env) })
    private val hikari = ProjectResource(resource { hikari(env.dataSource) })

    private val resetDatabaseListener =
        object : TestListener {
            override suspend fun afterTest(testCase: TestCase, result: TestResult) {
                super.afterTest(testCase, result)
                val logger = logger()
                hikari.get().connection.use { conn ->
                    logger.info("Resetting words database")
                    conn.prepareStatement("TRUNCATE words CASCADE").executeLargeUpdate()
                }
            }
        }

    override fun extensions(): List<Extension> =
        listOf(postgres, hikari, dependencies, resetDatabaseListener)
}
