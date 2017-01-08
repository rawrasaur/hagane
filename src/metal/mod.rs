use std;
use objc;

use std::os::raw::c_void;

use super::{ObjectiveC, CAMetalDrawable, NSArrayID, NSObject, NSErrorID, NSString, NSStringID};

#[link(name = "Metal", kind = "framework")]
extern {
  fn MTLCopyAllDevices() -> *mut c_void;
  fn MTLCreateSystemDefaultDevice() -> *mut c_void;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64
}

pub enum MTLCompareFunction {
  MTLCompareFunctionNever = 0,
  MTLCompareFunctionLess = 1,
  MTLCompareFunctionEqual = 2,
  MTLCompareFunctionLessEqual = 3,
  MTLCompareFunctionGreater = 4,
  MTLCompareFunctionNotEqual = 5,
  MTLCompareFunctionGreaterEqual = 6,
  MTLCompareFunctionAlways = 7
}

pub enum MTLCPUCacheMode {
  MTLCPUCacheModeDefaultCache = 0,
  MTLCPUCacheModeWriteCombined = 1
}

pub enum MTLCullMode {
  MTLCullModeNone = 0,
  MTLCullModeFront = 1,
  MTLCullModeBack = 2,
}

#[allow(non_camel_case_types)]
pub enum MTLFeatureSet {
  iOS_GPUFamily1_v1           = 00000,
  iOS_GPUFamily1_v2           = 00002,
  iOS_GPUFamily1_v3           = 00005,
  iOS_GPUFamily2_v1           = 00001,
  iOS_GPUFamily2_v2           = 00003,
  iOS_GPUFamily2_v3           = 00006,
  iOS_GPUFamily3_v1           = 00004,
  iOS_GPUFamily3_v2           = 00007,
  OSX_GPUFamily1_v1           = 10000,
  OSX_GPUFamily1_v2           = 10001,
  OSX_ReadWriteTextureTier2   = 10002,
  tvOS_GPUFamily1_v1          = 30000,
  tvOS_GPUFamily1_v2          = 30001
}

#[allow(non_camel_case_types)]
pub enum MTLIndexType {
  MTLIndexTypeUInt16 = 0,
  MTLIndexTypeUInt32 = 1
}

#[allow(non_camel_case_types)]
pub enum MTLPrimitiveType {
  MTLPrimitiveTypePoint = 0,
  MTLPrimitiveTypeLine = 1,
  MTLPrimitiveTypeLineStrip = 2,
  MTLPrimitiveTypeTriangle = 3,
  MTLPrimitiveTypeTriangleStrip = 4
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTLSize {
  pub width: usize,
  pub height: usize,
  pub depth: usize
}

pub enum MTLStorageMode {
  MTLStorageModeShared  = 0,
  MTLStorageModeManaged = 1,
  MTLStorageModePrivate = 2
}

pub enum MTLWinding {
  MTLWindingClockwise = 0,
  MTLWindingCounterClockwise = 1,
}

pub trait MTLBuffer : NSObject {
  forward!(contents, sel!(contents), () -> *mut c_void);
}

id!(MTLBufferID, MTLBuffer);

impl NSObject for MTLBufferID {}

pub trait MTLCommandBuffer : NSObject {
  forward!(commit, sel!(commit), () -> ());
  forward!(present_drawable, sel!(presentDrawable:), (drawable: T) -> (), <T: CAMetalDrawable>);
  forward!(render_command_encoder_with_descriptor, sel!(renderCommandEncoderWithDescriptor:), (render_pass_descriptor: T) -> MTLCommandEncoderID, <T: MTLRenderPassDescriptor>, retain);
}

id!(MTLCommandBufferID, MTLCommandBuffer);

impl NSObject for MTLCommandBufferID {}

pub trait MTLCommandEncoder : NSObject {
  forward!(draw_primitives_vertex_start_vertex_count, sel!(drawPrimitives:vertexStart:vertexCount:), (primitive_type: MTLPrimitiveType, start: usize, count: usize) -> ());
  forward!(draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset, sel!(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:), (primitive_type: MTLPrimitiveType, count: usize, index_type: MTLIndexType, index_buffer: T, offset: usize) -> (), <T: MTLBuffer>);
  forward!(end_encoding, sel!(endEncoding), () -> ());
  forward!(set_cull_mode, sel!(setCullMode:), (mode: MTLCullMode) -> ());
  forward!(set_depth_stencil_state, sel!(setDepthStencilState:), (state: T) -> (), <T: MTLDepthStencilState>);
  forward!(set_front_facing_winding, sel!(setFrontFacingWinding:), (winding: MTLWinding) -> ());
  forward!(set_render_pipeline_state, sel!(setRenderPipelineState:), (pipeline_state: T) -> (), <T: MTLRenderPipelineState>);
  forward!(set_vertex_buffer_offset_at_index, sel!(setVertexBuffer:offset:atIndex:), (buffer: T, offset: usize, index: usize) -> (), <T: MTLBuffer>);
}

id!(MTLCommandEncoderID, MTLCommandEncoder);

impl NSObject for MTLCommandEncoderID {}

pub trait MTLCommandQueue : NSObject {
  forward!(command_buffer, sel!(commandBuffer), () -> MTLCommandBufferID, retain);
}

id!(MTLCommandQueueID, MTLCommandQueue);

impl NSObject for MTLCommandQueueID {}

pub trait MTLDepthStencilDescriptor : NSObject {
  initializer!(init, sel!(init), ());

