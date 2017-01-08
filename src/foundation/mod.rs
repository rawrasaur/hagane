use std;
use objc;
use super::ObjectiveC;
use cocoa::*;

#[link(name = "Foundation", kind = "framework")]
extern {}

pub trait NSArray : NSObject {
  fn count(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(count), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn object_at_index<T5: 'static + ObjectiveC>(&self, index: NSUInteger) -> T5 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndex:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T5 = r;

          return result;
        }
      }
    }
  }

  fn object_at_indexed_subscript<T5: 'static + ObjectiveC>(&self, index: NSUInteger) -> T5 where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(objectAtIndexedSubscript:), (index,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: T5 = r;

          return result;
        }
      }
    }
  }

  fn to_vec<T: 'static + ObjectiveC>(&self) -> Vec<T> where Self: 'static + Sized {
    let n = self.count();
    let mut result = Vec::with_capacity(n);
  
    for i in 0 .. n {
      result.push(self.object_at_index::<T>(i));
    }
  
    return result;
  }
}

pub struct NSArrayID(*mut std::os::raw::c_void);

impl NSArrayID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSArrayID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSArrayID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSArrayID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSArray").unwrap();
  }
}

impl NSObject for NSArrayID {}
impl NSArray for NSArrayID {}

impl Clone for NSArrayID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSArrayID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSArrayID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSArrayID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSArrayID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

pub trait NSCoder : NSObject {
}

pub struct NSCoderID(*mut std::os::raw::c_void);

impl NSCoderID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSCoderID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSCoderID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSCoderID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for NSCoderID {}
impl NSCoder for NSCoderID {}

impl Clone for NSCoderID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSCoderID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSCoderID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSCoderID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSCoderID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

pub trait NSError : NSObject {
}

pub struct NSErrorID(*mut std::os::raw::c_void);

impl NSErrorID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSErrorID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSErrorID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSErrorID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSError").unwrap();
  }
}

impl NSObject for NSErrorID {}
impl NSError for NSErrorID {}

impl Clone for NSErrorID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSErrorID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSErrorID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSErrorID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSErrorID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

pub trait NSString : NSObject {
  fn init(self) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(init), ()) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn init_with_bytes_length_encoding(self, bytes: *const std::os::raw::c_void, len: NSUInteger, encoding: NSStringEncoding) -> Self where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(initWithBytes:length:encoding:), (bytes, len, encoding)) {
        Err(s) => panic!("{}", s),
        Ok(result) => {
          std::mem::forget(self);

          return result;
        }
      }
    }
  }

  fn length(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(length), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn length_of_bytes_using_encoding(&self, enc: NSStringEncoding) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(lengthOfBytesUsingEncoding:), (enc,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: NSUInteger = r;

          return result;
        }
      }
    }
  }

  fn maximum_length_of_bytes_using_encoding(&self, enc: NSStringEncoding) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(maximumLengthOfBytesUsingEncoding:), (enc,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: NSUInteger = r;

          return result;
        }
      }
    }
  }

  fn get_c_string_using_encoding(&self, encoding: NSStringEncoding) -> *const std::os::raw::c_char where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(getCStringUsing:encoding:), (encoding,)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: *const std::os::raw::c_char = r;

          return result;
        }
      }
    }
  }

  fn get_c_string_max_length_encoding(&self, buffer: *mut std::os::raw::c_char, max_buffer_count: NSUInteger, encoding: NSStringEncoding) -> bool where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(getCString:maxLength:encoding:), (buffer, max_buffer_count, encoding)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: bool = r;

          return result;
        }
      }
    }
  }

  fn utf8_string(&self) -> *const std::os::raw::c_char where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(UTF8String), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn as_str(&self) -> &str where Self: 'static + Sized {
    let bytes = self.utf8_string() as *const u8;
    let len = self.len();
  
    unsafe {
      let bytes = std::slice::from_raw_parts(bytes, len);
  
      return std::str::from_utf8(bytes).unwrap();
    }
  }

  fn len(&self) -> usize where Self: 'static + Sized {
    return self.length_of_bytes_using_encoding(NSStringEncoding::NSUTF8StringEncoding);
  }
}

pub struct NSStringID(*mut std::os::raw::c_void);

impl NSStringID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSStringID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSStringID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSStringID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }

  pub fn alloc() -> Self {
    return unsafe { msg_send![Self::class(), alloc] };
  }

  pub fn class() -> &'static objc::runtime::Class {
    return objc::runtime::Class::get("NSString").unwrap();
  }

  pub fn from_str(string: &str) -> NSStringID {
    let bytes = string.as_ptr() as *const std::os::raw::c_void;
  
    return Self::alloc().init_with_bytes_length_encoding(bytes, string.len(), NSStringEncoding::NSUTF8StringEncoding);
  }
}

impl NSObject for NSStringID {}
impl NSString for NSStringID {}

impl Clone for NSStringID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSStringID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSStringID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSStringID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSStringID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}

pub trait NSObject : ObjectiveC {
  fn is_equal<T5: 'static + NSObject>(&self, view: T5) -> bool where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(isEqual:), (view.as_ptr(),)) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: bool = r;

          return result;
        }
      }
    }
  }

  fn hash(&self) -> NSUInteger where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      return match objc::__send_message(target, sel!(hash), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => r
      }
    }
  }

  fn description(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(description), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn debug_description(&self) -> NSStringID where Self: 'static + Sized {
    unsafe {
      let target = self.as_object();

      match objc::__send_message(target, sel!(debugDescription), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let r: NSStringID = r;

          return r.retain();
        }
      }
    }
  }

  fn is_proxy(&self) -> bool where Self: 'static + Sized {
    unsafe {
      match objc::__send_message(self.as_object(), sel!(isProxy), ()) {
        Err(s) => panic!("{}", s),
        Ok(r) => {
          let result: bool = r;

          return result;
        }
      }
    }
  }
}

pub struct NSObjectID(*mut std::os::raw::c_void);

impl NSObjectID {
  pub fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSObjectID(ptr);
  }

  pub fn from_object(obj: &mut objc::runtime::Object) -> Self {
    return NSObjectID(obj as *mut objc::runtime::Object as *mut std::os::raw::c_void);
  }

  pub fn nil() -> Self {
    return NSObjectID(0 as *mut std::os::raw::c_void);
  }

  pub fn is_nil(&self) -> bool {
    return self.0 as usize == 0;
  }
}

impl NSObject for NSObjectID {}

impl Clone for NSObjectID {
  fn clone(&self) -> Self {
    let ptr = self.as_ptr();

    return Self::from_ptr(ptr).retain();
  }
}

impl Drop for NSObjectID {
  fn drop(&mut self) {
    if !self.is_nil() {
      self.release();
    }
  }
}

impl ObjectiveC for NSObjectID {
  fn from_ptr(ptr: *mut std::os::raw::c_void) -> Self {
    return NSObjectID::from_ptr(ptr);
  }

  fn as_ptr(&self) -> *mut std::os::raw::c_void {
    return self.0;
  }
}

unsafe impl objc::Encode for NSObjectID {
  fn encode() -> objc::Encoding {
    return unsafe { objc::Encoding::from_str("@") };
  }
}