use std::sync::OnceLock;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

struct ShareFileLib {
    handler: JoinHandle<()>,
    tx: oneshot::Sender<()>,
}

#[no_mangle]
pub extern "system" fn Java_com_kouqurong_sharefilelib_ShareFileLib_startServer(
    mut env: JNIEnv,
    _class: JClass,
    port: jint,
    path: JObject,
) -> jlong {
    let path = JString::from(path);

    let path = env.get_string(&path);

    if let Ok(path) = path {
        let path = path.to_string_lossy().to_string();

        let share_file_lib = start_server(port as u16, path);

        if let Ok(share_file_lib) = share_file_lib {
            return Box::into_raw(Box::new(share_file_lib)) as jlong;
        }
    }

    -1
}

#[no_mangle]
pub extern "system" fn Java_com_kouqurong_sharefilelib_ShareFileLib_stopServer(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    let share_file_lib = ptr as *mut ShareFileLib;

    if !share_file_lib.is_null() {
        let share_file_lib = unsafe { Box::from_raw(share_file_lib) };

        let _ = share_file_lib.tx.send(());
    }
}

static ONCE_RT: OnceLock<Option<tokio::runtime::Runtime>> = OnceLock::new();

fn start_server(port: u16, path: String) -> std::io::Result<ShareFileLib> {
    let rt = ONCE_RT.get_or_init(|| tokio::runtime::Runtime::new().ok());

    let rt = rt.as_ref().expect("tokio runtime init failed");

    let (tx, rx) = oneshot::channel::<()>();

    let handler = rt.spawn(async move {
        println!("port:{} path: {}", port, path);

        let _ = share_file_server::start_server(path, port, async {
            rx.await.expect("failed to install CTRL+C signal handler");
        })
        .await;
    });

    Ok(ShareFileLib { handler, tx })
}