  forward!(set_depth_compare_function, sel!(setDepthCompareFunction:), (compare_function: MTLCompareFunction) -> ());
  forward!(set_depth_write_enabled, sel!(setDepthWriteEnabled:), (enabled: bool) -> ());
}

id!(MTLDepthStencilDescriptorID, MTLDepthStencilDescriptor, "MTLDepthStencilDescriptor");

impl NSObject for MTLDepthStencilDescriptorID {}

pub trait MTLDepthStencilState : NSObject {
  
}

id!(MTLDepthStencilStateID, MTLDepthStencilState);

impl NSObject for MTLDepthStencilStateID {}

pub trait MTLDevice : NSObject {
  forward!(is_depth24_stencil8_pixel_format_supported, sel!(isDepth24Stencil8PixelFormatSupported), () -> bool);
  forward!(is_headless, sel!(isHeadless), () -> bool);
  forward!(is_low_power, sel!(isLowPower), () -> bool);
  forward!(max_threads_per_threadgroup, sel!(maxThreadsPerThreadgroup), () -> MTLSize);
  forward!(name, sel!(name), () -> NSStringID, retain);
  forward!(new_buffer_with_length_options, sel!(newBufferWithLength:options:), (length: usize, options: usize) -> MTLBufferID);
  forward!(new_buffer_with_bytes_length_options, sel!(newBufferWithBytes:length:options:), (bytes: *const c_void, length: usize, options: usize) -> MTLBufferID);
  forward!(new_command_queue, sel!(newCommandQueue), () -> MTLCommandQueueID);
  forward!(new_depth_stencil_state_with_descriptor, sel!(newDepthStencilStateWithDescriptor:), (descriptor: T) -> MTLDepthStencilStateID, <T: MTLDepthStencilDescriptor>);
  forward!(recommended_max_working_set_size, sel!(recommendedMaxWorkingSetSize), () -> u64);
  forward!(supports_feature_set, sel!(supportsFeatureSet:), (feature_set: MTLFeatureSet) -> bool);
  forward!(supports_texture_sample_count, sel!(supportsTextureSampleCount:), (i: usize) -> bool);

  fn new_library_with_file<T: NSString>(&self, filepath: T) -> Result<MTLLibraryID, NSErrorID> {
    let mut error = NSErrorID::nil();

    unsafe {
      let lib = msg_send![self.as_object(), newLibraryWithFile: filepath error: &mut error];

      if error.is_nil() {
        return Ok(lib);
      } else {
        return Err(error);
      }
    }
  }

