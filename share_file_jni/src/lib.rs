
use std::sync::OnceLock;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use tokio::task::JoinHandle;

static mut HANDLER: Option<JoinHandle<()>> = None;

#[no_mangle]
pub extern "system" fn Java_com_kouqurong_sharefilelib_ShareFileLib_startServer(
    mut env: JNIEnv,
    _class: JClass,
    port: jint,
    path: JObject,
) -> jboolean {
    let path = JString::from(path);

    let path = env.get_string(&path);

    if let Ok(path) = path {
        let path = path.to_string_lossy().to_string();

        let handler = start_server(port as u16, path);

        if let Ok(handler) = handler {
            unsafe {
                HANDLER = Some(handler);
            }

            return JNI_TRUE;
        }
    }

    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn Java_com_kouqurong_sharefilelib_ShareFileLib_stopServer(
    _env: JNIEnv,
    _class: JClass,
) {
    unsafe {
        if let Some(handler) = HANDLER.take() {
            let _ = handler.abort();
        }
    }
}

static mut ONCE_RT: OnceLock<Option<tokio::runtime::Runtime>> = OnceLock::new();

fn start_server(port: u16, path: String) -> std::io::Result<JoinHandle<()>> {
    let rt = unsafe { ONCE_RT.get_or_init(|| tokio::runtime::Runtime::new().ok()) };

    let rt = rt.as_ref().expect("tokio runtime init failed");

    let handler = rt.spawn(async move {
        println!("port:{} path: {}", port, path);

        let _ = share_file_server::start_server(path, port).await;
    });

    Ok(handler)
}
