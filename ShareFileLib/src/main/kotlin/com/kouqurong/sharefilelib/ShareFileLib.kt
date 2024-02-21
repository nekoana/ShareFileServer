package com.kouqurong.sharefilelib

class ShareFileLib {
    init {
        System.load("/Users/codin/RustroverProjects/ShareFileServer/target/debug/libshare_file_jni.dylib")
    }


    external fun startServer(port: Int, path: String): Boolean

    external fun stopServer()
}