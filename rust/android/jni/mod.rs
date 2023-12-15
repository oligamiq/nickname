use jni::{objects::JObject, AttachGuard, JavaVM};

use crate::NickName;

use self::nick_name::Context;
pub mod nick_name;

static mut JNI_ENV: Option<AttachGuard<'_>> = None;
static mut JAVA_VM: Option<JavaVM> = None;
static mut CONTEXT: Option<Context> = None;

pub fn init(jvm: &'static JavaVM, ctx: Context) -> crate::Result<()> {
    let env = jvm.attach_current_thread().unwrap();
    unsafe { JAVA_VM.replace(env.get_java_vm()?) };
    unsafe { JNI_ENV.replace(env) };
    unsafe { CONTEXT.replace(ctx) };
    Ok(())
}

pub fn attach_current_thread() {
    global_jvm().attach_current_thread().unwrap();
}

pub fn global_jvm() -> &'static JavaVM {
    unsafe {
        match JAVA_VM {
            Some(ref v) => return v,
            None => {
                let ctx = ndk_context::android_context();
                let jvm = jni::JavaVM::from_raw(ctx.vm().cast()).unwrap();
                JAVA_VM.replace(jvm);
                let env = JAVA_VM.as_ref().unwrap().attach_current_thread().unwrap();
                let context: JObject<'static> =
                    jni::objects::JObject::from_raw(ctx.context().cast());
                JNI_ENV.replace(env);
                CONTEXT.replace(context);
            }
        }
    };
    unsafe {
        match JAVA_VM {
            Some(ref v) => v,
            None => panic!("JVM has not been initialized. Please initialize it with init()."),
        }
    }
}

// .expect("JVM has not been initialized. Please initialize it with init().")

pub fn global_ctx() -> &'static Context {
    unsafe {
        match CONTEXT {
            Some(ref v) => return v,
            None => {
                let ctx = ndk_context::android_context();
                let jvm = jni::JavaVM::from_raw(ctx.vm().cast()).unwrap();
                JAVA_VM.replace(jvm);
                let env = JAVA_VM.as_ref().unwrap().attach_current_thread().unwrap();
                let context: JObject<'static> =
                    jni::objects::JObject::from_raw(ctx.context().cast());
                JNI_ENV.replace(env);
                CONTEXT.replace(context);
            }
        }
    };
    unsafe {
        match CONTEXT {
            Some(ref v) => v,
            None => panic!("JVM has not been initialized. Please initialize it with init()."),
        }
    }
}

impl Drop for NickName {
    fn drop(&mut self) {
        unsafe {
            if let Some(ref _v) = JNI_ENV {
                JNI_ENV = None;
                JAVA_VM = None;
                CONTEXT = None;
            }
        }
    }
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}
