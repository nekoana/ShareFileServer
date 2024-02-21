import java.nio.file.Paths

plugins {
    kotlin("jvm") version "1.9.22"
}

group = "com.kouqurong.sharefilelib"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.jetbrains.kotlin:kotlin-test")
}

tasks.test {
    useJUnitPlatform()
}
kotlin {
    jvmToolchain(17)
}

tasks.register<Exec>("buildLib") {
    val osName = System.getProperty("os.name")

    val isWin = osName.startsWith("Windows", true)
    val isMac = osName.startsWith("Mac", true)
    val isLinux = osName.contains("linux")

    val fileExtension = if (isWin) ".exe" else if (isLinux) ".so" else if(isMac) ".dylib"  else ""

    val fileName = "libshare_file_jni$fileExtension"

    val copyCommand = if (isWin) "copy" else "cp"


    commandLine = buildList {
        if (isWin) {
            add("cmd")
            add("/c")
        } else {
            add("sh")
            add("-c")
        }

        val srcPath = Paths.get("target", "release", fileName).toString()
        val targetPath = Paths.get("ShareFileLib","src", "main", "resources").toString()

        add("""
            cargo build --lib -p share_file_jni --release &&
            $copyCommand $srcPath $targetPath
        """)
    }

    workingDir = rootDir.parentFile
}

tasks.getByName("build").dependsOn(tasks.getByName("buildLib"))
