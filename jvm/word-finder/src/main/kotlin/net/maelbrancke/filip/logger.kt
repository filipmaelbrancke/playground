package net.maelbrancke.filip

import org.slf4j.Logger
import org.slf4j.LoggerFactory

inline fun <reified T> T.logger(clazz: Class<out T>? = T::class.java): Logger = LoggerFactory.getLogger(clazz)
