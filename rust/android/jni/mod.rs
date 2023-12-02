use std::{panic, ptr::null};

use jni::{objects::JObject, AttachGuard, JavaVM, sys::{JNI_VERSION_1_8, jint}};
use libc::c_void;

use crate::NickName;

use self::nick_name::Context;
pub mod nick_name;

static mut JNI_ENV: Option<AttachGuard<'_>> = None;
static mut JAVA_VM: Option<JavaVM> = None;
static mut CONTEXT: Option<Context> = None;

pub fn init(jvm: &'static JavaVM, ctx: Context) -> crate::Result<()> {
    let env = jvm.attach_current_thread().unwrap();
    unsafe { JNI_ENV.replace(env) };
    unsafe { CONTEXT.replace(ctx) };
    Ok(())
}

pub fn attach_current_thread() {
    global_jvm().attach_current_thread().unwrap();
}

fn set_from_android_context() {
  unsafe {
    let ctx = panic::catch_unwind(|| ndk_context::android_context());
    if let Ok(ctx) = ctx {
      let jvm = jni::JavaVM::from_raw(ctx.vm().cast()).unwrap();
      JAVA_VM.replace(jvm);
      let env = JAVA_VM.as_ref().unwrap().attach_current_thread().unwrap();
      let context: JObject<'static> =
          jni::objects::JObject::from_raw(ctx.context().cast());
      JNI_ENV.replace(env);
      CONTEXT.replace(context);
    }
  }
}

pub fn global_jvm() -> &'static JavaVM {
    unsafe {
        match JAVA_VM {
            Some(ref v) => return v,
            None => {
              set_from_android_context();
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
              set_from_android_context();
            }
        }
    };
    unsafe {
        match CONTEXT {
            Some(ref v) => v,
            None => panic!("Context has not been initialized. Please initialize it with init()."),
        }
    }
}

static mut J_NULL_PTR: Option<JObject<'static>> = None;

pub(crate) fn get_ctx_or_null_ptr<'other_local>() -> &'other_local JObject<'other_local>
{
  unsafe {
      match CONTEXT {
          Some(ref v) => return v,
          None => {
            set_from_android_context();
          }
      }
  };
  unsafe {
    match CONTEXT {
      Some(ref v) => v,
      None => match JAVA_VM {
        Some(_) => J_NULL_PTR.get_or_insert_with(|| JObject::null()),
        None => panic!("JVM has not been initialized. Please initialize it with init()."),
      },
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

// https://github.com/pleisto/flappy/blob/main/packages/rust-core/java/src/lib.rs
/// # Safety
///
/// This function could be only called by java vm when loading this lib.
#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(jvm: JavaVM, _: *mut c_void) -> jint {
  println!("JNI_OnLoad");

  JAVA_VM.replace(jvm);
  let env = JAVA_VM.as_ref().unwrap().attach_current_thread().unwrap();
  JNI_ENV.replace(env);
  JNI_VERSION_1_8
}
