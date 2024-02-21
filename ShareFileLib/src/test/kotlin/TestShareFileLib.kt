import com.kouqurong.sharefilelib.ShareFileLib
import kotlin.test.Test


class TestShareFileLib {


    @Test
    fun testLoadLib() {
        println(System.getProperty("user.dir"))
        val shareFileLib = ShareFileLib(8080,"../")

        assert(shareFileLib.startServer())

        Thread.sleep(1000 * 20)

        shareFileLib.stopServer()
    }
}