  fn new_render_pipeline_state_with_descriptor<T: MTLRenderPipelineDescriptor>(&self, descriptor: T) -> Result<MTLRenderPipelineStateID, NSErrorID> {
    let mut error = NSErrorID::nil();

    unsafe {
      let lib = msg_send![self.as_object(), newRenderPipelineStateWithDescriptor: descriptor error: &mut error];

      if error.is_nil() {
        return Ok(lib);
      } else {
        return Err(error);
      }
    }
  }

  // Rust Helpers

  fn texture_sample_counts(&self) -> Vec<usize> where Self: 'static + Sized {
    let mut result = Vec::new();

    for i in 1 .. 128 {
      if self.supports_texture_sample_count(i) {
        result.push(i);
      }
    }

    return result;
  }
}

id!(MTLDeviceID, MTLDevice);

impl NSObject for MTLDeviceID {}

pub trait MTLFunction : NSObject {

}

id!(MTLFunctionID, MTLFunction);

impl NSObject for MTLFunctionID {}

pub trait MTLLibrary : NSObject {
  forward!(new_function_with_name, sel!(newFunctionWithName:), (name: T) -> MTLFunctionID, <T: NSString>);
}

id!(MTLLibraryID, MTLLibrary);

impl NSObject for MTLLibraryID {}

pub trait MTLRenderPassDescriptor : NSObject {

}

id!(MTLRenderPassDescriptorID, MTLRenderPassDescriptor, "MTLRenderPassDescriptor");

impl NSObject for MTLRenderPassDescriptorID {}

pub trait MTLRenderPipelineColorAttachmentDescriptor : NSObject {
  forward!(set_pixel_format, sel!(setPixelFormat:), (format: usize) -> ());
}

id!(MTLRenderPipelineColorAttachmentDescriptorID, MTLRenderPipelineColorAttachmentDescriptor, "MTLRenderPipelineColorAttachmentDescriptor");

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorID {}

pub trait MTLRenderPipelineColorAttachmentDescriptorArray : NSObject {
  forward!(object_at_indexed_subscript, sel!(objectAtIndexedSubscript:), (i: usize) -> MTLRenderPipelineColorAttachmentDescriptorID, retain);
}

id!(MTLRenderPipelineColorAttachmentDescriptorArrayID, MTLRenderPipelineColorAttachmentDescriptorArray, "MTLRenderPipelineColorAttachmentDescriptorArray");

impl NSObject for MTLRenderPipelineColorAttachmentDescriptorArrayID {}

pub trait MTLRenderPipelineDescriptor : NSObject {
  initializer!(init, sel!(init), ());

  forward!(color_attachments, sel!(colorAttachments), () -> MTLRenderPipelineColorAttachmentDescriptorArrayID, retain);
  forward!(set_fragment_function, sel!(setFragmentFunction:), (function: T) -> (), <T: MTLFunction>);
  forward!(set_vertex_function, sel!(setVertexFunction:), (function: T) -> (), <T: MTLFunction>);
}

id!(MTLRenderPipelineDescriptorID, MTLRenderPipelineDescriptor, "MTLRenderPipelineDescriptor");

impl NSObject for MTLRenderPipelineDescriptorID {}

pub trait MTLRenderPipelineState {

}

id!(MTLRenderPipelineStateID, MTLRenderPipelineState);

impl NSObject for MTLRenderPipelineStateID {}

pub fn all_devices() -> NSArrayID {
  unsafe {
    return NSArrayID::from_ptr(MTLCopyAllDevices());
  }
}

pub fn system_default_device() -> MTLDeviceID {
  unsafe {
    let device = MTLDeviceID::from_ptr(MTLCreateSystemDefaultDevice());

    return device.retain();
  }
}
