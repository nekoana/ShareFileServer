import com.kouqurong.sharefilelib.ShareFileLib
import kotlin.test.Test


class TestShareFileLib {
    @Test
    fun testLoadLib() {
        println(System.getProperty("user.dir"))
        val shareFileLib = ShareFileLib(8081,"../")

        assert(shareFileLib.startServer())

        Thread.sleep(1000 * 10)

        shareFileLib.stopServer()

        Thread.sleep(1000 * 20)
    }
}