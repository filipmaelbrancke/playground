package net.maelbrancke.filip.env

import app.cash.sqldelight.ColumnAdapter
import app.cash.sqldelight.driver.jdbc.asJdbcDriver
import arrow.fx.coroutines.autoCloseable
import arrow.fx.coroutines.closeable
import arrow.fx.coroutines.continuations.ResourceScope
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import net.maelbrancke.filip.repo.WordId
import net.maelbrancke.filip.sqldelight.SqlDelight
import net.maelbrancke.filip.sqldelight.Words
import javax.sql.DataSource

suspend fun ResourceScope.hikari(env: Env.DataSource): HikariDataSource = autoCloseable {
  HikariDataSource(
    HikariConfig().apply {
      jdbcUrl = env.url
      username = env.username
      password = env.password
      driverClassName = env.driver
    }
  )
}

suspend fun ResourceScope.sqlDelight(dataSource: DataSource): SqlDelight {
  val driver = closeable { dataSource.asJdbcDriver() }
  SqlDelight.Schema.create(driver)
  return SqlDelight(
    driver,
    Words.Adapter(wordIdAdapter, partsAdapter = listOfStringsAdapter)
  )
}

private val wordIdAdapter = columnAdapter(::WordId, WordId::serial)
private val listOfStringsAdapter = object : ColumnAdapter<List<String>, String> {
  override fun decode(databaseValue: String) =
    if (databaseValue.isEmpty()) {
      listOf()
    } else {
      databaseValue.split(",")
    }
  override fun encode(value: List<String>) = value.joinToString(separator = ",")
}

private inline fun <A : Any, B> columnAdapter(
  crossinline decode: (databaseValue: B) -> A,
  crossinline encode: (value: A) -> B
): ColumnAdapter<A, B> =
  object : ColumnAdapter<A, B> {
    override fun decode(databaseValue: B): A = decode(databaseValue)
    override fun encode(value: A): B = encode(value)
  }
