import com.kouqurong.sharefilelib.ShareFileLib
import kotlin.test.Test


class TestShareFileLib {


    @Test
    fun testLoadLib() {
        val shareFileLib = ShareFileLib()

        val isStart = shareFileLib.startServer(8080, "../")

        println("isStart: $isStart")

        Thread.sleep(1000 * 20)

        shareFileLib.stopServer()
    }
}