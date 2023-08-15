import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

@Suppress("DSL_SCOPE_VIOLATION")
plugins {
  application
  alias(libs.plugins.kotest.multiplatform)
  id(libs.plugins.kotlin.jvm.get().pluginId)
  alias(libs.plugins.kotlinx.serialization)
  alias(libs.plugins.sqldelight)
  alias(libs.plugins.ktor)
  alias(libs.plugins.gradleVersions)
}

application {
  mainClass.set("net.maelbrancke.filip.MainKt")
}

java {
  sourceCompatibility = JavaVersion.VERSION_17
  targetCompatibility = JavaVersion.VERSION_17
}

sqldelight {
  databases {
    create("SqlDelight") {
      packageName.set("net.maelbrancke.filip.sqldelight")
      dialect(libs.sqldelight.postgresql.get())
    }
  }
}

tasks {
  withType<KotlinCompile>().configureEach {
    kotlinOptions {
      jvmTarget = "${JavaVersion.VERSION_17}"
      freeCompilerArgs = freeCompilerArgs + "-Xcontext-receivers"
    }
  }

  test {
    useJUnitPlatform()
  }
}


repositories {
  mavenCentral()
}

dependencies {
  implementation(libs.bundles.arrow)
  implementation(libs.bundles.ktor.server)
  implementation(libs.bundles.suspendapp)
  implementation(libs.logback.classic)
  implementation(libs.sqldelight.jdbc)
  implementation(libs.hikari)
  implementation(libs.postgresql)
  implementation(libs.bundles.cohort)

  testImplementation(libs.bundles.ktor.client)
  testImplementation(libs.testcontainers.postgresql)
  testImplementation(libs.ktor.server.tests)
  testImplementation(libs.bundles.kotest)
}

ktor {
  docker {
    jreVersion.set(io.ktor.plugin.features.JreVersion.JRE_17)
    localImageName.set("word-finder")
    imageTag.set("latest")
  }
}
