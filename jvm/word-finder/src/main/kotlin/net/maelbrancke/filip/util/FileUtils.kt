package net.maelbrancke.filip.util

import java.io.File
import java.nio.charset.Charset

fun readFileOrResource(path: String): ByteArray {
    return if (File(path).exists()) {
        File(path).readBytes()
    } else if (ResourceLoader::class.java.classLoader.getResource(path) != null) {
        ResourceLoader::class.java.classLoader.getResource(path)!!.readBytes()
    } else {
        ResourceLoader::class.java.getResource(path)!!.readBytes()
    }
}

private object ResourceLoader

fun readFileOrResourceAsString(path: String, charset: Charset = Charsets.UTF_8): String =
    String(readFileOrResource(path), charset)
