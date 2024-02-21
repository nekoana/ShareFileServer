package com.kouqurong.sharefilelib

import java.nio.file.Files
import java.nio.file.Paths
import kotlin.io.path.createParentDirectories


private const val LIB_NAME = "share_file_jni"
private const val LIB_PATH = "lib"

class ShareFileLib(private val port: Int, private val path: String) {
    private val osName = System.getProperty("os.name")

    private val isWin = osName.startsWith("Windows", true)

    private val isMac = osName.startsWith("Mac", true)

    private val workingDir = System.getProperty("user.dir")


    private val libName = if (isWin) {
        "lib$LIB_NAME.dll"
    } else if (isMac) {
        "lib$LIB_NAME.dylib"
    } else {
        "lib$LIB_NAME.so"
    }

    init {
        loadLib()
    }

    fun startServer(): Boolean {
        return startServer(port, path)
    }


    private external fun startServer(port: Int, path: String): Boolean

    external fun stopServer()

    private fun isLibExist(): Boolean {
        return Files.exists(Paths.get(workingDir, LIB_PATH, libName))
    }

    private fun copyLib() {
        val path = Paths.get(workingDir, LIB_PATH, libName).apply {
            createParentDirectories()
        }

        ShareFileLib::class.java.classLoader.getResourceAsStream(libName)?.use {
            Files.copy(it, path)
        }
    }

    private fun loadLib() {
        if (!isLibExist()) {
            copyLib()
        }

        System.load(Paths.get(LIB_PATH, libName).toAbsolutePath().toString())
    }
}