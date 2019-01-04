
        mod __gl_imports {
            pub use std::mem;
            pub use std::marker::Send;
            pub use std::os::raw;
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = extern "system" fn(source: GLenum,
                                          gltype: GLenum,
                                          id: GLuint,
                                          severity: GLenum,
                                          length: GLsizei,
                                          message: *const GLchar,
                                          userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum,
                                             gltype: GLenum,
                                             id: GLuint,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint,
                                             category: GLenum,
                                             severity: GLenum,
                                             length: GLsizei,
                                             message: *const GLchar,
                                             userParam: *mut super::__gl_imports::raw::c_void);
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

}
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_PROGRAM: types::GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_RESOURCES: types::GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINES: types::GLenum = 0x8DE5;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_MAX_LENGTH: types::GLenum = 0x8E48;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORMS: types::GLenum = 0x8DE6;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8E47;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8E49;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_VARIABLES: types::GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_BARRIER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_SHADER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED_CONSERVATIVE: types::GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_SIZE: types::GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_STRIDE: types::GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BARRIER_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER: types::GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: types::GLenum = 0x92C5;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: types::GLenum = 0x92C6;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_BINDING: types::GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: types::GLenum = 0x92C4;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90ED;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x92CB;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x92CA;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x92C8;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x92C9;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x92C7;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)] pub const ATOMIC_COUNTER_BUFFER_START: types::GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const AUTO_GENERATE_MIPMAP: types::GLenum = 0x8295;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA_INTEGER: types::GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)] pub const BGR_INTEGER: types::GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLOCK_INDEX: types::GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_INTEGER: types::GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_BINDING: types::GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_DATA_SIZE: types::GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_IMMUTABLE_STORAGE: types::GLenum = 0x821F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_STORAGE_FLAGS: types::GLenum = 0x8220;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_UPDATE_BARRIER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_VARIABLE: types::GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CAVEAT_SUPPORT: types::GLenum = 0x82B8;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_READ_COLOR: types::GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_BUFFER: types::GLenum = 0x82B4;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR_TEXTURE: types::GLenum = 0x9365;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_STORAGE_BIT: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DEPTH_MODE: types::GLenum = 0x935D;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE6: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE7: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_ORIGIN: types::GLenum = 0x935C;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_COMPONENTS: types::GLenum = 0x8283;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ENCODING: types::GLenum = 0x8296;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_RENDERABLE: types::GLenum = 0x8286;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMMAND_BARRIER_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4B;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_R11_EAC: types::GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED: types::GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED_RGTC1: types::GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG: types::GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG11_EAC: types::GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_ETC2: types::GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA8_ETC2_EAC: types::GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_BPTC_UNORM: types::GLenum = 0x8E8C;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: types::GLenum = 0x8E8E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: types::GLenum = 0x8E8F;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG_RGTC2: types::GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_R11_EAC: types::GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RED_RGTC1: types::GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG11_EAC: types::GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG_RGTC2: types::GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB: types::GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: types::GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_ETC2: types::GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA: types::GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: types::GLenum = 0x8E8D;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER: types::GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SHADER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE: types::GLenum = 0x92ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_SUBROUTINE_UNIFORM: types::GLenum = 0x92F3;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_TEXTURE: types::GLenum = 0x82A0;
#[allow(dead_code, non_upper_case_globals)] pub const COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_LOST: types::GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_PROFILE_MASK: types::GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR: types::GLenum = 0x82FB;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: types::GLenum = 0x82FC;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER_BINDING: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER_BINDING: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLAMP: types::GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32: types::GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENTS: types::GLenum = 0x8284;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RENDERABLE: types::GLenum = 0x8287;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_TEXTURE_MODE: types::GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER: types::GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)] pub const DISPATCH_INDIRECT_BUFFER_BINDING: types::GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)] pub const DISPLAY_LIST: types::GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2: types::GLenum = 0x8F46;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x3: types::GLenum = 0x8F49;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT2x4: types::GLenum = 0x8F4A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3: types::GLenum = 0x8F47;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x2: types::GLenum = 0x8F4B;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT3x4: types::GLenum = 0x8F4C;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4: types::GLenum = 0x8F48;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x2: types::GLenum = 0x8F4D;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_MAT4x3: types::GLenum = 0x8F4E;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC2: types::GLenum = 0x8FFC;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC3: types::GLenum = 0x8FFD;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE_VEC4: types::GLenum = 0x8FFE;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER: types::GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_INDIRECT_BUFFER_BINDING: types::GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_STORAGE_BIT: types::GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BARRIER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FILTER: types::GLenum = 0x829A;
#[allow(dead_code, non_upper_case_globals)] pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED_ONLY: types::GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_EVEN: types::GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)] pub const FRACTIONAL_ODD: types::GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: types::GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE: types::GLenum = 0x92EC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SUBROUTINE_UNIFORM: types::GLenum = 0x92F2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_TEXTURE: types::GLenum = 0x829F;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BARRIER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BLEND: types::GLenum = 0x828B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_HEIGHT: types::GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_LAYERS: types::GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_SAMPLES: types::GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT_WIDTH: types::GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: types::GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: types::GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE: types::GLenum = 0x8289;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_RENDERABLE_LAYERED: types::GLenum = 0x828A;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_SRGB: types::GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const FULL_SUPPORT: types::GLenum = 0x82B7;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE: types::GLenum = 0x92EB;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SUBROUTINE_UNIFORM: types::GLenum = 0x92F1;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_TEXTURE: types::GLenum = 0x829E;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_FORMAT: types::GLenum = 0x8291;
#[allow(dead_code, non_upper_case_globals)] pub const GET_TEXTURE_IMAGE_TYPE: types::GLenum = 0x8292;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_INTEGER: types::GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)] pub const GUILTY_CONTEXT_RESET: types::GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)] pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_FLOAT: types::GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_INT: types::GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D: types::GLenum = 0x904C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_1D_ARRAY: types::GLenum = 0x9052;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D: types::GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_ARRAY: types::GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9055;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9056;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_2D_RECT: types::GLenum = 0x904F;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_3D: types::GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_ACCESS: types::GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_FORMAT: types::GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYER: types::GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LAYERED: types::GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_LEVEL: types::GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BINDING_NAME: types::GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_BUFFER: types::GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_10_10_10_2: types::GLenum = 0x82C3;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_11_11_10: types::GLenum = 0x82C2;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_16: types::GLenum = 0x82BE;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_32: types::GLenum = 0x82BB;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_1_X_8: types::GLenum = 0x82C1;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_16: types::GLenum = 0x82BD;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_32: types::GLenum = 0x82BA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_2_X_8: types::GLenum = 0x82C0;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_16: types::GLenum = 0x82BC;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_32: types::GLenum = 0x82B9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CLASS_4_X_8: types::GLenum = 0x82BF;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_COMPATIBILITY_CLASS: types::GLenum = 0x82A8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE: types::GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: types::GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: types::GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: types::GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_FORMAT: types::GLenum = 0x82A9;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_PIXEL_TYPE: types::GLenum = 0x82AA;
#[allow(dead_code, non_upper_case_globals)] pub const IMAGE_TEXEL_SIZE: types::GLenum = 0x82A7;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_FORMAT: types::GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_TYPE: types::GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INNOCENT_CONTEXT_RESET: types::GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_SIZE: types::GLenum = 0x8274;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_ALPHA_TYPE: types::GLenum = 0x827B;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_SIZE: types::GLenum = 0x8273;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_BLUE_TYPE: types::GLenum = 0x827A;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_SIZE: types::GLenum = 0x8275;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_DEPTH_TYPE: types::GLenum = 0x827C;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_SIZE: types::GLenum = 0x8272;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_GREEN_TYPE: types::GLenum = 0x8279;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_PREFERRED: types::GLenum = 0x8270;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_SIZE: types::GLenum = 0x8271;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_RED_TYPE: types::GLenum = 0x8278;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SHARED_SIZE: types::GLenum = 0x8277;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_SIZE: types::GLenum = 0x8276;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_STENCIL_TYPE: types::GLenum = 0x827D;
#[allow(dead_code, non_upper_case_globals)] pub const INTERNALFORMAT_SUPPORTED: types::GLenum = 0x826F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D: types::GLenum = 0x9057;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_1D_ARRAY: types::GLenum = 0x905D;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D: types::GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_ARRAY: types::GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9060;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9061;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_2D_RECT: types::GLenum = 0x905A;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_3D: types::GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_BUFFER: types::GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE: types::GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)] pub const INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D: types::GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_RECT: types::GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const ISOLINES: types::GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)] pub const IS_PER_PATCH: types::GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)] pub const IS_ROW_MAJOR: types::GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)] pub const LAYER_PROVOKING_VERTEX: types::GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION: types::GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_COMPONENT: types::GLenum = 0x934A;
#[allow(dead_code, non_upper_case_globals)] pub const LOCATION_INDEX: types::GLenum = 0x930F;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOSE_CONTEXT_ON_RESET: types::GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)] pub const LOWER_LEFT: types::GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_FLOAT: types::GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_INT: types::GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)] pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)] pub const MANUAL_GENERATE_MIPMAP: types::GLenum = 0x8294;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_COHERENT_BIT: types::GLenum = 0x0080;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_PERSISTENT_BIT: types::GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_STRIDE: types::GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: types::GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_DISTANCES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTERS: types::GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: types::GLenum = 0x82FA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_DIMENSIONS: types::GLenum = 0x8282;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNIFORMS: types::GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTERS: types::GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_IMAGE_UNIFORMS: types::GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: types::GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: types::GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_BLOCKS: types::GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_COUNT: types::GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: types::GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CULL_DISTANCES: types::GLenum = 0x82F9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH: types::GLenum = 0x8280;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: types::GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENT_INDEX: types::GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTERS: types::GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_IMAGE_UNIFORMS: types::GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_VECTORS: types::GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_HEIGHT: types::GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_LAYERS: types::GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_SAMPLES: types::GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAMEBUFFER_WIDTH: types::GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTERS: types::GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_IMAGE_UNIFORMS: types::GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_HEIGHT: types::GLenum = 0x827F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_SAMPLES: types::GLenum = 0x906D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_IMAGE_UNITS: types::GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LAYERS: types::GLenum = 0x8281;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_LENGTH: types::GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_ACTIVE_VARIABLES: types::GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x92F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PATCH_VERTICES: types::GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RECTANGLE_TEXTURE_SIZE: types::GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BLOCK_SIZE: types::GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: types::GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINES: types::GLenum = 0x8DE7;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8DE8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: types::GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: types::GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: types::GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: types::GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: types::GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: types::GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: types::GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: types::GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: types::GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: types::GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_GEN_LEVEL: types::GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TESS_PATCH_COMPONENTS: types::GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: types::GLenum = 0x8E70;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_LOCATIONS: types::GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_FLOATS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_VECTORS: types::GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTERS: types::GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_BINDINGS: types::GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIB_STRIDE: types::GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_IMAGE_UNIFORMS: types::GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_STREAMS: types::GLenum = 0x8E71;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_VECTORS: types::GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORTS: types::GLenum = 0x825B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_WIDTH: types::GLenum = 0x827E;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_FLOAT: types::GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_INT: types::GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)] pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)] pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_MAP_BUFFER_ALIGNMENT: types::GLenum = 0x90BC;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_SAMPLE_SHADING_VALUE: types::GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)] pub const MIPMAP: types::GLenum = 0x8293;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const MIRROR_CLAMP_TO_EDGE: types::GLenum = 0x8743;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_LENGTH: types::GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEGATIVE_ONE_TO_ONE: types::GLenum = 0x935E;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NO_RESET_NOTIFICATION: types::GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_ACTIVE_VARIABLES: types::GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4A;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SAMPLE_COUNTS: types::GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADER_BINARY_FORMATS: types::GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADING_LANGUAGE_VERSIONS: types::GLenum = 0x82E9;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)] pub const OFFSET: types::GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_ALPHA: types::GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_COLOR: types::GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x912D;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x912C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912E;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x912B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_IMAGE_HEIGHT: types::GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_IMAGES: types::GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)] pub const PATCHES: types::GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_INNER_LEVEL: types::GLenum = 0x8E73;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_DEFAULT_OUTER_LEVEL: types::GLenum = 0x8E74;
#[allow(dead_code, non_upper_case_globals)] pub const PATCH_VERTICES: types::GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_BUFFER_BARRIER_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SPRITE_COORD_ORIGIN: types::GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART: types::GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FIXED_INDEX: types::GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: types::GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_INDEX: types::GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_LENGTH: types::GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_BINARY_RETRIEVABLE_HINT: types::GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_INPUT: types::GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_OUTPUT: types::GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE_BINDING: types::GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_SEPARABLE: types::GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)] pub const PROVOKING_VERTEX: types::GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D_ARRAY: types::GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_ARRAY: types::GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x900B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_RECTANGLE: types::GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: types::GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER: types::GLenum = 0x9192;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BARRIER_BIT: types::GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BUFFER_BINDING: types::GLenum = 0x9193;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT: types::GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT_INVERTED: types::GLenum = 0x8E1A;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT: types::GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT_INVERTED: types::GLenum = 0x8E19;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_COUNTER_BITS: types::GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT: types::GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT_INVERTED: types::GLenum = 0x8E18;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_NO_WAIT: types::GLenum = 0x9194;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_TARGET: types::GLenum = 0x82EA;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT: types::GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT_INVERTED: types::GLenum = 0x8E17;
#[allow(dead_code, non_upper_case_globals)] pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)] pub const R16: types::GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)] pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)] pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)] pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)] pub const R16_SNORM: types::GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)] pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)] pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)] pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)] pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)] pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)] pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)] pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)] pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS: types::GLenum = 0x828C;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_FORMAT: types::GLenum = 0x828D;
#[allow(dead_code, non_upper_case_globals)] pub const READ_PIXELS_TYPE: types::GLenum = 0x828E;
#[allow(dead_code, non_upper_case_globals)] pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)] pub const REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESET_NOTIFICATION_STRATEGY: types::GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)] pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)] pub const RG16: types::GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)] pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)] pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)] pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)] pub const RG16_SNORM: types::GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)] pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)] pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)] pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)] pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)] pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)] pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)] pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16_SNORM: types::GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)] pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16_SNORM: types::GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)] pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D: types::GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY: types::GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY_SHADOW: types::GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_SHADOW: types::GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT: types::GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT_SHADOW: types::GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: types::GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES_PASSED: types::GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_SHADING: types::GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_BINARY_FORMATS: types::GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_COMPILER: types::GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_ATOMIC: types::GLenum = 0x82A6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_LOAD: types::GLenum = 0x82A4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_IMAGE_STORE: types::GLenum = 0x82A5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BARRIER_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BLOCK: types::GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER: types::GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_BINDING: types::GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_SIZE: types::GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_STORAGE_BUFFER_START: types::GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: types::GLenum = 0x82AC;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: types::GLenum = 0x82AE;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: types::GLenum = 0x82AD;
#[allow(dead_code, non_upper_case_globals)] pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: types::GLenum = 0x82AF;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_COLOR: types::GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_ALPHA: types::GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_READ: types::GLenum = 0x8297;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_WRITE: types::GLenum = 0x8298;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_COMPONENTS: types::GLenum = 0x8285;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX1: types::GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX16: types::GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX4: types::GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_RENDERABLE: types::GLenum = 0x8288;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: types::GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_OUTPUT_VERTICES: types::GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER: types::GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SHADER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE: types::GLenum = 0x92E9;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_SUBROUTINE_UNIFORM: types::GLenum = 0x92EF;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_CONTROL_TEXTURE: types::GLenum = 0x829C;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER: types::GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SHADER_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE: types::GLenum = 0x92EA;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: types::GLenum = 0x92F0;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_EVALUATION_TEXTURE: types::GLenum = 0x829D;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_MODE: types::GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_POINT_MODE: types::GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_SPACING: types::GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)] pub const TESS_GEN_VERTEX_ORDER: types::GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D_ARRAY: types::GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D_ARRAY: types::GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: types::GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_RECTANGLE: types::GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_BINDING: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET: types::GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_SIZE: types::GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x82B2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x82B3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x82B1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_SEAMLESS: types::GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FETCH_BARRIER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER: types::GLenum = 0x82A2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GATHER_SHADOW: types::GLenum = 0x82A3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_FORMAT: types::GLenum = 0x828F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMAGE_TYPE: types::GLenum = 0x8290;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_FORMAT: types::GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_IMMUTABLE_LEVELS: types::GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LOD_BIAS: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RECTANGLE: types::GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHADOW: types::GLenum = 0x82A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_RGBA: types::GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_TARGET: types::GLenum = 0x1006;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_UPDATE_BARRIER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW: types::GLenum = 0x82B5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LAYER: types::GLenum = 0x82DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_MIN_LEVEL: types::GLenum = 0x82DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LAYERS: types::GLenum = 0x82DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_VIEW_NUM_LEVELS: types::GLenum = 0x82DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const TIMESTAMP: types::GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)] pub const TIME_ELAPSED: types::GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_SIZE: types::GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)] pub const TOP_LEVEL_ARRAY_STRIDE: types::GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK: types::GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BARRIER_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BINDING: types::GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: types::GLenum = 0x934B;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: types::GLenum = 0x934C;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING: types::GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const TYPE: types::GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNDEFINED_VERTEX: types::GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM: types::GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x92DA;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BARRIER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK: types::GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90EC;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x84F0;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x84F1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)] pub const UNKNOWN_CONTEXT_RESET: types::GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x9129;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x9128;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912A;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x9127;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_ATOMIC_COUNTER: types::GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D: types::GLenum = 0x9062;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_1D_ARRAY: types::GLenum = 0x9068;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D: types::GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_ARRAY: types::GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x906B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x906C;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_2D_RECT: types::GLenum = 0x9065;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_3D: types::GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_BUFFER: types::GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE: types::GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D: types::GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_RECT: types::GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)] pub const UPPER_LEFT: types::GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_LONG: types::GLenum = 0x874E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_BINDING: types::GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_BUFFER: types::GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_DIVISOR: types::GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_OFFSET: types::GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_BINDING_STRIDE: types::GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE: types::GLenum = 0x92E8;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SUBROUTINE_UNIFORM: types::GLenum = 0x92EE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_TEXTURE: types::GLenum = 0x829B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_BOUNDS_RANGE: types::GLenum = 0x825D;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_INDEX_PROVOKING_VERTEX: types::GLenum = 0x825F;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_SUBPIXEL_BITS: types::GLenum = 0x825C;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_128_BITS: types::GLenum = 0x82C4;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_16_BITS: types::GLenum = 0x82CA;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_24_BITS: types::GLenum = 0x82C9;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_32_BITS: types::GLenum = 0x82C8;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_48_BITS: types::GLenum = 0x82C7;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_64_BITS: types::GLenum = 0x82C6;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_8_BITS: types::GLenum = 0x82CB;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_96_BITS: types::GLenum = 0x82C5;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_FLOAT: types::GLenum = 0x82D3;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_BPTC_UNORM: types::GLenum = 0x82D2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC1_RED: types::GLenum = 0x82D0;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_RGTC2_RG: types::GLenum = 0x82D1;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGB: types::GLenum = 0x82CC;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT1_RGBA: types::GLenum = 0x82CD;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT3_RGBA: types::GLenum = 0x82CE;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_CLASS_S3TC_DXT5_RGBA: types::GLenum = 0x82CF;
#[allow(dead_code, non_upper_case_globals)] pub const VIEW_COMPATIBILITY_CLASS: types::GLenum = 0x82B6;
#[allow(dead_code, non_upper_case_globals)] pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO_TO_ONE: types::GLenum = 0x935F;

        #[allow(dead_code, missing_copy_implementations)]
        #[derive(Clone)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr {
                        f: missing_fn_panic as *const __gl_imports::raw::c_void,
                        is_loaded: false
                    }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {
                self.is_loaded
            }
        }
    
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gl function was not loaded")
        }

        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone)]
        pub struct GlFnPtrs {
pub ActiveShaderProgram: FnPtr,
/// Fallbacks: ActiveTextureARB
pub ActiveTexture: FnPtr,
/// Fallbacks: AttachObjectARB
pub AttachShader: FnPtr,
/// Fallbacks: BeginConditionalRenderNV
pub BeginConditionalRender: FnPtr,
/// Fallbacks: BeginQueryARB
pub BeginQuery: FnPtr,
pub BeginQueryIndexed: FnPtr,
/// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
pub BeginTransformFeedback: FnPtr,
/// Fallbacks: BindAttribLocationARB
pub BindAttribLocation: FnPtr,
/// Fallbacks: BindBufferARB
pub BindBuffer: FnPtr,
/// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
pub BindBufferBase: FnPtr,
/// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
pub BindBufferRange: FnPtr,
pub BindBuffersBase: FnPtr,
pub BindBuffersRange: FnPtr,
/// Fallbacks: BindFragDataLocationEXT
pub BindFragDataLocation: FnPtr,
/// Fallbacks: BindFragDataLocationIndexedEXT
pub BindFragDataLocationIndexed: FnPtr,
pub BindFramebuffer: FnPtr,
pub BindImageTexture: FnPtr,
pub BindImageTextures: FnPtr,
pub BindProgramPipeline: FnPtr,
pub BindRenderbuffer: FnPtr,
pub BindSampler: FnPtr,
pub BindSamplers: FnPtr,
/// Fallbacks: BindTextureEXT
pub BindTexture: FnPtr,
pub BindTextureUnit: FnPtr,
pub BindTextures: FnPtr,
pub BindTransformFeedback: FnPtr,
/// Fallbacks: BindVertexArrayOES
pub BindVertexArray: FnPtr,
pub BindVertexBuffer: FnPtr,
pub BindVertexBuffers: FnPtr,
/// Fallbacks: BlendColorEXT
pub BlendColor: FnPtr,
/// Fallbacks: BlendEquationEXT
pub BlendEquation: FnPtr,
/// Fallbacks: BlendEquationSeparateEXT
pub BlendEquationSeparate: FnPtr,
/// Fallbacks: BlendEquationSeparateIndexedAMD, BlendEquationSeparateiARB, BlendEquationSeparateiEXT, BlendEquationSeparateiOES
pub BlendEquationSeparatei: FnPtr,
/// Fallbacks: BlendEquationIndexedAMD, BlendEquationiARB, BlendEquationiEXT, BlendEquationiOES
pub BlendEquationi: FnPtr,
pub BlendFunc: FnPtr,
/// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
pub BlendFuncSeparate: FnPtr,
/// Fallbacks: BlendFuncSeparateIndexedAMD, BlendFuncSeparateiARB, BlendFuncSeparateiEXT, BlendFuncSeparateiOES
pub BlendFuncSeparatei: FnPtr,
/// Fallbacks: BlendFuncIndexedAMD, BlendFunciARB, BlendFunciEXT, BlendFunciOES
pub BlendFunci: FnPtr,
/// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
pub BlitFramebuffer: FnPtr,
pub BlitNamedFramebuffer: FnPtr,
/// Fallbacks: BufferDataARB
pub BufferData: FnPtr,
/// Fallbacks: BufferStorageEXT
pub BufferStorage: FnPtr,
/// Fallbacks: BufferSubDataARB
pub BufferSubData: FnPtr,
/// Fallbacks: CheckFramebufferStatusEXT
pub CheckFramebufferStatus: FnPtr,
pub CheckNamedFramebufferStatus: FnPtr,
/// Fallbacks: ClampColorARB
pub ClampColor: FnPtr,
pub Clear: FnPtr,
pub ClearBufferData: FnPtr,
pub ClearBufferSubData: FnPtr,
pub ClearBufferfi: FnPtr,
pub ClearBufferfv: FnPtr,
pub ClearBufferiv: FnPtr,
pub ClearBufferuiv: FnPtr,
pub ClearColor: FnPtr,
pub ClearDepth: FnPtr,
/// Fallbacks: ClearDepthfOES
pub ClearDepthf: FnPtr,
pub ClearNamedBufferData: FnPtr,
pub ClearNamedBufferSubData: FnPtr,
pub ClearNamedFramebufferfi: FnPtr,
pub ClearNamedFramebufferfv: FnPtr,
pub ClearNamedFramebufferiv: FnPtr,
pub ClearNamedFramebufferuiv: FnPtr,
pub ClearStencil: FnPtr,
/// Fallbacks: ClearTexImageEXT
pub ClearTexImage: FnPtr,
/// Fallbacks: ClearTexSubImageEXT
pub ClearTexSubImage: FnPtr,
/// Fallbacks: ClientWaitSyncAPPLE
pub ClientWaitSync: FnPtr,
/// Fallbacks: ClipControlEXT
pub ClipControl: FnPtr,
pub ColorMask: FnPtr,
/// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
pub ColorMaski: FnPtr,
pub ColorP3ui: FnPtr,
pub ColorP3uiv: FnPtr,
pub ColorP4ui: FnPtr,
pub ColorP4uiv: FnPtr,
/// Fallbacks: CompileShaderARB
pub CompileShader: FnPtr,
/// Fallbacks: CompressedTexImage1DARB
pub CompressedTexImage1D: FnPtr,
/// Fallbacks: CompressedTexImage2DARB
pub CompressedTexImage2D: FnPtr,
/// Fallbacks: CompressedTexImage3DARB
pub CompressedTexImage3D: FnPtr,
/// Fallbacks: CompressedTexSubImage1DARB
pub CompressedTexSubImage1D: FnPtr,
/// Fallbacks: CompressedTexSubImage2DARB
pub CompressedTexSubImage2D: FnPtr,
/// Fallbacks: CompressedTexSubImage3DARB
pub CompressedTexSubImage3D: FnPtr,
pub CompressedTextureSubImage1D: FnPtr,
pub CompressedTextureSubImage2D: FnPtr,
pub CompressedTextureSubImage3D: FnPtr,
/// Fallbacks: CopyBufferSubDataNV
pub CopyBufferSubData: FnPtr,
/// Fallbacks: CopyImageSubDataEXT, CopyImageSubDataOES
pub CopyImageSubData: FnPtr,
pub CopyNamedBufferSubData: FnPtr,
/// Fallbacks: CopyTexImage1DEXT
pub CopyTexImage1D: FnPtr,
/// Fallbacks: CopyTexImage2DEXT
pub CopyTexImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage1DEXT
pub CopyTexSubImage1D: FnPtr,
/// Fallbacks: CopyTexSubImage2DEXT
pub CopyTexSubImage2D: FnPtr,
/// Fallbacks: CopyTexSubImage3DEXT
pub CopyTexSubImage3D: FnPtr,
pub CopyTextureSubImage1D: FnPtr,
pub CopyTextureSubImage2D: FnPtr,
pub CopyTextureSubImage3D: FnPtr,
pub CreateBuffers: FnPtr,
pub CreateFramebuffers: FnPtr,
/// Fallbacks: CreateProgramObjectARB
pub CreateProgram: FnPtr,
pub CreateProgramPipelines: FnPtr,
pub CreateQueries: FnPtr,
pub CreateRenderbuffers: FnPtr,
pub CreateSamplers: FnPtr,
/// Fallbacks: CreateShaderObjectARB
pub CreateShader: FnPtr,
pub CreateShaderProgramv: FnPtr,
pub CreateTextures: FnPtr,
pub CreateTransformFeedbacks: FnPtr,
pub CreateVertexArrays: FnPtr,
pub CullFace: FnPtr,
/// Fallbacks: DebugMessageCallbackARB, DebugMessageCallbackKHR
pub DebugMessageCallback: FnPtr,
/// Fallbacks: DebugMessageControlARB, DebugMessageControlKHR
pub DebugMessageControl: FnPtr,
/// Fallbacks: DebugMessageInsertARB, DebugMessageInsertKHR
pub DebugMessageInsert: FnPtr,
/// Fallbacks: DeleteBuffersARB
pub DeleteBuffers: FnPtr,
/// Fallbacks: DeleteFramebuffersEXT
pub DeleteFramebuffers: FnPtr,
pub DeleteProgram: FnPtr,
pub DeleteProgramPipelines: FnPtr,
/// Fallbacks: DeleteQueriesARB
pub DeleteQueries: FnPtr,
/// Fallbacks: DeleteRenderbuffersEXT
pub DeleteRenderbuffers: FnPtr,
pub DeleteSamplers: FnPtr,
pub DeleteShader: FnPtr,
/// Fallbacks: DeleteSyncAPPLE
pub DeleteSync: FnPtr,
pub DeleteTextures: FnPtr,
/// Fallbacks: DeleteTransformFeedbacksNV
pub DeleteTransformFeedbacks: FnPtr,
/// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
pub DeleteVertexArrays: FnPtr,
pub DepthFunc: FnPtr,
pub DepthMask: FnPtr,
pub DepthRange: FnPtr,
pub DepthRangeArrayv: FnPtr,
pub DepthRangeIndexed: FnPtr,
/// Fallbacks: DepthRangefOES
pub DepthRangef: FnPtr,
/// Fallbacks: DetachObjectARB
pub DetachShader: FnPtr,
pub Disable: FnPtr,
pub DisableVertexArrayAttrib: FnPtr,
/// Fallbacks: DisableVertexAttribArrayARB
pub DisableVertexAttribArray: FnPtr,
/// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
pub Disablei: FnPtr,
pub DispatchCompute: FnPtr,
pub DispatchComputeIndirect: FnPtr,
/// Fallbacks: DrawArraysEXT
pub DrawArrays: FnPtr,
pub DrawArraysIndirect: FnPtr,
/// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
pub DrawArraysInstanced: FnPtr,
/// Fallbacks: DrawArraysInstancedBaseInstanceEXT
pub DrawArraysInstancedBaseInstance: FnPtr,
pub DrawBuffer: FnPtr,
/// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
pub DrawBuffers: FnPtr,
pub DrawElements: FnPtr,
/// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
pub DrawElementsBaseVertex: FnPtr,
pub DrawElementsIndirect: FnPtr,
/// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
pub DrawElementsInstanced: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseInstanceEXT
pub DrawElementsInstancedBaseInstance: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
pub DrawElementsInstancedBaseVertex: FnPtr,
/// Fallbacks: DrawElementsInstancedBaseVertexBaseInstanceEXT
pub DrawElementsInstancedBaseVertexBaseInstance: FnPtr,
/// Fallbacks: DrawRangeElementsEXT
pub DrawRangeElements: FnPtr,
/// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
pub DrawRangeElementsBaseVertex: FnPtr,
/// Fallbacks: DrawTransformFeedbackEXT, DrawTransformFeedbackNV
pub DrawTransformFeedback: FnPtr,
/// Fallbacks: DrawTransformFeedbackInstancedEXT
pub DrawTransformFeedbackInstanced: FnPtr,
pub DrawTransformFeedbackStream: FnPtr,
pub DrawTransformFeedbackStreamInstanced: FnPtr,
pub Enable: FnPtr,
pub EnableVertexArrayAttrib: FnPtr,
/// Fallbacks: EnableVertexAttribArrayARB
pub EnableVertexAttribArray: FnPtr,
/// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
pub Enablei: FnPtr,
/// Fallbacks: EndConditionalRenderNV, EndConditionalRenderNVX
pub EndConditionalRender: FnPtr,
/// Fallbacks: EndQueryARB
pub EndQuery: FnPtr,
pub EndQueryIndexed: FnPtr,
/// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
pub EndTransformFeedback: FnPtr,
/// Fallbacks: FenceSyncAPPLE
pub FenceSync: FnPtr,
pub Finish: FnPtr,
pub Flush: FnPtr,
/// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
pub FlushMappedBufferRange: FnPtr,
pub FlushMappedNamedBufferRange: FnPtr,
pub FramebufferParameteri: FnPtr,
/// Fallbacks: FramebufferRenderbufferEXT
pub FramebufferRenderbuffer: FnPtr,
/// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
pub FramebufferTexture: FnPtr,
/// Fallbacks: FramebufferTexture1DEXT
pub FramebufferTexture1D: FnPtr,
/// Fallbacks: FramebufferTexture2DEXT
pub FramebufferTexture2D: FnPtr,
/// Fallbacks: FramebufferTexture3DEXT
pub FramebufferTexture3D: FnPtr,
/// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
pub FramebufferTextureLayer: FnPtr,
pub FrontFace: FnPtr,
/// Fallbacks: GenBuffersARB
pub GenBuffers: FnPtr,
/// Fallbacks: GenFramebuffersEXT
pub GenFramebuffers: FnPtr,
pub GenProgramPipelines: FnPtr,
/// Fallbacks: GenQueriesARB
pub GenQueries: FnPtr,
/// Fallbacks: GenRenderbuffersEXT
pub GenRenderbuffers: FnPtr,
pub GenSamplers: FnPtr,
pub GenTextures: FnPtr,
/// Fallbacks: GenTransformFeedbacksNV
pub GenTransformFeedbacks: FnPtr,
/// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
pub GenVertexArrays: FnPtr,
/// Fallbacks: GenerateMipmapEXT
pub GenerateMipmap: FnPtr,
pub GenerateTextureMipmap: FnPtr,
pub GetActiveAtomicCounterBufferiv: FnPtr,
/// Fallbacks: GetActiveAttribARB
pub GetActiveAttrib: FnPtr,
pub GetActiveSubroutineName: FnPtr,
pub GetActiveSubroutineUniformName: FnPtr,
pub GetActiveSubroutineUniformiv: FnPtr,
/// Fallbacks: GetActiveUniformARB
pub GetActiveUniform: FnPtr,
pub GetActiveUniformBlockName: FnPtr,
pub GetActiveUniformBlockiv: FnPtr,
pub GetActiveUniformName: FnPtr,
pub GetActiveUniformsiv: FnPtr,
pub GetAttachedShaders: FnPtr,
/// Fallbacks: GetAttribLocationARB
pub GetAttribLocation: FnPtr,
/// Fallbacks: GetBooleanIndexedvEXT
pub GetBooleani_v: FnPtr,
pub GetBooleanv: FnPtr,
pub GetBufferParameteri64v: FnPtr,
/// Fallbacks: GetBufferParameterivARB
pub GetBufferParameteriv: FnPtr,
/// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
pub GetBufferPointerv: FnPtr,
/// Fallbacks: GetBufferSubDataARB
pub GetBufferSubData: FnPtr,
/// Fallbacks: GetCompressedTexImageARB
pub GetCompressedTexImage: FnPtr,
pub GetCompressedTextureImage: FnPtr,
pub GetCompressedTextureSubImage: FnPtr,
/// Fallbacks: GetDebugMessageLogARB, GetDebugMessageLogKHR
pub GetDebugMessageLog: FnPtr,
/// Fallbacks: GetDoubleIndexedvEXT, GetDoublei_vEXT
pub GetDoublei_v: FnPtr,
pub GetDoublev: FnPtr,
pub GetError: FnPtr,
/// Fallbacks: GetFloatIndexedvEXT, GetFloati_vEXT, GetFloati_vNV, GetFloati_vOES
pub GetFloati_v: FnPtr,
pub GetFloatv: FnPtr,
/// Fallbacks: GetFragDataIndexEXT
pub GetFragDataIndex: FnPtr,
/// Fallbacks: GetFragDataLocationEXT
pub GetFragDataLocation: FnPtr,
/// Fallbacks: GetFramebufferAttachmentParameterivEXT
pub GetFramebufferAttachmentParameteriv: FnPtr,
pub GetFramebufferParameteriv: FnPtr,
/// Fallbacks: GetGraphicsResetStatusEXT, GetGraphicsResetStatusKHR
pub GetGraphicsResetStatus: FnPtr,
pub GetInteger64i_v: FnPtr,
/// Fallbacks: GetInteger64vAPPLE
pub GetInteger64v: FnPtr,
/// Fallbacks: GetIntegerIndexedvEXT
pub GetIntegeri_v: FnPtr,
pub GetIntegerv: FnPtr,
pub GetInternalformati64v: FnPtr,
pub GetInternalformativ: FnPtr,
/// Fallbacks: GetMultisamplefvNV
pub GetMultisamplefv: FnPtr,
pub GetNamedBufferParameteri64v: FnPtr,
pub GetNamedBufferParameteriv: FnPtr,
pub GetNamedBufferPointerv: FnPtr,
pub GetNamedBufferSubData: FnPtr,
pub GetNamedFramebufferAttachmentParameteriv: FnPtr,
pub GetNamedFramebufferParameteriv: FnPtr,
pub GetNamedRenderbufferParameteriv: FnPtr,
/// Fallbacks: GetObjectLabelKHR
pub GetObjectLabel: FnPtr,
/// Fallbacks: GetObjectPtrLabelKHR
pub GetObjectPtrLabel: FnPtr,
/// Fallbacks: GetPointervEXT, GetPointervKHR
pub GetPointerv: FnPtr,
/// Fallbacks: GetProgramBinaryOES
pub GetProgramBinary: FnPtr,
pub GetProgramInfoLog: FnPtr,
pub GetProgramInterfaceiv: FnPtr,
pub GetProgramPipelineInfoLog: FnPtr,
pub GetProgramPipelineiv: FnPtr,
pub GetProgramResourceIndex: FnPtr,
pub GetProgramResourceLocation: FnPtr,
pub GetProgramResourceLocationIndex: FnPtr,
pub GetProgramResourceName: FnPtr,
pub GetProgramResourceiv: FnPtr,
pub GetProgramStageiv: FnPtr,
pub GetProgramiv: FnPtr,
pub GetQueryBufferObjecti64v: FnPtr,
pub GetQueryBufferObjectiv: FnPtr,
pub GetQueryBufferObjectui64v: FnPtr,
pub GetQueryBufferObjectuiv: FnPtr,
pub GetQueryIndexediv: FnPtr,
/// Fallbacks: GetQueryObjecti64vEXT
pub GetQueryObjecti64v: FnPtr,
/// Fallbacks: GetQueryObjectivARB, GetQueryObjectivEXT
pub GetQueryObjectiv: FnPtr,
/// Fallbacks: GetQueryObjectui64vEXT
pub GetQueryObjectui64v: FnPtr,
/// Fallbacks: GetQueryObjectuivARB
pub GetQueryObjectuiv: FnPtr,
/// Fallbacks: GetQueryivARB
pub GetQueryiv: FnPtr,
/// Fallbacks: GetRenderbufferParameterivEXT
pub GetRenderbufferParameteriv: FnPtr,
/// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
pub GetSamplerParameterIiv: FnPtr,
/// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
pub GetSamplerParameterIuiv: FnPtr,
pub GetSamplerParameterfv: FnPtr,
pub GetSamplerParameteriv: FnPtr,
pub GetShaderInfoLog: FnPtr,
pub GetShaderPrecisionFormat: FnPtr,
/// Fallbacks: GetShaderSourceARB
pub GetShaderSource: FnPtr,
pub GetShaderiv: FnPtr,
pub GetString: FnPtr,
pub GetStringi: FnPtr,
pub GetSubroutineIndex: FnPtr,
pub GetSubroutineUniformLocation: FnPtr,
/// Fallbacks: GetSyncivAPPLE
pub GetSynciv: FnPtr,
pub GetTexImage: FnPtr,
pub GetTexLevelParameterfv: FnPtr,
pub GetTexLevelParameteriv: FnPtr,
/// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
pub GetTexParameterIiv: FnPtr,
/// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
pub GetTexParameterIuiv: FnPtr,
pub GetTexParameterfv: FnPtr,
pub GetTexParameteriv: FnPtr,
pub GetTextureImage: FnPtr,
pub GetTextureLevelParameterfv: FnPtr,
pub GetTextureLevelParameteriv: FnPtr,
pub GetTextureParameterIiv: FnPtr,
pub GetTextureParameterIuiv: FnPtr,
pub GetTextureParameterfv: FnPtr,
pub GetTextureParameteriv: FnPtr,
pub GetTextureSubImage: FnPtr,
/// Fallbacks: GetTransformFeedbackVaryingEXT
pub GetTransformFeedbackVarying: FnPtr,
pub GetTransformFeedbacki64_v: FnPtr,
pub GetTransformFeedbacki_v: FnPtr,
pub GetTransformFeedbackiv: FnPtr,
pub GetUniformBlockIndex: FnPtr,
pub GetUniformIndices: FnPtr,
/// Fallbacks: GetUniformLocationARB
pub GetUniformLocation: FnPtr,
pub GetUniformSubroutineuiv: FnPtr,
pub GetUniformdv: FnPtr,
/// Fallbacks: GetUniformfvARB
pub GetUniformfv: FnPtr,
/// Fallbacks: GetUniformivARB
pub GetUniformiv: FnPtr,
/// Fallbacks: GetUniformuivEXT
pub GetUniformuiv: FnPtr,
pub GetVertexArrayIndexed64iv: FnPtr,
pub GetVertexArrayIndexediv: FnPtr,
pub GetVertexArrayiv: FnPtr,
/// Fallbacks: GetVertexAttribIivEXT
pub GetVertexAttribIiv: FnPtr,
/// Fallbacks: GetVertexAttribIuivEXT
pub GetVertexAttribIuiv: FnPtr,
/// Fallbacks: GetVertexAttribLdvEXT
pub GetVertexAttribLdv: FnPtr,
/// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
pub GetVertexAttribPointerv: FnPtr,
/// Fallbacks: GetVertexAttribdvARB, GetVertexAttribdvNV
pub GetVertexAttribdv: FnPtr,
/// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
pub GetVertexAttribfv: FnPtr,
/// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
pub GetVertexAttribiv: FnPtr,
pub GetnColorTable: FnPtr,
pub GetnCompressedTexImage: FnPtr,
pub GetnConvolutionFilter: FnPtr,
pub GetnHistogram: FnPtr,
pub GetnMapdv: FnPtr,
pub GetnMapfv: FnPtr,
pub GetnMapiv: FnPtr,
pub GetnMinmax: FnPtr,
pub GetnPixelMapfv: FnPtr,
pub GetnPixelMapuiv: FnPtr,
pub GetnPixelMapusv: FnPtr,
pub GetnPolygonStipple: FnPtr,
pub GetnSeparableFilter: FnPtr,
pub GetnTexImage: FnPtr,
pub GetnUniformdv: FnPtr,
/// Fallbacks: GetnUniformfvEXT, GetnUniformfvKHR
pub GetnUniformfv: FnPtr,
/// Fallbacks: GetnUniformivEXT, GetnUniformivKHR
pub GetnUniformiv: FnPtr,
/// Fallbacks: GetnUniformuivKHR
pub GetnUniformuiv: FnPtr,
pub Hint: FnPtr,
pub InvalidateBufferData: FnPtr,
pub InvalidateBufferSubData: FnPtr,
pub InvalidateFramebuffer: FnPtr,
pub InvalidateNamedFramebufferData: FnPtr,
pub InvalidateNamedFramebufferSubData: FnPtr,
pub InvalidateSubFramebuffer: FnPtr,
pub InvalidateTexImage: FnPtr,
pub InvalidateTexSubImage: FnPtr,
/// Fallbacks: IsBufferARB
pub IsBuffer: FnPtr,
pub IsEnabled: FnPtr,
/// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
pub IsEnabledi: FnPtr,
/// Fallbacks: IsFramebufferEXT
pub IsFramebuffer: FnPtr,
pub IsProgram: FnPtr,
pub IsProgramPipeline: FnPtr,
/// Fallbacks: IsQueryARB
pub IsQuery: FnPtr,
/// Fallbacks: IsRenderbufferEXT
pub IsRenderbuffer: FnPtr,
pub IsSampler: FnPtr,
pub IsShader: FnPtr,
/// Fallbacks: IsSyncAPPLE
pub IsSync: FnPtr,
pub IsTexture: FnPtr,
/// Fallbacks: IsTransformFeedbackNV
pub IsTransformFeedback: FnPtr,
/// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
pub IsVertexArray: FnPtr,
pub LineWidth: FnPtr,
/// Fallbacks: LinkProgramARB
pub LinkProgram: FnPtr,
pub LogicOp: FnPtr,
/// Fallbacks: MapBufferARB, MapBufferOES
pub MapBuffer: FnPtr,
/// Fallbacks: MapBufferRangeEXT
pub MapBufferRange: FnPtr,
pub MapNamedBuffer: FnPtr,
pub MapNamedBufferRange: FnPtr,
/// Fallbacks: MemoryBarrierEXT
pub MemoryBarrier: FnPtr,
pub MemoryBarrierByRegion: FnPtr,
/// Fallbacks: MinSampleShadingARB, MinSampleShadingOES
pub MinSampleShading: FnPtr,
/// Fallbacks: MultiDrawArraysEXT
pub MultiDrawArrays: FnPtr,
/// Fallbacks: MultiDrawArraysIndirectAMD, MultiDrawArraysIndirectEXT
pub MultiDrawArraysIndirect: FnPtr,
/// Fallbacks: MultiDrawElementsEXT
pub MultiDrawElements: FnPtr,
/// Fallbacks: MultiDrawElementsBaseVertexEXT
pub MultiDrawElementsBaseVertex: FnPtr,
/// Fallbacks: MultiDrawElementsIndirectAMD, MultiDrawElementsIndirectEXT
pub MultiDrawElementsIndirect: FnPtr,
pub MultiTexCoordP1ui: FnPtr,
pub MultiTexCoordP1uiv: FnPtr,
pub MultiTexCoordP2ui: FnPtr,
pub MultiTexCoordP2uiv: FnPtr,
pub MultiTexCoordP3ui: FnPtr,
pub MultiTexCoordP3uiv: FnPtr,
pub MultiTexCoordP4ui: FnPtr,
pub MultiTexCoordP4uiv: FnPtr,
pub NamedBufferData: FnPtr,
/// Fallbacks: NamedBufferStorageEXT
pub NamedBufferStorage: FnPtr,
/// Fallbacks: NamedBufferSubDataEXT
pub NamedBufferSubData: FnPtr,
pub NamedFramebufferDrawBuffer: FnPtr,
pub NamedFramebufferDrawBuffers: FnPtr,
pub NamedFramebufferParameteri: FnPtr,
pub NamedFramebufferReadBuffer: FnPtr,
pub NamedFramebufferRenderbuffer: FnPtr,
pub NamedFramebufferTexture: FnPtr,
pub NamedFramebufferTextureLayer: FnPtr,
pub NamedRenderbufferStorage: FnPtr,
pub NamedRenderbufferStorageMultisample: FnPtr,
pub NormalP3ui: FnPtr,
pub NormalP3uiv: FnPtr,
/// Fallbacks: ObjectLabelKHR
pub ObjectLabel: FnPtr,
/// Fallbacks: ObjectPtrLabelKHR
pub ObjectPtrLabel: FnPtr,
pub PatchParameterfv: FnPtr,
/// Fallbacks: PatchParameteriEXT, PatchParameteriOES
pub PatchParameteri: FnPtr,
/// Fallbacks: PauseTransformFeedbackNV
pub PauseTransformFeedback: FnPtr,
pub PixelStoref: FnPtr,
pub PixelStorei: FnPtr,
/// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
pub PointParameterf: FnPtr,
/// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
pub PointParameterfv: FnPtr,
/// Fallbacks: PointParameteriNV
pub PointParameteri: FnPtr,
/// Fallbacks: PointParameterivNV
pub PointParameteriv: FnPtr,
pub PointSize: FnPtr,
/// Fallbacks: PolygonModeNV
pub PolygonMode: FnPtr,
pub PolygonOffset: FnPtr,
/// Fallbacks: PopDebugGroupKHR
pub PopDebugGroup: FnPtr,
pub PrimitiveRestartIndex: FnPtr,
/// Fallbacks: ProgramBinaryOES
pub ProgramBinary: FnPtr,
/// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
pub ProgramParameteri: FnPtr,
pub ProgramUniform1d: FnPtr,
pub ProgramUniform1dv: FnPtr,
/// Fallbacks: ProgramUniform1fEXT
pub ProgramUniform1f: FnPtr,
/// Fallbacks: ProgramUniform1fvEXT
pub ProgramUniform1fv: FnPtr,
/// Fallbacks: ProgramUniform1iEXT
pub ProgramUniform1i: FnPtr,
/// Fallbacks: ProgramUniform1ivEXT
pub ProgramUniform1iv: FnPtr,
/// Fallbacks: ProgramUniform1uiEXT
pub ProgramUniform1ui: FnPtr,
/// Fallbacks: ProgramUniform1uivEXT
pub ProgramUniform1uiv: FnPtr,
pub ProgramUniform2d: FnPtr,
pub ProgramUniform2dv: FnPtr,
/// Fallbacks: ProgramUniform2fEXT
pub ProgramUniform2f: FnPtr,
/// Fallbacks: ProgramUniform2fvEXT
pub ProgramUniform2fv: FnPtr,
/// Fallbacks: ProgramUniform2iEXT
pub ProgramUniform2i: FnPtr,
/// Fallbacks: ProgramUniform2ivEXT
pub ProgramUniform2iv: FnPtr,
/// Fallbacks: ProgramUniform2uiEXT
pub ProgramUniform2ui: FnPtr,
/// Fallbacks: ProgramUniform2uivEXT
pub ProgramUniform2uiv: FnPtr,
pub ProgramUniform3d: FnPtr,
pub ProgramUniform3dv: FnPtr,
/// Fallbacks: ProgramUniform3fEXT
pub ProgramUniform3f: FnPtr,
/// Fallbacks: ProgramUniform3fvEXT
pub ProgramUniform3fv: FnPtr,
/// Fallbacks: ProgramUniform3iEXT
pub ProgramUniform3i: FnPtr,
/// Fallbacks: ProgramUniform3ivEXT
pub ProgramUniform3iv: FnPtr,
/// Fallbacks: ProgramUniform3uiEXT
pub ProgramUniform3ui: FnPtr,
/// Fallbacks: ProgramUniform3uivEXT
pub ProgramUniform3uiv: FnPtr,
pub ProgramUniform4d: FnPtr,
pub ProgramUniform4dv: FnPtr,
/// Fallbacks: ProgramUniform4fEXT
pub ProgramUniform4f: FnPtr,
/// Fallbacks: ProgramUniform4fvEXT
pub ProgramUniform4fv: FnPtr,
/// Fallbacks: ProgramUniform4iEXT
pub ProgramUniform4i: FnPtr,
/// Fallbacks: ProgramUniform4ivEXT
pub ProgramUniform4iv: FnPtr,
/// Fallbacks: ProgramUniform4uiEXT
pub ProgramUniform4ui: FnPtr,
/// Fallbacks: ProgramUniform4uivEXT
pub ProgramUniform4uiv: FnPtr,
pub ProgramUniformMatrix2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2fvEXT
pub ProgramUniformMatrix2fv: FnPtr,
pub ProgramUniformMatrix2x3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2x3fvEXT
pub ProgramUniformMatrix2x3fv: FnPtr,
pub ProgramUniformMatrix2x4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix2x4fvEXT
pub ProgramUniformMatrix2x4fv: FnPtr,
pub ProgramUniformMatrix3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3fvEXT
pub ProgramUniformMatrix3fv: FnPtr,
pub ProgramUniformMatrix3x2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3x2fvEXT
pub ProgramUniformMatrix3x2fv: FnPtr,
pub ProgramUniformMatrix3x4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix3x4fvEXT
pub ProgramUniformMatrix3x4fv: FnPtr,
pub ProgramUniformMatrix4dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4fvEXT
pub ProgramUniformMatrix4fv: FnPtr,
pub ProgramUniformMatrix4x2dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4x2fvEXT
pub ProgramUniformMatrix4x2fv: FnPtr,
pub ProgramUniformMatrix4x3dv: FnPtr,
/// Fallbacks: ProgramUniformMatrix4x3fvEXT
pub ProgramUniformMatrix4x3fv: FnPtr,
/// Fallbacks: ProvokingVertexEXT
pub ProvokingVertex: FnPtr,
/// Fallbacks: PushDebugGroupKHR
pub PushDebugGroup: FnPtr,
/// Fallbacks: QueryCounterEXT
pub QueryCounter: FnPtr,
pub ReadBuffer: FnPtr,
pub ReadPixels: FnPtr,
/// Fallbacks: ReadnPixelsARB, ReadnPixelsEXT, ReadnPixelsKHR
pub ReadnPixels: FnPtr,
pub ReleaseShaderCompiler: FnPtr,
/// Fallbacks: RenderbufferStorageEXT
pub RenderbufferStorage: FnPtr,
/// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
pub RenderbufferStorageMultisample: FnPtr,
/// Fallbacks: ResumeTransformFeedbackNV
pub ResumeTransformFeedback: FnPtr,
/// Fallbacks: SampleCoverageARB
pub SampleCoverage: FnPtr,
pub SampleMaski: FnPtr,
/// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
pub SamplerParameterIiv: FnPtr,
/// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
pub SamplerParameterIuiv: FnPtr,
pub SamplerParameterf: FnPtr,
pub SamplerParameterfv: FnPtr,
pub SamplerParameteri: FnPtr,
pub SamplerParameteriv: FnPtr,
pub Scissor: FnPtr,
/// Fallbacks: ScissorArrayvNV, ScissorArrayvOES
pub ScissorArrayv: FnPtr,
/// Fallbacks: ScissorIndexedNV, ScissorIndexedOES
pub ScissorIndexed: FnPtr,
/// Fallbacks: ScissorIndexedvNV, ScissorIndexedvOES
pub ScissorIndexedv: FnPtr,
pub SecondaryColorP3ui: FnPtr,
pub SecondaryColorP3uiv: FnPtr,
pub ShaderBinary: FnPtr,
/// Fallbacks: ShaderSourceARB
pub ShaderSource: FnPtr,
pub ShaderStorageBlockBinding: FnPtr,
pub StencilFunc: FnPtr,
pub StencilFuncSeparate: FnPtr,
pub StencilMask: FnPtr,
pub StencilMaskSeparate: FnPtr,
pub StencilOp: FnPtr,
/// Fallbacks: StencilOpSeparateATI
pub StencilOpSeparate: FnPtr,
/// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
pub TexBuffer: FnPtr,
/// Fallbacks: TexBufferRangeEXT, TexBufferRangeOES
pub TexBufferRange: FnPtr,
pub TexCoordP1ui: FnPtr,
pub TexCoordP1uiv: FnPtr,
pub TexCoordP2ui: FnPtr,
pub TexCoordP2uiv: FnPtr,
pub TexCoordP3ui: FnPtr,
pub TexCoordP3uiv: FnPtr,
pub TexCoordP4ui: FnPtr,
pub TexCoordP4uiv: FnPtr,
pub TexImage1D: FnPtr,
pub TexImage2D: FnPtr,
pub TexImage2DMultisample: FnPtr,
/// Fallbacks: TexImage3DEXT
pub TexImage3D: FnPtr,
pub TexImage3DMultisample: FnPtr,
/// Fallbacks: TexParameterIivEXT, TexParameterIivOES
pub TexParameterIiv: FnPtr,
/// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
pub TexParameterIuiv: FnPtr,
pub TexParameterf: FnPtr,
pub TexParameterfv: FnPtr,
pub TexParameteri: FnPtr,
pub TexParameteriv: FnPtr,
/// Fallbacks: TexStorage1DEXT
pub TexStorage1D: FnPtr,
/// Fallbacks: TexStorage2DEXT
pub TexStorage2D: FnPtr,
pub TexStorage2DMultisample: FnPtr,
/// Fallbacks: TexStorage3DEXT
pub TexStorage3D: FnPtr,
/// Fallbacks: TexStorage3DMultisampleOES
pub TexStorage3DMultisample: FnPtr,
/// Fallbacks: TexSubImage1DEXT
pub TexSubImage1D: FnPtr,
/// Fallbacks: TexSubImage2DEXT
pub TexSubImage2D: FnPtr,
/// Fallbacks: TexSubImage3DEXT
pub TexSubImage3D: FnPtr,
pub TextureBarrier: FnPtr,
pub TextureBuffer: FnPtr,
pub TextureBufferRange: FnPtr,
pub TextureParameterIiv: FnPtr,
pub TextureParameterIuiv: FnPtr,
pub TextureParameterf: FnPtr,
pub TextureParameterfv: FnPtr,
pub TextureParameteri: FnPtr,
pub TextureParameteriv: FnPtr,
pub TextureStorage1D: FnPtr,
pub TextureStorage2D: FnPtr,
pub TextureStorage2DMultisample: FnPtr,
pub TextureStorage3D: FnPtr,
pub TextureStorage3DMultisample: FnPtr,
pub TextureSubImage1D: FnPtr,
pub TextureSubImage2D: FnPtr,
pub TextureSubImage3D: FnPtr,
/// Fallbacks: TextureViewEXT, TextureViewOES
pub TextureView: FnPtr,
pub TransformFeedbackBufferBase: FnPtr,
pub TransformFeedbackBufferRange: FnPtr,
/// Fallbacks: TransformFeedbackVaryingsEXT
pub TransformFeedbackVaryings: FnPtr,
pub Uniform1d: FnPtr,
pub Uniform1dv: FnPtr,
/// Fallbacks: Uniform1fARB
pub Uniform1f: FnPtr,
/// Fallbacks: Uniform1fvARB
pub Uniform1fv: FnPtr,
/// Fallbacks: Uniform1iARB
pub Uniform1i: FnPtr,
/// Fallbacks: Uniform1ivARB
pub Uniform1iv: FnPtr,
/// Fallbacks: Uniform1uiEXT
pub Uniform1ui: FnPtr,
/// Fallbacks: Uniform1uivEXT
pub Uniform1uiv: FnPtr,
pub Uniform2d: FnPtr,
pub Uniform2dv: FnPtr,
/// Fallbacks: Uniform2fARB
pub Uniform2f: FnPtr,
/// Fallbacks: Uniform2fvARB
pub Uniform2fv: FnPtr,
/// Fallbacks: Uniform2iARB
pub Uniform2i: FnPtr,
/// Fallbacks: Uniform2ivARB
pub Uniform2iv: FnPtr,
/// Fallbacks: Uniform2uiEXT
pub Uniform2ui: FnPtr,
/// Fallbacks: Uniform2uivEXT
pub Uniform2uiv: FnPtr,
pub Uniform3d: FnPtr,
pub Uniform3dv: FnPtr,
/// Fallbacks: Uniform3fARB
pub Uniform3f: FnPtr,
/// Fallbacks: Uniform3fvARB
pub Uniform3fv: FnPtr,
/// Fallbacks: Uniform3iARB
pub Uniform3i: FnPtr,
/// Fallbacks: Uniform3ivARB
pub Uniform3iv: FnPtr,
/// Fallbacks: Uniform3uiEXT
pub Uniform3ui: FnPtr,
/// Fallbacks: Uniform3uivEXT
pub Uniform3uiv: FnPtr,
pub Uniform4d: FnPtr,
pub Uniform4dv: FnPtr,
/// Fallbacks: Uniform4fARB
pub Uniform4f: FnPtr,
/// Fallbacks: Uniform4fvARB
pub Uniform4fv: FnPtr,
/// Fallbacks: Uniform4iARB
pub Uniform4i: FnPtr,
/// Fallbacks: Uniform4ivARB
pub Uniform4iv: FnPtr,
/// Fallbacks: Uniform4uiEXT
pub Uniform4ui: FnPtr,
/// Fallbacks: Uniform4uivEXT
pub Uniform4uiv: FnPtr,
pub UniformBlockBinding: FnPtr,
pub UniformMatrix2dv: FnPtr,
/// Fallbacks: UniformMatrix2fvARB
pub UniformMatrix2fv: FnPtr,
pub UniformMatrix2x3dv: FnPtr,
/// Fallbacks: UniformMatrix2x3fvNV
pub UniformMatrix2x3fv: FnPtr,
pub UniformMatrix2x4dv: FnPtr,
/// Fallbacks: UniformMatrix2x4fvNV
pub UniformMatrix2x4fv: FnPtr,
pub UniformMatrix3dv: FnPtr,
/// Fallbacks: UniformMatrix3fvARB
pub UniformMatrix3fv: FnPtr,
pub UniformMatrix3x2dv: FnPtr,
/// Fallbacks: UniformMatrix3x2fvNV
pub UniformMatrix3x2fv: FnPtr,
pub UniformMatrix3x4dv: FnPtr,
/// Fallbacks: UniformMatrix3x4fvNV
pub UniformMatrix3x4fv: FnPtr,
pub UniformMatrix4dv: FnPtr,
/// Fallbacks: UniformMatrix4fvARB
pub UniformMatrix4fv: FnPtr,
pub UniformMatrix4x2dv: FnPtr,
/// Fallbacks: UniformMatrix4x2fvNV
pub UniformMatrix4x2fv: FnPtr,
pub UniformMatrix4x3dv: FnPtr,
/// Fallbacks: UniformMatrix4x3fvNV
pub UniformMatrix4x3fv: FnPtr,
pub UniformSubroutinesuiv: FnPtr,
/// Fallbacks: UnmapBufferARB, UnmapBufferOES
pub UnmapBuffer: FnPtr,
pub UnmapNamedBuffer: FnPtr,
/// Fallbacks: UseProgramObjectARB
pub UseProgram: FnPtr,
pub UseProgramStages: FnPtr,
/// Fallbacks: ValidateProgramARB
pub ValidateProgram: FnPtr,
pub ValidateProgramPipeline: FnPtr,
pub VertexArrayAttribBinding: FnPtr,
pub VertexArrayAttribFormat: FnPtr,
pub VertexArrayAttribIFormat: FnPtr,
pub VertexArrayAttribLFormat: FnPtr,
pub VertexArrayBindingDivisor: FnPtr,
pub VertexArrayElementBuffer: FnPtr,
pub VertexArrayVertexBuffer: FnPtr,
pub VertexArrayVertexBuffers: FnPtr,
/// Fallbacks: VertexAttrib1dARB, VertexAttrib1dNV
pub VertexAttrib1d: FnPtr,
/// Fallbacks: VertexAttrib1dvARB, VertexAttrib1dvNV
pub VertexAttrib1dv: FnPtr,
/// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
pub VertexAttrib1f: FnPtr,
/// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
pub VertexAttrib1fv: FnPtr,
/// Fallbacks: VertexAttrib1sARB, VertexAttrib1sNV
pub VertexAttrib1s: FnPtr,
/// Fallbacks: VertexAttrib1svARB, VertexAttrib1svNV
pub VertexAttrib1sv: FnPtr,
/// Fallbacks: VertexAttrib2dARB, VertexAttrib2dNV
pub VertexAttrib2d: FnPtr,
/// Fallbacks: VertexAttrib2dvARB, VertexAttrib2dvNV
pub VertexAttrib2dv: FnPtr,
/// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
pub VertexAttrib2f: FnPtr,
/// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
pub VertexAttrib2fv: FnPtr,
/// Fallbacks: VertexAttrib2sARB, VertexAttrib2sNV
pub VertexAttrib2s: FnPtr,
/// Fallbacks: VertexAttrib2svARB, VertexAttrib2svNV
pub VertexAttrib2sv: FnPtr,
/// Fallbacks: VertexAttrib3dARB, VertexAttrib3dNV
pub VertexAttrib3d: FnPtr,
/// Fallbacks: VertexAttrib3dvARB, VertexAttrib3dvNV
pub VertexAttrib3dv: FnPtr,
/// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
pub VertexAttrib3f: FnPtr,
/// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
pub VertexAttrib3fv: FnPtr,
/// Fallbacks: VertexAttrib3sARB, VertexAttrib3sNV
pub VertexAttrib3s: FnPtr,
/// Fallbacks: VertexAttrib3svARB, VertexAttrib3svNV
pub VertexAttrib3sv: FnPtr,
/// Fallbacks: VertexAttrib4NbvARB
pub VertexAttrib4Nbv: FnPtr,
/// Fallbacks: VertexAttrib4NivARB
pub VertexAttrib4Niv: FnPtr,
/// Fallbacks: VertexAttrib4NsvARB
pub VertexAttrib4Nsv: FnPtr,
/// Fallbacks: VertexAttrib4NubARB, VertexAttrib4ubNV
pub VertexAttrib4Nub: FnPtr,
/// Fallbacks: VertexAttrib4NubvARB, VertexAttrib4ubvNV
pub VertexAttrib4Nubv: FnPtr,
/// Fallbacks: VertexAttrib4NuivARB
pub VertexAttrib4Nuiv: FnPtr,
/// Fallbacks: VertexAttrib4NusvARB
pub VertexAttrib4Nusv: FnPtr,
/// Fallbacks: VertexAttrib4bvARB
pub VertexAttrib4bv: FnPtr,
/// Fallbacks: VertexAttrib4dARB, VertexAttrib4dNV
pub VertexAttrib4d: FnPtr,
/// Fallbacks: VertexAttrib4dvARB, VertexAttrib4dvNV
pub VertexAttrib4dv: FnPtr,
/// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
pub VertexAttrib4f: FnPtr,
/// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
pub VertexAttrib4fv: FnPtr,
/// Fallbacks: VertexAttrib4ivARB
pub VertexAttrib4iv: FnPtr,
/// Fallbacks: VertexAttrib4sARB, VertexAttrib4sNV
pub VertexAttrib4s: FnPtr,
/// Fallbacks: VertexAttrib4svARB, VertexAttrib4svNV
pub VertexAttrib4sv: FnPtr,
/// Fallbacks: VertexAttrib4ubvARB
pub VertexAttrib4ubv: FnPtr,
/// Fallbacks: VertexAttrib4uivARB
pub VertexAttrib4uiv: FnPtr,
/// Fallbacks: VertexAttrib4usvARB
pub VertexAttrib4usv: FnPtr,
pub VertexAttribBinding: FnPtr,
/// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
pub VertexAttribDivisor: FnPtr,
pub VertexAttribFormat: FnPtr,
/// Fallbacks: VertexAttribI1iEXT
pub VertexAttribI1i: FnPtr,
/// Fallbacks: VertexAttribI1ivEXT
pub VertexAttribI1iv: FnPtr,
/// Fallbacks: VertexAttribI1uiEXT
pub VertexAttribI1ui: FnPtr,
/// Fallbacks: VertexAttribI1uivEXT
pub VertexAttribI1uiv: FnPtr,
/// Fallbacks: VertexAttribI2iEXT
pub VertexAttribI2i: FnPtr,
/// Fallbacks: VertexAttribI2ivEXT
pub VertexAttribI2iv: FnPtr,
/// Fallbacks: VertexAttribI2uiEXT
pub VertexAttribI2ui: FnPtr,
/// Fallbacks: VertexAttribI2uivEXT
pub VertexAttribI2uiv: FnPtr,
/// Fallbacks: VertexAttribI3iEXT
pub VertexAttribI3i: FnPtr,
/// Fallbacks: VertexAttribI3ivEXT
pub VertexAttribI3iv: FnPtr,
/// Fallbacks: VertexAttribI3uiEXT
pub VertexAttribI3ui: FnPtr,
/// Fallbacks: VertexAttribI3uivEXT
pub VertexAttribI3uiv: FnPtr,
/// Fallbacks: VertexAttribI4bvEXT
pub VertexAttribI4bv: FnPtr,
/// Fallbacks: VertexAttribI4iEXT
pub VertexAttribI4i: FnPtr,
/// Fallbacks: VertexAttribI4ivEXT
pub VertexAttribI4iv: FnPtr,
/// Fallbacks: VertexAttribI4svEXT
pub VertexAttribI4sv: FnPtr,
/// Fallbacks: VertexAttribI4ubvEXT
pub VertexAttribI4ubv: FnPtr,
/// Fallbacks: VertexAttribI4uiEXT
pub VertexAttribI4ui: FnPtr,
/// Fallbacks: VertexAttribI4uivEXT
pub VertexAttribI4uiv: FnPtr,
/// Fallbacks: VertexAttribI4usvEXT
pub VertexAttribI4usv: FnPtr,
pub VertexAttribIFormat: FnPtr,
/// Fallbacks: VertexAttribIPointerEXT
pub VertexAttribIPointer: FnPtr,
/// Fallbacks: VertexAttribL1dEXT
pub VertexAttribL1d: FnPtr,
/// Fallbacks: VertexAttribL1dvEXT
pub VertexAttribL1dv: FnPtr,
/// Fallbacks: VertexAttribL2dEXT
pub VertexAttribL2d: FnPtr,
/// Fallbacks: VertexAttribL2dvEXT
pub VertexAttribL2dv: FnPtr,
/// Fallbacks: VertexAttribL3dEXT
pub VertexAttribL3d: FnPtr,
/// Fallbacks: VertexAttribL3dvEXT
pub VertexAttribL3dv: FnPtr,
/// Fallbacks: VertexAttribL4dEXT
pub VertexAttribL4d: FnPtr,
/// Fallbacks: VertexAttribL4dvEXT
pub VertexAttribL4dv: FnPtr,
pub VertexAttribLFormat: FnPtr,
/// Fallbacks: VertexAttribLPointerEXT
pub VertexAttribLPointer: FnPtr,
pub VertexAttribP1ui: FnPtr,
pub VertexAttribP1uiv: FnPtr,
pub VertexAttribP2ui: FnPtr,
pub VertexAttribP2uiv: FnPtr,
pub VertexAttribP3ui: FnPtr,
pub VertexAttribP3uiv: FnPtr,
pub VertexAttribP4ui: FnPtr,
pub VertexAttribP4uiv: FnPtr,
/// Fallbacks: VertexAttribPointerARB
pub VertexAttribPointer: FnPtr,
pub VertexBindingDivisor: FnPtr,
pub VertexP2ui: FnPtr,
pub VertexP2uiv: FnPtr,
pub VertexP3ui: FnPtr,
pub VertexP3uiv: FnPtr,
pub VertexP4ui: FnPtr,
pub VertexP4uiv: FnPtr,
pub Viewport: FnPtr,
/// Fallbacks: ViewportArrayvNV, ViewportArrayvOES
pub ViewportArrayv: FnPtr,
/// Fallbacks: ViewportIndexedfOES, ViewportIndexedfNV
pub ViewportIndexedf: FnPtr,
/// Fallbacks: ViewportIndexedfvOES, ViewportIndexedfvNV
pub ViewportIndexedfv: FnPtr,
/// Fallbacks: WaitSyncAPPLE
pub WaitSync: FnPtr,
}


        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone)]
        pub struct Gl {
            pub ptrs: GlFnPtrs,
            _priv: (),
        }
        
impl Gl {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code, unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Gl where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {
                #[inline(never)]
                fn do_metaloadfn(loadfn: &mut FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                                 symbol: &'static str,
                                 symbols: &[&'static str])
                                 -> *const __gl_imports::raw::c_void {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                }
                let mut metaloadfn = |symbol: &'static str, symbols: &[&'static str]| {
                    do_metaloadfn(&mut loadfn, symbol, symbols)
                };
                Gl::load_with_metaloadfn(&mut metaloadfn)
            }

            #[inline(never)]
            fn load_with_metaloadfn(metaloadfn: &mut FnMut(&'static str, &[&'static str]) -> *const __gl_imports::raw::c_void) -> Gl {
                
                Gl {
                    ptrs: GlFnPtrs {
ActiveShaderProgram: FnPtr::new(metaloadfn("glActiveShaderProgram", &[])),
ActiveTexture: FnPtr::new(metaloadfn("glActiveTexture", &["glActiveTextureARB"])),
AttachShader: FnPtr::new(metaloadfn("glAttachShader", &["glAttachObjectARB"])),
BeginConditionalRender: FnPtr::new(metaloadfn("glBeginConditionalRender", &["glBeginConditionalRenderNV"])),
BeginQuery: FnPtr::new(metaloadfn("glBeginQuery", &["glBeginQueryARB"])),
BeginQueryIndexed: FnPtr::new(metaloadfn("glBeginQueryIndexed", &[])),
BeginTransformFeedback: FnPtr::new(metaloadfn("glBeginTransformFeedback", &["glBeginTransformFeedbackEXT", "glBeginTransformFeedbackNV"])),
BindAttribLocation: FnPtr::new(metaloadfn("glBindAttribLocation", &["glBindAttribLocationARB"])),
BindBuffer: FnPtr::new(metaloadfn("glBindBuffer", &["glBindBufferARB"])),
BindBufferBase: FnPtr::new(metaloadfn("glBindBufferBase", &["glBindBufferBaseEXT", "glBindBufferBaseNV"])),
BindBufferRange: FnPtr::new(metaloadfn("glBindBufferRange", &["glBindBufferRangeEXT", "glBindBufferRangeNV"])),
BindBuffersBase: FnPtr::new(metaloadfn("glBindBuffersBase", &[])),
BindBuffersRange: FnPtr::new(metaloadfn("glBindBuffersRange", &[])),
BindFragDataLocation: FnPtr::new(metaloadfn("glBindFragDataLocation", &["glBindFragDataLocationEXT"])),
BindFragDataLocationIndexed: FnPtr::new(metaloadfn("glBindFragDataLocationIndexed", &["glBindFragDataLocationIndexedEXT"])),
BindFramebuffer: FnPtr::new(metaloadfn("glBindFramebuffer", &[])),
BindImageTexture: FnPtr::new(metaloadfn("glBindImageTexture", &[])),
BindImageTextures: FnPtr::new(metaloadfn("glBindImageTextures", &[])),
BindProgramPipeline: FnPtr::new(metaloadfn("glBindProgramPipeline", &[])),
BindRenderbuffer: FnPtr::new(metaloadfn("glBindRenderbuffer", &[])),
BindSampler: FnPtr::new(metaloadfn("glBindSampler", &[])),
BindSamplers: FnPtr::new(metaloadfn("glBindSamplers", &[])),
BindTexture: FnPtr::new(metaloadfn("glBindTexture", &["glBindTextureEXT"])),
BindTextureUnit: FnPtr::new(metaloadfn("glBindTextureUnit", &[])),
BindTextures: FnPtr::new(metaloadfn("glBindTextures", &[])),
BindTransformFeedback: FnPtr::new(metaloadfn("glBindTransformFeedback", &[])),
BindVertexArray: FnPtr::new(metaloadfn("glBindVertexArray", &["glBindVertexArrayOES"])),
BindVertexBuffer: FnPtr::new(metaloadfn("glBindVertexBuffer", &[])),
BindVertexBuffers: FnPtr::new(metaloadfn("glBindVertexBuffers", &[])),
BlendColor: FnPtr::new(metaloadfn("glBlendColor", &["glBlendColorEXT"])),
BlendEquation: FnPtr::new(metaloadfn("glBlendEquation", &["glBlendEquationEXT"])),
BlendEquationSeparate: FnPtr::new(metaloadfn("glBlendEquationSeparate", &["glBlendEquationSeparateEXT"])),
BlendEquationSeparatei: FnPtr::new(metaloadfn("glBlendEquationSeparatei", &["glBlendEquationSeparateIndexedAMD", "glBlendEquationSeparateiARB", "glBlendEquationSeparateiEXT", "glBlendEquationSeparateiOES"])),
BlendEquationi: FnPtr::new(metaloadfn("glBlendEquationi", &["glBlendEquationIndexedAMD", "glBlendEquationiARB", "glBlendEquationiEXT", "glBlendEquationiOES"])),
BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
BlendFuncSeparate: FnPtr::new(metaloadfn("glBlendFuncSeparate", &["glBlendFuncSeparateEXT", "glBlendFuncSeparateINGR"])),
BlendFuncSeparatei: FnPtr::new(metaloadfn("glBlendFuncSeparatei", &["glBlendFuncSeparateIndexedAMD", "glBlendFuncSeparateiARB", "glBlendFuncSeparateiEXT", "glBlendFuncSeparateiOES"])),
BlendFunci: FnPtr::new(metaloadfn("glBlendFunci", &["glBlendFuncIndexedAMD", "glBlendFunciARB", "glBlendFunciEXT", "glBlendFunciOES"])),
BlitFramebuffer: FnPtr::new(metaloadfn("glBlitFramebuffer", &["glBlitFramebufferEXT", "glBlitFramebufferNV"])),
BlitNamedFramebuffer: FnPtr::new(metaloadfn("glBlitNamedFramebuffer", &[])),
BufferData: FnPtr::new(metaloadfn("glBufferData", &["glBufferDataARB"])),
BufferStorage: FnPtr::new(metaloadfn("glBufferStorage", &["glBufferStorageEXT"])),
BufferSubData: FnPtr::new(metaloadfn("glBufferSubData", &["glBufferSubDataARB"])),
CheckFramebufferStatus: FnPtr::new(metaloadfn("glCheckFramebufferStatus", &["glCheckFramebufferStatusEXT"])),
CheckNamedFramebufferStatus: FnPtr::new(metaloadfn("glCheckNamedFramebufferStatus", &[])),
ClampColor: FnPtr::new(metaloadfn("glClampColor", &["glClampColorARB"])),
Clear: FnPtr::new(metaloadfn("glClear", &[])),
ClearBufferData: FnPtr::new(metaloadfn("glClearBufferData", &[])),
ClearBufferSubData: FnPtr::new(metaloadfn("glClearBufferSubData", &[])),
ClearBufferfi: FnPtr::new(metaloadfn("glClearBufferfi", &[])),
ClearBufferfv: FnPtr::new(metaloadfn("glClearBufferfv", &[])),
ClearBufferiv: FnPtr::new(metaloadfn("glClearBufferiv", &[])),
ClearBufferuiv: FnPtr::new(metaloadfn("glClearBufferuiv", &[])),
ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
ClearDepth: FnPtr::new(metaloadfn("glClearDepth", &[])),
ClearDepthf: FnPtr::new(metaloadfn("glClearDepthf", &["glClearDepthfOES"])),
ClearNamedBufferData: FnPtr::new(metaloadfn("glClearNamedBufferData", &[])),
ClearNamedBufferSubData: FnPtr::new(metaloadfn("glClearNamedBufferSubData", &[])),
ClearNamedFramebufferfi: FnPtr::new(metaloadfn("glClearNamedFramebufferfi", &[])),
ClearNamedFramebufferfv: FnPtr::new(metaloadfn("glClearNamedFramebufferfv", &[])),
ClearNamedFramebufferiv: FnPtr::new(metaloadfn("glClearNamedFramebufferiv", &[])),
ClearNamedFramebufferuiv: FnPtr::new(metaloadfn("glClearNamedFramebufferuiv", &[])),
ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
ClearTexImage: FnPtr::new(metaloadfn("glClearTexImage", &["glClearTexImageEXT"])),
ClearTexSubImage: FnPtr::new(metaloadfn("glClearTexSubImage", &["glClearTexSubImageEXT"])),
ClientWaitSync: FnPtr::new(metaloadfn("glClientWaitSync", &["glClientWaitSyncAPPLE"])),
ClipControl: FnPtr::new(metaloadfn("glClipControl", &["glClipControlEXT"])),
ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
ColorMaski: FnPtr::new(metaloadfn("glColorMaski", &["glColorMaskIndexedEXT", "glColorMaskiEXT", "glColorMaskiOES"])),
ColorP3ui: FnPtr::new(metaloadfn("glColorP3ui", &[])),
ColorP3uiv: FnPtr::new(metaloadfn("glColorP3uiv", &[])),
ColorP4ui: FnPtr::new(metaloadfn("glColorP4ui", &[])),
ColorP4uiv: FnPtr::new(metaloadfn("glColorP4uiv", &[])),
CompileShader: FnPtr::new(metaloadfn("glCompileShader", &["glCompileShaderARB"])),
CompressedTexImage1D: FnPtr::new(metaloadfn("glCompressedTexImage1D", &["glCompressedTexImage1DARB"])),
CompressedTexImage2D: FnPtr::new(metaloadfn("glCompressedTexImage2D", &["glCompressedTexImage2DARB"])),
CompressedTexImage3D: FnPtr::new(metaloadfn("glCompressedTexImage3D", &["glCompressedTexImage3DARB"])),
CompressedTexSubImage1D: FnPtr::new(metaloadfn("glCompressedTexSubImage1D", &["glCompressedTexSubImage1DARB"])),
CompressedTexSubImage2D: FnPtr::new(metaloadfn("glCompressedTexSubImage2D", &["glCompressedTexSubImage2DARB"])),
CompressedTexSubImage3D: FnPtr::new(metaloadfn("glCompressedTexSubImage3D", &["glCompressedTexSubImage3DARB"])),
CompressedTextureSubImage1D: FnPtr::new(metaloadfn("glCompressedTextureSubImage1D", &[])),
CompressedTextureSubImage2D: FnPtr::new(metaloadfn("glCompressedTextureSubImage2D", &[])),
CompressedTextureSubImage3D: FnPtr::new(metaloadfn("glCompressedTextureSubImage3D", &[])),
CopyBufferSubData: FnPtr::new(metaloadfn("glCopyBufferSubData", &["glCopyBufferSubDataNV"])),
CopyImageSubData: FnPtr::new(metaloadfn("glCopyImageSubData", &["glCopyImageSubDataEXT", "glCopyImageSubDataOES"])),
CopyNamedBufferSubData: FnPtr::new(metaloadfn("glCopyNamedBufferSubData", &[])),
CopyTexImage1D: FnPtr::new(metaloadfn("glCopyTexImage1D", &["glCopyTexImage1DEXT"])),
CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &["glCopyTexImage2DEXT"])),
CopyTexSubImage1D: FnPtr::new(metaloadfn("glCopyTexSubImage1D", &["glCopyTexSubImage1DEXT"])),
CopyTexSubImage2D: FnPtr::new(metaloadfn("glCopyTexSubImage2D", &["glCopyTexSubImage2DEXT"])),
CopyTexSubImage3D: FnPtr::new(metaloadfn("glCopyTexSubImage3D", &["glCopyTexSubImage3DEXT"])),
CopyTextureSubImage1D: FnPtr::new(metaloadfn("glCopyTextureSubImage1D", &[])),
CopyTextureSubImage2D: FnPtr::new(metaloadfn("glCopyTextureSubImage2D", &[])),
CopyTextureSubImage3D: FnPtr::new(metaloadfn("glCopyTextureSubImage3D", &[])),
CreateBuffers: FnPtr::new(metaloadfn("glCreateBuffers", &[])),
CreateFramebuffers: FnPtr::new(metaloadfn("glCreateFramebuffers", &[])),
CreateProgram: FnPtr::new(metaloadfn("glCreateProgram", &["glCreateProgramObjectARB"])),
CreateProgramPipelines: FnPtr::new(metaloadfn("glCreateProgramPipelines", &[])),
CreateQueries: FnPtr::new(metaloadfn("glCreateQueries", &[])),
CreateRenderbuffers: FnPtr::new(metaloadfn("glCreateRenderbuffers", &[])),
CreateSamplers: FnPtr::new(metaloadfn("glCreateSamplers", &[])),
CreateShader: FnPtr::new(metaloadfn("glCreateShader", &["glCreateShaderObjectARB"])),
CreateShaderProgramv: FnPtr::new(metaloadfn("glCreateShaderProgramv", &[])),
CreateTextures: FnPtr::new(metaloadfn("glCreateTextures", &[])),
CreateTransformFeedbacks: FnPtr::new(metaloadfn("glCreateTransformFeedbacks", &[])),
CreateVertexArrays: FnPtr::new(metaloadfn("glCreateVertexArrays", &[])),
CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
DebugMessageCallback: FnPtr::new(metaloadfn("glDebugMessageCallback", &["glDebugMessageCallbackARB", "glDebugMessageCallbackKHR"])),
DebugMessageControl: FnPtr::new(metaloadfn("glDebugMessageControl", &["glDebugMessageControlARB", "glDebugMessageControlKHR"])),
DebugMessageInsert: FnPtr::new(metaloadfn("glDebugMessageInsert", &["glDebugMessageInsertARB", "glDebugMessageInsertKHR"])),
DeleteBuffers: FnPtr::new(metaloadfn("glDeleteBuffers", &["glDeleteBuffersARB"])),
DeleteFramebuffers: FnPtr::new(metaloadfn("glDeleteFramebuffers", &["glDeleteFramebuffersEXT"])),
DeleteProgram: FnPtr::new(metaloadfn("glDeleteProgram", &[])),
DeleteProgramPipelines: FnPtr::new(metaloadfn("glDeleteProgramPipelines", &[])),
DeleteQueries: FnPtr::new(metaloadfn("glDeleteQueries", &["glDeleteQueriesARB"])),
DeleteRenderbuffers: FnPtr::new(metaloadfn("glDeleteRenderbuffers", &["glDeleteRenderbuffersEXT"])),
DeleteSamplers: FnPtr::new(metaloadfn("glDeleteSamplers", &[])),
DeleteShader: FnPtr::new(metaloadfn("glDeleteShader", &[])),
DeleteSync: FnPtr::new(metaloadfn("glDeleteSync", &["glDeleteSyncAPPLE"])),
DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
DeleteTransformFeedbacks: FnPtr::new(metaloadfn("glDeleteTransformFeedbacks", &["glDeleteTransformFeedbacksNV"])),
DeleteVertexArrays: FnPtr::new(metaloadfn("glDeleteVertexArrays", &["glDeleteVertexArraysAPPLE", "glDeleteVertexArraysOES"])),
DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
DepthRange: FnPtr::new(metaloadfn("glDepthRange", &[])),
DepthRangeArrayv: FnPtr::new(metaloadfn("glDepthRangeArrayv", &[])),
DepthRangeIndexed: FnPtr::new(metaloadfn("glDepthRangeIndexed", &[])),
DepthRangef: FnPtr::new(metaloadfn("glDepthRangef", &["glDepthRangefOES"])),
DetachShader: FnPtr::new(metaloadfn("glDetachShader", &["glDetachObjectARB"])),
Disable: FnPtr::new(metaloadfn("glDisable", &[])),
DisableVertexArrayAttrib: FnPtr::new(metaloadfn("glDisableVertexArrayAttrib", &[])),
DisableVertexAttribArray: FnPtr::new(metaloadfn("glDisableVertexAttribArray", &["glDisableVertexAttribArrayARB"])),
Disablei: FnPtr::new(metaloadfn("glDisablei", &["glDisableIndexedEXT", "glDisableiEXT", "glDisableiNV", "glDisableiOES"])),
DispatchCompute: FnPtr::new(metaloadfn("glDispatchCompute", &[])),
DispatchComputeIndirect: FnPtr::new(metaloadfn("glDispatchComputeIndirect", &[])),
DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &["glDrawArraysEXT"])),
DrawArraysIndirect: FnPtr::new(metaloadfn("glDrawArraysIndirect", &[])),
DrawArraysInstanced: FnPtr::new(metaloadfn("glDrawArraysInstanced", &["glDrawArraysInstancedANGLE", "glDrawArraysInstancedARB", "glDrawArraysInstancedEXT", "glDrawArraysInstancedNV"])),
DrawArraysInstancedBaseInstance: FnPtr::new(metaloadfn("glDrawArraysInstancedBaseInstance", &["glDrawArraysInstancedBaseInstanceEXT"])),
DrawBuffer: FnPtr::new(metaloadfn("glDrawBuffer", &[])),
DrawBuffers: FnPtr::new(metaloadfn("glDrawBuffers", &["glDrawBuffersARB", "glDrawBuffersATI", "glDrawBuffersEXT"])),
DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
DrawElementsBaseVertex: FnPtr::new(metaloadfn("glDrawElementsBaseVertex", &["glDrawElementsBaseVertexEXT", "glDrawElementsBaseVertexOES"])),
DrawElementsIndirect: FnPtr::new(metaloadfn("glDrawElementsIndirect", &[])),
DrawElementsInstanced: FnPtr::new(metaloadfn("glDrawElementsInstanced", &["glDrawElementsInstancedANGLE", "glDrawElementsInstancedARB", "glDrawElementsInstancedEXT", "glDrawElementsInstancedNV"])),
DrawElementsInstancedBaseInstance: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseInstance", &["glDrawElementsInstancedBaseInstanceEXT"])),
DrawElementsInstancedBaseVertex: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseVertex", &["glDrawElementsInstancedBaseVertexEXT", "glDrawElementsInstancedBaseVertexOES"])),
DrawElementsInstancedBaseVertexBaseInstance: FnPtr::new(metaloadfn("glDrawElementsInstancedBaseVertexBaseInstance", &["glDrawElementsInstancedBaseVertexBaseInstanceEXT"])),
DrawRangeElements: FnPtr::new(metaloadfn("glDrawRangeElements", &["glDrawRangeElementsEXT"])),
DrawRangeElementsBaseVertex: FnPtr::new(metaloadfn("glDrawRangeElementsBaseVertex", &["glDrawRangeElementsBaseVertexEXT", "glDrawRangeElementsBaseVertexOES"])),
DrawTransformFeedback: FnPtr::new(metaloadfn("glDrawTransformFeedback", &["glDrawTransformFeedbackEXT", "glDrawTransformFeedbackNV"])),
DrawTransformFeedbackInstanced: FnPtr::new(metaloadfn("glDrawTransformFeedbackInstanced", &["glDrawTransformFeedbackInstancedEXT"])),
DrawTransformFeedbackStream: FnPtr::new(metaloadfn("glDrawTransformFeedbackStream", &[])),
DrawTransformFeedbackStreamInstanced: FnPtr::new(metaloadfn("glDrawTransformFeedbackStreamInstanced", &[])),
Enable: FnPtr::new(metaloadfn("glEnable", &[])),
EnableVertexArrayAttrib: FnPtr::new(metaloadfn("glEnableVertexArrayAttrib", &[])),
EnableVertexAttribArray: FnPtr::new(metaloadfn("glEnableVertexAttribArray", &["glEnableVertexAttribArrayARB"])),
Enablei: FnPtr::new(metaloadfn("glEnablei", &["glEnableIndexedEXT", "glEnableiEXT", "glEnableiNV", "glEnableiOES"])),
EndConditionalRender: FnPtr::new(metaloadfn("glEndConditionalRender", &["glEndConditionalRenderNV", "glEndConditionalRenderNVX"])),
EndQuery: FnPtr::new(metaloadfn("glEndQuery", &["glEndQueryARB"])),
EndQueryIndexed: FnPtr::new(metaloadfn("glEndQueryIndexed", &[])),
EndTransformFeedback: FnPtr::new(metaloadfn("glEndTransformFeedback", &["glEndTransformFeedbackEXT", "glEndTransformFeedbackNV"])),
FenceSync: FnPtr::new(metaloadfn("glFenceSync", &["glFenceSyncAPPLE"])),
Finish: FnPtr::new(metaloadfn("glFinish", &[])),
Flush: FnPtr::new(metaloadfn("glFlush", &[])),
FlushMappedBufferRange: FnPtr::new(metaloadfn("glFlushMappedBufferRange", &["glFlushMappedBufferRangeAPPLE", "glFlushMappedBufferRangeEXT"])),
FlushMappedNamedBufferRange: FnPtr::new(metaloadfn("glFlushMappedNamedBufferRange", &[])),
FramebufferParameteri: FnPtr::new(metaloadfn("glFramebufferParameteri", &[])),
FramebufferRenderbuffer: FnPtr::new(metaloadfn("glFramebufferRenderbuffer", &["glFramebufferRenderbufferEXT"])),
FramebufferTexture: FnPtr::new(metaloadfn("glFramebufferTexture", &["glFramebufferTextureARB", "glFramebufferTextureEXT", "glFramebufferTextureOES"])),
FramebufferTexture1D: FnPtr::new(metaloadfn("glFramebufferTexture1D", &["glFramebufferTexture1DEXT"])),
FramebufferTexture2D: FnPtr::new(metaloadfn("glFramebufferTexture2D", &["glFramebufferTexture2DEXT"])),
FramebufferTexture3D: FnPtr::new(metaloadfn("glFramebufferTexture3D", &["glFramebufferTexture3DEXT"])),
FramebufferTextureLayer: FnPtr::new(metaloadfn("glFramebufferTextureLayer", &["glFramebufferTextureLayerARB", "glFramebufferTextureLayerEXT"])),
FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
GenBuffers: FnPtr::new(metaloadfn("glGenBuffers", &["glGenBuffersARB"])),
GenFramebuffers: FnPtr::new(metaloadfn("glGenFramebuffers", &["glGenFramebuffersEXT"])),
GenProgramPipelines: FnPtr::new(metaloadfn("glGenProgramPipelines", &[])),
GenQueries: FnPtr::new(metaloadfn("glGenQueries", &["glGenQueriesARB"])),
GenRenderbuffers: FnPtr::new(metaloadfn("glGenRenderbuffers", &["glGenRenderbuffersEXT"])),
GenSamplers: FnPtr::new(metaloadfn("glGenSamplers", &[])),
GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
GenTransformFeedbacks: FnPtr::new(metaloadfn("glGenTransformFeedbacks", &["glGenTransformFeedbacksNV"])),
GenVertexArrays: FnPtr::new(metaloadfn("glGenVertexArrays", &["glGenVertexArraysAPPLE", "glGenVertexArraysOES"])),
GenerateMipmap: FnPtr::new(metaloadfn("glGenerateMipmap", &["glGenerateMipmapEXT"])),
GenerateTextureMipmap: FnPtr::new(metaloadfn("glGenerateTextureMipmap", &[])),
GetActiveAtomicCounterBufferiv: FnPtr::new(metaloadfn("glGetActiveAtomicCounterBufferiv", &[])),
GetActiveAttrib: FnPtr::new(metaloadfn("glGetActiveAttrib", &["glGetActiveAttribARB"])),
GetActiveSubroutineName: FnPtr::new(metaloadfn("glGetActiveSubroutineName", &[])),
GetActiveSubroutineUniformName: FnPtr::new(metaloadfn("glGetActiveSubroutineUniformName", &[])),
GetActiveSubroutineUniformiv: FnPtr::new(metaloadfn("glGetActiveSubroutineUniformiv", &[])),
GetActiveUniform: FnPtr::new(metaloadfn("glGetActiveUniform", &["glGetActiveUniformARB"])),
GetActiveUniformBlockName: FnPtr::new(metaloadfn("glGetActiveUniformBlockName", &[])),
GetActiveUniformBlockiv: FnPtr::new(metaloadfn("glGetActiveUniformBlockiv", &[])),
GetActiveUniformName: FnPtr::new(metaloadfn("glGetActiveUniformName", &[])),
GetActiveUniformsiv: FnPtr::new(metaloadfn("glGetActiveUniformsiv", &[])),
GetAttachedShaders: FnPtr::new(metaloadfn("glGetAttachedShaders", &[])),
GetAttribLocation: FnPtr::new(metaloadfn("glGetAttribLocation", &["glGetAttribLocationARB"])),
GetBooleani_v: FnPtr::new(metaloadfn("glGetBooleani_v", &["glGetBooleanIndexedvEXT"])),
GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
GetBufferParameteri64v: FnPtr::new(metaloadfn("glGetBufferParameteri64v", &[])),
GetBufferParameteriv: FnPtr::new(metaloadfn("glGetBufferParameteriv", &["glGetBufferParameterivARB"])),
GetBufferPointerv: FnPtr::new(metaloadfn("glGetBufferPointerv", &["glGetBufferPointervARB", "glGetBufferPointervOES"])),
GetBufferSubData: FnPtr::new(metaloadfn("glGetBufferSubData", &["glGetBufferSubDataARB"])),
GetCompressedTexImage: FnPtr::new(metaloadfn("glGetCompressedTexImage", &["glGetCompressedTexImageARB"])),
GetCompressedTextureImage: FnPtr::new(metaloadfn("glGetCompressedTextureImage", &[])),
GetCompressedTextureSubImage: FnPtr::new(metaloadfn("glGetCompressedTextureSubImage", &[])),
GetDebugMessageLog: FnPtr::new(metaloadfn("glGetDebugMessageLog", &["glGetDebugMessageLogARB", "glGetDebugMessageLogKHR"])),
GetDoublei_v: FnPtr::new(metaloadfn("glGetDoublei_v", &["glGetDoubleIndexedvEXT", "glGetDoublei_vEXT"])),
GetDoublev: FnPtr::new(metaloadfn("glGetDoublev", &[])),
GetError: FnPtr::new(metaloadfn("glGetError", &[])),
GetFloati_v: FnPtr::new(metaloadfn("glGetFloati_v", &["glGetFloatIndexedvEXT", "glGetFloati_vEXT", "glGetFloati_vNV", "glGetFloati_vOES"])),
GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
GetFragDataIndex: FnPtr::new(metaloadfn("glGetFragDataIndex", &["glGetFragDataIndexEXT"])),
GetFragDataLocation: FnPtr::new(metaloadfn("glGetFragDataLocation", &["glGetFragDataLocationEXT"])),
GetFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn("glGetFramebufferAttachmentParameteriv", &["glGetFramebufferAttachmentParameterivEXT"])),
GetFramebufferParameteriv: FnPtr::new(metaloadfn("glGetFramebufferParameteriv", &[])),
GetGraphicsResetStatus: FnPtr::new(metaloadfn("glGetGraphicsResetStatus", &["glGetGraphicsResetStatusEXT", "glGetGraphicsResetStatusKHR"])),
GetInteger64i_v: FnPtr::new(metaloadfn("glGetInteger64i_v", &[])),
GetInteger64v: FnPtr::new(metaloadfn("glGetInteger64v", &["glGetInteger64vAPPLE"])),
GetIntegeri_v: FnPtr::new(metaloadfn("glGetIntegeri_v", &["glGetIntegerIndexedvEXT"])),
GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
GetInternalformati64v: FnPtr::new(metaloadfn("glGetInternalformati64v", &[])),
GetInternalformativ: FnPtr::new(metaloadfn("glGetInternalformativ", &[])),
GetMultisamplefv: FnPtr::new(metaloadfn("glGetMultisamplefv", &["glGetMultisamplefvNV"])),
GetNamedBufferParameteri64v: FnPtr::new(metaloadfn("glGetNamedBufferParameteri64v", &[])),
GetNamedBufferParameteriv: FnPtr::new(metaloadfn("glGetNamedBufferParameteriv", &[])),
GetNamedBufferPointerv: FnPtr::new(metaloadfn("glGetNamedBufferPointerv", &[])),
GetNamedBufferSubData: FnPtr::new(metaloadfn("glGetNamedBufferSubData", &[])),
GetNamedFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn("glGetNamedFramebufferAttachmentParameteriv", &[])),
GetNamedFramebufferParameteriv: FnPtr::new(metaloadfn("glGetNamedFramebufferParameteriv", &[])),
GetNamedRenderbufferParameteriv: FnPtr::new(metaloadfn("glGetNamedRenderbufferParameteriv", &[])),
GetObjectLabel: FnPtr::new(metaloadfn("glGetObjectLabel", &["glGetObjectLabelKHR"])),
GetObjectPtrLabel: FnPtr::new(metaloadfn("glGetObjectPtrLabel", &["glGetObjectPtrLabelKHR"])),
GetPointerv: FnPtr::new(metaloadfn("glGetPointerv", &["glGetPointervEXT", "glGetPointervKHR"])),
GetProgramBinary: FnPtr::new(metaloadfn("glGetProgramBinary", &["glGetProgramBinaryOES"])),
GetProgramInfoLog: FnPtr::new(metaloadfn("glGetProgramInfoLog", &[])),
GetProgramInterfaceiv: FnPtr::new(metaloadfn("glGetProgramInterfaceiv", &[])),
GetProgramPipelineInfoLog: FnPtr::new(metaloadfn("glGetProgramPipelineInfoLog", &[])),
GetProgramPipelineiv: FnPtr::new(metaloadfn("glGetProgramPipelineiv", &[])),
GetProgramResourceIndex: FnPtr::new(metaloadfn("glGetProgramResourceIndex", &[])),
GetProgramResourceLocation: FnPtr::new(metaloadfn("glGetProgramResourceLocation", &[])),
GetProgramResourceLocationIndex: FnPtr::new(metaloadfn("glGetProgramResourceLocationIndex", &[])),
GetProgramResourceName: FnPtr::new(metaloadfn("glGetProgramResourceName", &[])),
GetProgramResourceiv: FnPtr::new(metaloadfn("glGetProgramResourceiv", &[])),
GetProgramStageiv: FnPtr::new(metaloadfn("glGetProgramStageiv", &[])),
GetProgramiv: FnPtr::new(metaloadfn("glGetProgramiv", &[])),
GetQueryBufferObjecti64v: FnPtr::new(metaloadfn("glGetQueryBufferObjecti64v", &[])),
GetQueryBufferObjectiv: FnPtr::new(metaloadfn("glGetQueryBufferObjectiv", &[])),
GetQueryBufferObjectui64v: FnPtr::new(metaloadfn("glGetQueryBufferObjectui64v", &[])),
GetQueryBufferObjectuiv: FnPtr::new(metaloadfn("glGetQueryBufferObjectuiv", &[])),
GetQueryIndexediv: FnPtr::new(metaloadfn("glGetQueryIndexediv", &[])),
GetQueryObjecti64v: FnPtr::new(metaloadfn("glGetQueryObjecti64v", &["glGetQueryObjecti64vEXT"])),
GetQueryObjectiv: FnPtr::new(metaloadfn("glGetQueryObjectiv", &["glGetQueryObjectivARB", "glGetQueryObjectivEXT"])),
GetQueryObjectui64v: FnPtr::new(metaloadfn("glGetQueryObjectui64v", &["glGetQueryObjectui64vEXT"])),
GetQueryObjectuiv: FnPtr::new(metaloadfn("glGetQueryObjectuiv", &["glGetQueryObjectuivARB"])),
GetQueryiv: FnPtr::new(metaloadfn("glGetQueryiv", &["glGetQueryivARB"])),
GetRenderbufferParameteriv: FnPtr::new(metaloadfn("glGetRenderbufferParameteriv", &["glGetRenderbufferParameterivEXT"])),
GetSamplerParameterIiv: FnPtr::new(metaloadfn("glGetSamplerParameterIiv", &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIivOES"])),
GetSamplerParameterIuiv: FnPtr::new(metaloadfn("glGetSamplerParameterIuiv", &["glGetSamplerParameterIuivEXT", "glGetSamplerParameterIuivOES"])),
GetSamplerParameterfv: FnPtr::new(metaloadfn("glGetSamplerParameterfv", &[])),
GetSamplerParameteriv: FnPtr::new(metaloadfn("glGetSamplerParameteriv", &[])),
GetShaderInfoLog: FnPtr::new(metaloadfn("glGetShaderInfoLog", &[])),
GetShaderPrecisionFormat: FnPtr::new(metaloadfn("glGetShaderPrecisionFormat", &[])),
GetShaderSource: FnPtr::new(metaloadfn("glGetShaderSource", &["glGetShaderSourceARB"])),
GetShaderiv: FnPtr::new(metaloadfn("glGetShaderiv", &[])),
GetString: FnPtr::new(metaloadfn("glGetString", &[])),
GetStringi: FnPtr::new(metaloadfn("glGetStringi", &[])),
GetSubroutineIndex: FnPtr::new(metaloadfn("glGetSubroutineIndex", &[])),
GetSubroutineUniformLocation: FnPtr::new(metaloadfn("glGetSubroutineUniformLocation", &[])),
GetSynciv: FnPtr::new(metaloadfn("glGetSynciv", &["glGetSyncivAPPLE"])),
GetTexImage: FnPtr::new(metaloadfn("glGetTexImage", &[])),
GetTexLevelParameterfv: FnPtr::new(metaloadfn("glGetTexLevelParameterfv", &[])),
GetTexLevelParameteriv: FnPtr::new(metaloadfn("glGetTexLevelParameteriv", &[])),
GetTexParameterIiv: FnPtr::new(metaloadfn("glGetTexParameterIiv", &["glGetTexParameterIivEXT", "glGetTexParameterIivOES"])),
GetTexParameterIuiv: FnPtr::new(metaloadfn("glGetTexParameterIuiv", &["glGetTexParameterIuivEXT", "glGetTexParameterIuivOES"])),
GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
GetTextureImage: FnPtr::new(metaloadfn("glGetTextureImage", &[])),
GetTextureLevelParameterfv: FnPtr::new(metaloadfn("glGetTextureLevelParameterfv", &[])),
GetTextureLevelParameteriv: FnPtr::new(metaloadfn("glGetTextureLevelParameteriv", &[])),
GetTextureParameterIiv: FnPtr::new(metaloadfn("glGetTextureParameterIiv", &[])),
GetTextureParameterIuiv: FnPtr::new(metaloadfn("glGetTextureParameterIuiv", &[])),
GetTextureParameterfv: FnPtr::new(metaloadfn("glGetTextureParameterfv", &[])),
GetTextureParameteriv: FnPtr::new(metaloadfn("glGetTextureParameteriv", &[])),
GetTextureSubImage: FnPtr::new(metaloadfn("glGetTextureSubImage", &[])),
GetTransformFeedbackVarying: FnPtr::new(metaloadfn("glGetTransformFeedbackVarying", &["glGetTransformFeedbackVaryingEXT"])),
GetTransformFeedbacki64_v: FnPtr::new(metaloadfn("glGetTransformFeedbacki64_v", &[])),
GetTransformFeedbacki_v: FnPtr::new(metaloadfn("glGetTransformFeedbacki_v", &[])),
GetTransformFeedbackiv: FnPtr::new(metaloadfn("glGetTransformFeedbackiv", &[])),
GetUniformBlockIndex: FnPtr::new(metaloadfn("glGetUniformBlockIndex", &[])),
GetUniformIndices: FnPtr::new(metaloadfn("glGetUniformIndices", &[])),
GetUniformLocation: FnPtr::new(metaloadfn("glGetUniformLocation", &["glGetUniformLocationARB"])),
GetUniformSubroutineuiv: FnPtr::new(metaloadfn("glGetUniformSubroutineuiv", &[])),
GetUniformdv: FnPtr::new(metaloadfn("glGetUniformdv", &[])),
GetUniformfv: FnPtr::new(metaloadfn("glGetUniformfv", &["glGetUniformfvARB"])),
GetUniformiv: FnPtr::new(metaloadfn("glGetUniformiv", &["glGetUniformivARB"])),
GetUniformuiv: FnPtr::new(metaloadfn("glGetUniformuiv", &["glGetUniformuivEXT"])),
GetVertexArrayIndexed64iv: FnPtr::new(metaloadfn("glGetVertexArrayIndexed64iv", &[])),
GetVertexArrayIndexediv: FnPtr::new(metaloadfn("glGetVertexArrayIndexediv", &[])),
GetVertexArrayiv: FnPtr::new(metaloadfn("glGetVertexArrayiv", &[])),
GetVertexAttribIiv: FnPtr::new(metaloadfn("glGetVertexAttribIiv", &["glGetVertexAttribIivEXT"])),
GetVertexAttribIuiv: FnPtr::new(metaloadfn("glGetVertexAttribIuiv", &["glGetVertexAttribIuivEXT"])),
GetVertexAttribLdv: FnPtr::new(metaloadfn("glGetVertexAttribLdv", &["glGetVertexAttribLdvEXT"])),
GetVertexAttribPointerv: FnPtr::new(metaloadfn("glGetVertexAttribPointerv", &["glGetVertexAttribPointervARB", "glGetVertexAttribPointervNV"])),
GetVertexAttribdv: FnPtr::new(metaloadfn("glGetVertexAttribdv", &["glGetVertexAttribdvARB", "glGetVertexAttribdvNV"])),
GetVertexAttribfv: FnPtr::new(metaloadfn("glGetVertexAttribfv", &["glGetVertexAttribfvARB", "glGetVertexAttribfvNV"])),
GetVertexAttribiv: FnPtr::new(metaloadfn("glGetVertexAttribiv", &["glGetVertexAttribivARB", "glGetVertexAttribivNV"])),
GetnColorTable: FnPtr::new(metaloadfn("glGetnColorTable", &[])),
GetnCompressedTexImage: FnPtr::new(metaloadfn("glGetnCompressedTexImage", &[])),
GetnConvolutionFilter: FnPtr::new(metaloadfn("glGetnConvolutionFilter", &[])),
GetnHistogram: FnPtr::new(metaloadfn("glGetnHistogram", &[])),
GetnMapdv: FnPtr::new(metaloadfn("glGetnMapdv", &[])),
GetnMapfv: FnPtr::new(metaloadfn("glGetnMapfv", &[])),
GetnMapiv: FnPtr::new(metaloadfn("glGetnMapiv", &[])),
GetnMinmax: FnPtr::new(metaloadfn("glGetnMinmax", &[])),
GetnPixelMapfv: FnPtr::new(metaloadfn("glGetnPixelMapfv", &[])),
GetnPixelMapuiv: FnPtr::new(metaloadfn("glGetnPixelMapuiv", &[])),
GetnPixelMapusv: FnPtr::new(metaloadfn("glGetnPixelMapusv", &[])),
GetnPolygonStipple: FnPtr::new(metaloadfn("glGetnPolygonStipple", &[])),
GetnSeparableFilter: FnPtr::new(metaloadfn("glGetnSeparableFilter", &[])),
GetnTexImage: FnPtr::new(metaloadfn("glGetnTexImage", &[])),
GetnUniformdv: FnPtr::new(metaloadfn("glGetnUniformdv", &[])),
GetnUniformfv: FnPtr::new(metaloadfn("glGetnUniformfv", &["glGetnUniformfvEXT", "glGetnUniformfvKHR"])),
GetnUniformiv: FnPtr::new(metaloadfn("glGetnUniformiv", &["glGetnUniformivEXT", "glGetnUniformivKHR"])),
GetnUniformuiv: FnPtr::new(metaloadfn("glGetnUniformuiv", &["glGetnUniformuivKHR"])),
Hint: FnPtr::new(metaloadfn("glHint", &[])),
InvalidateBufferData: FnPtr::new(metaloadfn("glInvalidateBufferData", &[])),
InvalidateBufferSubData: FnPtr::new(metaloadfn("glInvalidateBufferSubData", &[])),
InvalidateFramebuffer: FnPtr::new(metaloadfn("glInvalidateFramebuffer", &[])),
InvalidateNamedFramebufferData: FnPtr::new(metaloadfn("glInvalidateNamedFramebufferData", &[])),
InvalidateNamedFramebufferSubData: FnPtr::new(metaloadfn("glInvalidateNamedFramebufferSubData", &[])),
InvalidateSubFramebuffer: FnPtr::new(metaloadfn("glInvalidateSubFramebuffer", &[])),
InvalidateTexImage: FnPtr::new(metaloadfn("glInvalidateTexImage", &[])),
InvalidateTexSubImage: FnPtr::new(metaloadfn("glInvalidateTexSubImage", &[])),
IsBuffer: FnPtr::new(metaloadfn("glIsBuffer", &["glIsBufferARB"])),
IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
IsEnabledi: FnPtr::new(metaloadfn("glIsEnabledi", &["glIsEnabledIndexedEXT", "glIsEnablediEXT", "glIsEnablediNV", "glIsEnablediOES"])),
IsFramebuffer: FnPtr::new(metaloadfn("glIsFramebuffer", &["glIsFramebufferEXT"])),
IsProgram: FnPtr::new(metaloadfn("glIsProgram", &[])),
IsProgramPipeline: FnPtr::new(metaloadfn("glIsProgramPipeline", &[])),
IsQuery: FnPtr::new(metaloadfn("glIsQuery", &["glIsQueryARB"])),
IsRenderbuffer: FnPtr::new(metaloadfn("glIsRenderbuffer", &["glIsRenderbufferEXT"])),
IsSampler: FnPtr::new(metaloadfn("glIsSampler", &[])),
IsShader: FnPtr::new(metaloadfn("glIsShader", &[])),
IsSync: FnPtr::new(metaloadfn("glIsSync", &["glIsSyncAPPLE"])),
IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
IsTransformFeedback: FnPtr::new(metaloadfn("glIsTransformFeedback", &["glIsTransformFeedbackNV"])),
IsVertexArray: FnPtr::new(metaloadfn("glIsVertexArray", &["glIsVertexArrayAPPLE", "glIsVertexArrayOES"])),
LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
LinkProgram: FnPtr::new(metaloadfn("glLinkProgram", &["glLinkProgramARB"])),
LogicOp: FnPtr::new(metaloadfn("glLogicOp", &[])),
MapBuffer: FnPtr::new(metaloadfn("glMapBuffer", &["glMapBufferARB", "glMapBufferOES"])),
MapBufferRange: FnPtr::new(metaloadfn("glMapBufferRange", &["glMapBufferRangeEXT"])),
MapNamedBuffer: FnPtr::new(metaloadfn("glMapNamedBuffer", &[])),
MapNamedBufferRange: FnPtr::new(metaloadfn("glMapNamedBufferRange", &[])),
MemoryBarrier: FnPtr::new(metaloadfn("glMemoryBarrier", &["glMemoryBarrierEXT"])),
MemoryBarrierByRegion: FnPtr::new(metaloadfn("glMemoryBarrierByRegion", &[])),
MinSampleShading: FnPtr::new(metaloadfn("glMinSampleShading", &["glMinSampleShadingARB", "glMinSampleShadingOES"])),
MultiDrawArrays: FnPtr::new(metaloadfn("glMultiDrawArrays", &["glMultiDrawArraysEXT"])),
MultiDrawArraysIndirect: FnPtr::new(metaloadfn("glMultiDrawArraysIndirect", &["glMultiDrawArraysIndirectAMD", "glMultiDrawArraysIndirectEXT"])),
MultiDrawElements: FnPtr::new(metaloadfn("glMultiDrawElements", &["glMultiDrawElementsEXT"])),
MultiDrawElementsBaseVertex: FnPtr::new(metaloadfn("glMultiDrawElementsBaseVertex", &["glMultiDrawElementsBaseVertexEXT"])),
MultiDrawElementsIndirect: FnPtr::new(metaloadfn("glMultiDrawElementsIndirect", &["glMultiDrawElementsIndirectAMD", "glMultiDrawElementsIndirectEXT"])),
MultiTexCoordP1ui: FnPtr::new(metaloadfn("glMultiTexCoordP1ui", &[])),
MultiTexCoordP1uiv: FnPtr::new(metaloadfn("glMultiTexCoordP1uiv", &[])),
MultiTexCoordP2ui: FnPtr::new(metaloadfn("glMultiTexCoordP2ui", &[])),
MultiTexCoordP2uiv: FnPtr::new(metaloadfn("glMultiTexCoordP2uiv", &[])),
MultiTexCoordP3ui: FnPtr::new(metaloadfn("glMultiTexCoordP3ui", &[])),
MultiTexCoordP3uiv: FnPtr::new(metaloadfn("glMultiTexCoordP3uiv", &[])),
MultiTexCoordP4ui: FnPtr::new(metaloadfn("glMultiTexCoordP4ui", &[])),
MultiTexCoordP4uiv: FnPtr::new(metaloadfn("glMultiTexCoordP4uiv", &[])),
NamedBufferData: FnPtr::new(metaloadfn("glNamedBufferData", &[])),
NamedBufferStorage: FnPtr::new(metaloadfn("glNamedBufferStorage", &["glNamedBufferStorageEXT"])),
NamedBufferSubData: FnPtr::new(metaloadfn("glNamedBufferSubData", &["glNamedBufferSubDataEXT"])),
NamedFramebufferDrawBuffer: FnPtr::new(metaloadfn("glNamedFramebufferDrawBuffer", &[])),
NamedFramebufferDrawBuffers: FnPtr::new(metaloadfn("glNamedFramebufferDrawBuffers", &[])),
NamedFramebufferParameteri: FnPtr::new(metaloadfn("glNamedFramebufferParameteri", &[])),
NamedFramebufferReadBuffer: FnPtr::new(metaloadfn("glNamedFramebufferReadBuffer", &[])),
NamedFramebufferRenderbuffer: FnPtr::new(metaloadfn("glNamedFramebufferRenderbuffer", &[])),
NamedFramebufferTexture: FnPtr::new(metaloadfn("glNamedFramebufferTexture", &[])),
NamedFramebufferTextureLayer: FnPtr::new(metaloadfn("glNamedFramebufferTextureLayer", &[])),
NamedRenderbufferStorage: FnPtr::new(metaloadfn("glNamedRenderbufferStorage", &[])),
NamedRenderbufferStorageMultisample: FnPtr::new(metaloadfn("glNamedRenderbufferStorageMultisample", &[])),
NormalP3ui: FnPtr::new(metaloadfn("glNormalP3ui", &[])),
NormalP3uiv: FnPtr::new(metaloadfn("glNormalP3uiv", &[])),
ObjectLabel: FnPtr::new(metaloadfn("glObjectLabel", &["glObjectLabelKHR"])),
ObjectPtrLabel: FnPtr::new(metaloadfn("glObjectPtrLabel", &["glObjectPtrLabelKHR"])),
PatchParameterfv: FnPtr::new(metaloadfn("glPatchParameterfv", &[])),
PatchParameteri: FnPtr::new(metaloadfn("glPatchParameteri", &["glPatchParameteriEXT", "glPatchParameteriOES"])),
PauseTransformFeedback: FnPtr::new(metaloadfn("glPauseTransformFeedback", &["glPauseTransformFeedbackNV"])),
PixelStoref: FnPtr::new(metaloadfn("glPixelStoref", &[])),
PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
PointParameterf: FnPtr::new(metaloadfn("glPointParameterf", &["glPointParameterfARB", "glPointParameterfEXT", "glPointParameterfSGIS"])),
PointParameterfv: FnPtr::new(metaloadfn("glPointParameterfv", &["glPointParameterfvARB", "glPointParameterfvEXT", "glPointParameterfvSGIS"])),
PointParameteri: FnPtr::new(metaloadfn("glPointParameteri", &["glPointParameteriNV"])),
PointParameteriv: FnPtr::new(metaloadfn("glPointParameteriv", &["glPointParameterivNV"])),
PointSize: FnPtr::new(metaloadfn("glPointSize", &[])),
PolygonMode: FnPtr::new(metaloadfn("glPolygonMode", &["glPolygonModeNV"])),
PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
PopDebugGroup: FnPtr::new(metaloadfn("glPopDebugGroup", &["glPopDebugGroupKHR"])),
PrimitiveRestartIndex: FnPtr::new(metaloadfn("glPrimitiveRestartIndex", &[])),
ProgramBinary: FnPtr::new(metaloadfn("glProgramBinary", &["glProgramBinaryOES"])),
ProgramParameteri: FnPtr::new(metaloadfn("glProgramParameteri", &["glProgramParameteriARB", "glProgramParameteriEXT"])),
ProgramUniform1d: FnPtr::new(metaloadfn("glProgramUniform1d", &[])),
ProgramUniform1dv: FnPtr::new(metaloadfn("glProgramUniform1dv", &[])),
ProgramUniform1f: FnPtr::new(metaloadfn("glProgramUniform1f", &["glProgramUniform1fEXT"])),
ProgramUniform1fv: FnPtr::new(metaloadfn("glProgramUniform1fv", &["glProgramUniform1fvEXT"])),
ProgramUniform1i: FnPtr::new(metaloadfn("glProgramUniform1i", &["glProgramUniform1iEXT"])),
ProgramUniform1iv: FnPtr::new(metaloadfn("glProgramUniform1iv", &["glProgramUniform1ivEXT"])),
ProgramUniform1ui: FnPtr::new(metaloadfn("glProgramUniform1ui", &["glProgramUniform1uiEXT"])),
ProgramUniform1uiv: FnPtr::new(metaloadfn("glProgramUniform1uiv", &["glProgramUniform1uivEXT"])),
ProgramUniform2d: FnPtr::new(metaloadfn("glProgramUniform2d", &[])),
ProgramUniform2dv: FnPtr::new(metaloadfn("glProgramUniform2dv", &[])),
ProgramUniform2f: FnPtr::new(metaloadfn("glProgramUniform2f", &["glProgramUniform2fEXT"])),
ProgramUniform2fv: FnPtr::new(metaloadfn("glProgramUniform2fv", &["glProgramUniform2fvEXT"])),
ProgramUniform2i: FnPtr::new(metaloadfn("glProgramUniform2i", &["glProgramUniform2iEXT"])),
ProgramUniform2iv: FnPtr::new(metaloadfn("glProgramUniform2iv", &["glProgramUniform2ivEXT"])),
ProgramUniform2ui: FnPtr::new(metaloadfn("glProgramUniform2ui", &["glProgramUniform2uiEXT"])),
ProgramUniform2uiv: FnPtr::new(metaloadfn("glProgramUniform2uiv", &["glProgramUniform2uivEXT"])),
ProgramUniform3d: FnPtr::new(metaloadfn("glProgramUniform3d", &[])),
ProgramUniform3dv: FnPtr::new(metaloadfn("glProgramUniform3dv", &[])),
ProgramUniform3f: FnPtr::new(metaloadfn("glProgramUniform3f", &["glProgramUniform3fEXT"])),
ProgramUniform3fv: FnPtr::new(metaloadfn("glProgramUniform3fv", &["glProgramUniform3fvEXT"])),
ProgramUniform3i: FnPtr::new(metaloadfn("glProgramUniform3i", &["glProgramUniform3iEXT"])),
ProgramUniform3iv: FnPtr::new(metaloadfn("glProgramUniform3iv", &["glProgramUniform3ivEXT"])),
ProgramUniform3ui: FnPtr::new(metaloadfn("glProgramUniform3ui", &["glProgramUniform3uiEXT"])),
ProgramUniform3uiv: FnPtr::new(metaloadfn("glProgramUniform3uiv", &["glProgramUniform3uivEXT"])),
ProgramUniform4d: FnPtr::new(metaloadfn("glProgramUniform4d", &[])),
ProgramUniform4dv: FnPtr::new(metaloadfn("glProgramUniform4dv", &[])),
ProgramUniform4f: FnPtr::new(metaloadfn("glProgramUniform4f", &["glProgramUniform4fEXT"])),
ProgramUniform4fv: FnPtr::new(metaloadfn("glProgramUniform4fv", &["glProgramUniform4fvEXT"])),
ProgramUniform4i: FnPtr::new(metaloadfn("glProgramUniform4i", &["glProgramUniform4iEXT"])),
ProgramUniform4iv: FnPtr::new(metaloadfn("glProgramUniform4iv", &["glProgramUniform4ivEXT"])),
ProgramUniform4ui: FnPtr::new(metaloadfn("glProgramUniform4ui", &["glProgramUniform4uiEXT"])),
ProgramUniform4uiv: FnPtr::new(metaloadfn("glProgramUniform4uiv", &["glProgramUniform4uivEXT"])),
ProgramUniformMatrix2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2dv", &[])),
ProgramUniformMatrix2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2fv", &["glProgramUniformMatrix2fvEXT"])),
ProgramUniformMatrix2x3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x3dv", &[])),
ProgramUniformMatrix2x3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x3fv", &["glProgramUniformMatrix2x3fvEXT"])),
ProgramUniformMatrix2x4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x4dv", &[])),
ProgramUniformMatrix2x4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix2x4fv", &["glProgramUniformMatrix2x4fvEXT"])),
ProgramUniformMatrix3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3dv", &[])),
ProgramUniformMatrix3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3fv", &["glProgramUniformMatrix3fvEXT"])),
ProgramUniformMatrix3x2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x2dv", &[])),
ProgramUniformMatrix3x2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x2fv", &["glProgramUniformMatrix3x2fvEXT"])),
ProgramUniformMatrix3x4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x4dv", &[])),
ProgramUniformMatrix3x4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix3x4fv", &["glProgramUniformMatrix3x4fvEXT"])),
ProgramUniformMatrix4dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4dv", &[])),
ProgramUniformMatrix4fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4fv", &["glProgramUniformMatrix4fvEXT"])),
ProgramUniformMatrix4x2dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x2dv", &[])),
ProgramUniformMatrix4x2fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x2fv", &["glProgramUniformMatrix4x2fvEXT"])),
ProgramUniformMatrix4x3dv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x3dv", &[])),
ProgramUniformMatrix4x3fv: FnPtr::new(metaloadfn("glProgramUniformMatrix4x3fv", &["glProgramUniformMatrix4x3fvEXT"])),
ProvokingVertex: FnPtr::new(metaloadfn("glProvokingVertex", &["glProvokingVertexEXT"])),
PushDebugGroup: FnPtr::new(metaloadfn("glPushDebugGroup", &["glPushDebugGroupKHR"])),
QueryCounter: FnPtr::new(metaloadfn("glQueryCounter", &["glQueryCounterEXT"])),
ReadBuffer: FnPtr::new(metaloadfn("glReadBuffer", &[])),
ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
ReadnPixels: FnPtr::new(metaloadfn("glReadnPixels", &["glReadnPixelsARB", "glReadnPixelsEXT", "glReadnPixelsKHR"])),
ReleaseShaderCompiler: FnPtr::new(metaloadfn("glReleaseShaderCompiler", &[])),
RenderbufferStorage: FnPtr::new(metaloadfn("glRenderbufferStorage", &["glRenderbufferStorageEXT"])),
RenderbufferStorageMultisample: FnPtr::new(metaloadfn("glRenderbufferStorageMultisample", &["glRenderbufferStorageMultisampleEXT", "glRenderbufferStorageMultisampleNV"])),
ResumeTransformFeedback: FnPtr::new(metaloadfn("glResumeTransformFeedback", &["glResumeTransformFeedbackNV"])),
SampleCoverage: FnPtr::new(metaloadfn("glSampleCoverage", &["glSampleCoverageARB"])),
SampleMaski: FnPtr::new(metaloadfn("glSampleMaski", &[])),
SamplerParameterIiv: FnPtr::new(metaloadfn("glSamplerParameterIiv", &["glSamplerParameterIivEXT", "glSamplerParameterIivOES"])),
SamplerParameterIuiv: FnPtr::new(metaloadfn("glSamplerParameterIuiv", &["glSamplerParameterIuivEXT", "glSamplerParameterIuivOES"])),
SamplerParameterf: FnPtr::new(metaloadfn("glSamplerParameterf", &[])),
SamplerParameterfv: FnPtr::new(metaloadfn("glSamplerParameterfv", &[])),
SamplerParameteri: FnPtr::new(metaloadfn("glSamplerParameteri", &[])),
SamplerParameteriv: FnPtr::new(metaloadfn("glSamplerParameteriv", &[])),
Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
ScissorArrayv: FnPtr::new(metaloadfn("glScissorArrayv", &["glScissorArrayvNV", "glScissorArrayvOES"])),
ScissorIndexed: FnPtr::new(metaloadfn("glScissorIndexed", &["glScissorIndexedNV", "glScissorIndexedOES"])),
ScissorIndexedv: FnPtr::new(metaloadfn("glScissorIndexedv", &["glScissorIndexedvNV", "glScissorIndexedvOES"])),
SecondaryColorP3ui: FnPtr::new(metaloadfn("glSecondaryColorP3ui", &[])),
SecondaryColorP3uiv: FnPtr::new(metaloadfn("glSecondaryColorP3uiv", &[])),
ShaderBinary: FnPtr::new(metaloadfn("glShaderBinary", &[])),
ShaderSource: FnPtr::new(metaloadfn("glShaderSource", &["glShaderSourceARB"])),
ShaderStorageBlockBinding: FnPtr::new(metaloadfn("glShaderStorageBlockBinding", &[])),
StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
StencilFuncSeparate: FnPtr::new(metaloadfn("glStencilFuncSeparate", &[])),
StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
StencilMaskSeparate: FnPtr::new(metaloadfn("glStencilMaskSeparate", &[])),
StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
StencilOpSeparate: FnPtr::new(metaloadfn("glStencilOpSeparate", &["glStencilOpSeparateATI"])),
TexBuffer: FnPtr::new(metaloadfn("glTexBuffer", &["glTexBufferARB", "glTexBufferEXT", "glTexBufferOES"])),
TexBufferRange: FnPtr::new(metaloadfn("glTexBufferRange", &["glTexBufferRangeEXT", "glTexBufferRangeOES"])),
TexCoordP1ui: FnPtr::new(metaloadfn("glTexCoordP1ui", &[])),
TexCoordP1uiv: FnPtr::new(metaloadfn("glTexCoordP1uiv", &[])),
TexCoordP2ui: FnPtr::new(metaloadfn("glTexCoordP2ui", &[])),
TexCoordP2uiv: FnPtr::new(metaloadfn("glTexCoordP2uiv", &[])),
TexCoordP3ui: FnPtr::new(metaloadfn("glTexCoordP3ui", &[])),
TexCoordP3uiv: FnPtr::new(metaloadfn("glTexCoordP3uiv", &[])),
TexCoordP4ui: FnPtr::new(metaloadfn("glTexCoordP4ui", &[])),
TexCoordP4uiv: FnPtr::new(metaloadfn("glTexCoordP4uiv", &[])),
TexImage1D: FnPtr::new(metaloadfn("glTexImage1D", &[])),
TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
TexImage2DMultisample: FnPtr::new(metaloadfn("glTexImage2DMultisample", &[])),
TexImage3D: FnPtr::new(metaloadfn("glTexImage3D", &["glTexImage3DEXT"])),
TexImage3DMultisample: FnPtr::new(metaloadfn("glTexImage3DMultisample", &[])),
TexParameterIiv: FnPtr::new(metaloadfn("glTexParameterIiv", &["glTexParameterIivEXT", "glTexParameterIivOES"])),
TexParameterIuiv: FnPtr::new(metaloadfn("glTexParameterIuiv", &["glTexParameterIuivEXT", "glTexParameterIuivOES"])),
TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
TexStorage1D: FnPtr::new(metaloadfn("glTexStorage1D", &["glTexStorage1DEXT"])),
TexStorage2D: FnPtr::new(metaloadfn("glTexStorage2D", &["glTexStorage2DEXT"])),
TexStorage2DMultisample: FnPtr::new(metaloadfn("glTexStorage2DMultisample", &[])),
TexStorage3D: FnPtr::new(metaloadfn("glTexStorage3D", &["glTexStorage3DEXT"])),
TexStorage3DMultisample: FnPtr::new(metaloadfn("glTexStorage3DMultisample", &["glTexStorage3DMultisampleOES"])),
TexSubImage1D: FnPtr::new(metaloadfn("glTexSubImage1D", &["glTexSubImage1DEXT"])),
TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &["glTexSubImage2DEXT"])),
TexSubImage3D: FnPtr::new(metaloadfn("glTexSubImage3D", &["glTexSubImage3DEXT"])),
TextureBarrier: FnPtr::new(metaloadfn("glTextureBarrier", &[])),
TextureBuffer: FnPtr::new(metaloadfn("glTextureBuffer", &[])),
TextureBufferRange: FnPtr::new(metaloadfn("glTextureBufferRange", &[])),
TextureParameterIiv: FnPtr::new(metaloadfn("glTextureParameterIiv", &[])),
TextureParameterIuiv: FnPtr::new(metaloadfn("glTextureParameterIuiv", &[])),
TextureParameterf: FnPtr::new(metaloadfn("glTextureParameterf", &[])),
TextureParameterfv: FnPtr::new(metaloadfn("glTextureParameterfv", &[])),
TextureParameteri: FnPtr::new(metaloadfn("glTextureParameteri", &[])),
TextureParameteriv: FnPtr::new(metaloadfn("glTextureParameteriv", &[])),
TextureStorage1D: FnPtr::new(metaloadfn("glTextureStorage1D", &[])),
TextureStorage2D: FnPtr::new(metaloadfn("glTextureStorage2D", &[])),
TextureStorage2DMultisample: FnPtr::new(metaloadfn("glTextureStorage2DMultisample", &[])),
TextureStorage3D: FnPtr::new(metaloadfn("glTextureStorage3D", &[])),
TextureStorage3DMultisample: FnPtr::new(metaloadfn("glTextureStorage3DMultisample", &[])),
TextureSubImage1D: FnPtr::new(metaloadfn("glTextureSubImage1D", &[])),
TextureSubImage2D: FnPtr::new(metaloadfn("glTextureSubImage2D", &[])),
TextureSubImage3D: FnPtr::new(metaloadfn("glTextureSubImage3D", &[])),
TextureView: FnPtr::new(metaloadfn("glTextureView", &["glTextureViewEXT", "glTextureViewOES"])),
TransformFeedbackBufferBase: FnPtr::new(metaloadfn("glTransformFeedbackBufferBase", &[])),
TransformFeedbackBufferRange: FnPtr::new(metaloadfn("glTransformFeedbackBufferRange", &[])),
TransformFeedbackVaryings: FnPtr::new(metaloadfn("glTransformFeedbackVaryings", &["glTransformFeedbackVaryingsEXT"])),
Uniform1d: FnPtr::new(metaloadfn("glUniform1d", &[])),
Uniform1dv: FnPtr::new(metaloadfn("glUniform1dv", &[])),
Uniform1f: FnPtr::new(metaloadfn("glUniform1f", &["glUniform1fARB"])),
Uniform1fv: FnPtr::new(metaloadfn("glUniform1fv", &["glUniform1fvARB"])),
Uniform1i: FnPtr::new(metaloadfn("glUniform1i", &["glUniform1iARB"])),
Uniform1iv: FnPtr::new(metaloadfn("glUniform1iv", &["glUniform1ivARB"])),
Uniform1ui: FnPtr::new(metaloadfn("glUniform1ui", &["glUniform1uiEXT"])),
Uniform1uiv: FnPtr::new(metaloadfn("glUniform1uiv", &["glUniform1uivEXT"])),
Uniform2d: FnPtr::new(metaloadfn("glUniform2d", &[])),
Uniform2dv: FnPtr::new(metaloadfn("glUniform2dv", &[])),
Uniform2f: FnPtr::new(metaloadfn("glUniform2f", &["glUniform2fARB"])),
Uniform2fv: FnPtr::new(metaloadfn("glUniform2fv", &["glUniform2fvARB"])),
Uniform2i: FnPtr::new(metaloadfn("glUniform2i", &["glUniform2iARB"])),
Uniform2iv: FnPtr::new(metaloadfn("glUniform2iv", &["glUniform2ivARB"])),
Uniform2ui: FnPtr::new(metaloadfn("glUniform2ui", &["glUniform2uiEXT"])),
Uniform2uiv: FnPtr::new(metaloadfn("glUniform2uiv", &["glUniform2uivEXT"])),
Uniform3d: FnPtr::new(metaloadfn("glUniform3d", &[])),
Uniform3dv: FnPtr::new(metaloadfn("glUniform3dv", &[])),
Uniform3f: FnPtr::new(metaloadfn("glUniform3f", &["glUniform3fARB"])),
Uniform3fv: FnPtr::new(metaloadfn("glUniform3fv", &["glUniform3fvARB"])),
Uniform3i: FnPtr::new(metaloadfn("glUniform3i", &["glUniform3iARB"])),
Uniform3iv: FnPtr::new(metaloadfn("glUniform3iv", &["glUniform3ivARB"])),
Uniform3ui: FnPtr::new(metaloadfn("glUniform3ui", &["glUniform3uiEXT"])),
Uniform3uiv: FnPtr::new(metaloadfn("glUniform3uiv", &["glUniform3uivEXT"])),
Uniform4d: FnPtr::new(metaloadfn("glUniform4d", &[])),
Uniform4dv: FnPtr::new(metaloadfn("glUniform4dv", &[])),
Uniform4f: FnPtr::new(metaloadfn("glUniform4f", &["glUniform4fARB"])),
Uniform4fv: FnPtr::new(metaloadfn("glUniform4fv", &["glUniform4fvARB"])),
Uniform4i: FnPtr::new(metaloadfn("glUniform4i", &["glUniform4iARB"])),
Uniform4iv: FnPtr::new(metaloadfn("glUniform4iv", &["glUniform4ivARB"])),
Uniform4ui: FnPtr::new(metaloadfn("glUniform4ui", &["glUniform4uiEXT"])),
Uniform4uiv: FnPtr::new(metaloadfn("glUniform4uiv", &["glUniform4uivEXT"])),
UniformBlockBinding: FnPtr::new(metaloadfn("glUniformBlockBinding", &[])),
UniformMatrix2dv: FnPtr::new(metaloadfn("glUniformMatrix2dv", &[])),
UniformMatrix2fv: FnPtr::new(metaloadfn("glUniformMatrix2fv", &["glUniformMatrix2fvARB"])),
UniformMatrix2x3dv: FnPtr::new(metaloadfn("glUniformMatrix2x3dv", &[])),
UniformMatrix2x3fv: FnPtr::new(metaloadfn("glUniformMatrix2x3fv", &["glUniformMatrix2x3fvNV"])),
UniformMatrix2x4dv: FnPtr::new(metaloadfn("glUniformMatrix2x4dv", &[])),
UniformMatrix2x4fv: FnPtr::new(metaloadfn("glUniformMatrix2x4fv", &["glUniformMatrix2x4fvNV"])),
UniformMatrix3dv: FnPtr::new(metaloadfn("glUniformMatrix3dv", &[])),
UniformMatrix3fv: FnPtr::new(metaloadfn("glUniformMatrix3fv", &["glUniformMatrix3fvARB"])),
UniformMatrix3x2dv: FnPtr::new(metaloadfn("glUniformMatrix3x2dv", &[])),
UniformMatrix3x2fv: FnPtr::new(metaloadfn("glUniformMatrix3x2fv", &["glUniformMatrix3x2fvNV"])),
UniformMatrix3x4dv: FnPtr::new(metaloadfn("glUniformMatrix3x4dv", &[])),
UniformMatrix3x4fv: FnPtr::new(metaloadfn("glUniformMatrix3x4fv", &["glUniformMatrix3x4fvNV"])),
UniformMatrix4dv: FnPtr::new(metaloadfn("glUniformMatrix4dv", &[])),
UniformMatrix4fv: FnPtr::new(metaloadfn("glUniformMatrix4fv", &["glUniformMatrix4fvARB"])),
UniformMatrix4x2dv: FnPtr::new(metaloadfn("glUniformMatrix4x2dv", &[])),
UniformMatrix4x2fv: FnPtr::new(metaloadfn("glUniformMatrix4x2fv", &["glUniformMatrix4x2fvNV"])),
UniformMatrix4x3dv: FnPtr::new(metaloadfn("glUniformMatrix4x3dv", &[])),
UniformMatrix4x3fv: FnPtr::new(metaloadfn("glUniformMatrix4x3fv", &["glUniformMatrix4x3fvNV"])),
UniformSubroutinesuiv: FnPtr::new(metaloadfn("glUniformSubroutinesuiv", &[])),
UnmapBuffer: FnPtr::new(metaloadfn("glUnmapBuffer", &["glUnmapBufferARB", "glUnmapBufferOES"])),
UnmapNamedBuffer: FnPtr::new(metaloadfn("glUnmapNamedBuffer", &[])),
UseProgram: FnPtr::new(metaloadfn("glUseProgram", &["glUseProgramObjectARB"])),
UseProgramStages: FnPtr::new(metaloadfn("glUseProgramStages", &[])),
ValidateProgram: FnPtr::new(metaloadfn("glValidateProgram", &["glValidateProgramARB"])),
ValidateProgramPipeline: FnPtr::new(metaloadfn("glValidateProgramPipeline", &[])),
VertexArrayAttribBinding: FnPtr::new(metaloadfn("glVertexArrayAttribBinding", &[])),
VertexArrayAttribFormat: FnPtr::new(metaloadfn("glVertexArrayAttribFormat", &[])),
VertexArrayAttribIFormat: FnPtr::new(metaloadfn("glVertexArrayAttribIFormat", &[])),
VertexArrayAttribLFormat: FnPtr::new(metaloadfn("glVertexArrayAttribLFormat", &[])),
VertexArrayBindingDivisor: FnPtr::new(metaloadfn("glVertexArrayBindingDivisor", &[])),
VertexArrayElementBuffer: FnPtr::new(metaloadfn("glVertexArrayElementBuffer", &[])),
VertexArrayVertexBuffer: FnPtr::new(metaloadfn("glVertexArrayVertexBuffer", &[])),
VertexArrayVertexBuffers: FnPtr::new(metaloadfn("glVertexArrayVertexBuffers", &[])),
VertexAttrib1d: FnPtr::new(metaloadfn("glVertexAttrib1d", &["glVertexAttrib1dARB", "glVertexAttrib1dNV"])),
VertexAttrib1dv: FnPtr::new(metaloadfn("glVertexAttrib1dv", &["glVertexAttrib1dvARB", "glVertexAttrib1dvNV"])),
VertexAttrib1f: FnPtr::new(metaloadfn("glVertexAttrib1f", &["glVertexAttrib1fARB", "glVertexAttrib1fNV"])),
VertexAttrib1fv: FnPtr::new(metaloadfn("glVertexAttrib1fv", &["glVertexAttrib1fvARB", "glVertexAttrib1fvNV"])),
VertexAttrib1s: FnPtr::new(metaloadfn("glVertexAttrib1s", &["glVertexAttrib1sARB", "glVertexAttrib1sNV"])),
VertexAttrib1sv: FnPtr::new(metaloadfn("glVertexAttrib1sv", &["glVertexAttrib1svARB", "glVertexAttrib1svNV"])),
VertexAttrib2d: FnPtr::new(metaloadfn("glVertexAttrib2d", &["glVertexAttrib2dARB", "glVertexAttrib2dNV"])),
VertexAttrib2dv: FnPtr::new(metaloadfn("glVertexAttrib2dv", &["glVertexAttrib2dvARB", "glVertexAttrib2dvNV"])),
VertexAttrib2f: FnPtr::new(metaloadfn("glVertexAttrib2f", &["glVertexAttrib2fARB", "glVertexAttrib2fNV"])),
VertexAttrib2fv: FnPtr::new(metaloadfn("glVertexAttrib2fv", &["glVertexAttrib2fvARB", "glVertexAttrib2fvNV"])),
VertexAttrib2s: FnPtr::new(metaloadfn("glVertexAttrib2s", &["glVertexAttrib2sARB", "glVertexAttrib2sNV"])),
VertexAttrib2sv: FnPtr::new(metaloadfn("glVertexAttrib2sv", &["glVertexAttrib2svARB", "glVertexAttrib2svNV"])),
VertexAttrib3d: FnPtr::new(metaloadfn("glVertexAttrib3d", &["glVertexAttrib3dARB", "glVertexAttrib3dNV"])),
VertexAttrib3dv: FnPtr::new(metaloadfn("glVertexAttrib3dv", &["glVertexAttrib3dvARB", "glVertexAttrib3dvNV"])),
VertexAttrib3f: FnPtr::new(metaloadfn("glVertexAttrib3f", &["glVertexAttrib3fARB", "glVertexAttrib3fNV"])),
VertexAttrib3fv: FnPtr::new(metaloadfn("glVertexAttrib3fv", &["glVertexAttrib3fvARB", "glVertexAttrib3fvNV"])),
VertexAttrib3s: FnPtr::new(metaloadfn("glVertexAttrib3s", &["glVertexAttrib3sARB", "glVertexAttrib3sNV"])),
VertexAttrib3sv: FnPtr::new(metaloadfn("glVertexAttrib3sv", &["glVertexAttrib3svARB", "glVertexAttrib3svNV"])),
VertexAttrib4Nbv: FnPtr::new(metaloadfn("glVertexAttrib4Nbv", &["glVertexAttrib4NbvARB"])),
VertexAttrib4Niv: FnPtr::new(metaloadfn("glVertexAttrib4Niv", &["glVertexAttrib4NivARB"])),
VertexAttrib4Nsv: FnPtr::new(metaloadfn("glVertexAttrib4Nsv", &["glVertexAttrib4NsvARB"])),
VertexAttrib4Nub: FnPtr::new(metaloadfn("glVertexAttrib4Nub", &["glVertexAttrib4NubARB", "glVertexAttrib4ubNV"])),
VertexAttrib4Nubv: FnPtr::new(metaloadfn("glVertexAttrib4Nubv", &["glVertexAttrib4NubvARB", "glVertexAttrib4ubvNV"])),
VertexAttrib4Nuiv: FnPtr::new(metaloadfn("glVertexAttrib4Nuiv", &["glVertexAttrib4NuivARB"])),
VertexAttrib4Nusv: FnPtr::new(metaloadfn("glVertexAttrib4Nusv", &["glVertexAttrib4NusvARB"])),
VertexAttrib4bv: FnPtr::new(metaloadfn("glVertexAttrib4bv", &["glVertexAttrib4bvARB"])),
VertexAttrib4d: FnPtr::new(metaloadfn("glVertexAttrib4d", &["glVertexAttrib4dARB", "glVertexAttrib4dNV"])),
VertexAttrib4dv: FnPtr::new(metaloadfn("glVertexAttrib4dv", &["glVertexAttrib4dvARB", "glVertexAttrib4dvNV"])),
VertexAttrib4f: FnPtr::new(metaloadfn("glVertexAttrib4f", &["glVertexAttrib4fARB", "glVertexAttrib4fNV"])),
VertexAttrib4fv: FnPtr::new(metaloadfn("glVertexAttrib4fv", &["glVertexAttrib4fvARB", "glVertexAttrib4fvNV"])),
VertexAttrib4iv: FnPtr::new(metaloadfn("glVertexAttrib4iv", &["glVertexAttrib4ivARB"])),
VertexAttrib4s: FnPtr::new(metaloadfn("glVertexAttrib4s", &["glVertexAttrib4sARB", "glVertexAttrib4sNV"])),
VertexAttrib4sv: FnPtr::new(metaloadfn("glVertexAttrib4sv", &["glVertexAttrib4svARB", "glVertexAttrib4svNV"])),
VertexAttrib4ubv: FnPtr::new(metaloadfn("glVertexAttrib4ubv", &["glVertexAttrib4ubvARB"])),
VertexAttrib4uiv: FnPtr::new(metaloadfn("glVertexAttrib4uiv", &["glVertexAttrib4uivARB"])),
VertexAttrib4usv: FnPtr::new(metaloadfn("glVertexAttrib4usv", &["glVertexAttrib4usvARB"])),
VertexAttribBinding: FnPtr::new(metaloadfn("glVertexAttribBinding", &[])),
VertexAttribDivisor: FnPtr::new(metaloadfn("glVertexAttribDivisor", &["glVertexAttribDivisorANGLE", "glVertexAttribDivisorARB", "glVertexAttribDivisorEXT", "glVertexAttribDivisorNV"])),
VertexAttribFormat: FnPtr::new(metaloadfn("glVertexAttribFormat", &[])),
VertexAttribI1i: FnPtr::new(metaloadfn("glVertexAttribI1i", &["glVertexAttribI1iEXT"])),
VertexAttribI1iv: FnPtr::new(metaloadfn("glVertexAttribI1iv", &["glVertexAttribI1ivEXT"])),
VertexAttribI1ui: FnPtr::new(metaloadfn("glVertexAttribI1ui", &["glVertexAttribI1uiEXT"])),
VertexAttribI1uiv: FnPtr::new(metaloadfn("glVertexAttribI1uiv", &["glVertexAttribI1uivEXT"])),
VertexAttribI2i: FnPtr::new(metaloadfn("glVertexAttribI2i", &["glVertexAttribI2iEXT"])),
VertexAttribI2iv: FnPtr::new(metaloadfn("glVertexAttribI2iv", &["glVertexAttribI2ivEXT"])),
VertexAttribI2ui: FnPtr::new(metaloadfn("glVertexAttribI2ui", &["glVertexAttribI2uiEXT"])),
VertexAttribI2uiv: FnPtr::new(metaloadfn("glVertexAttribI2uiv", &["glVertexAttribI2uivEXT"])),
VertexAttribI3i: FnPtr::new(metaloadfn("glVertexAttribI3i", &["glVertexAttribI3iEXT"])),
VertexAttribI3iv: FnPtr::new(metaloadfn("glVertexAttribI3iv", &["glVertexAttribI3ivEXT"])),
VertexAttribI3ui: FnPtr::new(metaloadfn("glVertexAttribI3ui", &["glVertexAttribI3uiEXT"])),
VertexAttribI3uiv: FnPtr::new(metaloadfn("glVertexAttribI3uiv", &["glVertexAttribI3uivEXT"])),
VertexAttribI4bv: FnPtr::new(metaloadfn("glVertexAttribI4bv", &["glVertexAttribI4bvEXT"])),
VertexAttribI4i: FnPtr::new(metaloadfn("glVertexAttribI4i", &["glVertexAttribI4iEXT"])),
VertexAttribI4iv: FnPtr::new(metaloadfn("glVertexAttribI4iv", &["glVertexAttribI4ivEXT"])),
VertexAttribI4sv: FnPtr::new(metaloadfn("glVertexAttribI4sv", &["glVertexAttribI4svEXT"])),
VertexAttribI4ubv: FnPtr::new(metaloadfn("glVertexAttribI4ubv", &["glVertexAttribI4ubvEXT"])),
VertexAttribI4ui: FnPtr::new(metaloadfn("glVertexAttribI4ui", &["glVertexAttribI4uiEXT"])),
VertexAttribI4uiv: FnPtr::new(metaloadfn("glVertexAttribI4uiv", &["glVertexAttribI4uivEXT"])),
VertexAttribI4usv: FnPtr::new(metaloadfn("glVertexAttribI4usv", &["glVertexAttribI4usvEXT"])),
VertexAttribIFormat: FnPtr::new(metaloadfn("glVertexAttribIFormat", &[])),
VertexAttribIPointer: FnPtr::new(metaloadfn("glVertexAttribIPointer", &["glVertexAttribIPointerEXT"])),
VertexAttribL1d: FnPtr::new(metaloadfn("glVertexAttribL1d", &["glVertexAttribL1dEXT"])),
VertexAttribL1dv: FnPtr::new(metaloadfn("glVertexAttribL1dv", &["glVertexAttribL1dvEXT"])),
VertexAttribL2d: FnPtr::new(metaloadfn("glVertexAttribL2d", &["glVertexAttribL2dEXT"])),
VertexAttribL2dv: FnPtr::new(metaloadfn("glVertexAttribL2dv", &["glVertexAttribL2dvEXT"])),
VertexAttribL3d: FnPtr::new(metaloadfn("glVertexAttribL3d", &["glVertexAttribL3dEXT"])),
VertexAttribL3dv: FnPtr::new(metaloadfn("glVertexAttribL3dv", &["glVertexAttribL3dvEXT"])),
VertexAttribL4d: FnPtr::new(metaloadfn("glVertexAttribL4d", &["glVertexAttribL4dEXT"])),
VertexAttribL4dv: FnPtr::new(metaloadfn("glVertexAttribL4dv", &["glVertexAttribL4dvEXT"])),
VertexAttribLFormat: FnPtr::new(metaloadfn("glVertexAttribLFormat", &[])),
VertexAttribLPointer: FnPtr::new(metaloadfn("glVertexAttribLPointer", &["glVertexAttribLPointerEXT"])),
VertexAttribP1ui: FnPtr::new(metaloadfn("glVertexAttribP1ui", &[])),
VertexAttribP1uiv: FnPtr::new(metaloadfn("glVertexAttribP1uiv", &[])),
VertexAttribP2ui: FnPtr::new(metaloadfn("glVertexAttribP2ui", &[])),
VertexAttribP2uiv: FnPtr::new(metaloadfn("glVertexAttribP2uiv", &[])),
VertexAttribP3ui: FnPtr::new(metaloadfn("glVertexAttribP3ui", &[])),
VertexAttribP3uiv: FnPtr::new(metaloadfn("glVertexAttribP3uiv", &[])),
VertexAttribP4ui: FnPtr::new(metaloadfn("glVertexAttribP4ui", &[])),
VertexAttribP4uiv: FnPtr::new(metaloadfn("glVertexAttribP4uiv", &[])),
VertexAttribPointer: FnPtr::new(metaloadfn("glVertexAttribPointer", &["glVertexAttribPointerARB"])),
VertexBindingDivisor: FnPtr::new(metaloadfn("glVertexBindingDivisor", &[])),
VertexP2ui: FnPtr::new(metaloadfn("glVertexP2ui", &[])),
VertexP2uiv: FnPtr::new(metaloadfn("glVertexP2uiv", &[])),
VertexP3ui: FnPtr::new(metaloadfn("glVertexP3ui", &[])),
VertexP3uiv: FnPtr::new(metaloadfn("glVertexP3uiv", &[])),
VertexP4ui: FnPtr::new(metaloadfn("glVertexP4ui", &[])),
VertexP4uiv: FnPtr::new(metaloadfn("glVertexP4uiv", &[])),
Viewport: FnPtr::new(metaloadfn("glViewport", &[])),
ViewportArrayv: FnPtr::new(metaloadfn("glViewportArrayv", &["glViewportArrayvNV", "glViewportArrayvOES"])),
ViewportIndexedf: FnPtr::new(metaloadfn("glViewportIndexedf", &["glViewportIndexedfOES", "glViewportIndexedfNV"])),
ViewportIndexedfv: FnPtr::new(metaloadfn("glViewportIndexedfv", &["glViewportIndexedfvOES", "glViewportIndexedfvNV"])),
WaitSync: FnPtr::new(metaloadfn("glWaitSync", &["glWaitSyncAPPLE"])),
},
_priv: ()
}
        }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ActiveShaderProgram(&self, pipeline: types::GLuint, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.ActiveShaderProgram.f)(pipeline, program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ActiveTexture(&self, texture: enums::TextureUnit) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit) -> ()>(self.ptrs.ActiveTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn AttachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.AttachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginConditionalRender(&self, id: types::GLuint, mode: enums::TypeEnum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TypeEnum) -> ()>(self.ptrs.BeginConditionalRender.f)(id, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginQuery(&self, target: enums::QueryTarget, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget, types::GLuint) -> ()>(self.ptrs.BeginQuery.f)(target, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginQueryIndexed(&self, target: enums::QueryTarget, index: types::GLuint, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget, types::GLuint, types::GLuint) -> ()>(self.ptrs.BeginQueryIndexed.f)(target, index, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BeginTransformFeedback(&self, primitiveMode: enums::PrimitiveType) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType) -> ()>(self.ptrs.BeginTransformFeedback.f)(primitiveMode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindAttribLocation(&self, program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.ptrs.BindAttribLocation.f)(program, index, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffer(&self, target: enums::BufferTargetARB, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint) -> ()>(self.ptrs.BindBuffer.f)(target, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBufferBase(&self, target: enums::BufferTargetARB, index: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint, types::GLuint) -> ()>(self.ptrs.BindBufferBase.f)(target, index, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBufferRange(&self, target: enums::BufferTargetARB, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.BindBufferRange.f)(target, index, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffersBase(&self, target: enums::BufferTargetARB, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.BindBuffersBase.f)(target, first, count, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffersRange(&self, target: enums::BufferTargetARB, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, sizes: *const types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizeiptr) -> ()>(self.ptrs.BindBuffersRange.f)(target, first, count, buffers, offsets, sizes) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFragDataLocation(&self, program: types::GLuint, color: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.ptrs.BindFragDataLocation.f)(program, color, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFragDataLocationIndexed(&self, program: types::GLuint, colorNumber: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.ptrs.BindFragDataLocationIndexed.f)(program, colorNumber, index, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFramebuffer(&self, target: enums::FramebufferTarget, framebuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, types::GLuint) -> ()>(self.ptrs.BindFramebuffer.f)(target, framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindImageTexture(&self, unit: types::GLuint, texture: types::GLuint, level: types::GLint, layered: enums::Boolean, layer: types::GLint, access: enums::BufferAccessARB, format: enums::InternalFormat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, enums::Boolean, types::GLint, enums::BufferAccessARB, enums::InternalFormat) -> ()>(self.ptrs.BindImageTexture.f)(unit, texture, level, layered, layer, access, format) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindImageTextures(&self, first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.BindImageTextures.f)(first, count, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindProgramPipeline(&self, pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.BindProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindRenderbuffer(&self, target: enums::RenderbufferTarget, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::RenderbufferTarget, types::GLuint) -> ()>(self.ptrs.BindRenderbuffer.f)(target, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindSampler(&self, unit: types::GLuint, sampler: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.BindSampler.f)(unit, sampler) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindSamplers(&self, first: types::GLuint, count: types::GLsizei, samplers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.BindSamplers.f)(first, count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTexture(&self, target: enums::TextureTarget, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLuint) -> ()>(self.ptrs.BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTextureUnit(&self, unit: types::GLuint, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.BindTextureUnit.f)(unit, texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTextures(&self, first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.BindTextures.f)(first, count, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTransformFeedback(&self, target: enums::BindTransformFeedbackTarget, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BindTransformFeedbackTarget, types::GLuint) -> ()>(self.ptrs.BindTransformFeedback.f)(target, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexArray(&self, array: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.BindVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexBuffer(&self, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(self.ptrs.BindVertexBuffer.f)(bindingindex, buffer, offset, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexBuffers(&self, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(self.ptrs.BindVertexBuffers.f)(first, count, buffers, offsets, strides) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.BlendColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquation(&self, mode: enums::BlendEquationModeEXT) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BlendEquationModeEXT) -> ()>(self.ptrs.BlendEquation.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparate(&self, modeRGB: enums::BlendEquationModeEXT, modeAlpha: enums::BlendEquationModeEXT) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BlendEquationModeEXT, enums::BlendEquationModeEXT) -> ()>(self.ptrs.BlendEquationSeparate.f)(modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparatei(&self, buf: types::GLuint, modeRGB: enums::BlendEquationModeEXT, modeAlpha: enums::BlendEquationModeEXT) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::BlendEquationModeEXT, enums::BlendEquationModeEXT) -> ()>(self.ptrs.BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationi(&self, buf: types::GLuint, mode: enums::BlendEquationModeEXT) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::BlendEquationModeEXT) -> ()>(self.ptrs.BlendEquationi.f)(buf, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunc(&self, sfactor: enums::BlendingFactor, dfactor: enums::BlendingFactor) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BlendingFactor, enums::BlendingFactor) -> ()>(self.ptrs.BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: enums::BlendingFactor, dfactorRGB: enums::BlendingFactor, sfactorAlpha: enums::BlendingFactor, dfactorAlpha: enums::BlendingFactor) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BlendingFactor, enums::BlendingFactor, enums::BlendingFactor, enums::BlendingFactor) -> ()>(self.ptrs.BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparatei(&self, buf: types::GLuint, srcRGB: enums::BlendingFactor, dstRGB: enums::BlendingFactor, srcAlpha: enums::BlendingFactor, dstAlpha: enums::BlendingFactor) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::BlendingFactor, enums::BlendingFactor, enums::BlendingFactor, enums::BlendingFactor) -> ()>(self.ptrs.BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunci(&self, buf: types::GLuint, src: enums::BlendingFactor, dst: enums::BlendingFactor) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::BlendingFactor, enums::BlendingFactor) -> ()>(self.ptrs.BlendFunci.f)(buf, src, dst) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlitFramebuffer(&self, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: enums::ClearBufferMask, filter: enums::BlitFramebufferFilter) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, enums::ClearBufferMask, enums::BlitFramebufferFilter) -> ()>(self.ptrs.BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlitNamedFramebuffer(&self, readFramebuffer: types::GLuint, drawFramebuffer: types::GLuint, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: enums::ClearBufferMask, filter: enums::BlitFramebufferFilter) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, enums::ClearBufferMask, enums::BlitFramebufferFilter) -> ()>(self.ptrs.BlitNamedFramebuffer.f)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferData(&self, target: enums::BufferTargetARB, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: enums::BufferUsageARB) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLsizeiptr, *const __gl_imports::raw::c_void, enums::BufferUsageARB) -> ()>(self.ptrs.BufferData.f)(target, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferStorage(&self, target: enums::BufferStorageTarget, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: enums::MapBufferUsageMask) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferStorageTarget, types::GLsizeiptr, *const __gl_imports::raw::c_void, enums::MapBufferUsageMask) -> ()>(self.ptrs.BufferStorage.f)(target, size, data, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferSubData(&self, target: enums::BufferTargetARB, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.BufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CheckFramebufferStatus(&self, target: enums::FramebufferTarget) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget) -> types::GLenum>(self.ptrs.CheckFramebufferStatus.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CheckNamedFramebufferStatus(&self, framebuffer: types::GLuint, target: enums::FramebufferTarget) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferTarget) -> types::GLenum>(self.ptrs.CheckNamedFramebufferStatus.f)(framebuffer, target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClampColor(&self, target: types::GLenum, clamp: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.ptrs.ClampColor.f)(target, clamp) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Clear(&self, mask: enums::ClearBufferMask) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ClearBufferMask) -> ()>(self.ptrs.Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferData(&self, target: enums::BufferStorageTarget, internalformat: enums::InternalFormat, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferStorageTarget, enums::InternalFormat, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearBufferData.f)(target, internalformat, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferSubData(&self, target: types::GLenum, internalformat: enums::InternalFormat, offset: types::GLintptr, size: types::GLsizeiptr, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, enums::InternalFormat, types::GLintptr, types::GLsizeiptr, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferfi(&self, buffer: enums::Buffer, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Buffer, types::GLint, types::GLfloat, types::GLint) -> ()>(self.ptrs.ClearBufferfi.f)(buffer, drawbuffer, depth, stencil) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferfv(&self, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Buffer, types::GLint, *const types::GLfloat) -> ()>(self.ptrs.ClearBufferfv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferiv(&self, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Buffer, types::GLint, *const types::GLint) -> ()>(self.ptrs.ClearBufferiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearBufferuiv(&self, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Buffer, types::GLint, *const types::GLuint) -> ()>(self.ptrs.ClearBufferuiv.f)(buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearDepth(&self, depth: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(self.ptrs.ClearDepth.f)(depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearDepthf(&self, d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ptrs.ClearDepthf.f)(d) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedBufferData(&self, buffer: types::GLuint, internalformat: enums::InternalFormat, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::InternalFormat, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearNamedBufferData.f)(buffer, internalformat, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedBufferSubData(&self, buffer: types::GLuint, internalformat: enums::InternalFormat, offset: types::GLintptr, size: types::GLsizeiptr, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::InternalFormat, types::GLintptr, types::GLsizeiptr, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearNamedBufferSubData.f)(buffer, internalformat, offset, size, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferfi(&self, framebuffer: types::GLuint, buffer: enums::Buffer, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::Buffer, types::GLint, types::GLfloat, types::GLint) -> ()>(self.ptrs.ClearNamedFramebufferfi.f)(framebuffer, buffer, drawbuffer, depth, stencil) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferfv(&self, framebuffer: types::GLuint, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::Buffer, types::GLint, *const types::GLfloat) -> ()>(self.ptrs.ClearNamedFramebufferfv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferiv(&self, framebuffer: types::GLuint, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::Buffer, types::GLint, *const types::GLint) -> ()>(self.ptrs.ClearNamedFramebufferiv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearNamedFramebufferuiv(&self, framebuffer: types::GLuint, buffer: enums::Buffer, drawbuffer: types::GLint, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::Buffer, types::GLint, *const types::GLuint) -> ()>(self.ptrs.ClearNamedFramebufferuiv.f)(framebuffer, buffer, drawbuffer, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearStencil(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ptrs.ClearStencil.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearTexImage(&self, texture: types::GLuint, level: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearTexImage.f)(texture, level, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearTexSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.ClearTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClientWaitSync(&self, sync: types::GLsync, flags: enums::SyncObjectMask, timeout: types::GLuint64) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, enums::SyncObjectMask, types::GLuint64) -> types::GLenum>(self.ptrs.ClientWaitSync.f)(sync, flags, timeout) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClipControl(&self, origin: enums::ClipControlOrigin, depth: enums::ClipControlDepth) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ClipControlOrigin, enums::ClipControlDepth) -> ()>(self.ptrs.ClipControl.f)(origin, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorMask(&self, red: enums::Boolean, green: enums::Boolean, blue: enums::Boolean, alpha: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Boolean, enums::Boolean, enums::Boolean, enums::Boolean) -> ()>(self.ptrs.ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorMaski(&self, index: types::GLuint, r: enums::Boolean, g: enums::Boolean, b: enums::Boolean, a: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::Boolean, enums::Boolean, enums::Boolean, enums::Boolean) -> ()>(self.ptrs.ColorMaski.f)(index, r, g, b, a) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP3ui(&self, type_: enums::ColorPointerType, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, types::GLuint) -> ()>(self.ptrs.ColorP3ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP3uiv(&self, type_: enums::ColorPointerType, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, *const types::GLuint) -> ()>(self.ptrs.ColorP3uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP4ui(&self, type_: enums::ColorPointerType, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, types::GLuint) -> ()>(self.ptrs.ColorP4ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorP4uiv(&self, type_: enums::ColorPointerType, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, *const types::GLuint) -> ()>(self.ptrs.ColorP4uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompileShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.CompileShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage1D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage2D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage3D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage1D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage2D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage3D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTextureSubImage1D.f)(texture, level, xoffset, width, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.CompressedTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyBufferSubData(&self, readTarget: enums::CopyBufferSubDataTarget, writeTarget: enums::CopyBufferSubDataTarget, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::CopyBufferSubDataTarget, enums::CopyBufferSubDataTarget, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyImageSubData(&self, srcName: types::GLuint, srcTarget: enums::CopyBufferSubDataTarget, srcLevel: types::GLint, srcX: types::GLint, srcY: types::GLint, srcZ: types::GLint, dstName: types::GLuint, dstTarget: enums::CopyBufferSubDataTarget, dstLevel: types::GLint, dstX: types::GLint, dstY: types::GLint, dstZ: types::GLint, srcWidth: types::GLsizei, srcHeight: types::GLsizei, srcDepth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::CopyBufferSubDataTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLuint, enums::CopyBufferSubDataTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyNamedBufferSubData(&self, readBuffer: types::GLuint, writeBuffer: types::GLuint, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.CopyNamedBufferSubData.f)(readBuffer, writeBuffer, readOffset, writeOffset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexImage1D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLint, types::GLint, types::GLsizei, types::GLint) -> ()>(self.ptrs.CopyTexImage1D.f)(target, level, internalformat, x, y, width, border) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexImage2D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(self.ptrs.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage1D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(self.ptrs.CopyTexSubImage1D.f)(target, level, xoffset, x, y, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage2D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage3D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(self.ptrs.CopyTextureSubImage1D.f)(texture, level, xoffset, x, y, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.CopyTextureSubImage2D.f)(texture, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.CopyTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateFramebuffers(&self, n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateProgram(&self, ) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLuint>(self.ptrs.CreateProgram.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateProgramPipelines(&self, n: types::GLsizei, pipelines: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateQueries(&self, target: enums::QueryTarget, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget, types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateQueries.f)(target, n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateRenderbuffers(&self, n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateSamplers(&self, n: types::GLsizei, samplers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateSamplers.f)(n, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateShader(&self, type_: enums::ShaderType) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ShaderType) -> types::GLuint>(self.ptrs.CreateShader.f)(type_) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateShaderProgramv(&self, type_: enums::ShaderType, count: types::GLsizei, strings: *const *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ShaderType, types::GLsizei, *const *const types::GLchar) -> types::GLuint>(self.ptrs.CreateShaderProgramv.f)(type_, count, strings) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateTextures(&self, target: enums::TextureTarget, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateTextures.f)(target, n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateTransformFeedbacks(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateVertexArrays(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.CreateVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CullFace(&self, mode: enums::CullFaceMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::CullFaceMode) -> ()>(self.ptrs.CullFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageCallback(&self, callback: types::GLDEBUGPROC, userParam: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLDEBUGPROC, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.DebugMessageCallback.f)(callback, userParam) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageControl(&self, source: enums::DebugSource, type_: enums::DebugType, severity: enums::DebugSeverity, count: types::GLsizei, ids: *const types::GLuint, enabled: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::DebugSource, enums::DebugType, enums::DebugSeverity, types::GLsizei, *const types::GLuint, enums::Boolean) -> ()>(self.ptrs.DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageInsert(&self, source: enums::DebugSource, type_: enums::DebugType, id: types::GLuint, severity: enums::DebugSeverity, length: types::GLsizei, buf: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::DebugSource, enums::DebugType, types::GLuint, enums::DebugSeverity, types::GLsizei, *const types::GLchar) -> ()>(self.ptrs.DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteBuffers(&self, n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteFramebuffers(&self, n: types::GLsizei, framebuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.DeleteProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteProgramPipelines(&self, n: types::GLsizei, pipelines: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteQueries(&self, n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteQueries.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteRenderbuffers(&self, n: types::GLsizei, renderbuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteSamplers(&self, count: types::GLsizei, samplers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.DeleteShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteSync(&self, sync: types::GLsync) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> ()>(self.ptrs.DeleteSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteTransformFeedbacks(&self, n: types::GLsizei, ids: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteVertexArrays(&self, n: types::GLsizei, arrays: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.DeleteVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthFunc(&self, func: enums::DepthFunction) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::DepthFunction) -> ()>(self.ptrs.DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthMask(&self, flag: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::Boolean) -> ()>(self.ptrs.DepthMask.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRange(&self, n: types::GLdouble, f: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(self.ptrs.DepthRange.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangeArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.DepthRangeArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangeIndexed(&self, index: types::GLuint, n: types::GLdouble, f: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.DepthRangeIndexed.f)(index, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangef(&self, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.ptrs.DepthRangef.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DetachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.DetachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Disable(&self, cap: enums::EnableCap) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap) -> ()>(self.ptrs.Disable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DisableVertexArrayAttrib(&self, vaobj: types::GLuint, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.DisableVertexArrayAttrib.f)(vaobj, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DisableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.DisableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Disablei(&self, target: enums::EnableCap, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap, types::GLuint) -> ()>(self.ptrs.Disablei.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DispatchCompute(&self, num_groups_x: types::GLuint, num_groups_y: types::GLuint, num_groups_z: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DispatchComputeIndirect(&self, indirect: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLintptr) -> ()>(self.ptrs.DispatchComputeIndirect.f)(indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArrays(&self, mode: enums::PrimitiveType, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLint, types::GLsizei) -> ()>(self.ptrs.DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysIndirect(&self, mode: enums::PrimitiveType, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.DrawArraysIndirect.f)(mode, indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysInstanced(&self, mode: enums::PrimitiveType, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.DrawArraysInstanced.f)(mode, first, count, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArraysInstancedBaseInstance(&self, mode: enums::PrimitiveType, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLint, types::GLsizei, types::GLsizei, types::GLuint) -> ()>(self.ptrs.DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawBuffer(&self, buf: enums::DrawBufferMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::DrawBufferMode) -> ()>(self.ptrs.DrawBuffer.f)(buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawBuffers(&self, n: types::GLsizei, bufs: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLenum) -> ()>(self.ptrs.DrawBuffers.f)(n, bufs) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElements(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsBaseVertex(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void, types::GLint) -> ()>(self.ptrs.DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsIndirect(&self, mode: enums::PrimitiveType, type_: enums::DrawElementsType, indirect: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, enums::DrawElementsType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.DrawElementsIndirect.f)(mode, type_, indirect) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstanced(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ptrs.DrawElementsInstanced.f)(mode, count, type_, indices, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseInstance(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::PrimitiveType, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::PrimitiveType, *const __gl_imports::raw::c_void, types::GLsizei, types::GLuint) -> ()>(self.ptrs.DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void, types::GLsizei, types::GLint) -> ()>(self.ptrs.DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(&self, mode: enums::PrimitiveType, count: types::GLsizei, type_: enums::PrimitiveType, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint, baseinstance: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLsizei, enums::PrimitiveType, *const __gl_imports::raw::c_void, types::GLsizei, types::GLint, types::GLuint) -> ()>(self.ptrs.DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawRangeElements(&self, mode: enums::PrimitiveType, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint, types::GLuint, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.DrawRangeElements.f)(mode, start, end, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: enums::PrimitiveType, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: enums::DrawElementsType, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint, types::GLuint, types::GLsizei, enums::DrawElementsType, *const __gl_imports::raw::c_void, types::GLint) -> ()>(self.ptrs.DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedback(&self, mode: enums::PrimitiveType, id: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint) -> ()>(self.ptrs.DrawTransformFeedback.f)(mode, id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackInstanced(&self, mode: enums::PrimitiveType, id: types::GLuint, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint, types::GLsizei) -> ()>(self.ptrs.DrawTransformFeedbackInstanced.f)(mode, id, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackStream(&self, mode: enums::PrimitiveType, id: types::GLuint, stream: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint, types::GLuint) -> ()>(self.ptrs.DrawTransformFeedbackStream.f)(mode, id, stream) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawTransformFeedbackStreamInstanced(&self, mode: enums::PrimitiveType, id: types::GLuint, stream: types::GLuint, instancecount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, types::GLuint, types::GLuint, types::GLsizei) -> ()>(self.ptrs.DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Enable(&self, cap: enums::EnableCap) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap) -> ()>(self.ptrs.Enable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EnableVertexArrayAttrib(&self, vaobj: types::GLuint, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.EnableVertexArrayAttrib.f)(vaobj, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EnableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.EnableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Enablei(&self, target: enums::EnableCap, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap, types::GLuint) -> ()>(self.ptrs.Enablei.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndConditionalRender(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.EndConditionalRender.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndQuery(&self, target: enums::QueryTarget) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget) -> ()>(self.ptrs.EndQuery.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndQueryIndexed(&self, target: enums::QueryTarget, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget, types::GLuint) -> ()>(self.ptrs.EndQueryIndexed.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EndTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.EndTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FenceSync(&self, condition: enums::SyncCondition, flags: types::GLbitfield) -> types::GLsync { __gl_imports::mem::transmute::<_, extern "system" fn(enums::SyncCondition, types::GLbitfield) -> types::GLsync>(self.ptrs.FenceSync.f)(condition, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Finish(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Flush(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.Flush.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FlushMappedBufferRange(&self, target: enums::BufferTargetARB, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.FlushMappedBufferRange.f)(target, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FlushMappedNamedBufferRange(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.FlushMappedNamedBufferRange.f)(buffer, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferParameteri(&self, target: enums::FramebufferTarget, pname: enums::FramebufferParameterName, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferParameterName, types::GLint) -> ()>(self.ptrs.FramebufferParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferRenderbuffer(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, renderbuffertarget: enums::RenderbufferTarget, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, enums::RenderbufferTarget, types::GLuint) -> ()>(self.ptrs.FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, types::GLuint, types::GLint) -> ()>(self.ptrs.FramebufferTexture.f)(target, attachment, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture1D(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, textarget: enums::TextureTarget, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, enums::TextureTarget, types::GLuint, types::GLint) -> ()>(self.ptrs.FramebufferTexture1D.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture2D(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, textarget: enums::TextureTarget, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, enums::TextureTarget, types::GLuint, types::GLint) -> ()>(self.ptrs.FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture3D(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, textarget: enums::TextureTarget, texture: types::GLuint, level: types::GLint, zoffset: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, enums::TextureTarget, types::GLuint, types::GLint, types::GLint) -> ()>(self.ptrs.FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTextureLayer(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, types::GLuint, types::GLint, types::GLint) -> ()>(self.ptrs.FramebufferTextureLayer.f)(target, attachment, texture, level, layer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FrontFace(&self, mode: enums::FrontFaceDirection) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FrontFaceDirection) -> ()>(self.ptrs.FrontFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenFramebuffers(&self, n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenProgramPipelines(&self, n: types::GLsizei, pipelines: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenProgramPipelines.f)(n, pipelines) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenQueries(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenQueries.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenRenderbuffers(&self, n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenSamplers(&self, count: types::GLsizei, samplers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenSamplers.f)(count, samplers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenTransformFeedbacks(&self, n: types::GLsizei, ids: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenTransformFeedbacks.f)(n, ids) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenVertexArrays(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GenVertexArrays.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenerateMipmap(&self, target: enums::TextureTarget) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget) -> ()>(self.ptrs.GenerateMipmap.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenerateTextureMipmap(&self, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.GenerateTextureMipmap.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveAtomicCounterBufferiv(&self, program: types::GLuint, bufferIndex: types::GLuint, pname: enums::AtomicCounterBufferPName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::AtomicCounterBufferPName, *mut types::GLint) -> ()>(self.ptrs.GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveAttrib(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut enums::AttributeType, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut enums::AttributeType, *mut types::GLchar) -> ()>(self.ptrs.GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineName(&self, program: types::GLuint, shadertype: enums::ShaderType, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineUniformName(&self, program: types::GLuint, shadertype: enums::ShaderType, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveSubroutineUniformiv(&self, program: types::GLuint, shadertype: enums::ShaderType, index: types::GLuint, pname: enums::SubroutineParameterName, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, types::GLuint, enums::SubroutineParameterName, *mut types::GLint) -> ()>(self.ptrs.GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniform(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut enums::AttributeType, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut enums::AttributeType, *mut types::GLchar) -> ()>(self.ptrs.GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformBlockName(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformBlockiv(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, pname: enums::UniformBlockPName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::UniformBlockPName, *mut types::GLint) -> ()>(self.ptrs.GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformName(&self, program: types::GLuint, uniformIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformName: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniformsiv(&self, program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: enums::UniformPName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, enums::UniformPName, *mut types::GLint) -> ()>(self.ptrs.GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttachedShaders(&self, program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttribLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.ptrs.GetAttribLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBooleani_v(&self, target: enums::BufferTargetARB, index: types::GLuint, data: *mut enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLuint, *mut enums::Boolean) -> ()>(self.ptrs.GetBooleani_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBooleanv(&self, pname: enums::GetPName, data: *mut enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPName, *mut enums::Boolean) -> ()>(self.ptrs.GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferParameteri64v(&self, target: enums::BufferTargetARB, pname: types::GLenum, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLenum, *mut types::GLint64) -> ()>(self.ptrs.GetBufferParameteri64v.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferParameteriv(&self, target: enums::BufferTargetARB, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLenum, *mut types::GLint) -> ()>(self.ptrs.GetBufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferPointerv(&self, target: enums::BufferTargetARB, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetBufferPointerv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferSubData(&self, target: enums::BufferTargetARB, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLintptr, types::GLsizeiptr, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetBufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTexImage(&self, target: enums::TextureTarget, level: types::GLint, img: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetCompressedTexImage.f)(target, level, img) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTextureImage(&self, texture: types::GLuint, level: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetCompressedTextureImage.f)(texture, level, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetCompressedTextureSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetCompressedTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDebugMessageLog(&self, count: types::GLuint, bufSize: types::GLsizei, sources: *mut enums::DebugSource, types: *mut enums::DebugType, ids: *mut types::GLuint, severities: *mut enums::DebugSeverity, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut enums::DebugSource, *mut enums::DebugType, *mut types::GLuint, *mut enums::DebugSeverity, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(self.ptrs.GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDoublei_v(&self, target: enums::TypeEnum, index: types::GLuint, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TypeEnum, types::GLuint, *mut types::GLdouble) -> ()>(self.ptrs.GetDoublei_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDoublev(&self, pname: enums::GetPName, data: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPName, *mut types::GLdouble) -> ()>(self.ptrs.GetDoublev.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetError(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.ptrs.GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFloati_v(&self, target: enums::TypeEnum, index: types::GLuint, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TypeEnum, types::GLuint, *mut types::GLfloat) -> ()>(self.ptrs.GetFloati_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFloatv(&self, pname: enums::GetPName, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPName, *mut types::GLfloat) -> ()>(self.ptrs.GetFloatv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFragDataIndex(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.ptrs.GetFragDataIndex.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFragDataLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.ptrs.GetFragDataLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: enums::FramebufferTarget, attachment: enums::FramebufferAttachment, pname: enums::FramebufferAttachmentParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachment, enums::FramebufferAttachmentParameterName, *mut types::GLint) -> ()>(self.ptrs.GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFramebufferParameteriv(&self, target: enums::FramebufferTarget, pname: enums::FramebufferAttachmentParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, enums::FramebufferAttachmentParameterName, *mut types::GLint) -> ()>(self.ptrs.GetFramebufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetGraphicsResetStatus(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.ptrs.GetGraphicsResetStatus.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInteger64i_v(&self, target: enums::TypeEnum, index: types::GLuint, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TypeEnum, types::GLuint, *mut types::GLint64) -> ()>(self.ptrs.GetInteger64i_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInteger64v(&self, pname: enums::GetPName, data: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPName, *mut types::GLint64) -> ()>(self.ptrs.GetInteger64v.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetIntegeri_v(&self, target: enums::TypeEnum, index: types::GLuint, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TypeEnum, types::GLuint, *mut types::GLint) -> ()>(self.ptrs.GetIntegeri_v.f)(target, index, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetIntegerv(&self, pname: enums::GetPName, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPName, *mut types::GLint) -> ()>(self.ptrs.GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInternalformati64v(&self, target: enums::TextureTarget, internalformat: enums::InternalFormat, pname: enums::InternalFormatPName, bufSize: types::GLsizei, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::InternalFormat, enums::InternalFormatPName, types::GLsizei, *mut types::GLint64) -> ()>(self.ptrs.GetInternalformati64v.f)(target, internalformat, pname, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetInternalformativ(&self, target: enums::TextureTarget, internalformat: enums::InternalFormat, pname: enums::InternalFormatPName, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::InternalFormat, enums::InternalFormatPName, types::GLsizei, *mut types::GLint) -> ()>(self.ptrs.GetInternalformativ.f)(target, internalformat, pname, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetMultisamplefv(&self, pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(self.ptrs.GetMultisamplefv.f)(pname, index, val) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferParameteri64v(&self, buffer: types::GLuint, pname: enums::VertexBufferObjectParameter, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexBufferObjectParameter, *mut types::GLint64) -> ()>(self.ptrs.GetNamedBufferParameteri64v.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferParameteriv(&self, buffer: types::GLuint, pname: enums::VertexBufferObjectParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexBufferObjectParameter, *mut types::GLint) -> ()>(self.ptrs.GetNamedBufferParameteriv.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferPointerv(&self, buffer: types::GLuint, pname: enums::VertexBufferObjectParameter, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexBufferObjectParameter, *const *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetNamedBufferPointerv.f)(buffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetNamedBufferSubData.f)(buffer, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedFramebufferAttachmentParameteriv(&self, framebuffer: types::GLuint, attachment: enums::FramebufferAttachment, pname: enums::FramebufferAttachmentParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferAttachment, enums::FramebufferAttachmentParameterName, *mut types::GLint) -> ()>(self.ptrs.GetNamedFramebufferAttachmentParameteriv.f)(framebuffer, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedFramebufferParameteriv(&self, framebuffer: types::GLuint, pname: enums::GetFramebufferParameter, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::GetFramebufferParameter, *mut types::GLint) -> ()>(self.ptrs.GetNamedFramebufferParameteriv.f)(framebuffer, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetNamedRenderbufferParameteriv(&self, renderbuffer: types::GLuint, pname: enums::RenderbufferParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::RenderbufferParameterName, *mut types::GLint) -> ()>(self.ptrs.GetNamedRenderbufferParameteriv.f)(renderbuffer, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectLabel(&self, identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetObjectLabel.f)(identifier, name, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetPointerv(&self, pname: enums::GetPointervPName, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::GetPointervPName, *const *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetPointerv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramBinary(&self, program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, binaryFormat: *mut types::GLenum, binary: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramInfoLog(&self, program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramInterfaceiv(&self, program: types::GLuint, programInterface: enums::ProgramInterface, pname: enums::ProgramInterfacePName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, enums::ProgramInterfacePName, *mut types::GLint) -> ()>(self.ptrs.GetProgramInterfaceiv.f)(program, programInterface, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramPipelineInfoLog(&self, pipeline: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramPipelineiv(&self, pipeline: types::GLuint, pname: enums::PipelineParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::PipelineParameterName, *mut types::GLint) -> ()>(self.ptrs.GetProgramPipelineiv.f)(pipeline, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceIndex(&self, program: types::GLuint, programInterface: enums::ProgramInterface, name: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, *const types::GLchar) -> types::GLuint>(self.ptrs.GetProgramResourceIndex.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceLocation(&self, program: types::GLuint, programInterface: enums::ProgramInterface, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, *const types::GLchar) -> types::GLint>(self.ptrs.GetProgramResourceLocation.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceLocationIndex(&self, program: types::GLuint, programInterface: enums::ProgramInterface, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, *const types::GLchar) -> types::GLint>(self.ptrs.GetProgramResourceLocationIndex.f)(program, programInterface, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceName(&self, program: types::GLuint, programInterface: enums::ProgramInterface, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramResourceiv(&self, program: types::GLuint, programInterface: enums::ProgramInterface, index: types::GLuint, propCount: types::GLsizei, props: *const types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramInterface, types::GLuint, types::GLsizei, *const types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(self.ptrs.GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramStageiv(&self, program: types::GLuint, shadertype: enums::ShaderType, pname: enums::ProgramStagePName, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, enums::ProgramStagePName, *mut types::GLint) -> ()>(self.ptrs.GetProgramStageiv.f)(program, shadertype, pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramiv(&self, program: types::GLuint, pname: enums::ProgramPropertyARB, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramPropertyARB, *mut types::GLint) -> ()>(self.ptrs.GetProgramiv.f)(program, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjecti64v(&self, id: types::GLuint, buffer: types::GLuint, pname: enums::QueryObjectParameterName, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::QueryObjectParameterName, types::GLintptr) -> ()>(self.ptrs.GetQueryBufferObjecti64v.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectiv(&self, id: types::GLuint, buffer: types::GLuint, pname: enums::QueryObjectParameterName, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::QueryObjectParameterName, types::GLintptr) -> ()>(self.ptrs.GetQueryBufferObjectiv.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectui64v(&self, id: types::GLuint, buffer: types::GLuint, pname: enums::QueryObjectParameterName, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::QueryObjectParameterName, types::GLintptr) -> ()>(self.ptrs.GetQueryBufferObjectui64v.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryBufferObjectuiv(&self, id: types::GLuint, buffer: types::GLuint, pname: enums::QueryObjectParameterName, offset: types::GLintptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::QueryObjectParameterName, types::GLintptr) -> ()>(self.ptrs.GetQueryBufferObjectuiv.f)(id, buffer, pname, offset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryIndexediv(&self, target: types::GLenum, index: types::GLuint, pname: enums::QueryParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, enums::QueryParameterName, *mut types::GLint) -> ()>(self.ptrs.GetQueryIndexediv.f)(target, index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjecti64v(&self, id: types::GLuint, pname: enums::QueryObjectParameterName, params: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::QueryObjectParameterName, *mut types::GLint64) -> ()>(self.ptrs.GetQueryObjecti64v.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectiv(&self, id: types::GLuint, pname: enums::QueryObjectParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::QueryObjectParameterName, *mut types::GLint) -> ()>(self.ptrs.GetQueryObjectiv.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectui64v(&self, id: types::GLuint, pname: enums::QueryObjectParameterName, params: *mut types::GLuint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::QueryObjectParameterName, *mut types::GLuint64) -> ()>(self.ptrs.GetQueryObjectui64v.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryObjectuiv(&self, id: types::GLuint, pname: enums::QueryObjectParameterName, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::QueryObjectParameterName, *mut types::GLuint) -> ()>(self.ptrs.GetQueryObjectuiv.f)(id, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetQueryiv(&self, target: enums::QueryTarget, pname: enums::QueryParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::QueryTarget, enums::QueryParameterName, *mut types::GLint) -> ()>(self.ptrs.GetQueryiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetRenderbufferParameteriv(&self, target: enums::RenderbufferTarget, pname: enums::RenderbufferParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::RenderbufferTarget, enums::RenderbufferParameterName, *mut types::GLint) -> ()>(self.ptrs.GetRenderbufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterIiv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *mut types::GLint) -> ()>(self.ptrs.GetSamplerParameterIiv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterIuiv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *mut types::GLuint) -> ()>(self.ptrs.GetSamplerParameterIuiv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameterfv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *mut types::GLfloat) -> ()>(self.ptrs.GetSamplerParameterfv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSamplerParameteriv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *mut types::GLint) -> ()>(self.ptrs.GetSamplerParameteriv.f)(sampler, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderInfoLog(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderPrecisionFormat(&self, shadertype: enums::ShaderType, precisiontype: enums::PrecisionType, range: *mut types::GLint, precision: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ShaderType, enums::PrecisionType, *mut types::GLint, *mut types::GLint) -> ()>(self.ptrs.GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderSource(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.ptrs.GetShaderSource.f)(shader, bufSize, length, source) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderiv(&self, shader: types::GLuint, pname: enums::ShaderParameterName, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderParameterName, *mut types::GLint) -> ()>(self.ptrs.GetShaderiv.f)(shader, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetString(&self, name: enums::StringName) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StringName) -> *const types::GLubyte>(self.ptrs.GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetStringi(&self, name: enums::StringName, index: types::GLuint) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StringName, types::GLuint) -> *const types::GLubyte>(self.ptrs.GetStringi.f)(name, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSubroutineIndex(&self, program: types::GLuint, shadertype: enums::ShaderType, name: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, *const types::GLchar) -> types::GLuint>(self.ptrs.GetSubroutineIndex.f)(program, shadertype, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSubroutineUniformLocation(&self, program: types::GLuint, shadertype: enums::ShaderType, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ShaderType, *const types::GLchar) -> types::GLint>(self.ptrs.GetSubroutineUniformLocation.f)(program, shadertype, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetSynciv(&self, sync: types::GLsync, pname: enums::SyncParameterName, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, enums::SyncParameterName, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(self.ptrs.GetSynciv.f)(sync, pname, bufSize, length, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexImage(&self, target: enums::TextureTarget, level: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::PixelFormat, enums::PixelType, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetTexImage.f)(target, level, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameterfv(&self, target: enums::TextureTarget, level: types::GLint, pname: enums::GetTextureParameter, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::GetTextureParameter, *mut types::GLfloat) -> ()>(self.ptrs.GetTexLevelParameterfv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexLevelParameteriv(&self, target: enums::TextureTarget, level: types::GLint, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTexLevelParameteriv.f)(target, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterIiv(&self, target: enums::TextureTarget, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTexParameterIiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterIuiv(&self, target: enums::TextureTarget, pname: enums::GetTextureParameter, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::GetTextureParameter, *mut types::GLuint) -> ()>(self.ptrs.GetTexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterfv(&self, target: enums::TextureTarget, pname: enums::GetTextureParameter, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::GetTextureParameter, *mut types::GLfloat) -> ()>(self.ptrs.GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameteriv(&self, target: enums::TextureTarget, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureImage(&self, texture: types::GLuint, level: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetTextureImage.f)(texture, level, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureLevelParameterfv(&self, texture: types::GLuint, level: types::GLint, pname: enums::GetTextureParameter, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::GetTextureParameter, *mut types::GLfloat) -> ()>(self.ptrs.GetTextureLevelParameterfv.f)(texture, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureLevelParameteriv(&self, texture: types::GLuint, level: types::GLint, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTextureLevelParameteriv.f)(texture, level, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterIiv(&self, texture: types::GLuint, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTextureParameterIiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterIuiv(&self, texture: types::GLuint, pname: enums::GetTextureParameter, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::GetTextureParameter, *mut types::GLuint) -> ()>(self.ptrs.GetTextureParameterIuiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameterfv(&self, texture: types::GLuint, pname: enums::GetTextureParameter, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::GetTextureParameter, *mut types::GLfloat) -> ()>(self.ptrs.GetTextureParameterfv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureParameteriv(&self, texture: types::GLuint, pname: enums::GetTextureParameter, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::GetTextureParameter, *mut types::GLint) -> ()>(self.ptrs.GetTextureParameteriv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTextureSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbackVarying(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut types::GLchar) -> ()>(self.ptrs.GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbacki64_v(&self, xfb: types::GLuint, pname: enums::TransformFeedbackPName, index: types::GLuint, param: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TransformFeedbackPName, types::GLuint, *mut types::GLint64) -> ()>(self.ptrs.GetTransformFeedbacki64_v.f)(xfb, pname, index, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbacki_v(&self, xfb: types::GLuint, pname: enums::TransformFeedbackPName, index: types::GLuint, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TransformFeedbackPName, types::GLuint, *mut types::GLint) -> ()>(self.ptrs.GetTransformFeedbacki_v.f)(xfb, pname, index, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTransformFeedbackiv(&self, xfb: types::GLuint, pname: enums::TransformFeedbackPName, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TransformFeedbackPName, *mut types::GLint) -> ()>(self.ptrs.GetTransformFeedbackiv.f)(xfb, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformBlockIndex(&self, program: types::GLuint, uniformBlockName: *const types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLuint>(self.ptrs.GetUniformBlockIndex.f)(program, uniformBlockName) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformIndices(&self, program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *mut types::GLuint) -> ()>(self.ptrs.GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.ptrs.GetUniformLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformSubroutineuiv(&self, shadertype: enums::ShaderType, location: types::GLint, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ShaderType, types::GLint, *mut types::GLuint) -> ()>(self.ptrs.GetUniformSubroutineuiv.f)(shadertype, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformdv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLdouble) -> ()>(self.ptrs.GetUniformdv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformfv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> ()>(self.ptrs.GetUniformfv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformiv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> ()>(self.ptrs.GetUniformiv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformuiv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLuint) -> ()>(self.ptrs.GetUniformuiv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayIndexed64iv(&self, vaobj: types::GLuint, index: types::GLuint, pname: enums::VertexArrayPName, param: *mut types::GLint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::VertexArrayPName, *mut types::GLint64) -> ()>(self.ptrs.GetVertexArrayIndexed64iv.f)(vaobj, index, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayIndexediv(&self, vaobj: types::GLuint, index: types::GLuint, pname: enums::VertexArrayPName, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, enums::VertexArrayPName, *mut types::GLint) -> ()>(self.ptrs.GetVertexArrayIndexediv.f)(vaobj, index, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexArrayiv(&self, vaobj: types::GLuint, pname: enums::VertexArrayPName, param: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexArrayPName, *mut types::GLint) -> ()>(self.ptrs.GetVertexArrayiv.f)(vaobj, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribIiv(&self, index: types::GLuint, pname: enums::VertexAttribEnum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribEnum, *mut types::GLint) -> ()>(self.ptrs.GetVertexAttribIiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribIuiv(&self, index: types::GLuint, pname: enums::VertexAttribEnum, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribEnum, *mut types::GLuint) -> ()>(self.ptrs.GetVertexAttribIuiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribLdv(&self, index: types::GLuint, pname: enums::VertexAttribEnum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribEnum, *mut types::GLdouble) -> ()>(self.ptrs.GetVertexAttribLdv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribPointerv(&self, index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetVertexAttribPointerv.f)(index, pname, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribdv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> ()>(self.ptrs.GetVertexAttribdv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribfv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(self.ptrs.GetVertexAttribfv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribiv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.ptrs.GetVertexAttribiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnColorTable(&self, target: enums::ColorTableTarget, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, table: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorTableTarget, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnColorTable.f)(target, format, type_, bufSize, table) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnCompressedTexImage(&self, target: enums::TextureTarget, lod: types::GLint, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnCompressedTexImage.f)(target, lod, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnConvolutionFilter(&self, target: enums::ConvolutionTarget, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, image: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ConvolutionTarget, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnConvolutionFilter.f)(target, format, type_, bufSize, image) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnHistogram(&self, target: enums::HistogramTargetEXT, reset: types::GLboolean, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::HistogramTargetEXT, types::GLboolean, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnHistogram.f)(target, reset, format, type_, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapdv(&self, target: enums::MapTarget, query: enums::MapQuery, bufSize: types::GLsizei, v: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MapTarget, enums::MapQuery, types::GLsizei, *mut types::GLdouble) -> ()>(self.ptrs.GetnMapdv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapfv(&self, target: enums::MapTarget, query: enums::MapQuery, bufSize: types::GLsizei, v: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MapTarget, enums::MapQuery, types::GLsizei, *mut types::GLfloat) -> ()>(self.ptrs.GetnMapfv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMapiv(&self, target: enums::MapTarget, query: enums::MapQuery, bufSize: types::GLsizei, v: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MapTarget, enums::MapQuery, types::GLsizei, *mut types::GLint) -> ()>(self.ptrs.GetnMapiv.f)(target, query, bufSize, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnMinmax(&self, target: enums::MinmaxTargetEXT, reset: types::GLboolean, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, values: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MinmaxTargetEXT, types::GLboolean, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnMinmax.f)(target, reset, format, type_, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapfv(&self, map: enums::PixelMap, bufSize: types::GLsizei, values: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PixelMap, types::GLsizei, *mut types::GLfloat) -> ()>(self.ptrs.GetnPixelMapfv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapuiv(&self, map: enums::PixelMap, bufSize: types::GLsizei, values: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PixelMap, types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GetnPixelMapuiv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPixelMapusv(&self, map: enums::PixelMap, bufSize: types::GLsizei, values: *mut types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PixelMap, types::GLsizei, *mut types::GLushort) -> ()>(self.ptrs.GetnPixelMapusv.f)(map, bufSize, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnPolygonStipple(&self, bufSize: types::GLsizei, pattern: *mut types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLubyte) -> ()>(self.ptrs.GetnPolygonStipple.f)(bufSize, pattern) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnSeparableFilter(&self, target: enums::SeparableTargetEXT, format: enums::PixelFormat, type_: enums::PixelType, rowBufSize: types::GLsizei, row: *mut __gl_imports::raw::c_void, columnBufSize: types::GLsizei, column: *mut __gl_imports::raw::c_void, span: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::SeparableTargetEXT, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void, types::GLsizei, *mut __gl_imports::raw::c_void, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnSeparableFilter.f)(target, format, type_, rowBufSize, row, columnBufSize, column, span) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnTexImage(&self, target: enums::TextureTarget, level: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.GetnTexImage.f)(target, level, format, type_, bufSize, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformdv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLdouble) -> ()>(self.ptrs.GetnUniformdv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformfv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLfloat) -> ()>(self.ptrs.GetnUniformfv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformiv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLint) -> ()>(self.ptrs.GetnUniformiv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetnUniformuiv(&self, program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLuint) -> ()>(self.ptrs.GetnUniformuiv.f)(program, location, bufSize, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Hint(&self, target: enums::HintTarget, mode: enums::HintMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::HintTarget, enums::HintMode) -> ()>(self.ptrs.Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateBufferData(&self, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.InvalidateBufferData.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.InvalidateBufferSubData.f)(buffer, offset, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateFramebuffer(&self, target: enums::FramebufferTarget, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::FramebufferTarget, types::GLsizei, *const types::GLenum) -> ()>(self.ptrs.InvalidateFramebuffer.f)(target, numAttachments, attachments) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateNamedFramebufferData(&self, framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const enums::FramebufferAttachment) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const enums::FramebufferAttachment) -> ()>(self.ptrs.InvalidateNamedFramebufferData.f)(framebuffer, numAttachments, attachments) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateNamedFramebufferSubData(&self, framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const enums::FramebufferAttachment, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const enums::FramebufferAttachment, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.InvalidateNamedFramebufferSubData.f)(framebuffer, numAttachments, attachments, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateSubFramebuffer(&self, target: types::GLenum, numAttachments: types::GLsizei, attachments: *const enums::FramebufferAttachment, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const enums::FramebufferAttachment, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateTexImage(&self, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(self.ptrs.InvalidateTexImage.f)(texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn InvalidateTexSubImage(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsEnabled(&self, cap: enums::EnableCap) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap) -> types::GLboolean>(self.ptrs.IsEnabled.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsEnabledi(&self, target: enums::EnableCap, index: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(enums::EnableCap, types::GLuint) -> types::GLboolean>(self.ptrs.IsEnabledi.f)(target, index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsFramebuffer(&self, framebuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsFramebuffer.f)(framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsProgram(&self, program: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsProgramPipeline(&self, pipeline: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsQuery(&self, id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsQuery.f)(id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsRenderbuffer(&self, renderbuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsRenderbuffer.f)(renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsSampler(&self, sampler: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsSampler.f)(sampler) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsShader(&self, shader: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsSync(&self, sync: types::GLsync) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync) -> types::GLboolean>(self.ptrs.IsSync.f)(sync) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsTransformFeedback(&self, id: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsTransformFeedback.f)(id) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsVertexArray(&self, array: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.IsVertexArray.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ptrs.LineWidth.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LinkProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.LinkProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LogicOp(&self, opcode: enums::LogicOp) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::LogicOp) -> ()>(self.ptrs.LogicOp.f)(opcode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapBuffer(&self, target: enums::BufferTargetARB, access: enums::BufferAccessARB) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, enums::BufferAccessARB) -> *mut __gl_imports::raw::c_void>(self.ptrs.MapBuffer.f)(target, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapBufferRange(&self, target: enums::BufferTargetARB, offset: types::GLintptr, length: types::GLsizeiptr, access: enums::BufferAccessMask) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB, types::GLintptr, types::GLsizeiptr, enums::BufferAccessMask) -> *mut __gl_imports::raw::c_void>(self.ptrs.MapBufferRange.f)(target, offset, length, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapNamedBuffer(&self, buffer: types::GLuint, access: enums::BufferAccessARB) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::BufferAccessARB) -> *mut __gl_imports::raw::c_void>(self.ptrs.MapNamedBuffer.f)(buffer, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MapNamedBufferRange(&self, buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr, access: enums::BufferAccessMask) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, enums::BufferAccessMask) -> *mut __gl_imports::raw::c_void>(self.ptrs.MapNamedBufferRange.f)(buffer, offset, length, access) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MemoryBarrier(&self, barriers: enums::MemoryBarrierMask) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MemoryBarrierMask) -> ()>(self.ptrs.MemoryBarrier.f)(barriers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MemoryBarrierByRegion(&self, barriers: enums::MemoryBarrierMask) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MemoryBarrierMask) -> ()>(self.ptrs.MemoryBarrierByRegion.f)(barriers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MinSampleShading(&self, value: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ptrs.MinSampleShading.f)(value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawArrays(&self, mode: enums::PrimitiveType, first: *const types::GLint, count: *const types::GLsizei, drawcount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, *const types::GLint, *const types::GLsizei, types::GLsizei) -> ()>(self.ptrs.MultiDrawArrays.f)(mode, first, count, drawcount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawArraysIndirect(&self, mode: enums::PrimitiveType, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, *const __gl_imports::raw::c_void, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElements(&self, mode: enums::PrimitiveType, count: *const types::GLsizei, type_: enums::DrawElementsType, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, *const types::GLsizei, enums::DrawElementsType, *const *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ptrs.MultiDrawElements.f)(mode, count, type_, indices, drawcount) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: enums::PrimitiveType, count: *const types::GLsizei, type_: enums::DrawElementsType, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei, basevertex: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, *const types::GLsizei, enums::DrawElementsType, *const *const __gl_imports::raw::c_void, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiDrawElementsIndirect(&self, mode: enums::PrimitiveType, type_: enums::DrawElementsType, indirect: *const __gl_imports::raw::c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PrimitiveType, enums::DrawElementsType, *const __gl_imports::raw::c_void, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP1ui(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.MultiTexCoordP1ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP1uiv(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.MultiTexCoordP1uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP2ui(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.MultiTexCoordP2ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP2uiv(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.MultiTexCoordP2uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP3ui(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.MultiTexCoordP3ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP3uiv(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.MultiTexCoordP3uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP4ui(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.MultiTexCoordP4ui.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn MultiTexCoordP4uiv(&self, texture: enums::TextureUnit, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureUnit, enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.MultiTexCoordP4uiv.f)(texture, type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferData(&self, buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: enums::VertexBufferObjectUsage) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const __gl_imports::raw::c_void, enums::VertexBufferObjectUsage) -> ()>(self.ptrs.NamedBufferData.f)(buffer, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferStorage(&self, buffer: types::GLuint, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, flags: enums::MapBufferUsageMask) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const __gl_imports::raw::c_void, enums::MapBufferUsageMask) -> ()>(self.ptrs.NamedBufferStorage.f)(buffer, size, data, flags) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedBufferSubData(&self, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.NamedBufferSubData.f)(buffer, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferDrawBuffer(&self, framebuffer: types::GLuint, buf: enums::ColorBuffer) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ColorBuffer) -> ()>(self.ptrs.NamedFramebufferDrawBuffer.f)(framebuffer, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferDrawBuffers(&self, framebuffer: types::GLuint, n: types::GLsizei, bufs: *const enums::ColorBuffer) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const enums::ColorBuffer) -> ()>(self.ptrs.NamedFramebufferDrawBuffers.f)(framebuffer, n, bufs) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferParameteri(&self, framebuffer: types::GLuint, pname: enums::FramebufferParameterName, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferParameterName, types::GLint) -> ()>(self.ptrs.NamedFramebufferParameteri.f)(framebuffer, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferReadBuffer(&self, framebuffer: types::GLuint, src: enums::ColorBuffer) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ColorBuffer) -> ()>(self.ptrs.NamedFramebufferReadBuffer.f)(framebuffer, src) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferRenderbuffer(&self, framebuffer: types::GLuint, attachment: enums::FramebufferAttachment, renderbuffertarget: enums::RenderbufferTarget, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferAttachment, enums::RenderbufferTarget, types::GLuint) -> ()>(self.ptrs.NamedFramebufferRenderbuffer.f)(framebuffer, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferTexture(&self, framebuffer: types::GLuint, attachment: enums::FramebufferAttachment, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferAttachment, types::GLuint, types::GLint) -> ()>(self.ptrs.NamedFramebufferTexture.f)(framebuffer, attachment, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedFramebufferTextureLayer(&self, framebuffer: types::GLuint, attachment: enums::FramebufferAttachment, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::FramebufferAttachment, types::GLuint, types::GLint, types::GLint) -> ()>(self.ptrs.NamedFramebufferTextureLayer.f)(framebuffer, attachment, texture, level, layer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedRenderbufferStorage(&self, renderbuffer: types::GLuint, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.NamedRenderbufferStorage.f)(renderbuffer, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NamedRenderbufferStorageMultisample(&self, renderbuffer: types::GLuint, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.NamedRenderbufferStorageMultisample.f)(renderbuffer, samples, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NormalP3ui(&self, type_: enums::NormalPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::NormalPointerType, types::GLuint) -> ()>(self.ptrs.NormalP3ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn NormalP3uiv(&self, type_: enums::NormalPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::NormalPointerType, *const types::GLuint) -> ()>(self.ptrs.NormalP3uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectLabel(&self, identifier: enums::ObjectIdentifier, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ObjectIdentifier, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.ptrs.ObjectLabel.f)(identifier, name, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *const types::GLchar) -> ()>(self.ptrs.ObjectPtrLabel.f)(ptr, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PatchParameterfv(&self, pname: enums::PatchParameterName, values: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PatchParameterName, *const types::GLfloat) -> ()>(self.ptrs.PatchParameterfv.f)(pname, values) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PatchParameteri(&self, pname: enums::PatchParameterName, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PatchParameterName, types::GLint) -> ()>(self.ptrs.PatchParameteri.f)(pname, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PauseTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.PauseTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PixelStoref(&self, pname: enums::PixelStoreParameter, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PixelStoreParameter, types::GLfloat) -> ()>(self.ptrs.PixelStoref.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PixelStorei(&self, pname: enums::PixelStoreParameter, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::PixelStoreParameter, types::GLint) -> ()>(self.ptrs.PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameterf(&self, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(self.ptrs.PointParameterf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameterfv(&self, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(self.ptrs.PointParameterfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameteri(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.ptrs.PointParameteri.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointParameteriv(&self, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(self.ptrs.PointParameteriv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PointSize(&self, size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ptrs.PointSize.f)(size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PolygonMode(&self, face: enums::MaterialFace, mode: enums::PolygonMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::MaterialFace, enums::PolygonMode) -> ()>(self.ptrs.PolygonMode.f)(face, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.ptrs.PolygonOffset.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PopDebugGroup(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.PopDebugGroup.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PrimitiveRestartIndex(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.PrimitiveRestartIndex.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramBinary(&self, program: types::GLuint, binaryFormat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ptrs.ProgramBinary.f)(program, binaryFormat, binary, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramParameteri(&self, program: types::GLuint, pname: enums::ProgramParameterPName, value: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::ProgramParameterPName, types::GLint) -> ()>(self.ptrs.ProgramParameteri.f)(program, pname, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble) -> ()>(self.ptrs.ProgramUniform1d.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniform1dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat) -> ()>(self.ptrs.ProgramUniform1f.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniform1fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(self.ptrs.ProgramUniform1i.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.ProgramUniform1iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint) -> ()>(self.ptrs.ProgramUniform1ui.f)(program, location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform1uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.ProgramUniform1uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.ProgramUniform2d.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniform2dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.ProgramUniform2f.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniform2fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.ProgramUniform2i.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.ProgramUniform2iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint) -> ()>(self.ptrs.ProgramUniform2ui.f)(program, location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform2uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.ProgramUniform2uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.ProgramUniform3d.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniform3dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.ProgramUniform3f.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniform3fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.ProgramUniform3i.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.ProgramUniform3iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.ProgramUniform3ui.f)(program, location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform3uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.ProgramUniform3uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4d(&self, program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble, v3: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.ProgramUniform4d.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniform4dv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4f(&self, program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.ProgramUniform4f.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniform4fv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4i(&self, program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.ProgramUniform4i.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4iv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.ProgramUniform4iv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4ui(&self, program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.ProgramUniform4ui.f)(program, location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniform4uiv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.ProgramUniform4uiv.f)(program, location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix2x4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix3x4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix4dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix4fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x2dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x2fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x3dv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProgramUniformMatrix4x3fv(&self, program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ProvokingVertex(&self, mode: enums::VertexProvokingMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexProvokingMode) -> ()>(self.ptrs.ProvokingVertex.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PushDebugGroup(&self, source: enums::DebugSource, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::DebugSource, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.ptrs.PushDebugGroup.f)(source, id, length, message) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn QueryCounter(&self, id: types::GLuint, target: enums::QueryCounterTarget) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::QueryCounterTarget) -> ()>(self.ptrs.QueryCounter.f)(id, target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadBuffer(&self, src: enums::ReadBufferMode) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ReadBufferMode) -> ()>(self.ptrs.ReadBuffer.f)(src) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadnPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, bufSize: types::GLsizei, data: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, types::GLsizei, *mut __gl_imports::raw::c_void) -> ()>(self.ptrs.ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReleaseShaderCompiler(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.ReleaseShaderCompiler.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn RenderbufferStorage(&self, target: enums::RenderbufferTarget, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::RenderbufferTarget, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.RenderbufferStorage.f)(target, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn RenderbufferStorageMultisample(&self, target: enums::RenderbufferTarget, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::RenderbufferTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ResumeTransformFeedback(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.ResumeTransformFeedback.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SampleCoverage(&self, value: types::GLfloat, invert: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, enums::Boolean) -> ()>(self.ptrs.SampleCoverage.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SampleMaski(&self, maskNumber: types::GLuint, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield) -> ()>(self.ptrs.SampleMaski.f)(maskNumber, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterIiv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *const types::GLint) -> ()>(self.ptrs.SamplerParameterIiv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterIuiv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *const types::GLuint) -> ()>(self.ptrs.SamplerParameterIuiv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterf(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, types::GLfloat) -> ()>(self.ptrs.SamplerParameterf.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameterfv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *const types::GLfloat) -> ()>(self.ptrs.SamplerParameterfv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameteri(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, types::GLint) -> ()>(self.ptrs.SamplerParameteri.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SamplerParameteriv(&self, sampler: types::GLuint, pname: enums::SamplerParameterName, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::SamplerParameterName, *const types::GLint) -> ()>(self.ptrs.SamplerParameteriv.f)(sampler, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Scissor(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.ScissorArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorIndexed(&self, index: types::GLuint, left: types::GLint, bottom: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.ScissorIndexed.f)(index, left, bottom, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ScissorIndexedv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.ScissorIndexedv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SecondaryColorP3ui(&self, type_: enums::ColorPointerType, color: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, types::GLuint) -> ()>(self.ptrs.SecondaryColorP3ui.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SecondaryColorP3uiv(&self, type_: enums::ColorPointerType, color: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ColorPointerType, *const types::GLuint) -> ()>(self.ptrs.SecondaryColorP3uiv.f)(type_, color) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderBinary(&self, count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ptrs.ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderSource(&self, shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *const types::GLint) -> ()>(self.ptrs.ShaderSource.f)(shader, count, string, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderStorageBlockBinding(&self, program: types::GLuint, storageBlockIndex: types::GLuint, storageBlockBinding: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFunc(&self, func: enums::StencilFunction, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StencilFunction, types::GLint, types::GLuint) -> ()>(self.ptrs.StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFuncSeparate(&self, face: enums::StencilFaceDirection, func: enums::StencilFunction, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StencilFaceDirection, enums::StencilFunction, types::GLint, types::GLuint) -> ()>(self.ptrs.StencilFuncSeparate.f)(face, func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMaskSeparate(&self, face: enums::StencilFaceDirection, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StencilFaceDirection, types::GLuint) -> ()>(self.ptrs.StencilMaskSeparate.f)(face, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOp(&self, fail: enums::StencilOp, zfail: enums::StencilOp, zpass: enums::StencilOp) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StencilOp, enums::StencilOp, enums::StencilOp) -> ()>(self.ptrs.StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOpSeparate(&self, face: enums::StencilFaceDirection, sfail: enums::StencilOp, dpfail: enums::StencilOp, dppass: enums::StencilOp) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::StencilFaceDirection, enums::StencilOp, enums::StencilOp, enums::StencilOp) -> ()>(self.ptrs.StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexBuffer(&self, target: enums::TextureTarget, internalformat: enums::InternalFormat, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::InternalFormat, types::GLuint) -> ()>(self.ptrs.TexBuffer.f)(target, internalformat, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexBufferRange(&self, target: enums::TextureTarget, internalformat: enums::InternalFormat, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::InternalFormat, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.TexBufferRange.f)(target, internalformat, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP1ui(&self, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.TexCoordP1ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP1uiv(&self, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.TexCoordP1uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP2ui(&self, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.TexCoordP2ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP2uiv(&self, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.TexCoordP2uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP3ui(&self, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.TexCoordP3ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP3uiv(&self, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.TexCoordP3uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP4ui(&self, type_: enums::TexCoordPointerType, coords: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, types::GLuint) -> ()>(self.ptrs.TexCoordP4ui.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexCoordP4uiv(&self, type_: enums::TexCoordPointerType, coords: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TexCoordPointerType, *const types::GLuint) -> ()>(self.ptrs.TexCoordP4uiv.f)(type_, coords) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage1D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, border: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLint, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage2D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLint, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage2DMultisample(&self, target: enums::TextureTarget, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, enums::Boolean) -> ()>(self.ptrs.TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage3D(&self, target: enums::TextureTarget, level: types::GLint, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage3DMultisample(&self, target: enums::TextureTarget, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei, enums::Boolean) -> ()>(self.ptrs.TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterIiv(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, *const types::GLint) -> ()>(self.ptrs.TexParameterIiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterIuiv(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, params: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, *const types::GLuint) -> ()>(self.ptrs.TexParameterIuiv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterf(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, types::GLfloat) -> ()>(self.ptrs.TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterfv(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, *const types::GLfloat) -> ()>(self.ptrs.TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteri(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, types::GLint) -> ()>(self.ptrs.TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteriv(&self, target: enums::TextureTarget, pname: enums::TextureParameterName, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, enums::TextureParameterName, *const types::GLint) -> ()>(self.ptrs.TexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage1D(&self, target: enums::TextureTarget, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei) -> ()>(self.ptrs.TexStorage1D.f)(target, levels, internalformat, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage2D(&self, target: enums::TextureTarget, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.TexStorage2D.f)(target, levels, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage2DMultisample(&self, target: enums::TextureTarget, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, enums::Boolean) -> ()>(self.ptrs.TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage3D(&self, target: enums::TextureTarget, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.TexStorage3D.f)(target, levels, internalformat, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexStorage3DMultisample(&self, target: enums::TextureTarget, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: enums::Boolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei, enums::Boolean) -> ()>(self.ptrs.TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage1D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage2D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage3D(&self, target: enums::TextureTarget, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::TextureTarget, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBarrier(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ptrs.TextureBarrier.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBuffer(&self, texture: types::GLuint, internalformat: enums::InternalFormat, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::InternalFormat, types::GLuint) -> ()>(self.ptrs.TextureBuffer.f)(texture, internalformat, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureBufferRange(&self, texture: types::GLuint, internalformat: enums::InternalFormat, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::InternalFormat, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.TextureBufferRange.f)(texture, internalformat, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterIiv(&self, texture: types::GLuint, pname: enums::TextureParameterName, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, *const types::GLint) -> ()>(self.ptrs.TextureParameterIiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterIuiv(&self, texture: types::GLuint, pname: enums::TextureParameterName, params: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, *const types::GLuint) -> ()>(self.ptrs.TextureParameterIuiv.f)(texture, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterf(&self, texture: types::GLuint, pname: enums::TextureParameterName, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, types::GLfloat) -> ()>(self.ptrs.TextureParameterf.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameterfv(&self, texture: types::GLuint, pname: enums::TextureParameterName, param: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, *const types::GLfloat) -> ()>(self.ptrs.TextureParameterfv.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameteri(&self, texture: types::GLuint, pname: enums::TextureParameterName, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, types::GLint) -> ()>(self.ptrs.TextureParameteri.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureParameteriv(&self, texture: types::GLuint, pname: enums::TextureParameterName, param: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureParameterName, *const types::GLint) -> ()>(self.ptrs.TextureParameteriv.f)(texture, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage1D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei) -> ()>(self.ptrs.TextureStorage1D.f)(texture, levels, internalformat, width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage2D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.TextureStorage2D.f)(texture, levels, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage2DMultisample(&self, texture: types::GLuint, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.ptrs.TextureStorage2DMultisample.f)(texture, samples, internalformat, width, height, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage3D(&self, texture: types::GLuint, levels: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.TextureStorage3D.f)(texture, levels, internalformat, width, height, depth) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureStorage3DMultisample(&self, texture: types::GLuint, samples: types::GLsizei, internalformat: enums::InternalFormat, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, enums::InternalFormat, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(self.ptrs.TextureStorage3DMultisample.f)(texture, samples, internalformat, width, height, depth, fixedsamplelocations) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage1D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TextureSubImage1D.f)(texture, level, xoffset, width, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage2D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureSubImage3D(&self, texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: enums::PixelFormat, type_: enums::PixelType, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, enums::PixelFormat, enums::PixelType, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.TextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TextureView(&self, texture: types::GLuint, target: enums::TextureTarget, origtexture: types::GLuint, internalformat: enums::InternalFormat, minlevel: types::GLuint, numlevels: types::GLuint, minlayer: types::GLuint, numlayers: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::TextureTarget, types::GLuint, enums::InternalFormat, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackBufferBase(&self, xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.TransformFeedbackBufferBase.f)(xfb, index, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackBufferRange(&self, xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(self.ptrs.TransformFeedbackBufferRange.f)(xfb, index, buffer, offset, size) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TransformFeedbackVaryings(&self, program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, types::GLenum) -> ()>(self.ptrs.TransformFeedbackVaryings.f)(program, count, varyings, bufferMode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1d(&self, location: types::GLint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble) -> ()>(self.ptrs.Uniform1d.f)(location, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.Uniform1dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1f(&self, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(self.ptrs.Uniform1f.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.Uniform1fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1i(&self, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.ptrs.Uniform1i.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.Uniform1iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1ui(&self, location: types::GLint, v0: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint) -> ()>(self.ptrs.Uniform1ui.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.Uniform1uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.Uniform2d.f)(location, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.Uniform2dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.Uniform2f.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.Uniform2fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.Uniform2i.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.Uniform2iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint) -> ()>(self.ptrs.Uniform2ui.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.Uniform2uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.Uniform3d.f)(location, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.Uniform3dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.Uniform3f.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.Uniform3fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.Uniform3i.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.Uniform3iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.Uniform3ui.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.Uniform3uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4d(&self, location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.Uniform4d.f)(location, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4dv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(self.ptrs.Uniform4dv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.Uniform4f.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.Uniform4fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.Uniform4i.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.ptrs.Uniform4iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4ui(&self, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.Uniform4ui.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4uiv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.Uniform4uiv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformBlockBinding(&self, program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x3dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix2x3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x3fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix2x3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x4dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix2x4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2x4fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix2x4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x2dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix3x2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x2fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix3x2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x4dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix3x4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3x4fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix3x4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix4dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x2dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix4x2dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x2fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix4x2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x3dv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLdouble) -> ()>(self.ptrs.UniformMatrix4x3dv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4x3fv(&self, location: types::GLint, count: types::GLsizei, transpose: enums::Boolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, enums::Boolean, *const types::GLfloat) -> ()>(self.ptrs.UniformMatrix4x3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformSubroutinesuiv(&self, shadertype: enums::ShaderType, count: types::GLsizei, indices: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::ShaderType, types::GLsizei, *const types::GLuint) -> ()>(self.ptrs.UniformSubroutinesuiv.f)(shadertype, count, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UnmapBuffer(&self, target: enums::BufferTargetARB) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(enums::BufferTargetARB) -> types::GLboolean>(self.ptrs.UnmapBuffer.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UnmapNamedBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.ptrs.UnmapNamedBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UseProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.UseProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UseProgramStages(&self, pipeline: types::GLuint, stages: enums::UseProgramStageMask, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::UseProgramStageMask, types::GLuint) -> ()>(self.ptrs.UseProgramStages.f)(pipeline, stages, program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ValidateProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.ValidateProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ValidateProgramPipeline(&self, pipeline: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ptrs.ValidateProgramPipeline.f)(pipeline) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribBinding(&self, vaobj: types::GLuint, attribindex: types::GLuint, bindingindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexArrayAttribBinding.f)(vaobj, attribindex, bindingindex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: enums::VertexAttribType, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, enums::VertexAttribType, types::GLboolean, types::GLuint) -> ()>(self.ptrs.VertexArrayAttribFormat.f)(vaobj, attribindex, size, type_, normalized, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribIFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: enums::VertexAttribType, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, enums::VertexAttribType, types::GLuint) -> ()>(self.ptrs.VertexArrayAttribIFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayAttribLFormat(&self, vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: enums::VertexAttribType, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, enums::VertexAttribType, types::GLuint) -> ()>(self.ptrs.VertexArrayAttribLFormat.f)(vaobj, attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayBindingDivisor(&self, vaobj: types::GLuint, bindingindex: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexArrayBindingDivisor.f)(vaobj, bindingindex, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayElementBuffer(&self, vaobj: types::GLuint, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexArrayElementBuffer.f)(vaobj, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayVertexBuffer(&self, vaobj: types::GLuint, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(self.ptrs.VertexArrayVertexBuffer.f)(vaobj, bindingindex, buffer, offset, stride) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexArrayVertexBuffers(&self, vaobj: types::GLuint, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(self.ptrs.VertexArrayVertexBuffers.f)(vaobj, first, count, buffers, offsets, strides) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1d(&self, index: types::GLuint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(self.ptrs.VertexAttrib1d.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttrib1dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1f(&self, index: types::GLuint, x: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(self.ptrs.VertexAttrib1f.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ptrs.VertexAttrib1fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1s(&self, index: types::GLuint, x: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort) -> ()>(self.ptrs.VertexAttrib1s.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttrib1sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttrib2d.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttrib2dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.VertexAttrib2f.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ptrs.VertexAttrib2fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort) -> ()>(self.ptrs.VertexAttrib2s.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttrib2sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttrib3d.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttrib3dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.VertexAttrib3f.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ptrs.VertexAttrib3fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.ptrs.VertexAttrib3s.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttrib3sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nbv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.ptrs.VertexAttrib4Nbv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Niv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttrib4Niv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nsv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttrib4Nsv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nub(&self, index: types::GLuint, x: types::GLubyte, y: types::GLubyte, z: types::GLubyte, w: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(self.ptrs.VertexAttrib4Nub.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.ptrs.VertexAttrib4Nubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nuiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttrib4Nuiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4Nusv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.ptrs.VertexAttrib4Nusv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.ptrs.VertexAttrib4bv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttrib4d.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttrib4dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.VertexAttrib4f.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ptrs.VertexAttrib4fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttrib4iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4s(&self, index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(self.ptrs.VertexAttrib4s.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttrib4sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.ptrs.VertexAttrib4ubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttrib4uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.ptrs.VertexAttrib4usv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribBinding(&self, attribindex: types::GLuint, bindingindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribBinding.f)(attribindex, bindingindex) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribDivisor(&self, index: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribDivisor.f)(index, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: enums::Boolean, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, enums::Boolean, types::GLuint) -> ()>(self.ptrs.VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1i(&self, index: types::GLuint, x: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(self.ptrs.VertexAttribI1i.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttribI1iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1ui(&self, index: types::GLuint, x: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribI1ui.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI1uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttribI1uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2i(&self, index: types::GLuint, x: types::GLint, y: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(self.ptrs.VertexAttribI2i.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttribI2iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribI2ui.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI2uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttribI2uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3i(&self, index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.VertexAttribI3i.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttribI3iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribI3ui.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI3uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttribI3uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4bv(&self, index: types::GLuint, v: *const types::GLbyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(self.ptrs.VertexAttribI4bv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4i(&self, index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.ptrs.VertexAttribI4i.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4iv(&self, index: types::GLuint, v: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(self.ptrs.VertexAttribI4iv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4sv(&self, index: types::GLuint, v: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(self.ptrs.VertexAttribI4sv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4ubv(&self, index: types::GLuint, v: *const types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(self.ptrs.VertexAttribI4ubv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4ui(&self, index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexAttribI4ui.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4uiv(&self, index: types::GLuint, v: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(self.ptrs.VertexAttribI4uiv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribI4usv(&self, index: types::GLuint, v: *const types::GLushort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(self.ptrs.VertexAttribI4usv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribIFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(self.ptrs.VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribIPointer(&self, index: types::GLuint, size: types::GLint, type_: enums::VertexAttribPointerType, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::VertexAttribPointerType, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.VertexAttribIPointer.f)(index, size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL1d(&self, index: types::GLuint, x: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(self.ptrs.VertexAttribL1d.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL1dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttribL1dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL2d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttribL2d.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL2dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttribL2dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL3d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttribL3d.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL3dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttribL3dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL4d(&self, index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(self.ptrs.VertexAttribL4d.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribL4dv(&self, index: types::GLuint, v: *const types::GLdouble) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(self.ptrs.VertexAttribL4dv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribLFormat(&self, attribindex: types::GLuint, size: types::GLint, type_: enums::VertexAttribType, relativeoffset: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::VertexAttribType, types::GLuint) -> ()>(self.ptrs.VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribLPointer(&self, index: types::GLuint, size: types::GLint, type_: enums::VertexAttribPointerType, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::VertexAttribPointerType, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.VertexAttribLPointer.f)(index, size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP1ui(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, types::GLuint) -> ()>(self.ptrs.VertexAttribP1ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP1uiv(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, *const types::GLuint) -> ()>(self.ptrs.VertexAttribP1uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP2ui(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, types::GLuint) -> ()>(self.ptrs.VertexAttribP2ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP2uiv(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, *const types::GLuint) -> ()>(self.ptrs.VertexAttribP2uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP3ui(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, types::GLuint) -> ()>(self.ptrs.VertexAttribP3ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP3uiv(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, *const types::GLuint) -> ()>(self.ptrs.VertexAttribP3uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP4ui(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, types::GLuint) -> ()>(self.ptrs.VertexAttribP4ui.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribP4uiv(&self, index: types::GLuint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, enums::VertexAttribPointerType, enums::Boolean, *const types::GLuint) -> ()>(self.ptrs.VertexAttribP4uiv.f)(index, type_, normalized, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribPointer(&self, index: types::GLuint, size: types::GLint, type_: enums::VertexAttribPointerType, normalized: enums::Boolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, enums::VertexAttribPointerType, enums::Boolean, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.ptrs.VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexBindingDivisor(&self, bindingindex: types::GLuint, divisor: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.ptrs.VertexBindingDivisor.f)(bindingindex, divisor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP2ui(&self, type_: enums::VertexPointerType, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, types::GLuint) -> ()>(self.ptrs.VertexP2ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP2uiv(&self, type_: enums::VertexPointerType, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, *const types::GLuint) -> ()>(self.ptrs.VertexP2uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP3ui(&self, type_: enums::VertexPointerType, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, types::GLuint) -> ()>(self.ptrs.VertexP3ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP3uiv(&self, type_: enums::VertexPointerType, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, *const types::GLuint) -> ()>(self.ptrs.VertexP3uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP4ui(&self, type_: enums::VertexPointerType, value: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, types::GLuint) -> ()>(self.ptrs.VertexP4ui.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexP4uiv(&self, type_: enums::VertexPointerType, value: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(enums::VertexPointerType, *const types::GLuint) -> ()>(self.ptrs.VertexP4uiv.f)(type_, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Viewport(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.ptrs.Viewport.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportArrayv(&self, first: types::GLuint, count: types::GLsizei, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLfloat) -> ()>(self.ptrs.ViewportArrayv.f)(first, count, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportIndexedf(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, w: types::GLfloat, h: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ptrs.ViewportIndexedf.f)(index, x, y, w, h) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ViewportIndexedfv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.ptrs.ViewportIndexedfv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn WaitSync(&self, sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> ()>(self.ptrs.WaitSync.f)(sync, flags, timeout) }
}

        unsafe impl __gl_imports::Send for Gl {}
macro_rules! impl_enum_traits {
        ($Name:ident) => {

        }
    }

macro_rules! impl_enum_bitmask_traits {
        ($Name:ident) => {
            
        }
    }

pub mod enums {
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, unreachable_patterns)]

use super::types;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct AccumOp(pub types::GLenum);

impl AccumOp {
}

impl ::std::fmt::Debug for AccumOp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "AccumOp({})", self.0),
        }
    }
}

impl_enum_traits!(AccumOp);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct AlphaFunction(pub types::GLenum);

impl AlphaFunction {
    pub const ALWAYS: AlphaFunction = AlphaFunction(super::ALWAYS);
    pub const EQUAL: AlphaFunction = AlphaFunction(super::EQUAL);
    pub const GEQUAL: AlphaFunction = AlphaFunction(super::GEQUAL);
    pub const GREATER: AlphaFunction = AlphaFunction(super::GREATER);
    pub const LEQUAL: AlphaFunction = AlphaFunction(super::LEQUAL);
    pub const LESS: AlphaFunction = AlphaFunction(super::LESS);
    pub const NEVER: AlphaFunction = AlphaFunction(super::NEVER);
    pub const NOTEQUAL: AlphaFunction = AlphaFunction(super::NOTEQUAL);
}

impl ::std::fmt::Debug for AlphaFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AlphaFunction::ALWAYS => write!(fmt, "AlphaFunction(ALWAYS)"),
            AlphaFunction::EQUAL => write!(fmt, "AlphaFunction(EQUAL)"),
            AlphaFunction::GEQUAL => write!(fmt, "AlphaFunction(GEQUAL)"),
            AlphaFunction::GREATER => write!(fmt, "AlphaFunction(GREATER)"),
            AlphaFunction::LEQUAL => write!(fmt, "AlphaFunction(LEQUAL)"),
            AlphaFunction::LESS => write!(fmt, "AlphaFunction(LESS)"),
            AlphaFunction::NEVER => write!(fmt, "AlphaFunction(NEVER)"),
            AlphaFunction::NOTEQUAL => write!(fmt, "AlphaFunction(NOTEQUAL)"),
            _ => write!(fmt, "AlphaFunction({})", self.0),
        }
    }
}

impl_enum_traits!(AlphaFunction);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct AtomicCounterBufferPName(pub types::GLenum);

impl AtomicCounterBufferPName {
    pub const ATOMIC_COUNTER_BUFFER_BINDING: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_BINDING);
    pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_DATA_SIZE);
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS);
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER);
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: AtomicCounterBufferPName = AtomicCounterBufferPName(super::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER);
}

impl ::std::fmt::Debug for AtomicCounterBufferPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_BINDING => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_BINDING)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_DATA_SIZE => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_DATA_SIZE)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER)"),
            AtomicCounterBufferPName::ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER => write!(fmt, "AtomicCounterBufferPName(ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER)"),
            _ => write!(fmt, "AtomicCounterBufferPName({})", self.0),
        }
    }
}

impl_enum_traits!(AtomicCounterBufferPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct AttribMask(pub types::GLenum);

impl AttribMask {
    pub const COLOR_BUFFER_BIT: AttribMask = AttribMask(super::COLOR_BUFFER_BIT);
    pub const DEPTH_BUFFER_BIT: AttribMask = AttribMask(super::DEPTH_BUFFER_BIT);
    pub const STENCIL_BUFFER_BIT: AttribMask = AttribMask(super::STENCIL_BUFFER_BIT);
    pub const Empty: AttribMask = AttribMask(0);
}

impl ::std::fmt::Debug for AttribMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AttribMask::COLOR_BUFFER_BIT => write!(fmt, "AttribMask(COLOR_BUFFER_BIT)"),
            AttribMask::DEPTH_BUFFER_BIT => write!(fmt, "AttribMask(DEPTH_BUFFER_BIT)"),
            AttribMask::STENCIL_BUFFER_BIT => write!(fmt, "AttribMask(STENCIL_BUFFER_BIT)"),
            _ => write!(fmt, "AttribMask({})", self.0),
        }
    }
}

impl_enum_traits!(AttribMask);

impl_enum_bitmask_traits!(AttribMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct AttributeType(pub types::GLenum);

impl AttributeType {
    pub const FLOAT_VEC2: AttributeType = AttributeType(super::FLOAT_VEC2);
    pub const FLOAT_VEC3: AttributeType = AttributeType(super::FLOAT_VEC3);
    pub const FLOAT_VEC4: AttributeType = AttributeType(super::FLOAT_VEC4);
    pub const INT_VEC2: AttributeType = AttributeType(super::INT_VEC2);
    pub const INT_VEC3: AttributeType = AttributeType(super::INT_VEC3);
    pub const INT_VEC4: AttributeType = AttributeType(super::INT_VEC4);
    pub const BOOL: AttributeType = AttributeType(super::BOOL);
    pub const BOOL_VEC2: AttributeType = AttributeType(super::BOOL_VEC2);
    pub const BOOL_VEC3: AttributeType = AttributeType(super::BOOL_VEC3);
    pub const BOOL_VEC4: AttributeType = AttributeType(super::BOOL_VEC4);
    pub const FLOAT_MAT2: AttributeType = AttributeType(super::FLOAT_MAT2);
    pub const FLOAT_MAT3: AttributeType = AttributeType(super::FLOAT_MAT3);
    pub const FLOAT_MAT4: AttributeType = AttributeType(super::FLOAT_MAT4);
    pub const SAMPLER_1D: AttributeType = AttributeType(super::SAMPLER_1D);
    pub const SAMPLER_2D: AttributeType = AttributeType(super::SAMPLER_2D);
    pub const SAMPLER_3D: AttributeType = AttributeType(super::SAMPLER_3D);
    pub const SAMPLER_CUBE: AttributeType = AttributeType(super::SAMPLER_CUBE);
    pub const SAMPLER_1D_SHADOW: AttributeType = AttributeType(super::SAMPLER_1D_SHADOW);
    pub const SAMPLER_2D_SHADOW: AttributeType = AttributeType(super::SAMPLER_2D_SHADOW);
    pub const SAMPLER_2D_RECT: AttributeType = AttributeType(super::SAMPLER_2D_RECT);
    pub const SAMPLER_2D_RECT_SHADOW: AttributeType = AttributeType(super::SAMPLER_2D_RECT_SHADOW);
    pub const FLOAT_MAT2x3: AttributeType = AttributeType(super::FLOAT_MAT2x3);
    pub const FLOAT_MAT2x4: AttributeType = AttributeType(super::FLOAT_MAT2x4);
    pub const FLOAT_MAT3x2: AttributeType = AttributeType(super::FLOAT_MAT3x2);
    pub const FLOAT_MAT3x4: AttributeType = AttributeType(super::FLOAT_MAT3x4);
    pub const FLOAT_MAT4x2: AttributeType = AttributeType(super::FLOAT_MAT4x2);
    pub const FLOAT_MAT4x3: AttributeType = AttributeType(super::FLOAT_MAT4x3);
}

impl ::std::fmt::Debug for AttributeType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AttributeType::BOOL => write!(fmt, "AttributeType(BOOL)"),
            AttributeType::BOOL_VEC2 => write!(fmt, "AttributeType(BOOL_VEC2)"),
            AttributeType::BOOL_VEC3 => write!(fmt, "AttributeType(BOOL_VEC3)"),
            AttributeType::BOOL_VEC4 => write!(fmt, "AttributeType(BOOL_VEC4)"),
            AttributeType::FLOAT_MAT2 => write!(fmt, "AttributeType(FLOAT_MAT2)"),
            AttributeType::FLOAT_MAT2x3 => write!(fmt, "AttributeType(FLOAT_MAT2x3)"),
            AttributeType::FLOAT_MAT2x4 => write!(fmt, "AttributeType(FLOAT_MAT2x4)"),
            AttributeType::FLOAT_MAT3 => write!(fmt, "AttributeType(FLOAT_MAT3)"),
            AttributeType::FLOAT_MAT3x2 => write!(fmt, "AttributeType(FLOAT_MAT3x2)"),
            AttributeType::FLOAT_MAT3x4 => write!(fmt, "AttributeType(FLOAT_MAT3x4)"),
            AttributeType::FLOAT_MAT4 => write!(fmt, "AttributeType(FLOAT_MAT4)"),
            AttributeType::FLOAT_MAT4x2 => write!(fmt, "AttributeType(FLOAT_MAT4x2)"),
            AttributeType::FLOAT_MAT4x3 => write!(fmt, "AttributeType(FLOAT_MAT4x3)"),
            AttributeType::FLOAT_VEC2 => write!(fmt, "AttributeType(FLOAT_VEC2)"),
            AttributeType::FLOAT_VEC3 => write!(fmt, "AttributeType(FLOAT_VEC3)"),
            AttributeType::FLOAT_VEC4 => write!(fmt, "AttributeType(FLOAT_VEC4)"),
            AttributeType::INT_VEC2 => write!(fmt, "AttributeType(INT_VEC2)"),
            AttributeType::INT_VEC3 => write!(fmt, "AttributeType(INT_VEC3)"),
            AttributeType::INT_VEC4 => write!(fmt, "AttributeType(INT_VEC4)"),
            AttributeType::SAMPLER_1D => write!(fmt, "AttributeType(SAMPLER_1D)"),
            AttributeType::SAMPLER_1D_SHADOW => write!(fmt, "AttributeType(SAMPLER_1D_SHADOW)"),
            AttributeType::SAMPLER_2D => write!(fmt, "AttributeType(SAMPLER_2D)"),
            AttributeType::SAMPLER_2D_RECT => write!(fmt, "AttributeType(SAMPLER_2D_RECT)"),
            AttributeType::SAMPLER_2D_RECT_SHADOW => write!(fmt, "AttributeType(SAMPLER_2D_RECT_SHADOW)"),
            AttributeType::SAMPLER_2D_SHADOW => write!(fmt, "AttributeType(SAMPLER_2D_SHADOW)"),
            AttributeType::SAMPLER_3D => write!(fmt, "AttributeType(SAMPLER_3D)"),
            AttributeType::SAMPLER_CUBE => write!(fmt, "AttributeType(SAMPLER_CUBE)"),
            _ => write!(fmt, "AttributeType({})", self.0),
        }
    }
}

impl_enum_traits!(AttributeType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BindTransformFeedbackTarget(pub types::GLenum);

impl BindTransformFeedbackTarget {
    pub const TRANSFORM_FEEDBACK: BindTransformFeedbackTarget = BindTransformFeedbackTarget(super::TRANSFORM_FEEDBACK);
}

impl ::std::fmt::Debug for BindTransformFeedbackTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BindTransformFeedbackTarget::TRANSFORM_FEEDBACK => write!(fmt, "BindTransformFeedbackTarget(TRANSFORM_FEEDBACK)"),
            _ => write!(fmt, "BindTransformFeedbackTarget({})", self.0),
        }
    }
}

impl_enum_traits!(BindTransformFeedbackTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BlendEquationModeEXT(pub types::GLenum);

impl BlendEquationModeEXT {
    pub const FUNC_ADD: BlendEquationModeEXT = BlendEquationModeEXT(super::FUNC_ADD);
    pub const FUNC_REVERSE_SUBTRACT: BlendEquationModeEXT = BlendEquationModeEXT(super::FUNC_REVERSE_SUBTRACT);
    pub const FUNC_SUBTRACT: BlendEquationModeEXT = BlendEquationModeEXT(super::FUNC_SUBTRACT);
    pub const MAX: BlendEquationModeEXT = BlendEquationModeEXT(super::MAX);
    pub const MIN: BlendEquationModeEXT = BlendEquationModeEXT(super::MIN);
}

impl ::std::fmt::Debug for BlendEquationModeEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BlendEquationModeEXT::FUNC_ADD => write!(fmt, "BlendEquationModeEXT(FUNC_ADD)"),
            BlendEquationModeEXT::FUNC_REVERSE_SUBTRACT => write!(fmt, "BlendEquationModeEXT(FUNC_REVERSE_SUBTRACT)"),
            BlendEquationModeEXT::FUNC_SUBTRACT => write!(fmt, "BlendEquationModeEXT(FUNC_SUBTRACT)"),
            BlendEquationModeEXT::MAX => write!(fmt, "BlendEquationModeEXT(MAX)"),
            BlendEquationModeEXT::MIN => write!(fmt, "BlendEquationModeEXT(MIN)"),
            _ => write!(fmt, "BlendEquationModeEXT({})", self.0),
        }
    }
}

impl_enum_traits!(BlendEquationModeEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BlendingFactor(pub types::GLenum);

impl BlendingFactor {
    pub const ZERO: BlendingFactor = BlendingFactor(super::ZERO);
    pub const ONE: BlendingFactor = BlendingFactor(super::ONE);
    pub const SRC_COLOR: BlendingFactor = BlendingFactor(super::SRC_COLOR);
    pub const ONE_MINUS_SRC_COLOR: BlendingFactor = BlendingFactor(super::ONE_MINUS_SRC_COLOR);
    pub const DST_COLOR: BlendingFactor = BlendingFactor(super::DST_COLOR);
    pub const ONE_MINUS_DST_COLOR: BlendingFactor = BlendingFactor(super::ONE_MINUS_DST_COLOR);
    pub const SRC_ALPHA: BlendingFactor = BlendingFactor(super::SRC_ALPHA);
    pub const ONE_MINUS_SRC_ALPHA: BlendingFactor = BlendingFactor(super::ONE_MINUS_SRC_ALPHA);
    pub const DST_ALPHA: BlendingFactor = BlendingFactor(super::DST_ALPHA);
    pub const ONE_MINUS_DST_ALPHA: BlendingFactor = BlendingFactor(super::ONE_MINUS_DST_ALPHA);
    pub const CONSTANT_COLOR: BlendingFactor = BlendingFactor(super::CONSTANT_COLOR);
    pub const ONE_MINUS_CONSTANT_COLOR: BlendingFactor = BlendingFactor(super::ONE_MINUS_CONSTANT_COLOR);
    pub const CONSTANT_ALPHA: BlendingFactor = BlendingFactor(super::CONSTANT_ALPHA);
    pub const ONE_MINUS_CONSTANT_ALPHA: BlendingFactor = BlendingFactor(super::ONE_MINUS_CONSTANT_ALPHA);
    pub const SRC_ALPHA_SATURATE: BlendingFactor = BlendingFactor(super::SRC_ALPHA_SATURATE);
    pub const SRC1_COLOR: BlendingFactor = BlendingFactor(super::SRC1_COLOR);
    pub const ONE_MINUS_SRC1_COLOR: BlendingFactor = BlendingFactor(super::ONE_MINUS_SRC1_COLOR);
    pub const SRC1_ALPHA: BlendingFactor = BlendingFactor(super::SRC1_ALPHA);
    pub const ONE_MINUS_SRC1_ALPHA: BlendingFactor = BlendingFactor(super::ONE_MINUS_SRC1_ALPHA);
}

impl ::std::fmt::Debug for BlendingFactor {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BlendingFactor::CONSTANT_ALPHA => write!(fmt, "BlendingFactor(CONSTANT_ALPHA)"),
            BlendingFactor::CONSTANT_COLOR => write!(fmt, "BlendingFactor(CONSTANT_COLOR)"),
            BlendingFactor::DST_ALPHA => write!(fmt, "BlendingFactor(DST_ALPHA)"),
            BlendingFactor::DST_COLOR => write!(fmt, "BlendingFactor(DST_COLOR)"),
            BlendingFactor::ONE => write!(fmt, "BlendingFactor(ONE)"),
            BlendingFactor::ONE_MINUS_CONSTANT_ALPHA => write!(fmt, "BlendingFactor(ONE_MINUS_CONSTANT_ALPHA)"),
            BlendingFactor::ONE_MINUS_CONSTANT_COLOR => write!(fmt, "BlendingFactor(ONE_MINUS_CONSTANT_COLOR)"),
            BlendingFactor::ONE_MINUS_DST_ALPHA => write!(fmt, "BlendingFactor(ONE_MINUS_DST_ALPHA)"),
            BlendingFactor::ONE_MINUS_DST_COLOR => write!(fmt, "BlendingFactor(ONE_MINUS_DST_COLOR)"),
            BlendingFactor::ONE_MINUS_SRC1_ALPHA => write!(fmt, "BlendingFactor(ONE_MINUS_SRC1_ALPHA)"),
            BlendingFactor::ONE_MINUS_SRC1_COLOR => write!(fmt, "BlendingFactor(ONE_MINUS_SRC1_COLOR)"),
            BlendingFactor::ONE_MINUS_SRC_ALPHA => write!(fmt, "BlendingFactor(ONE_MINUS_SRC_ALPHA)"),
            BlendingFactor::ONE_MINUS_SRC_COLOR => write!(fmt, "BlendingFactor(ONE_MINUS_SRC_COLOR)"),
            BlendingFactor::SRC1_ALPHA => write!(fmt, "BlendingFactor(SRC1_ALPHA)"),
            BlendingFactor::SRC1_COLOR => write!(fmt, "BlendingFactor(SRC1_COLOR)"),
            BlendingFactor::SRC_ALPHA => write!(fmt, "BlendingFactor(SRC_ALPHA)"),
            BlendingFactor::SRC_ALPHA_SATURATE => write!(fmt, "BlendingFactor(SRC_ALPHA_SATURATE)"),
            BlendingFactor::SRC_COLOR => write!(fmt, "BlendingFactor(SRC_COLOR)"),
            BlendingFactor::ZERO => write!(fmt, "BlendingFactor(ZERO)"),
            _ => write!(fmt, "BlendingFactor({})", self.0),
        }
    }
}

impl_enum_traits!(BlendingFactor);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BlitFramebufferFilter(pub types::GLenum);

impl BlitFramebufferFilter {
    pub const NEAREST: BlitFramebufferFilter = BlitFramebufferFilter(super::NEAREST);
    pub const LINEAR: BlitFramebufferFilter = BlitFramebufferFilter(super::LINEAR);
}

impl ::std::fmt::Debug for BlitFramebufferFilter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BlitFramebufferFilter::LINEAR => write!(fmt, "BlitFramebufferFilter(LINEAR)"),
            BlitFramebufferFilter::NEAREST => write!(fmt, "BlitFramebufferFilter(NEAREST)"),
            _ => write!(fmt, "BlitFramebufferFilter({})", self.0),
        }
    }
}

impl_enum_traits!(BlitFramebufferFilter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Boolean(pub types::GLboolean);

impl Boolean {
    pub const FALSE: Boolean = Boolean(super::FALSE);
    pub const TRUE: Boolean = Boolean(super::TRUE);
}

impl ::std::fmt::Debug for Boolean {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Boolean::FALSE => write!(fmt, "Boolean(FALSE)"),
            Boolean::TRUE => write!(fmt, "Boolean(TRUE)"),
            _ => write!(fmt, "Boolean({})", self.0),
        }
    }
}

impl_enum_traits!(Boolean);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Buffer(pub types::GLenum);

impl Buffer {
    pub const COLOR: Buffer = Buffer(super::COLOR);
    pub const DEPTH: Buffer = Buffer(super::DEPTH);
    pub const STENCIL: Buffer = Buffer(super::STENCIL);
}

impl ::std::fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Buffer::COLOR => write!(fmt, "Buffer(COLOR)"),
            Buffer::DEPTH => write!(fmt, "Buffer(DEPTH)"),
            Buffer::STENCIL => write!(fmt, "Buffer(STENCIL)"),
            _ => write!(fmt, "Buffer({})", self.0),
        }
    }
}

impl_enum_traits!(Buffer);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferAccessARB(pub types::GLenum);

impl BufferAccessARB {
    pub const READ_ONLY: BufferAccessARB = BufferAccessARB(super::READ_ONLY);
    pub const WRITE_ONLY: BufferAccessARB = BufferAccessARB(super::WRITE_ONLY);
    pub const READ_WRITE: BufferAccessARB = BufferAccessARB(super::READ_WRITE);
}

impl ::std::fmt::Debug for BufferAccessARB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BufferAccessARB::READ_ONLY => write!(fmt, "BufferAccessARB(READ_ONLY)"),
            BufferAccessARB::READ_WRITE => write!(fmt, "BufferAccessARB(READ_WRITE)"),
            BufferAccessARB::WRITE_ONLY => write!(fmt, "BufferAccessARB(WRITE_ONLY)"),
            _ => write!(fmt, "BufferAccessARB({})", self.0),
        }
    }
}

impl_enum_traits!(BufferAccessARB);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferAccessMask(pub types::GLenum);

impl BufferAccessMask {
    pub const MAP_COHERENT_BIT: BufferAccessMask = BufferAccessMask(super::MAP_COHERENT_BIT);
    pub const MAP_FLUSH_EXPLICIT_BIT: BufferAccessMask = BufferAccessMask(super::MAP_FLUSH_EXPLICIT_BIT);
    pub const MAP_INVALIDATE_BUFFER_BIT: BufferAccessMask = BufferAccessMask(super::MAP_INVALIDATE_BUFFER_BIT);
    pub const MAP_INVALIDATE_RANGE_BIT: BufferAccessMask = BufferAccessMask(super::MAP_INVALIDATE_RANGE_BIT);
    pub const MAP_PERSISTENT_BIT: BufferAccessMask = BufferAccessMask(super::MAP_PERSISTENT_BIT);
    pub const MAP_READ_BIT: BufferAccessMask = BufferAccessMask(super::MAP_READ_BIT);
    pub const MAP_UNSYNCHRONIZED_BIT: BufferAccessMask = BufferAccessMask(super::MAP_UNSYNCHRONIZED_BIT);
    pub const MAP_WRITE_BIT: BufferAccessMask = BufferAccessMask(super::MAP_WRITE_BIT);
    pub const Empty: BufferAccessMask = BufferAccessMask(0);
}

impl ::std::fmt::Debug for BufferAccessMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BufferAccessMask::MAP_COHERENT_BIT => write!(fmt, "BufferAccessMask(MAP_COHERENT_BIT)"),
            BufferAccessMask::MAP_FLUSH_EXPLICIT_BIT => write!(fmt, "BufferAccessMask(MAP_FLUSH_EXPLICIT_BIT)"),
            BufferAccessMask::MAP_INVALIDATE_BUFFER_BIT => write!(fmt, "BufferAccessMask(MAP_INVALIDATE_BUFFER_BIT)"),
            BufferAccessMask::MAP_INVALIDATE_RANGE_BIT => write!(fmt, "BufferAccessMask(MAP_INVALIDATE_RANGE_BIT)"),
            BufferAccessMask::MAP_PERSISTENT_BIT => write!(fmt, "BufferAccessMask(MAP_PERSISTENT_BIT)"),
            BufferAccessMask::MAP_READ_BIT => write!(fmt, "BufferAccessMask(MAP_READ_BIT)"),
            BufferAccessMask::MAP_UNSYNCHRONIZED_BIT => write!(fmt, "BufferAccessMask(MAP_UNSYNCHRONIZED_BIT)"),
            BufferAccessMask::MAP_WRITE_BIT => write!(fmt, "BufferAccessMask(MAP_WRITE_BIT)"),
            _ => write!(fmt, "BufferAccessMask({})", self.0),
        }
    }
}

impl_enum_traits!(BufferAccessMask);

impl_enum_bitmask_traits!(BufferAccessMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferBitQCOM(pub types::GLenum);

impl BufferBitQCOM {
    pub const Empty: BufferBitQCOM = BufferBitQCOM(0);
}

impl ::std::fmt::Debug for BufferBitQCOM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "BufferBitQCOM({})", self.0),
        }
    }
}

impl_enum_traits!(BufferBitQCOM);

impl_enum_bitmask_traits!(BufferBitQCOM);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferStorageTarget(pub types::GLenum);

impl BufferStorageTarget {
    pub const ARRAY_BUFFER: BufferStorageTarget = BufferStorageTarget(super::ARRAY_BUFFER);
    pub const ATOMIC_COUNTER_BUFFER: BufferStorageTarget = BufferStorageTarget(super::ATOMIC_COUNTER_BUFFER);
    pub const COPY_READ_BUFFER: BufferStorageTarget = BufferStorageTarget(super::COPY_READ_BUFFER);
    pub const COPY_WRITE_BUFFER: BufferStorageTarget = BufferStorageTarget(super::COPY_WRITE_BUFFER);
    pub const DISPATCH_INDIRECT_BUFFER: BufferStorageTarget = BufferStorageTarget(super::DISPATCH_INDIRECT_BUFFER);
    pub const DRAW_INDIRECT_BUFFER: BufferStorageTarget = BufferStorageTarget(super::DRAW_INDIRECT_BUFFER);
    pub const ELEMENT_ARRAY_BUFFER: BufferStorageTarget = BufferStorageTarget(super::ELEMENT_ARRAY_BUFFER);
    pub const PIXEL_PACK_BUFFER: BufferStorageTarget = BufferStorageTarget(super::PIXEL_PACK_BUFFER);
    pub const PIXEL_UNPACK_BUFFER: BufferStorageTarget = BufferStorageTarget(super::PIXEL_UNPACK_BUFFER);
    pub const QUERY_BUFFER: BufferStorageTarget = BufferStorageTarget(super::QUERY_BUFFER);
    pub const SHADER_STORAGE_BUFFER: BufferStorageTarget = BufferStorageTarget(super::SHADER_STORAGE_BUFFER);
    pub const TEXTURE_BUFFER: BufferStorageTarget = BufferStorageTarget(super::TEXTURE_BUFFER);
    pub const TRANSFORM_FEEDBACK_BUFFER: BufferStorageTarget = BufferStorageTarget(super::TRANSFORM_FEEDBACK_BUFFER);
    pub const UNIFORM_BUFFER: BufferStorageTarget = BufferStorageTarget(super::UNIFORM_BUFFER);
}

impl ::std::fmt::Debug for BufferStorageTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BufferStorageTarget::ARRAY_BUFFER => write!(fmt, "BufferStorageTarget(ARRAY_BUFFER)"),
            BufferStorageTarget::ATOMIC_COUNTER_BUFFER => write!(fmt, "BufferStorageTarget(ATOMIC_COUNTER_BUFFER)"),
            BufferStorageTarget::COPY_READ_BUFFER => write!(fmt, "BufferStorageTarget(COPY_READ_BUFFER)"),
            BufferStorageTarget::COPY_WRITE_BUFFER => write!(fmt, "BufferStorageTarget(COPY_WRITE_BUFFER)"),
            BufferStorageTarget::DISPATCH_INDIRECT_BUFFER => write!(fmt, "BufferStorageTarget(DISPATCH_INDIRECT_BUFFER)"),
            BufferStorageTarget::DRAW_INDIRECT_BUFFER => write!(fmt, "BufferStorageTarget(DRAW_INDIRECT_BUFFER)"),
            BufferStorageTarget::ELEMENT_ARRAY_BUFFER => write!(fmt, "BufferStorageTarget(ELEMENT_ARRAY_BUFFER)"),
            BufferStorageTarget::PIXEL_PACK_BUFFER => write!(fmt, "BufferStorageTarget(PIXEL_PACK_BUFFER)"),
            BufferStorageTarget::PIXEL_UNPACK_BUFFER => write!(fmt, "BufferStorageTarget(PIXEL_UNPACK_BUFFER)"),
            BufferStorageTarget::QUERY_BUFFER => write!(fmt, "BufferStorageTarget(QUERY_BUFFER)"),
            BufferStorageTarget::SHADER_STORAGE_BUFFER => write!(fmt, "BufferStorageTarget(SHADER_STORAGE_BUFFER)"),
            BufferStorageTarget::TEXTURE_BUFFER => write!(fmt, "BufferStorageTarget(TEXTURE_BUFFER)"),
            BufferStorageTarget::TRANSFORM_FEEDBACK_BUFFER => write!(fmt, "BufferStorageTarget(TRANSFORM_FEEDBACK_BUFFER)"),
            BufferStorageTarget::UNIFORM_BUFFER => write!(fmt, "BufferStorageTarget(UNIFORM_BUFFER)"),
            _ => write!(fmt, "BufferStorageTarget({})", self.0),
        }
    }
}

impl_enum_traits!(BufferStorageTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferTargetARB(pub types::GLenum);

impl BufferTargetARB {
    pub const ARRAY_BUFFER: BufferTargetARB = BufferTargetARB(super::ARRAY_BUFFER);
    pub const ATOMIC_COUNTER_BUFFER: BufferTargetARB = BufferTargetARB(super::ATOMIC_COUNTER_BUFFER);
    pub const COPY_READ_BUFFER: BufferTargetARB = BufferTargetARB(super::COPY_READ_BUFFER);
    pub const COPY_WRITE_BUFFER: BufferTargetARB = BufferTargetARB(super::COPY_WRITE_BUFFER);
    pub const DISPATCH_INDIRECT_BUFFER: BufferTargetARB = BufferTargetARB(super::DISPATCH_INDIRECT_BUFFER);
    pub const DRAW_INDIRECT_BUFFER: BufferTargetARB = BufferTargetARB(super::DRAW_INDIRECT_BUFFER);
    pub const ELEMENT_ARRAY_BUFFER: BufferTargetARB = BufferTargetARB(super::ELEMENT_ARRAY_BUFFER);
    pub const PIXEL_PACK_BUFFER: BufferTargetARB = BufferTargetARB(super::PIXEL_PACK_BUFFER);
    pub const PIXEL_UNPACK_BUFFER: BufferTargetARB = BufferTargetARB(super::PIXEL_UNPACK_BUFFER);
    pub const QUERY_BUFFER: BufferTargetARB = BufferTargetARB(super::QUERY_BUFFER);
    pub const SHADER_STORAGE_BUFFER: BufferTargetARB = BufferTargetARB(super::SHADER_STORAGE_BUFFER);
    pub const TEXTURE_BUFFER: BufferTargetARB = BufferTargetARB(super::TEXTURE_BUFFER);
    pub const TRANSFORM_FEEDBACK_BUFFER: BufferTargetARB = BufferTargetARB(super::TRANSFORM_FEEDBACK_BUFFER);
    pub const UNIFORM_BUFFER: BufferTargetARB = BufferTargetARB(super::UNIFORM_BUFFER);
}

impl ::std::fmt::Debug for BufferTargetARB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BufferTargetARB::ARRAY_BUFFER => write!(fmt, "BufferTargetARB(ARRAY_BUFFER)"),
            BufferTargetARB::ATOMIC_COUNTER_BUFFER => write!(fmt, "BufferTargetARB(ATOMIC_COUNTER_BUFFER)"),
            BufferTargetARB::COPY_READ_BUFFER => write!(fmt, "BufferTargetARB(COPY_READ_BUFFER)"),
            BufferTargetARB::COPY_WRITE_BUFFER => write!(fmt, "BufferTargetARB(COPY_WRITE_BUFFER)"),
            BufferTargetARB::DISPATCH_INDIRECT_BUFFER => write!(fmt, "BufferTargetARB(DISPATCH_INDIRECT_BUFFER)"),
            BufferTargetARB::DRAW_INDIRECT_BUFFER => write!(fmt, "BufferTargetARB(DRAW_INDIRECT_BUFFER)"),
            BufferTargetARB::ELEMENT_ARRAY_BUFFER => write!(fmt, "BufferTargetARB(ELEMENT_ARRAY_BUFFER)"),
            BufferTargetARB::PIXEL_PACK_BUFFER => write!(fmt, "BufferTargetARB(PIXEL_PACK_BUFFER)"),
            BufferTargetARB::PIXEL_UNPACK_BUFFER => write!(fmt, "BufferTargetARB(PIXEL_UNPACK_BUFFER)"),
            BufferTargetARB::QUERY_BUFFER => write!(fmt, "BufferTargetARB(QUERY_BUFFER)"),
            BufferTargetARB::SHADER_STORAGE_BUFFER => write!(fmt, "BufferTargetARB(SHADER_STORAGE_BUFFER)"),
            BufferTargetARB::TEXTURE_BUFFER => write!(fmt, "BufferTargetARB(TEXTURE_BUFFER)"),
            BufferTargetARB::TRANSFORM_FEEDBACK_BUFFER => write!(fmt, "BufferTargetARB(TRANSFORM_FEEDBACK_BUFFER)"),
            BufferTargetARB::UNIFORM_BUFFER => write!(fmt, "BufferTargetARB(UNIFORM_BUFFER)"),
            _ => write!(fmt, "BufferTargetARB({})", self.0),
        }
    }
}

impl_enum_traits!(BufferTargetARB);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferUsageARB(pub types::GLenum);

impl BufferUsageARB {
    pub const STREAM_DRAW: BufferUsageARB = BufferUsageARB(super::STREAM_DRAW);
    pub const STREAM_READ: BufferUsageARB = BufferUsageARB(super::STREAM_READ);
    pub const STREAM_COPY: BufferUsageARB = BufferUsageARB(super::STREAM_COPY);
    pub const STATIC_DRAW: BufferUsageARB = BufferUsageARB(super::STATIC_DRAW);
    pub const STATIC_READ: BufferUsageARB = BufferUsageARB(super::STATIC_READ);
    pub const STATIC_COPY: BufferUsageARB = BufferUsageARB(super::STATIC_COPY);
    pub const DYNAMIC_DRAW: BufferUsageARB = BufferUsageARB(super::DYNAMIC_DRAW);
    pub const DYNAMIC_READ: BufferUsageARB = BufferUsageARB(super::DYNAMIC_READ);
    pub const DYNAMIC_COPY: BufferUsageARB = BufferUsageARB(super::DYNAMIC_COPY);
}

impl ::std::fmt::Debug for BufferUsageARB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BufferUsageARB::DYNAMIC_COPY => write!(fmt, "BufferUsageARB(DYNAMIC_COPY)"),
            BufferUsageARB::DYNAMIC_DRAW => write!(fmt, "BufferUsageARB(DYNAMIC_DRAW)"),
            BufferUsageARB::DYNAMIC_READ => write!(fmt, "BufferUsageARB(DYNAMIC_READ)"),
            BufferUsageARB::STATIC_COPY => write!(fmt, "BufferUsageARB(STATIC_COPY)"),
            BufferUsageARB::STATIC_DRAW => write!(fmt, "BufferUsageARB(STATIC_DRAW)"),
            BufferUsageARB::STATIC_READ => write!(fmt, "BufferUsageARB(STATIC_READ)"),
            BufferUsageARB::STREAM_COPY => write!(fmt, "BufferUsageARB(STREAM_COPY)"),
            BufferUsageARB::STREAM_DRAW => write!(fmt, "BufferUsageARB(STREAM_DRAW)"),
            BufferUsageARB::STREAM_READ => write!(fmt, "BufferUsageARB(STREAM_READ)"),
            _ => write!(fmt, "BufferUsageARB({})", self.0),
        }
    }
}

impl_enum_traits!(BufferUsageARB);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct CheckFramebufferStatusTarget(pub types::GLenum);

impl CheckFramebufferStatusTarget {
    pub const DRAW_FRAMEBUFFER: CheckFramebufferStatusTarget = CheckFramebufferStatusTarget(super::DRAW_FRAMEBUFFER);
    pub const READ_FRAMEBUFFER: CheckFramebufferStatusTarget = CheckFramebufferStatusTarget(super::READ_FRAMEBUFFER);
    pub const FRAMEBUFFER: CheckFramebufferStatusTarget = CheckFramebufferStatusTarget(super::FRAMEBUFFER);
}

impl ::std::fmt::Debug for CheckFramebufferStatusTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            CheckFramebufferStatusTarget::DRAW_FRAMEBUFFER => write!(fmt, "CheckFramebufferStatusTarget(DRAW_FRAMEBUFFER)"),
            CheckFramebufferStatusTarget::FRAMEBUFFER => write!(fmt, "CheckFramebufferStatusTarget(FRAMEBUFFER)"),
            CheckFramebufferStatusTarget::READ_FRAMEBUFFER => write!(fmt, "CheckFramebufferStatusTarget(READ_FRAMEBUFFER)"),
            _ => write!(fmt, "CheckFramebufferStatusTarget({})", self.0),
        }
    }
}

impl_enum_traits!(CheckFramebufferStatusTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClearBufferMask(pub types::GLenum);

impl ClearBufferMask {
    pub const COLOR_BUFFER_BIT: ClearBufferMask = ClearBufferMask(super::COLOR_BUFFER_BIT);
    pub const DEPTH_BUFFER_BIT: ClearBufferMask = ClearBufferMask(super::DEPTH_BUFFER_BIT);
    pub const STENCIL_BUFFER_BIT: ClearBufferMask = ClearBufferMask(super::STENCIL_BUFFER_BIT);
    pub const Empty: ClearBufferMask = ClearBufferMask(0);
}

impl ::std::fmt::Debug for ClearBufferMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ClearBufferMask::COLOR_BUFFER_BIT => write!(fmt, "ClearBufferMask(COLOR_BUFFER_BIT)"),
            ClearBufferMask::DEPTH_BUFFER_BIT => write!(fmt, "ClearBufferMask(DEPTH_BUFFER_BIT)"),
            ClearBufferMask::STENCIL_BUFFER_BIT => write!(fmt, "ClearBufferMask(STENCIL_BUFFER_BIT)"),
            _ => write!(fmt, "ClearBufferMask({})", self.0),
        }
    }
}

impl_enum_traits!(ClearBufferMask);

impl_enum_bitmask_traits!(ClearBufferMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClientAttribMask(pub types::GLenum);

impl ClientAttribMask {
    pub const Empty: ClientAttribMask = ClientAttribMask(0);
}

impl ::std::fmt::Debug for ClientAttribMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ClientAttribMask({})", self.0),
        }
    }
}

impl_enum_traits!(ClientAttribMask);

impl_enum_bitmask_traits!(ClientAttribMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClipControlDepth(pub types::GLenum);

impl ClipControlDepth {
    pub const NEGATIVE_ONE_TO_ONE: ClipControlDepth = ClipControlDepth(super::NEGATIVE_ONE_TO_ONE);
    pub const ZERO_TO_ONE: ClipControlDepth = ClipControlDepth(super::ZERO_TO_ONE);
}

impl ::std::fmt::Debug for ClipControlDepth {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ClipControlDepth::NEGATIVE_ONE_TO_ONE => write!(fmt, "ClipControlDepth(NEGATIVE_ONE_TO_ONE)"),
            ClipControlDepth::ZERO_TO_ONE => write!(fmt, "ClipControlDepth(ZERO_TO_ONE)"),
            _ => write!(fmt, "ClipControlDepth({})", self.0),
        }
    }
}

impl_enum_traits!(ClipControlDepth);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClipControlOrigin(pub types::GLenum);

impl ClipControlOrigin {
    pub const LOWER_LEFT: ClipControlOrigin = ClipControlOrigin(super::LOWER_LEFT);
    pub const UPPER_LEFT: ClipControlOrigin = ClipControlOrigin(super::UPPER_LEFT);
}

impl ::std::fmt::Debug for ClipControlOrigin {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ClipControlOrigin::LOWER_LEFT => write!(fmt, "ClipControlOrigin(LOWER_LEFT)"),
            ClipControlOrigin::UPPER_LEFT => write!(fmt, "ClipControlOrigin(UPPER_LEFT)"),
            _ => write!(fmt, "ClipControlOrigin({})", self.0),
        }
    }
}

impl_enum_traits!(ClipControlOrigin);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ClipPlaneName(pub types::GLenum);

impl ClipPlaneName {
    pub const CLIP_DISTANCE0: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE0);
    pub const CLIP_DISTANCE1: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE1);
    pub const CLIP_DISTANCE2: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE2);
    pub const CLIP_DISTANCE3: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE3);
    pub const CLIP_DISTANCE4: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE4);
    pub const CLIP_DISTANCE5: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE5);
    pub const CLIP_DISTANCE6: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE6);
    pub const CLIP_DISTANCE7: ClipPlaneName = ClipPlaneName(super::CLIP_DISTANCE7);
}

impl ::std::fmt::Debug for ClipPlaneName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ClipPlaneName::CLIP_DISTANCE0 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE0)"),
            ClipPlaneName::CLIP_DISTANCE1 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE1)"),
            ClipPlaneName::CLIP_DISTANCE2 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE2)"),
            ClipPlaneName::CLIP_DISTANCE3 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE3)"),
            ClipPlaneName::CLIP_DISTANCE4 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE4)"),
            ClipPlaneName::CLIP_DISTANCE5 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE5)"),
            ClipPlaneName::CLIP_DISTANCE6 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE6)"),
            ClipPlaneName::CLIP_DISTANCE7 => write!(fmt, "ClipPlaneName(CLIP_DISTANCE7)"),
            _ => write!(fmt, "ClipPlaneName({})", self.0),
        }
    }
}

impl_enum_traits!(ClipPlaneName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorBuffer(pub types::GLenum);

impl ColorBuffer {
    pub const NONE: ColorBuffer = ColorBuffer(super::NONE);
    pub const FRONT_LEFT: ColorBuffer = ColorBuffer(super::FRONT_LEFT);
    pub const FRONT_RIGHT: ColorBuffer = ColorBuffer(super::FRONT_RIGHT);
    pub const BACK_LEFT: ColorBuffer = ColorBuffer(super::BACK_LEFT);
    pub const BACK_RIGHT: ColorBuffer = ColorBuffer(super::BACK_RIGHT);
    pub const FRONT: ColorBuffer = ColorBuffer(super::FRONT);
    pub const BACK: ColorBuffer = ColorBuffer(super::BACK);
    pub const LEFT: ColorBuffer = ColorBuffer(super::LEFT);
    pub const RIGHT: ColorBuffer = ColorBuffer(super::RIGHT);
    pub const FRONT_AND_BACK: ColorBuffer = ColorBuffer(super::FRONT_AND_BACK);
    pub const COLOR_ATTACHMENT0: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT0);
    pub const COLOR_ATTACHMENT1: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT1);
    pub const COLOR_ATTACHMENT2: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT2);
    pub const COLOR_ATTACHMENT3: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT3);
    pub const COLOR_ATTACHMENT4: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT4);
    pub const COLOR_ATTACHMENT5: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT5);
    pub const COLOR_ATTACHMENT6: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT6);
    pub const COLOR_ATTACHMENT7: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT7);
    pub const COLOR_ATTACHMENT8: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT8);
    pub const COLOR_ATTACHMENT9: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT9);
    pub const COLOR_ATTACHMENT10: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT10);
    pub const COLOR_ATTACHMENT11: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT11);
    pub const COLOR_ATTACHMENT12: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT12);
    pub const COLOR_ATTACHMENT13: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT13);
    pub const COLOR_ATTACHMENT14: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT14);
    pub const COLOR_ATTACHMENT15: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT15);
    pub const COLOR_ATTACHMENT16: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT16);
    pub const COLOR_ATTACHMENT17: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT17);
    pub const COLOR_ATTACHMENT18: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT18);
    pub const COLOR_ATTACHMENT19: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT19);
    pub const COLOR_ATTACHMENT20: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT20);
    pub const COLOR_ATTACHMENT21: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT21);
    pub const COLOR_ATTACHMENT22: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT22);
    pub const COLOR_ATTACHMENT23: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT23);
    pub const COLOR_ATTACHMENT24: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT24);
    pub const COLOR_ATTACHMENT25: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT25);
    pub const COLOR_ATTACHMENT26: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT26);
    pub const COLOR_ATTACHMENT27: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT27);
    pub const COLOR_ATTACHMENT28: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT28);
    pub const COLOR_ATTACHMENT29: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT29);
    pub const COLOR_ATTACHMENT30: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT30);
    pub const COLOR_ATTACHMENT31: ColorBuffer = ColorBuffer(super::COLOR_ATTACHMENT31);
}

impl ::std::fmt::Debug for ColorBuffer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ColorBuffer::BACK => write!(fmt, "ColorBuffer(BACK)"),
            ColorBuffer::BACK_LEFT => write!(fmt, "ColorBuffer(BACK_LEFT)"),
            ColorBuffer::BACK_RIGHT => write!(fmt, "ColorBuffer(BACK_RIGHT)"),
            ColorBuffer::COLOR_ATTACHMENT0 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT0)"),
            ColorBuffer::COLOR_ATTACHMENT1 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT1)"),
            ColorBuffer::COLOR_ATTACHMENT10 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT10)"),
            ColorBuffer::COLOR_ATTACHMENT11 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT11)"),
            ColorBuffer::COLOR_ATTACHMENT12 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT12)"),
            ColorBuffer::COLOR_ATTACHMENT13 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT13)"),
            ColorBuffer::COLOR_ATTACHMENT14 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT14)"),
            ColorBuffer::COLOR_ATTACHMENT15 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT15)"),
            ColorBuffer::COLOR_ATTACHMENT16 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT16)"),
            ColorBuffer::COLOR_ATTACHMENT17 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT17)"),
            ColorBuffer::COLOR_ATTACHMENT18 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT18)"),
            ColorBuffer::COLOR_ATTACHMENT19 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT19)"),
            ColorBuffer::COLOR_ATTACHMENT2 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT2)"),
            ColorBuffer::COLOR_ATTACHMENT20 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT20)"),
            ColorBuffer::COLOR_ATTACHMENT21 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT21)"),
            ColorBuffer::COLOR_ATTACHMENT22 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT22)"),
            ColorBuffer::COLOR_ATTACHMENT23 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT23)"),
            ColorBuffer::COLOR_ATTACHMENT24 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT24)"),
            ColorBuffer::COLOR_ATTACHMENT25 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT25)"),
            ColorBuffer::COLOR_ATTACHMENT26 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT26)"),
            ColorBuffer::COLOR_ATTACHMENT27 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT27)"),
            ColorBuffer::COLOR_ATTACHMENT28 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT28)"),
            ColorBuffer::COLOR_ATTACHMENT29 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT29)"),
            ColorBuffer::COLOR_ATTACHMENT3 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT3)"),
            ColorBuffer::COLOR_ATTACHMENT30 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT30)"),
            ColorBuffer::COLOR_ATTACHMENT31 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT31)"),
            ColorBuffer::COLOR_ATTACHMENT4 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT4)"),
            ColorBuffer::COLOR_ATTACHMENT5 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT5)"),
            ColorBuffer::COLOR_ATTACHMENT6 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT6)"),
            ColorBuffer::COLOR_ATTACHMENT7 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT7)"),
            ColorBuffer::COLOR_ATTACHMENT8 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT8)"),
            ColorBuffer::COLOR_ATTACHMENT9 => write!(fmt, "ColorBuffer(COLOR_ATTACHMENT9)"),
            ColorBuffer::FRONT => write!(fmt, "ColorBuffer(FRONT)"),
            ColorBuffer::FRONT_AND_BACK => write!(fmt, "ColorBuffer(FRONT_AND_BACK)"),
            ColorBuffer::FRONT_LEFT => write!(fmt, "ColorBuffer(FRONT_LEFT)"),
            ColorBuffer::FRONT_RIGHT => write!(fmt, "ColorBuffer(FRONT_RIGHT)"),
            ColorBuffer::LEFT => write!(fmt, "ColorBuffer(LEFT)"),
            ColorBuffer::NONE => write!(fmt, "ColorBuffer(NONE)"),
            ColorBuffer::RIGHT => write!(fmt, "ColorBuffer(RIGHT)"),
            _ => write!(fmt, "ColorBuffer({})", self.0),
        }
    }
}

impl_enum_traits!(ColorBuffer);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorMaterialFace(pub types::GLenum);

impl ColorMaterialFace {
    pub const BACK: ColorMaterialFace = ColorMaterialFace(super::BACK);
    pub const FRONT: ColorMaterialFace = ColorMaterialFace(super::FRONT);
    pub const FRONT_AND_BACK: ColorMaterialFace = ColorMaterialFace(super::FRONT_AND_BACK);
}

impl ::std::fmt::Debug for ColorMaterialFace {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ColorMaterialFace::BACK => write!(fmt, "ColorMaterialFace(BACK)"),
            ColorMaterialFace::FRONT => write!(fmt, "ColorMaterialFace(FRONT)"),
            ColorMaterialFace::FRONT_AND_BACK => write!(fmt, "ColorMaterialFace(FRONT_AND_BACK)"),
            _ => write!(fmt, "ColorMaterialFace({})", self.0),
        }
    }
}

impl_enum_traits!(ColorMaterialFace);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorMaterialParameter(pub types::GLenum);

impl ColorMaterialParameter {
}

impl ::std::fmt::Debug for ColorMaterialParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ColorMaterialParameter({})", self.0),
        }
    }
}

impl_enum_traits!(ColorMaterialParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorPointerType(pub types::GLenum);

impl ColorPointerType {
    pub const BYTE: ColorPointerType = ColorPointerType(super::BYTE);
    pub const DOUBLE: ColorPointerType = ColorPointerType(super::DOUBLE);
    pub const FLOAT: ColorPointerType = ColorPointerType(super::FLOAT);
    pub const INT: ColorPointerType = ColorPointerType(super::INT);
    pub const SHORT: ColorPointerType = ColorPointerType(super::SHORT);
    pub const UNSIGNED_BYTE: ColorPointerType = ColorPointerType(super::UNSIGNED_BYTE);
    pub const UNSIGNED_INT: ColorPointerType = ColorPointerType(super::UNSIGNED_INT);
    pub const UNSIGNED_SHORT: ColorPointerType = ColorPointerType(super::UNSIGNED_SHORT);
}

impl ::std::fmt::Debug for ColorPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ColorPointerType::BYTE => write!(fmt, "ColorPointerType(BYTE)"),
            ColorPointerType::DOUBLE => write!(fmt, "ColorPointerType(DOUBLE)"),
            ColorPointerType::FLOAT => write!(fmt, "ColorPointerType(FLOAT)"),
            ColorPointerType::INT => write!(fmt, "ColorPointerType(INT)"),
            ColorPointerType::SHORT => write!(fmt, "ColorPointerType(SHORT)"),
            ColorPointerType::UNSIGNED_BYTE => write!(fmt, "ColorPointerType(UNSIGNED_BYTE)"),
            ColorPointerType::UNSIGNED_INT => write!(fmt, "ColorPointerType(UNSIGNED_INT)"),
            ColorPointerType::UNSIGNED_SHORT => write!(fmt, "ColorPointerType(UNSIGNED_SHORT)"),
            _ => write!(fmt, "ColorPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(ColorPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorTableParameterPNameSGI(pub types::GLenum);

impl ColorTableParameterPNameSGI {
}

impl ::std::fmt::Debug for ColorTableParameterPNameSGI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ColorTableParameterPNameSGI({})", self.0),
        }
    }
}

impl_enum_traits!(ColorTableParameterPNameSGI);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorTableTarget(pub types::GLenum);

impl ColorTableTarget {
}

impl ::std::fmt::Debug for ColorTableTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ColorTableTarget({})", self.0),
        }
    }
}

impl_enum_traits!(ColorTableTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ColorTableTargetSGI(pub types::GLenum);

impl ColorTableTargetSGI {
}

impl ::std::fmt::Debug for ColorTableTargetSGI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ColorTableTargetSGI({})", self.0),
        }
    }
}

impl_enum_traits!(ColorTableTargetSGI);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ContextFlagMask(pub types::GLenum);

impl ContextFlagMask {
    pub const CONTEXT_FLAG_DEBUG_BIT: ContextFlagMask = ContextFlagMask(super::CONTEXT_FLAG_DEBUG_BIT);
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: ContextFlagMask = ContextFlagMask(super::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT);
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: ContextFlagMask = ContextFlagMask(super::CONTEXT_FLAG_ROBUST_ACCESS_BIT);
    pub const Empty: ContextFlagMask = ContextFlagMask(0);
}

impl ::std::fmt::Debug for ContextFlagMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ContextFlagMask::CONTEXT_FLAG_DEBUG_BIT => write!(fmt, "ContextFlagMask(CONTEXT_FLAG_DEBUG_BIT)"),
            ContextFlagMask::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT => write!(fmt, "ContextFlagMask(CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT)"),
            ContextFlagMask::CONTEXT_FLAG_ROBUST_ACCESS_BIT => write!(fmt, "ContextFlagMask(CONTEXT_FLAG_ROBUST_ACCESS_BIT)"),
            _ => write!(fmt, "ContextFlagMask({})", self.0),
        }
    }
}

impl_enum_traits!(ContextFlagMask);

impl_enum_bitmask_traits!(ContextFlagMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ContextProfileMask(pub types::GLenum);

impl ContextProfileMask {
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: ContextProfileMask = ContextProfileMask(super::CONTEXT_COMPATIBILITY_PROFILE_BIT);
    pub const CONTEXT_CORE_PROFILE_BIT: ContextProfileMask = ContextProfileMask(super::CONTEXT_CORE_PROFILE_BIT);
    pub const Empty: ContextProfileMask = ContextProfileMask(0);
}

impl ::std::fmt::Debug for ContextProfileMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ContextProfileMask::CONTEXT_COMPATIBILITY_PROFILE_BIT => write!(fmt, "ContextProfileMask(CONTEXT_COMPATIBILITY_PROFILE_BIT)"),
            ContextProfileMask::CONTEXT_CORE_PROFILE_BIT => write!(fmt, "ContextProfileMask(CONTEXT_CORE_PROFILE_BIT)"),
            _ => write!(fmt, "ContextProfileMask({})", self.0),
        }
    }
}

impl_enum_traits!(ContextProfileMask);

impl_enum_bitmask_traits!(ContextProfileMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ConvolutionBorderModeEXT(pub types::GLenum);

impl ConvolutionBorderModeEXT {
}

impl ::std::fmt::Debug for ConvolutionBorderModeEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ConvolutionBorderModeEXT({})", self.0),
        }
    }
}

impl_enum_traits!(ConvolutionBorderModeEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ConvolutionParameterEXT(pub types::GLenum);

impl ConvolutionParameterEXT {
}

impl ::std::fmt::Debug for ConvolutionParameterEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ConvolutionParameterEXT({})", self.0),
        }
    }
}

impl_enum_traits!(ConvolutionParameterEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ConvolutionTarget(pub types::GLenum);

impl ConvolutionTarget {
}

impl ::std::fmt::Debug for ConvolutionTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ConvolutionTarget({})", self.0),
        }
    }
}

impl_enum_traits!(ConvolutionTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ConvolutionTargetEXT(pub types::GLenum);

impl ConvolutionTargetEXT {
}

impl ::std::fmt::Debug for ConvolutionTargetEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ConvolutionTargetEXT({})", self.0),
        }
    }
}

impl_enum_traits!(ConvolutionTargetEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct CopyBufferSubDataTarget(pub types::GLenum);

impl CopyBufferSubDataTarget {
    pub const ARRAY_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::ARRAY_BUFFER);
    pub const ATOMIC_COUNTER_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::ATOMIC_COUNTER_BUFFER);
    pub const COPY_READ_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::COPY_READ_BUFFER);
    pub const COPY_WRITE_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::COPY_WRITE_BUFFER);
    pub const DISPATCH_INDIRECT_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::DISPATCH_INDIRECT_BUFFER);
    pub const DRAW_INDIRECT_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::DRAW_INDIRECT_BUFFER);
    pub const ELEMENT_ARRAY_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::ELEMENT_ARRAY_BUFFER);
    pub const PIXEL_PACK_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::PIXEL_PACK_BUFFER);
    pub const PIXEL_UNPACK_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::PIXEL_UNPACK_BUFFER);
    pub const QUERY_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::QUERY_BUFFER);
    pub const SHADER_STORAGE_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::SHADER_STORAGE_BUFFER);
    pub const TEXTURE_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::TEXTURE_BUFFER);
    pub const TRANSFORM_FEEDBACK_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::TRANSFORM_FEEDBACK_BUFFER);
    pub const UNIFORM_BUFFER: CopyBufferSubDataTarget = CopyBufferSubDataTarget(super::UNIFORM_BUFFER);
}

impl ::std::fmt::Debug for CopyBufferSubDataTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            CopyBufferSubDataTarget::ARRAY_BUFFER => write!(fmt, "CopyBufferSubDataTarget(ARRAY_BUFFER)"),
            CopyBufferSubDataTarget::ATOMIC_COUNTER_BUFFER => write!(fmt, "CopyBufferSubDataTarget(ATOMIC_COUNTER_BUFFER)"),
            CopyBufferSubDataTarget::COPY_READ_BUFFER => write!(fmt, "CopyBufferSubDataTarget(COPY_READ_BUFFER)"),
            CopyBufferSubDataTarget::COPY_WRITE_BUFFER => write!(fmt, "CopyBufferSubDataTarget(COPY_WRITE_BUFFER)"),
            CopyBufferSubDataTarget::DISPATCH_INDIRECT_BUFFER => write!(fmt, "CopyBufferSubDataTarget(DISPATCH_INDIRECT_BUFFER)"),
            CopyBufferSubDataTarget::DRAW_INDIRECT_BUFFER => write!(fmt, "CopyBufferSubDataTarget(DRAW_INDIRECT_BUFFER)"),
            CopyBufferSubDataTarget::ELEMENT_ARRAY_BUFFER => write!(fmt, "CopyBufferSubDataTarget(ELEMENT_ARRAY_BUFFER)"),
            CopyBufferSubDataTarget::PIXEL_PACK_BUFFER => write!(fmt, "CopyBufferSubDataTarget(PIXEL_PACK_BUFFER)"),
            CopyBufferSubDataTarget::PIXEL_UNPACK_BUFFER => write!(fmt, "CopyBufferSubDataTarget(PIXEL_UNPACK_BUFFER)"),
            CopyBufferSubDataTarget::QUERY_BUFFER => write!(fmt, "CopyBufferSubDataTarget(QUERY_BUFFER)"),
            CopyBufferSubDataTarget::SHADER_STORAGE_BUFFER => write!(fmt, "CopyBufferSubDataTarget(SHADER_STORAGE_BUFFER)"),
            CopyBufferSubDataTarget::TEXTURE_BUFFER => write!(fmt, "CopyBufferSubDataTarget(TEXTURE_BUFFER)"),
            CopyBufferSubDataTarget::TRANSFORM_FEEDBACK_BUFFER => write!(fmt, "CopyBufferSubDataTarget(TRANSFORM_FEEDBACK_BUFFER)"),
            CopyBufferSubDataTarget::UNIFORM_BUFFER => write!(fmt, "CopyBufferSubDataTarget(UNIFORM_BUFFER)"),
            _ => write!(fmt, "CopyBufferSubDataTarget({})", self.0),
        }
    }
}

impl_enum_traits!(CopyBufferSubDataTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct CullFaceMode(pub types::GLenum);

impl CullFaceMode {
    pub const BACK: CullFaceMode = CullFaceMode(super::BACK);
    pub const FRONT: CullFaceMode = CullFaceMode(super::FRONT);
    pub const FRONT_AND_BACK: CullFaceMode = CullFaceMode(super::FRONT_AND_BACK);
}

impl ::std::fmt::Debug for CullFaceMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            CullFaceMode::BACK => write!(fmt, "CullFaceMode(BACK)"),
            CullFaceMode::FRONT => write!(fmt, "CullFaceMode(FRONT)"),
            CullFaceMode::FRONT_AND_BACK => write!(fmt, "CullFaceMode(FRONT_AND_BACK)"),
            _ => write!(fmt, "CullFaceMode({})", self.0),
        }
    }
}

impl_enum_traits!(CullFaceMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DataType(pub types::GLenum);

impl DataType {
}

impl ::std::fmt::Debug for DataType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "DataType({})", self.0),
        }
    }
}

impl_enum_traits!(DataType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DebugSeverity(pub types::GLenum);

impl DebugSeverity {
    pub const DEBUG_SEVERITY_LOW: DebugSeverity = DebugSeverity(super::DEBUG_SEVERITY_LOW);
    pub const DEBUG_SEVERITY_MEDIUM: DebugSeverity = DebugSeverity(super::DEBUG_SEVERITY_MEDIUM);
    pub const DEBUG_SEVERITY_HIGH: DebugSeverity = DebugSeverity(super::DEBUG_SEVERITY_HIGH);
    pub const DEBUG_SEVERITY_NOTIFICATION: DebugSeverity = DebugSeverity(super::DEBUG_SEVERITY_NOTIFICATION);
    pub const DONT_CARE: DebugSeverity = DebugSeverity(super::DONT_CARE);
}

impl ::std::fmt::Debug for DebugSeverity {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DebugSeverity::DEBUG_SEVERITY_HIGH => write!(fmt, "DebugSeverity(DEBUG_SEVERITY_HIGH)"),
            DebugSeverity::DEBUG_SEVERITY_LOW => write!(fmt, "DebugSeverity(DEBUG_SEVERITY_LOW)"),
            DebugSeverity::DEBUG_SEVERITY_MEDIUM => write!(fmt, "DebugSeverity(DEBUG_SEVERITY_MEDIUM)"),
            DebugSeverity::DEBUG_SEVERITY_NOTIFICATION => write!(fmt, "DebugSeverity(DEBUG_SEVERITY_NOTIFICATION)"),
            DebugSeverity::DONT_CARE => write!(fmt, "DebugSeverity(DONT_CARE)"),
            _ => write!(fmt, "DebugSeverity({})", self.0),
        }
    }
}

impl_enum_traits!(DebugSeverity);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DebugSource(pub types::GLenum);

impl DebugSource {
    pub const DEBUG_SOURCE_API: DebugSource = DebugSource(super::DEBUG_SOURCE_API);
    pub const DEBUG_SOURCE_WINDOW_SYSTEM: DebugSource = DebugSource(super::DEBUG_SOURCE_WINDOW_SYSTEM);
    pub const DEBUG_SOURCE_SHADER_COMPILER: DebugSource = DebugSource(super::DEBUG_SOURCE_SHADER_COMPILER);
    pub const DEBUG_SOURCE_THIRD_PARTY: DebugSource = DebugSource(super::DEBUG_SOURCE_THIRD_PARTY);
    pub const DEBUG_SOURCE_APPLICATION: DebugSource = DebugSource(super::DEBUG_SOURCE_APPLICATION);
    pub const DEBUG_SOURCE_OTHER: DebugSource = DebugSource(super::DEBUG_SOURCE_OTHER);
    pub const DONT_CARE: DebugSource = DebugSource(super::DONT_CARE);
}

impl ::std::fmt::Debug for DebugSource {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DebugSource::DEBUG_SOURCE_API => write!(fmt, "DebugSource(DEBUG_SOURCE_API)"),
            DebugSource::DEBUG_SOURCE_APPLICATION => write!(fmt, "DebugSource(DEBUG_SOURCE_APPLICATION)"),
            DebugSource::DEBUG_SOURCE_OTHER => write!(fmt, "DebugSource(DEBUG_SOURCE_OTHER)"),
            DebugSource::DEBUG_SOURCE_SHADER_COMPILER => write!(fmt, "DebugSource(DEBUG_SOURCE_SHADER_COMPILER)"),
            DebugSource::DEBUG_SOURCE_THIRD_PARTY => write!(fmt, "DebugSource(DEBUG_SOURCE_THIRD_PARTY)"),
            DebugSource::DEBUG_SOURCE_WINDOW_SYSTEM => write!(fmt, "DebugSource(DEBUG_SOURCE_WINDOW_SYSTEM)"),
            DebugSource::DONT_CARE => write!(fmt, "DebugSource(DONT_CARE)"),
            _ => write!(fmt, "DebugSource({})", self.0),
        }
    }
}

impl_enum_traits!(DebugSource);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DebugType(pub types::GLenum);

impl DebugType {
    pub const DEBUG_TYPE_ERROR: DebugType = DebugType(super::DEBUG_TYPE_ERROR);
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: DebugType = DebugType(super::DEBUG_TYPE_DEPRECATED_BEHAVIOR);
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: DebugType = DebugType(super::DEBUG_TYPE_UNDEFINED_BEHAVIOR);
    pub const DEBUG_TYPE_PORTABILITY: DebugType = DebugType(super::DEBUG_TYPE_PORTABILITY);
    pub const DEBUG_TYPE_PERFORMANCE: DebugType = DebugType(super::DEBUG_TYPE_PERFORMANCE);
    pub const DEBUG_TYPE_MARKER: DebugType = DebugType(super::DEBUG_TYPE_MARKER);
    pub const DEBUG_TYPE_PUSH_GROUP: DebugType = DebugType(super::DEBUG_TYPE_PUSH_GROUP);
    pub const DEBUG_TYPE_POP_GROUP: DebugType = DebugType(super::DEBUG_TYPE_POP_GROUP);
    pub const DEBUG_TYPE_OTHER: DebugType = DebugType(super::DEBUG_TYPE_OTHER);
    pub const DONT_CARE: DebugType = DebugType(super::DONT_CARE);
}

impl ::std::fmt::Debug for DebugType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DebugType::DEBUG_TYPE_DEPRECATED_BEHAVIOR => write!(fmt, "DebugType(DEBUG_TYPE_DEPRECATED_BEHAVIOR)"),
            DebugType::DEBUG_TYPE_ERROR => write!(fmt, "DebugType(DEBUG_TYPE_ERROR)"),
            DebugType::DEBUG_TYPE_MARKER => write!(fmt, "DebugType(DEBUG_TYPE_MARKER)"),
            DebugType::DEBUG_TYPE_OTHER => write!(fmt, "DebugType(DEBUG_TYPE_OTHER)"),
            DebugType::DEBUG_TYPE_PERFORMANCE => write!(fmt, "DebugType(DEBUG_TYPE_PERFORMANCE)"),
            DebugType::DEBUG_TYPE_POP_GROUP => write!(fmt, "DebugType(DEBUG_TYPE_POP_GROUP)"),
            DebugType::DEBUG_TYPE_PORTABILITY => write!(fmt, "DebugType(DEBUG_TYPE_PORTABILITY)"),
            DebugType::DEBUG_TYPE_PUSH_GROUP => write!(fmt, "DebugType(DEBUG_TYPE_PUSH_GROUP)"),
            DebugType::DEBUG_TYPE_UNDEFINED_BEHAVIOR => write!(fmt, "DebugType(DEBUG_TYPE_UNDEFINED_BEHAVIOR)"),
            DebugType::DONT_CARE => write!(fmt, "DebugType(DONT_CARE)"),
            _ => write!(fmt, "DebugType({})", self.0),
        }
    }
}

impl_enum_traits!(DebugType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DepthFunction(pub types::GLenum);

impl DepthFunction {
    pub const ALWAYS: DepthFunction = DepthFunction(super::ALWAYS);
    pub const EQUAL: DepthFunction = DepthFunction(super::EQUAL);
    pub const GEQUAL: DepthFunction = DepthFunction(super::GEQUAL);
    pub const GREATER: DepthFunction = DepthFunction(super::GREATER);
    pub const LEQUAL: DepthFunction = DepthFunction(super::LEQUAL);
    pub const LESS: DepthFunction = DepthFunction(super::LESS);
    pub const NEVER: DepthFunction = DepthFunction(super::NEVER);
    pub const NOTEQUAL: DepthFunction = DepthFunction(super::NOTEQUAL);
}

impl ::std::fmt::Debug for DepthFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DepthFunction::ALWAYS => write!(fmt, "DepthFunction(ALWAYS)"),
            DepthFunction::EQUAL => write!(fmt, "DepthFunction(EQUAL)"),
            DepthFunction::GEQUAL => write!(fmt, "DepthFunction(GEQUAL)"),
            DepthFunction::GREATER => write!(fmt, "DepthFunction(GREATER)"),
            DepthFunction::LEQUAL => write!(fmt, "DepthFunction(LEQUAL)"),
            DepthFunction::LESS => write!(fmt, "DepthFunction(LESS)"),
            DepthFunction::NEVER => write!(fmt, "DepthFunction(NEVER)"),
            DepthFunction::NOTEQUAL => write!(fmt, "DepthFunction(NOTEQUAL)"),
            _ => write!(fmt, "DepthFunction({})", self.0),
        }
    }
}

impl_enum_traits!(DepthFunction);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DrawBufferMode(pub types::GLenum);

impl DrawBufferMode {
    pub const BACK: DrawBufferMode = DrawBufferMode(super::BACK);
    pub const BACK_LEFT: DrawBufferMode = DrawBufferMode(super::BACK_LEFT);
    pub const BACK_RIGHT: DrawBufferMode = DrawBufferMode(super::BACK_RIGHT);
    pub const FRONT: DrawBufferMode = DrawBufferMode(super::FRONT);
    pub const FRONT_AND_BACK: DrawBufferMode = DrawBufferMode(super::FRONT_AND_BACK);
    pub const FRONT_LEFT: DrawBufferMode = DrawBufferMode(super::FRONT_LEFT);
    pub const FRONT_RIGHT: DrawBufferMode = DrawBufferMode(super::FRONT_RIGHT);
    pub const LEFT: DrawBufferMode = DrawBufferMode(super::LEFT);
    pub const NONE: DrawBufferMode = DrawBufferMode(super::NONE);
    pub const RIGHT: DrawBufferMode = DrawBufferMode(super::RIGHT);
}

impl ::std::fmt::Debug for DrawBufferMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DrawBufferMode::BACK => write!(fmt, "DrawBufferMode(BACK)"),
            DrawBufferMode::BACK_LEFT => write!(fmt, "DrawBufferMode(BACK_LEFT)"),
            DrawBufferMode::BACK_RIGHT => write!(fmt, "DrawBufferMode(BACK_RIGHT)"),
            DrawBufferMode::FRONT => write!(fmt, "DrawBufferMode(FRONT)"),
            DrawBufferMode::FRONT_AND_BACK => write!(fmt, "DrawBufferMode(FRONT_AND_BACK)"),
            DrawBufferMode::FRONT_LEFT => write!(fmt, "DrawBufferMode(FRONT_LEFT)"),
            DrawBufferMode::FRONT_RIGHT => write!(fmt, "DrawBufferMode(FRONT_RIGHT)"),
            DrawBufferMode::LEFT => write!(fmt, "DrawBufferMode(LEFT)"),
            DrawBufferMode::NONE => write!(fmt, "DrawBufferMode(NONE)"),
            DrawBufferMode::RIGHT => write!(fmt, "DrawBufferMode(RIGHT)"),
            _ => write!(fmt, "DrawBufferMode({})", self.0),
        }
    }
}

impl_enum_traits!(DrawBufferMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct DrawElementsType(pub types::GLenum);

impl DrawElementsType {
    pub const UNSIGNED_BYTE: DrawElementsType = DrawElementsType(super::UNSIGNED_BYTE);
    pub const UNSIGNED_SHORT: DrawElementsType = DrawElementsType(super::UNSIGNED_SHORT);
    pub const UNSIGNED_INT: DrawElementsType = DrawElementsType(super::UNSIGNED_INT);
}

impl ::std::fmt::Debug for DrawElementsType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DrawElementsType::UNSIGNED_BYTE => write!(fmt, "DrawElementsType(UNSIGNED_BYTE)"),
            DrawElementsType::UNSIGNED_INT => write!(fmt, "DrawElementsType(UNSIGNED_INT)"),
            DrawElementsType::UNSIGNED_SHORT => write!(fmt, "DrawElementsType(UNSIGNED_SHORT)"),
            _ => write!(fmt, "DrawElementsType({})", self.0),
        }
    }
}

impl_enum_traits!(DrawElementsType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct EnableCap(pub types::GLenum);

impl EnableCap {
    pub const BLEND: EnableCap = EnableCap(super::BLEND);
    pub const COLOR_LOGIC_OP: EnableCap = EnableCap(super::COLOR_LOGIC_OP);
    pub const CULL_FACE: EnableCap = EnableCap(super::CULL_FACE);
    pub const DEPTH_TEST: EnableCap = EnableCap(super::DEPTH_TEST);
    pub const DITHER: EnableCap = EnableCap(super::DITHER);
    pub const LINE_SMOOTH: EnableCap = EnableCap(super::LINE_SMOOTH);
    pub const POLYGON_OFFSET_FILL: EnableCap = EnableCap(super::POLYGON_OFFSET_FILL);
    pub const POLYGON_OFFSET_LINE: EnableCap = EnableCap(super::POLYGON_OFFSET_LINE);
    pub const POLYGON_OFFSET_POINT: EnableCap = EnableCap(super::POLYGON_OFFSET_POINT);
    pub const POLYGON_SMOOTH: EnableCap = EnableCap(super::POLYGON_SMOOTH);
    pub const SCISSOR_TEST: EnableCap = EnableCap(super::SCISSOR_TEST);
    pub const STENCIL_TEST: EnableCap = EnableCap(super::STENCIL_TEST);
    pub const TEXTURE_1D: EnableCap = EnableCap(super::TEXTURE_1D);
    pub const TEXTURE_2D: EnableCap = EnableCap(super::TEXTURE_2D);
    pub const VERTEX_ARRAY: EnableCap = EnableCap(super::VERTEX_ARRAY);
}

impl ::std::fmt::Debug for EnableCap {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EnableCap::BLEND => write!(fmt, "EnableCap(BLEND)"),
            EnableCap::COLOR_LOGIC_OP => write!(fmt, "EnableCap(COLOR_LOGIC_OP)"),
            EnableCap::CULL_FACE => write!(fmt, "EnableCap(CULL_FACE)"),
            EnableCap::DEPTH_TEST => write!(fmt, "EnableCap(DEPTH_TEST)"),
            EnableCap::DITHER => write!(fmt, "EnableCap(DITHER)"),
            EnableCap::LINE_SMOOTH => write!(fmt, "EnableCap(LINE_SMOOTH)"),
            EnableCap::POLYGON_OFFSET_FILL => write!(fmt, "EnableCap(POLYGON_OFFSET_FILL)"),
            EnableCap::POLYGON_OFFSET_LINE => write!(fmt, "EnableCap(POLYGON_OFFSET_LINE)"),
            EnableCap::POLYGON_OFFSET_POINT => write!(fmt, "EnableCap(POLYGON_OFFSET_POINT)"),
            EnableCap::POLYGON_SMOOTH => write!(fmt, "EnableCap(POLYGON_SMOOTH)"),
            EnableCap::SCISSOR_TEST => write!(fmt, "EnableCap(SCISSOR_TEST)"),
            EnableCap::STENCIL_TEST => write!(fmt, "EnableCap(STENCIL_TEST)"),
            EnableCap::TEXTURE_1D => write!(fmt, "EnableCap(TEXTURE_1D)"),
            EnableCap::TEXTURE_2D => write!(fmt, "EnableCap(TEXTURE_2D)"),
            EnableCap::VERTEX_ARRAY => write!(fmt, "EnableCap(VERTEX_ARRAY)"),
            _ => write!(fmt, "EnableCap({})", self.0),
        }
    }
}

impl_enum_traits!(EnableCap);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ErrorCode(pub types::GLenum);

impl ErrorCode {
    pub const INVALID_ENUM: ErrorCode = ErrorCode(super::INVALID_ENUM);
    pub const INVALID_FRAMEBUFFER_OPERATION: ErrorCode = ErrorCode(super::INVALID_FRAMEBUFFER_OPERATION);
    pub const INVALID_OPERATION: ErrorCode = ErrorCode(super::INVALID_OPERATION);
    pub const INVALID_VALUE: ErrorCode = ErrorCode(super::INVALID_VALUE);
    pub const NO_ERROR: ErrorCode = ErrorCode(super::NO_ERROR);
    pub const OUT_OF_MEMORY: ErrorCode = ErrorCode(super::OUT_OF_MEMORY);
    pub const STACK_OVERFLOW: ErrorCode = ErrorCode(super::STACK_OVERFLOW);
    pub const STACK_UNDERFLOW: ErrorCode = ErrorCode(super::STACK_UNDERFLOW);
}

impl ::std::fmt::Debug for ErrorCode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ErrorCode::INVALID_ENUM => write!(fmt, "ErrorCode(INVALID_ENUM)"),
            ErrorCode::INVALID_FRAMEBUFFER_OPERATION => write!(fmt, "ErrorCode(INVALID_FRAMEBUFFER_OPERATION)"),
            ErrorCode::INVALID_OPERATION => write!(fmt, "ErrorCode(INVALID_OPERATION)"),
            ErrorCode::INVALID_VALUE => write!(fmt, "ErrorCode(INVALID_VALUE)"),
            ErrorCode::NO_ERROR => write!(fmt, "ErrorCode(NO_ERROR)"),
            ErrorCode::OUT_OF_MEMORY => write!(fmt, "ErrorCode(OUT_OF_MEMORY)"),
            ErrorCode::STACK_OVERFLOW => write!(fmt, "ErrorCode(STACK_OVERFLOW)"),
            ErrorCode::STACK_UNDERFLOW => write!(fmt, "ErrorCode(STACK_UNDERFLOW)"),
            _ => write!(fmt, "ErrorCode({})", self.0),
        }
    }
}

impl_enum_traits!(ErrorCode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ExternalHandleType(pub types::GLenum);

impl ExternalHandleType {
}

impl ::std::fmt::Debug for ExternalHandleType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ExternalHandleType({})", self.0),
        }
    }
}

impl_enum_traits!(ExternalHandleType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FeedBackToken(pub types::GLenum);

impl FeedBackToken {
}

impl ::std::fmt::Debug for FeedBackToken {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FeedBackToken({})", self.0),
        }
    }
}

impl_enum_traits!(FeedBackToken);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FeedbackType(pub types::GLenum);

impl FeedbackType {
}

impl ::std::fmt::Debug for FeedbackType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FeedbackType({})", self.0),
        }
    }
}

impl_enum_traits!(FeedbackType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FfdMaskSGIX(pub types::GLenum);

impl FfdMaskSGIX {
    pub const Empty: FfdMaskSGIX = FfdMaskSGIX(0);
}

impl ::std::fmt::Debug for FfdMaskSGIX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FfdMaskSGIX({})", self.0),
        }
    }
}

impl_enum_traits!(FfdMaskSGIX);

impl_enum_bitmask_traits!(FfdMaskSGIX);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FfdTargetSGIX(pub types::GLenum);

impl FfdTargetSGIX {
}

impl ::std::fmt::Debug for FfdTargetSGIX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FfdTargetSGIX({})", self.0),
        }
    }
}

impl_enum_traits!(FfdTargetSGIX);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogCoordinatePointerType(pub types::GLenum);

impl FogCoordinatePointerType {
    pub const FLOAT: FogCoordinatePointerType = FogCoordinatePointerType(super::FLOAT);
    pub const DOUBLE: FogCoordinatePointerType = FogCoordinatePointerType(super::DOUBLE);
}

impl ::std::fmt::Debug for FogCoordinatePointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FogCoordinatePointerType::DOUBLE => write!(fmt, "FogCoordinatePointerType(DOUBLE)"),
            FogCoordinatePointerType::FLOAT => write!(fmt, "FogCoordinatePointerType(FLOAT)"),
            _ => write!(fmt, "FogCoordinatePointerType({})", self.0),
        }
    }
}

impl_enum_traits!(FogCoordinatePointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogMode(pub types::GLenum);

impl FogMode {
    pub const LINEAR: FogMode = FogMode(super::LINEAR);
}

impl ::std::fmt::Debug for FogMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FogMode::LINEAR => write!(fmt, "FogMode(LINEAR)"),
            _ => write!(fmt, "FogMode({})", self.0),
        }
    }
}

impl_enum_traits!(FogMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogPName(pub types::GLenum);

impl FogPName {
}

impl ::std::fmt::Debug for FogPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FogPName({})", self.0),
        }
    }
}

impl_enum_traits!(FogPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogParameter(pub types::GLenum);

impl FogParameter {
}

impl ::std::fmt::Debug for FogParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FogParameter({})", self.0),
        }
    }
}

impl_enum_traits!(FogParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogPointerTypeEXT(pub types::GLenum);

impl FogPointerTypeEXT {
    pub const FLOAT: FogPointerTypeEXT = FogPointerTypeEXT(super::FLOAT);
    pub const DOUBLE: FogPointerTypeEXT = FogPointerTypeEXT(super::DOUBLE);
}

impl ::std::fmt::Debug for FogPointerTypeEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FogPointerTypeEXT::DOUBLE => write!(fmt, "FogPointerTypeEXT(DOUBLE)"),
            FogPointerTypeEXT::FLOAT => write!(fmt, "FogPointerTypeEXT(FLOAT)"),
            _ => write!(fmt, "FogPointerTypeEXT({})", self.0),
        }
    }
}

impl_enum_traits!(FogPointerTypeEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FogPointerTypeIBM(pub types::GLenum);

impl FogPointerTypeIBM {
    pub const FLOAT: FogPointerTypeIBM = FogPointerTypeIBM(super::FLOAT);
    pub const DOUBLE: FogPointerTypeIBM = FogPointerTypeIBM(super::DOUBLE);
}

impl ::std::fmt::Debug for FogPointerTypeIBM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FogPointerTypeIBM::DOUBLE => write!(fmt, "FogPointerTypeIBM(DOUBLE)"),
            FogPointerTypeIBM::FLOAT => write!(fmt, "FogPointerTypeIBM(FLOAT)"),
            _ => write!(fmt, "FogPointerTypeIBM({})", self.0),
        }
    }
}

impl_enum_traits!(FogPointerTypeIBM);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FragmentLightModelParameterSGIX(pub types::GLenum);

impl FragmentLightModelParameterSGIX {
}

impl ::std::fmt::Debug for FragmentLightModelParameterSGIX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FragmentLightModelParameterSGIX({})", self.0),
        }
    }
}

impl_enum_traits!(FragmentLightModelParameterSGIX);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FragmentOpATI(pub types::GLenum);

impl FragmentOpATI {
}

impl ::std::fmt::Debug for FragmentOpATI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FragmentOpATI({})", self.0),
        }
    }
}

impl_enum_traits!(FragmentOpATI);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferAttachment(pub types::GLenum);

impl FramebufferAttachment {
    pub const MAX_COLOR_ATTACHMENTS: FramebufferAttachment = FramebufferAttachment(super::MAX_COLOR_ATTACHMENTS);
    pub const COLOR_ATTACHMENT0: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT0);
    pub const COLOR_ATTACHMENT1: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT1);
    pub const COLOR_ATTACHMENT2: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT2);
    pub const COLOR_ATTACHMENT3: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT3);
    pub const COLOR_ATTACHMENT4: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT4);
    pub const COLOR_ATTACHMENT5: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT5);
    pub const COLOR_ATTACHMENT6: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT6);
    pub const COLOR_ATTACHMENT7: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT7);
    pub const COLOR_ATTACHMENT8: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT8);
    pub const COLOR_ATTACHMENT9: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT9);
    pub const COLOR_ATTACHMENT10: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT10);
    pub const COLOR_ATTACHMENT11: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT11);
    pub const COLOR_ATTACHMENT12: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT12);
    pub const COLOR_ATTACHMENT13: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT13);
    pub const COLOR_ATTACHMENT14: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT14);
    pub const COLOR_ATTACHMENT15: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT15);
    pub const COLOR_ATTACHMENT16: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT16);
    pub const COLOR_ATTACHMENT17: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT17);
    pub const COLOR_ATTACHMENT18: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT18);
    pub const COLOR_ATTACHMENT19: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT19);
    pub const COLOR_ATTACHMENT20: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT20);
    pub const COLOR_ATTACHMENT21: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT21);
    pub const COLOR_ATTACHMENT22: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT22);
    pub const COLOR_ATTACHMENT23: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT23);
    pub const COLOR_ATTACHMENT24: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT24);
    pub const COLOR_ATTACHMENT25: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT25);
    pub const COLOR_ATTACHMENT26: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT26);
    pub const COLOR_ATTACHMENT27: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT27);
    pub const COLOR_ATTACHMENT28: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT28);
    pub const COLOR_ATTACHMENT29: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT29);
    pub const COLOR_ATTACHMENT30: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT30);
    pub const COLOR_ATTACHMENT31: FramebufferAttachment = FramebufferAttachment(super::COLOR_ATTACHMENT31);
    pub const DEPTH_ATTACHMENT: FramebufferAttachment = FramebufferAttachment(super::DEPTH_ATTACHMENT);
    pub const DEPTH_STENCIL_ATTACHMENT: FramebufferAttachment = FramebufferAttachment(super::DEPTH_STENCIL_ATTACHMENT);
}

impl ::std::fmt::Debug for FramebufferAttachment {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FramebufferAttachment::COLOR_ATTACHMENT0 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT0)"),
            FramebufferAttachment::COLOR_ATTACHMENT1 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT1)"),
            FramebufferAttachment::COLOR_ATTACHMENT10 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT10)"),
            FramebufferAttachment::COLOR_ATTACHMENT11 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT11)"),
            FramebufferAttachment::COLOR_ATTACHMENT12 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT12)"),
            FramebufferAttachment::COLOR_ATTACHMENT13 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT13)"),
            FramebufferAttachment::COLOR_ATTACHMENT14 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT14)"),
            FramebufferAttachment::COLOR_ATTACHMENT15 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT15)"),
            FramebufferAttachment::COLOR_ATTACHMENT16 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT16)"),
            FramebufferAttachment::COLOR_ATTACHMENT17 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT17)"),
            FramebufferAttachment::COLOR_ATTACHMENT18 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT18)"),
            FramebufferAttachment::COLOR_ATTACHMENT19 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT19)"),
            FramebufferAttachment::COLOR_ATTACHMENT2 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT2)"),
            FramebufferAttachment::COLOR_ATTACHMENT20 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT20)"),
            FramebufferAttachment::COLOR_ATTACHMENT21 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT21)"),
            FramebufferAttachment::COLOR_ATTACHMENT22 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT22)"),
            FramebufferAttachment::COLOR_ATTACHMENT23 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT23)"),
            FramebufferAttachment::COLOR_ATTACHMENT24 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT24)"),
            FramebufferAttachment::COLOR_ATTACHMENT25 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT25)"),
            FramebufferAttachment::COLOR_ATTACHMENT26 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT26)"),
            FramebufferAttachment::COLOR_ATTACHMENT27 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT27)"),
            FramebufferAttachment::COLOR_ATTACHMENT28 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT28)"),
            FramebufferAttachment::COLOR_ATTACHMENT29 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT29)"),
            FramebufferAttachment::COLOR_ATTACHMENT3 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT3)"),
            FramebufferAttachment::COLOR_ATTACHMENT30 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT30)"),
            FramebufferAttachment::COLOR_ATTACHMENT31 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT31)"),
            FramebufferAttachment::COLOR_ATTACHMENT4 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT4)"),
            FramebufferAttachment::COLOR_ATTACHMENT5 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT5)"),
            FramebufferAttachment::COLOR_ATTACHMENT6 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT6)"),
            FramebufferAttachment::COLOR_ATTACHMENT7 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT7)"),
            FramebufferAttachment::COLOR_ATTACHMENT8 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT8)"),
            FramebufferAttachment::COLOR_ATTACHMENT9 => write!(fmt, "FramebufferAttachment(COLOR_ATTACHMENT9)"),
            FramebufferAttachment::DEPTH_ATTACHMENT => write!(fmt, "FramebufferAttachment(DEPTH_ATTACHMENT)"),
            FramebufferAttachment::DEPTH_STENCIL_ATTACHMENT => write!(fmt, "FramebufferAttachment(DEPTH_STENCIL_ATTACHMENT)"),
            FramebufferAttachment::MAX_COLOR_ATTACHMENTS => write!(fmt, "FramebufferAttachment(MAX_COLOR_ATTACHMENTS)"),
            _ => write!(fmt, "FramebufferAttachment({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferAttachment);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferAttachmentParameterName(pub types::GLenum);

impl FramebufferAttachmentParameterName {
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_RED_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_GREEN_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_BLUE_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE);
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE);
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING);
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME);
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL);
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE);
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_LAYERED);
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: FramebufferAttachmentParameterName = FramebufferAttachmentParameterName(super::FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER);
}

impl ::std::fmt::Debug for FramebufferAttachmentParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_BLUE_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_BLUE_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_GREEN_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_GREEN_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_LAYERED => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_LAYERED)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_OBJECT_NAME => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_OBJECT_NAME)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_RED_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_RED_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER)"),
            FramebufferAttachmentParameterName::FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL => write!(fmt, "FramebufferAttachmentParameterName(FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL)"),
            _ => write!(fmt, "FramebufferAttachmentParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferAttachmentParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferFetchNoncoherent(pub types::GLenum);

impl FramebufferFetchNoncoherent {
}

impl ::std::fmt::Debug for FramebufferFetchNoncoherent {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "FramebufferFetchNoncoherent({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferFetchNoncoherent);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferParameterName(pub types::GLenum);

impl FramebufferParameterName {
    pub const FRAMEBUFFER_DEFAULT_WIDTH: FramebufferParameterName = FramebufferParameterName(super::FRAMEBUFFER_DEFAULT_WIDTH);
    pub const FRAMEBUFFER_DEFAULT_HEIGHT: FramebufferParameterName = FramebufferParameterName(super::FRAMEBUFFER_DEFAULT_HEIGHT);
    pub const FRAMEBUFFER_DEFAULT_LAYERS: FramebufferParameterName = FramebufferParameterName(super::FRAMEBUFFER_DEFAULT_LAYERS);
    pub const FRAMEBUFFER_DEFAULT_SAMPLES: FramebufferParameterName = FramebufferParameterName(super::FRAMEBUFFER_DEFAULT_SAMPLES);
    pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: FramebufferParameterName = FramebufferParameterName(super::FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS);
}

impl ::std::fmt::Debug for FramebufferParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FramebufferParameterName::FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS => write!(fmt, "FramebufferParameterName(FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS)"),
            FramebufferParameterName::FRAMEBUFFER_DEFAULT_HEIGHT => write!(fmt, "FramebufferParameterName(FRAMEBUFFER_DEFAULT_HEIGHT)"),
            FramebufferParameterName::FRAMEBUFFER_DEFAULT_LAYERS => write!(fmt, "FramebufferParameterName(FRAMEBUFFER_DEFAULT_LAYERS)"),
            FramebufferParameterName::FRAMEBUFFER_DEFAULT_SAMPLES => write!(fmt, "FramebufferParameterName(FRAMEBUFFER_DEFAULT_SAMPLES)"),
            FramebufferParameterName::FRAMEBUFFER_DEFAULT_WIDTH => write!(fmt, "FramebufferParameterName(FRAMEBUFFER_DEFAULT_WIDTH)"),
            _ => write!(fmt, "FramebufferParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferStatus(pub types::GLenum);

impl FramebufferStatus {
    pub const FRAMEBUFFER_COMPLETE: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_COMPLETE);
    pub const FRAMEBUFFER_UNDEFINED: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_UNDEFINED);
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_ATTACHMENT);
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT);
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER);
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_READ_BUFFER);
    pub const FRAMEBUFFER_UNSUPPORTED: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_UNSUPPORTED);
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE);
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: FramebufferStatus = FramebufferStatus(super::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS);
}

impl ::std::fmt::Debug for FramebufferStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FramebufferStatus::FRAMEBUFFER_COMPLETE => write!(fmt, "FramebufferStatus(FRAMEBUFFER_COMPLETE)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_ATTACHMENT)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_MULTISAMPLE)"),
            FramebufferStatus::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => write!(fmt, "FramebufferStatus(FRAMEBUFFER_INCOMPLETE_READ_BUFFER)"),
            FramebufferStatus::FRAMEBUFFER_UNDEFINED => write!(fmt, "FramebufferStatus(FRAMEBUFFER_UNDEFINED)"),
            FramebufferStatus::FRAMEBUFFER_UNSUPPORTED => write!(fmt, "FramebufferStatus(FRAMEBUFFER_UNSUPPORTED)"),
            _ => write!(fmt, "FramebufferStatus({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferStatus);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FramebufferTarget(pub types::GLenum);

impl FramebufferTarget {
    pub const FRAMEBUFFER: FramebufferTarget = FramebufferTarget(super::FRAMEBUFFER);
    pub const DRAW_FRAMEBUFFER: FramebufferTarget = FramebufferTarget(super::DRAW_FRAMEBUFFER);
    pub const READ_FRAMEBUFFER: FramebufferTarget = FramebufferTarget(super::READ_FRAMEBUFFER);
}

impl ::std::fmt::Debug for FramebufferTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FramebufferTarget::DRAW_FRAMEBUFFER => write!(fmt, "FramebufferTarget(DRAW_FRAMEBUFFER)"),
            FramebufferTarget::FRAMEBUFFER => write!(fmt, "FramebufferTarget(FRAMEBUFFER)"),
            FramebufferTarget::READ_FRAMEBUFFER => write!(fmt, "FramebufferTarget(READ_FRAMEBUFFER)"),
            _ => write!(fmt, "FramebufferTarget({})", self.0),
        }
    }
}

impl_enum_traits!(FramebufferTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FrontFaceDirection(pub types::GLenum);

impl FrontFaceDirection {
    pub const CCW: FrontFaceDirection = FrontFaceDirection(super::CCW);
    pub const CW: FrontFaceDirection = FrontFaceDirection(super::CW);
}

impl ::std::fmt::Debug for FrontFaceDirection {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FrontFaceDirection::CCW => write!(fmt, "FrontFaceDirection(CCW)"),
            FrontFaceDirection::CW => write!(fmt, "FrontFaceDirection(CW)"),
            _ => write!(fmt, "FrontFaceDirection({})", self.0),
        }
    }
}

impl_enum_traits!(FrontFaceDirection);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetColorTableParameterPNameSGI(pub types::GLenum);

impl GetColorTableParameterPNameSGI {
}

impl ::std::fmt::Debug for GetColorTableParameterPNameSGI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetColorTableParameterPNameSGI({})", self.0),
        }
    }
}

impl_enum_traits!(GetColorTableParameterPNameSGI);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetConvolutionParameter(pub types::GLenum);

impl GetConvolutionParameter {
}

impl ::std::fmt::Debug for GetConvolutionParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetConvolutionParameter({})", self.0),
        }
    }
}

impl_enum_traits!(GetConvolutionParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetFramebufferParameter(pub types::GLenum);

impl GetFramebufferParameter {
    pub const FRAMEBUFFER_DEFAULT_WIDTH: GetFramebufferParameter = GetFramebufferParameter(super::FRAMEBUFFER_DEFAULT_WIDTH);
    pub const FRAMEBUFFER_DEFAULT_HEIGHT: GetFramebufferParameter = GetFramebufferParameter(super::FRAMEBUFFER_DEFAULT_HEIGHT);
    pub const FRAMEBUFFER_DEFAULT_LAYERS: GetFramebufferParameter = GetFramebufferParameter(super::FRAMEBUFFER_DEFAULT_LAYERS);
    pub const FRAMEBUFFER_DEFAULT_SAMPLES: GetFramebufferParameter = GetFramebufferParameter(super::FRAMEBUFFER_DEFAULT_SAMPLES);
    pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GetFramebufferParameter = GetFramebufferParameter(super::FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS);
    pub const DOUBLEBUFFER: GetFramebufferParameter = GetFramebufferParameter(super::DOUBLEBUFFER);
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GetFramebufferParameter = GetFramebufferParameter(super::IMPLEMENTATION_COLOR_READ_FORMAT);
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GetFramebufferParameter = GetFramebufferParameter(super::IMPLEMENTATION_COLOR_READ_TYPE);
    pub const SAMPLES: GetFramebufferParameter = GetFramebufferParameter(super::SAMPLES);
    pub const SAMPLE_BUFFERS: GetFramebufferParameter = GetFramebufferParameter(super::SAMPLE_BUFFERS);
    pub const STEREO: GetFramebufferParameter = GetFramebufferParameter(super::STEREO);
}

impl ::std::fmt::Debug for GetFramebufferParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GetFramebufferParameter::DOUBLEBUFFER => write!(fmt, "GetFramebufferParameter(DOUBLEBUFFER)"),
            GetFramebufferParameter::FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS => write!(fmt, "GetFramebufferParameter(FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS)"),
            GetFramebufferParameter::FRAMEBUFFER_DEFAULT_HEIGHT => write!(fmt, "GetFramebufferParameter(FRAMEBUFFER_DEFAULT_HEIGHT)"),
            GetFramebufferParameter::FRAMEBUFFER_DEFAULT_LAYERS => write!(fmt, "GetFramebufferParameter(FRAMEBUFFER_DEFAULT_LAYERS)"),
            GetFramebufferParameter::FRAMEBUFFER_DEFAULT_SAMPLES => write!(fmt, "GetFramebufferParameter(FRAMEBUFFER_DEFAULT_SAMPLES)"),
            GetFramebufferParameter::FRAMEBUFFER_DEFAULT_WIDTH => write!(fmt, "GetFramebufferParameter(FRAMEBUFFER_DEFAULT_WIDTH)"),
            GetFramebufferParameter::IMPLEMENTATION_COLOR_READ_FORMAT => write!(fmt, "GetFramebufferParameter(IMPLEMENTATION_COLOR_READ_FORMAT)"),
            GetFramebufferParameter::IMPLEMENTATION_COLOR_READ_TYPE => write!(fmt, "GetFramebufferParameter(IMPLEMENTATION_COLOR_READ_TYPE)"),
            GetFramebufferParameter::SAMPLES => write!(fmt, "GetFramebufferParameter(SAMPLES)"),
            GetFramebufferParameter::SAMPLE_BUFFERS => write!(fmt, "GetFramebufferParameter(SAMPLE_BUFFERS)"),
            GetFramebufferParameter::STEREO => write!(fmt, "GetFramebufferParameter(STEREO)"),
            _ => write!(fmt, "GetFramebufferParameter({})", self.0),
        }
    }
}

impl_enum_traits!(GetFramebufferParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetHistogramParameterPNameEXT(pub types::GLenum);

impl GetHistogramParameterPNameEXT {
}

impl ::std::fmt::Debug for GetHistogramParameterPNameEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetHistogramParameterPNameEXT({})", self.0),
        }
    }
}

impl_enum_traits!(GetHistogramParameterPNameEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetMapQuery(pub types::GLenum);

impl GetMapQuery {
}

impl ::std::fmt::Debug for GetMapQuery {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetMapQuery({})", self.0),
        }
    }
}

impl_enum_traits!(GetMapQuery);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetMinmaxParameterPNameEXT(pub types::GLenum);

impl GetMinmaxParameterPNameEXT {
}

impl ::std::fmt::Debug for GetMinmaxParameterPNameEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetMinmaxParameterPNameEXT({})", self.0),
        }
    }
}

impl_enum_traits!(GetMinmaxParameterPNameEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetPName(pub types::GLenum);

impl GetPName {
    pub const ACTIVE_TEXTURE: GetPName = GetPName(super::ACTIVE_TEXTURE);
    pub const ALIASED_LINE_WIDTH_RANGE: GetPName = GetPName(super::ALIASED_LINE_WIDTH_RANGE);
    pub const ARRAY_BUFFER_BINDING: GetPName = GetPName(super::ARRAY_BUFFER_BINDING);
    pub const BLEND: GetPName = GetPName(super::BLEND);
    pub const BLEND_COLOR: GetPName = GetPName(super::BLEND_COLOR);
    pub const BLEND_DST: GetPName = GetPName(super::BLEND_DST);
    pub const BLEND_DST_ALPHA: GetPName = GetPName(super::BLEND_DST_ALPHA);
    pub const BLEND_DST_RGB: GetPName = GetPName(super::BLEND_DST_RGB);
    pub const BLEND_EQUATION_ALPHA: GetPName = GetPName(super::BLEND_EQUATION_ALPHA);
    pub const BLEND_EQUATION_RGB: GetPName = GetPName(super::BLEND_EQUATION_RGB);
    pub const BLEND_SRC: GetPName = GetPName(super::BLEND_SRC);
    pub const BLEND_SRC_ALPHA: GetPName = GetPName(super::BLEND_SRC_ALPHA);
    pub const BLEND_SRC_RGB: GetPName = GetPName(super::BLEND_SRC_RGB);
    pub const COLOR_CLEAR_VALUE: GetPName = GetPName(super::COLOR_CLEAR_VALUE);
    pub const COLOR_LOGIC_OP: GetPName = GetPName(super::COLOR_LOGIC_OP);
    pub const COLOR_WRITEMASK: GetPName = GetPName(super::COLOR_WRITEMASK);
    pub const COMPRESSED_TEXTURE_FORMATS: GetPName = GetPName(super::COMPRESSED_TEXTURE_FORMATS);
    pub const CONTEXT_FLAGS: GetPName = GetPName(super::CONTEXT_FLAGS);
    pub const CULL_FACE: GetPName = GetPName(super::CULL_FACE);
    pub const CULL_FACE_MODE: GetPName = GetPName(super::CULL_FACE_MODE);
    pub const CURRENT_PROGRAM: GetPName = GetPName(super::CURRENT_PROGRAM);
    pub const DEBUG_GROUP_STACK_DEPTH: GetPName = GetPName(super::DEBUG_GROUP_STACK_DEPTH);
    pub const DEPTH_CLEAR_VALUE: GetPName = GetPName(super::DEPTH_CLEAR_VALUE);
    pub const DEPTH_FUNC: GetPName = GetPName(super::DEPTH_FUNC);
    pub const DEPTH_RANGE: GetPName = GetPName(super::DEPTH_RANGE);
    pub const DEPTH_TEST: GetPName = GetPName(super::DEPTH_TEST);
    pub const DEPTH_WRITEMASK: GetPName = GetPName(super::DEPTH_WRITEMASK);
    pub const DISPATCH_INDIRECT_BUFFER_BINDING: GetPName = GetPName(super::DISPATCH_INDIRECT_BUFFER_BINDING);
    pub const DITHER: GetPName = GetPName(super::DITHER);
    pub const DOUBLEBUFFER: GetPName = GetPName(super::DOUBLEBUFFER);
    pub const DRAW_BUFFER: GetPName = GetPName(super::DRAW_BUFFER);
    pub const DRAW_FRAMEBUFFER_BINDING: GetPName = GetPName(super::DRAW_FRAMEBUFFER_BINDING);
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GetPName = GetPName(super::ELEMENT_ARRAY_BUFFER_BINDING);
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GetPName = GetPName(super::FRAGMENT_SHADER_DERIVATIVE_HINT);
    pub const FRONT_FACE: GetPName = GetPName(super::FRONT_FACE);
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GetPName = GetPName(super::IMPLEMENTATION_COLOR_READ_FORMAT);
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GetPName = GetPName(super::IMPLEMENTATION_COLOR_READ_TYPE);
    pub const LAYER_PROVOKING_VERTEX: GetPName = GetPName(super::LAYER_PROVOKING_VERTEX);
    pub const LINE_SMOOTH: GetPName = GetPName(super::LINE_SMOOTH);
    pub const LINE_SMOOTH_HINT: GetPName = GetPName(super::LINE_SMOOTH_HINT);
    pub const LINE_WIDTH: GetPName = GetPName(super::LINE_WIDTH);
    pub const LINE_WIDTH_GRANULARITY: GetPName = GetPName(super::LINE_WIDTH_GRANULARITY);
    pub const LINE_WIDTH_RANGE: GetPName = GetPName(super::LINE_WIDTH_RANGE);
    pub const LOGIC_OP_MODE: GetPName = GetPName(super::LOGIC_OP_MODE);
    pub const MAJOR_VERSION: GetPName = GetPName(super::MAJOR_VERSION);
    pub const MAX_3D_TEXTURE_SIZE: GetPName = GetPName(super::MAX_3D_TEXTURE_SIZE);
    pub const MAX_ARRAY_TEXTURE_LAYERS: GetPName = GetPName(super::MAX_ARRAY_TEXTURE_LAYERS);
    pub const MAX_CLIP_DISTANCES: GetPName = GetPName(super::MAX_CLIP_DISTANCES);
    pub const MAX_COLOR_TEXTURE_SAMPLES: GetPName = GetPName(super::MAX_COLOR_TEXTURE_SAMPLES);
    pub const MAX_COMBINED_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_COMBINED_ATOMIC_COUNTERS);
    pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS);
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS);
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS);
    pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_COMBINED_SHADER_STORAGE_BLOCKS);
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GetPName = GetPName(super::MAX_COMBINED_TEXTURE_IMAGE_UNITS);
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GetPName = GetPName(super::MAX_COMBINED_UNIFORM_BLOCKS);
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS);
    pub const MAX_COMPUTE_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_COMPUTE_ATOMIC_COUNTERS);
    pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GetPName = GetPName(super::MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS);
    pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_COMPUTE_SHADER_STORAGE_BLOCKS);
    pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GetPName = GetPName(super::MAX_COMPUTE_TEXTURE_IMAGE_UNITS);
    pub const MAX_COMPUTE_UNIFORM_BLOCKS: GetPName = GetPName(super::MAX_COMPUTE_UNIFORM_BLOCKS);
    pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_COMPUTE_UNIFORM_COMPONENTS);
    pub const MAX_COMPUTE_WORK_GROUP_COUNT: GetPName = GetPName(super::MAX_COMPUTE_WORK_GROUP_COUNT);
    pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GetPName = GetPName(super::MAX_COMPUTE_WORK_GROUP_INVOCATIONS);
    pub const MAX_COMPUTE_WORK_GROUP_SIZE: GetPName = GetPName(super::MAX_COMPUTE_WORK_GROUP_SIZE);
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GetPName = GetPName(super::MAX_CUBE_MAP_TEXTURE_SIZE);
    pub const MAX_DEBUG_GROUP_STACK_DEPTH: GetPName = GetPName(super::MAX_DEBUG_GROUP_STACK_DEPTH);
    pub const MAX_DEPTH_TEXTURE_SAMPLES: GetPName = GetPName(super::MAX_DEPTH_TEXTURE_SAMPLES);
    pub const MAX_DRAW_BUFFERS: GetPName = GetPName(super::MAX_DRAW_BUFFERS);
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GetPName = GetPName(super::MAX_DUAL_SOURCE_DRAW_BUFFERS);
    pub const MAX_ELEMENTS_INDICES: GetPName = GetPName(super::MAX_ELEMENTS_INDICES);
    pub const MAX_ELEMENTS_VERTICES: GetPName = GetPName(super::MAX_ELEMENTS_VERTICES);
    pub const MAX_ELEMENT_INDEX: GetPName = GetPName(super::MAX_ELEMENT_INDEX);
    pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_FRAGMENT_ATOMIC_COUNTERS);
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GetPName = GetPName(super::MAX_FRAGMENT_INPUT_COMPONENTS);
    pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_FRAGMENT_SHADER_STORAGE_BLOCKS);
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GetPName = GetPName(super::MAX_FRAGMENT_UNIFORM_BLOCKS);
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_FRAGMENT_UNIFORM_COMPONENTS);
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GetPName = GetPName(super::MAX_FRAGMENT_UNIFORM_VECTORS);
    pub const MAX_FRAMEBUFFER_HEIGHT: GetPName = GetPName(super::MAX_FRAMEBUFFER_HEIGHT);
    pub const MAX_FRAMEBUFFER_LAYERS: GetPName = GetPName(super::MAX_FRAMEBUFFER_LAYERS);
    pub const MAX_FRAMEBUFFER_SAMPLES: GetPName = GetPName(super::MAX_FRAMEBUFFER_SAMPLES);
    pub const MAX_FRAMEBUFFER_WIDTH: GetPName = GetPName(super::MAX_FRAMEBUFFER_WIDTH);
    pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_GEOMETRY_ATOMIC_COUNTERS);
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: GetPName = GetPName(super::MAX_GEOMETRY_INPUT_COMPONENTS);
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GetPName = GetPName(super::MAX_GEOMETRY_OUTPUT_COMPONENTS);
    pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_GEOMETRY_SHADER_STORAGE_BLOCKS);
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GetPName = GetPName(super::MAX_GEOMETRY_TEXTURE_IMAGE_UNITS);
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GetPName = GetPName(super::MAX_GEOMETRY_UNIFORM_BLOCKS);
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_GEOMETRY_UNIFORM_COMPONENTS);
    pub const MAX_INTEGER_SAMPLES: GetPName = GetPName(super::MAX_INTEGER_SAMPLES);
    pub const MAX_LABEL_LENGTH: GetPName = GetPName(super::MAX_LABEL_LENGTH);
    pub const MAX_PROGRAM_TEXEL_OFFSET: GetPName = GetPName(super::MAX_PROGRAM_TEXEL_OFFSET);
    pub const MAX_RECTANGLE_TEXTURE_SIZE: GetPName = GetPName(super::MAX_RECTANGLE_TEXTURE_SIZE);
    pub const MAX_RENDERBUFFER_SIZE: GetPName = GetPName(super::MAX_RENDERBUFFER_SIZE);
    pub const MAX_SAMPLE_MASK_WORDS: GetPName = GetPName(super::MAX_SAMPLE_MASK_WORDS);
    pub const MAX_SERVER_WAIT_TIMEOUT: GetPName = GetPName(super::MAX_SERVER_WAIT_TIMEOUT);
    pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GetPName = GetPName(super::MAX_SHADER_STORAGE_BUFFER_BINDINGS);
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_TESS_CONTROL_ATOMIC_COUNTERS);
    pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS);
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_TESS_EVALUATION_ATOMIC_COUNTERS);
    pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS);
    pub const MAX_TEXTURE_BUFFER_SIZE: GetPName = GetPName(super::MAX_TEXTURE_BUFFER_SIZE);
    pub const MAX_TEXTURE_IMAGE_UNITS: GetPName = GetPName(super::MAX_TEXTURE_IMAGE_UNITS);
    pub const MAX_TEXTURE_LOD_BIAS: GetPName = GetPName(super::MAX_TEXTURE_LOD_BIAS);
    pub const MAX_TEXTURE_SIZE: GetPName = GetPName(super::MAX_TEXTURE_SIZE);
    pub const MAX_UNIFORM_BLOCK_SIZE: GetPName = GetPName(super::MAX_UNIFORM_BLOCK_SIZE);
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GetPName = GetPName(super::MAX_UNIFORM_BUFFER_BINDINGS);
    pub const MAX_UNIFORM_LOCATIONS: GetPName = GetPName(super::MAX_UNIFORM_LOCATIONS);
    pub const MAX_VARYING_COMPONENTS: GetPName = GetPName(super::MAX_VARYING_COMPONENTS);
    pub const MAX_VARYING_FLOATS: GetPName = GetPName(super::MAX_VARYING_FLOATS);
    pub const MAX_VARYING_VECTORS: GetPName = GetPName(super::MAX_VARYING_VECTORS);
    pub const MAX_VERTEX_ATOMIC_COUNTERS: GetPName = GetPName(super::MAX_VERTEX_ATOMIC_COUNTERS);
    pub const MAX_VERTEX_ATTRIBS: GetPName = GetPName(super::MAX_VERTEX_ATTRIBS);
    pub const MAX_VERTEX_ATTRIB_BINDINGS: GetPName = GetPName(super::MAX_VERTEX_ATTRIB_BINDINGS);
    pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GetPName = GetPName(super::MAX_VERTEX_ATTRIB_RELATIVE_OFFSET);
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GetPName = GetPName(super::MAX_VERTEX_OUTPUT_COMPONENTS);
    pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GetPName = GetPName(super::MAX_VERTEX_SHADER_STORAGE_BLOCKS);
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GetPName = GetPName(super::MAX_VERTEX_TEXTURE_IMAGE_UNITS);
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GetPName = GetPName(super::MAX_VERTEX_UNIFORM_BLOCKS);
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GetPName = GetPName(super::MAX_VERTEX_UNIFORM_COMPONENTS);
    pub const MAX_VERTEX_UNIFORM_VECTORS: GetPName = GetPName(super::MAX_VERTEX_UNIFORM_VECTORS);
    pub const MAX_VIEWPORTS: GetPName = GetPName(super::MAX_VIEWPORTS);
    pub const MAX_VIEWPORT_DIMS: GetPName = GetPName(super::MAX_VIEWPORT_DIMS);
    pub const MINOR_VERSION: GetPName = GetPName(super::MINOR_VERSION);
    pub const MIN_MAP_BUFFER_ALIGNMENT: GetPName = GetPName(super::MIN_MAP_BUFFER_ALIGNMENT);
    pub const MIN_PROGRAM_TEXEL_OFFSET: GetPName = GetPName(super::MIN_PROGRAM_TEXEL_OFFSET);
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: GetPName = GetPName(super::NUM_COMPRESSED_TEXTURE_FORMATS);
    pub const NUM_EXTENSIONS: GetPName = GetPName(super::NUM_EXTENSIONS);
    pub const NUM_PROGRAM_BINARY_FORMATS: GetPName = GetPName(super::NUM_PROGRAM_BINARY_FORMATS);
    pub const NUM_SHADER_BINARY_FORMATS: GetPName = GetPName(super::NUM_SHADER_BINARY_FORMATS);
    pub const PACK_ALIGNMENT: GetPName = GetPName(super::PACK_ALIGNMENT);
    pub const PACK_IMAGE_HEIGHT: GetPName = GetPName(super::PACK_IMAGE_HEIGHT);
    pub const PACK_LSB_FIRST: GetPName = GetPName(super::PACK_LSB_FIRST);
    pub const PACK_ROW_LENGTH: GetPName = GetPName(super::PACK_ROW_LENGTH);
    pub const PACK_SKIP_IMAGES: GetPName = GetPName(super::PACK_SKIP_IMAGES);
    pub const PACK_SKIP_PIXELS: GetPName = GetPName(super::PACK_SKIP_PIXELS);
    pub const PACK_SKIP_ROWS: GetPName = GetPName(super::PACK_SKIP_ROWS);
    pub const PACK_SWAP_BYTES: GetPName = GetPName(super::PACK_SWAP_BYTES);
    pub const PIXEL_PACK_BUFFER_BINDING: GetPName = GetPName(super::PIXEL_PACK_BUFFER_BINDING);
    pub const PIXEL_UNPACK_BUFFER_BINDING: GetPName = GetPName(super::PIXEL_UNPACK_BUFFER_BINDING);
    pub const POINT_FADE_THRESHOLD_SIZE: GetPName = GetPName(super::POINT_FADE_THRESHOLD_SIZE);
    pub const POINT_SIZE: GetPName = GetPName(super::POINT_SIZE);
    pub const POINT_SIZE_GRANULARITY: GetPName = GetPName(super::POINT_SIZE_GRANULARITY);
    pub const POINT_SIZE_RANGE: GetPName = GetPName(super::POINT_SIZE_RANGE);
    pub const POLYGON_MODE: GetPName = GetPName(super::POLYGON_MODE);
    pub const POLYGON_OFFSET_FACTOR: GetPName = GetPName(super::POLYGON_OFFSET_FACTOR);
    pub const POLYGON_OFFSET_FILL: GetPName = GetPName(super::POLYGON_OFFSET_FILL);
    pub const POLYGON_OFFSET_LINE: GetPName = GetPName(super::POLYGON_OFFSET_LINE);
    pub const POLYGON_OFFSET_POINT: GetPName = GetPName(super::POLYGON_OFFSET_POINT);
    pub const POLYGON_OFFSET_UNITS: GetPName = GetPName(super::POLYGON_OFFSET_UNITS);
    pub const POLYGON_SMOOTH: GetPName = GetPName(super::POLYGON_SMOOTH);
    pub const POLYGON_SMOOTH_HINT: GetPName = GetPName(super::POLYGON_SMOOTH_HINT);
    pub const PRIMITIVE_RESTART_INDEX: GetPName = GetPName(super::PRIMITIVE_RESTART_INDEX);
    pub const PROGRAM_BINARY_FORMATS: GetPName = GetPName(super::PROGRAM_BINARY_FORMATS);
    pub const PROGRAM_PIPELINE_BINDING: GetPName = GetPName(super::PROGRAM_PIPELINE_BINDING);
    pub const PROGRAM_POINT_SIZE: GetPName = GetPName(super::PROGRAM_POINT_SIZE);
    pub const PROVOKING_VERTEX: GetPName = GetPName(super::PROVOKING_VERTEX);
    pub const READ_BUFFER: GetPName = GetPName(super::READ_BUFFER);
    pub const READ_FRAMEBUFFER_BINDING: GetPName = GetPName(super::READ_FRAMEBUFFER_BINDING);
    pub const RENDERBUFFER_BINDING: GetPName = GetPName(super::RENDERBUFFER_BINDING);
    pub const SAMPLER_BINDING: GetPName = GetPName(super::SAMPLER_BINDING);
    pub const SAMPLES: GetPName = GetPName(super::SAMPLES);
    pub const SAMPLE_BUFFERS: GetPName = GetPName(super::SAMPLE_BUFFERS);
    pub const SAMPLE_COVERAGE_INVERT: GetPName = GetPName(super::SAMPLE_COVERAGE_INVERT);
    pub const SAMPLE_COVERAGE_VALUE: GetPName = GetPName(super::SAMPLE_COVERAGE_VALUE);
    pub const SCISSOR_BOX: GetPName = GetPName(super::SCISSOR_BOX);
    pub const SCISSOR_TEST: GetPName = GetPName(super::SCISSOR_TEST);
    pub const SHADER_COMPILER: GetPName = GetPName(super::SHADER_COMPILER);
    pub const SHADER_STORAGE_BUFFER_BINDING: GetPName = GetPName(super::SHADER_STORAGE_BUFFER_BINDING);
    pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GetPName = GetPName(super::SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT);
    pub const SHADER_STORAGE_BUFFER_SIZE: GetPName = GetPName(super::SHADER_STORAGE_BUFFER_SIZE);
    pub const SHADER_STORAGE_BUFFER_START: GetPName = GetPName(super::SHADER_STORAGE_BUFFER_START);
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: GetPName = GetPName(super::SMOOTH_LINE_WIDTH_GRANULARITY);
    pub const SMOOTH_LINE_WIDTH_RANGE: GetPName = GetPName(super::SMOOTH_LINE_WIDTH_RANGE);
    pub const SMOOTH_POINT_SIZE_GRANULARITY: GetPName = GetPName(super::SMOOTH_POINT_SIZE_GRANULARITY);
    pub const SMOOTH_POINT_SIZE_RANGE: GetPName = GetPName(super::SMOOTH_POINT_SIZE_RANGE);
    pub const STENCIL_BACK_FAIL: GetPName = GetPName(super::STENCIL_BACK_FAIL);
    pub const STENCIL_BACK_FUNC: GetPName = GetPName(super::STENCIL_BACK_FUNC);
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GetPName = GetPName(super::STENCIL_BACK_PASS_DEPTH_FAIL);
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GetPName = GetPName(super::STENCIL_BACK_PASS_DEPTH_PASS);
    pub const STENCIL_BACK_REF: GetPName = GetPName(super::STENCIL_BACK_REF);
    pub const STENCIL_BACK_VALUE_MASK: GetPName = GetPName(super::STENCIL_BACK_VALUE_MASK);
    pub const STENCIL_BACK_WRITEMASK: GetPName = GetPName(super::STENCIL_BACK_WRITEMASK);
    pub const STENCIL_CLEAR_VALUE: GetPName = GetPName(super::STENCIL_CLEAR_VALUE);
    pub const STENCIL_FAIL: GetPName = GetPName(super::STENCIL_FAIL);
    pub const STENCIL_FUNC: GetPName = GetPName(super::STENCIL_FUNC);
    pub const STENCIL_PASS_DEPTH_FAIL: GetPName = GetPName(super::STENCIL_PASS_DEPTH_FAIL);
    pub const STENCIL_PASS_DEPTH_PASS: GetPName = GetPName(super::STENCIL_PASS_DEPTH_PASS);
    pub const STENCIL_REF: GetPName = GetPName(super::STENCIL_REF);
    pub const STENCIL_TEST: GetPName = GetPName(super::STENCIL_TEST);
    pub const STENCIL_VALUE_MASK: GetPName = GetPName(super::STENCIL_VALUE_MASK);
    pub const STENCIL_WRITEMASK: GetPName = GetPName(super::STENCIL_WRITEMASK);
    pub const STEREO: GetPName = GetPName(super::STEREO);
    pub const SUBPIXEL_BITS: GetPName = GetPName(super::SUBPIXEL_BITS);
    pub const TEXTURE_1D: GetPName = GetPName(super::TEXTURE_1D);
    pub const TEXTURE_2D: GetPName = GetPName(super::TEXTURE_2D);
    pub const TEXTURE_BINDING_1D: GetPName = GetPName(super::TEXTURE_BINDING_1D);
    pub const TEXTURE_BINDING_1D_ARRAY: GetPName = GetPName(super::TEXTURE_BINDING_1D_ARRAY);
    pub const TEXTURE_BINDING_2D: GetPName = GetPName(super::TEXTURE_BINDING_2D);
    pub const TEXTURE_BINDING_2D_ARRAY: GetPName = GetPName(super::TEXTURE_BINDING_2D_ARRAY);
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: GetPName = GetPName(super::TEXTURE_BINDING_2D_MULTISAMPLE);
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GetPName = GetPName(super::TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY);
    pub const TEXTURE_BINDING_3D: GetPName = GetPName(super::TEXTURE_BINDING_3D);
    pub const TEXTURE_BINDING_BUFFER: GetPName = GetPName(super::TEXTURE_BINDING_BUFFER);
    pub const TEXTURE_BINDING_CUBE_MAP: GetPName = GetPName(super::TEXTURE_BINDING_CUBE_MAP);
    pub const TEXTURE_BINDING_RECTANGLE: GetPName = GetPName(super::TEXTURE_BINDING_RECTANGLE);
    pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GetPName = GetPName(super::TEXTURE_BUFFER_OFFSET_ALIGNMENT);
    pub const TEXTURE_COMPRESSION_HINT: GetPName = GetPName(super::TEXTURE_COMPRESSION_HINT);
    pub const TIMESTAMP: GetPName = GetPName(super::TIMESTAMP);
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GetPName = GetPName(super::TRANSFORM_FEEDBACK_BUFFER_BINDING);
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GetPName = GetPName(super::TRANSFORM_FEEDBACK_BUFFER_SIZE);
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GetPName = GetPName(super::TRANSFORM_FEEDBACK_BUFFER_START);
    pub const UNIFORM_BUFFER_BINDING: GetPName = GetPName(super::UNIFORM_BUFFER_BINDING);
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GetPName = GetPName(super::UNIFORM_BUFFER_OFFSET_ALIGNMENT);
    pub const UNIFORM_BUFFER_SIZE: GetPName = GetPName(super::UNIFORM_BUFFER_SIZE);
    pub const UNIFORM_BUFFER_START: GetPName = GetPName(super::UNIFORM_BUFFER_START);
    pub const UNPACK_ALIGNMENT: GetPName = GetPName(super::UNPACK_ALIGNMENT);
    pub const UNPACK_IMAGE_HEIGHT: GetPName = GetPName(super::UNPACK_IMAGE_HEIGHT);
    pub const UNPACK_LSB_FIRST: GetPName = GetPName(super::UNPACK_LSB_FIRST);
    pub const UNPACK_ROW_LENGTH: GetPName = GetPName(super::UNPACK_ROW_LENGTH);
    pub const UNPACK_SKIP_IMAGES: GetPName = GetPName(super::UNPACK_SKIP_IMAGES);
    pub const UNPACK_SKIP_PIXELS: GetPName = GetPName(super::UNPACK_SKIP_PIXELS);
    pub const UNPACK_SKIP_ROWS: GetPName = GetPName(super::UNPACK_SKIP_ROWS);
    pub const UNPACK_SWAP_BYTES: GetPName = GetPName(super::UNPACK_SWAP_BYTES);
    pub const VERTEX_ARRAY: GetPName = GetPName(super::VERTEX_ARRAY);
    pub const VERTEX_ARRAY_BINDING: GetPName = GetPName(super::VERTEX_ARRAY_BINDING);
    pub const VERTEX_BINDING_DIVISOR: GetPName = GetPName(super::VERTEX_BINDING_DIVISOR);
    pub const VERTEX_BINDING_OFFSET: GetPName = GetPName(super::VERTEX_BINDING_OFFSET);
    pub const VERTEX_BINDING_STRIDE: GetPName = GetPName(super::VERTEX_BINDING_STRIDE);
    pub const VIEWPORT: GetPName = GetPName(super::VIEWPORT);
    pub const VIEWPORT_BOUNDS_RANGE: GetPName = GetPName(super::VIEWPORT_BOUNDS_RANGE);
    pub const VIEWPORT_INDEX_PROVOKING_VERTEX: GetPName = GetPName(super::VIEWPORT_INDEX_PROVOKING_VERTEX);
    pub const VIEWPORT_SUBPIXEL_BITS: GetPName = GetPName(super::VIEWPORT_SUBPIXEL_BITS);
}

impl ::std::fmt::Debug for GetPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GetPName::ACTIVE_TEXTURE => write!(fmt, "GetPName(ACTIVE_TEXTURE)"),
            GetPName::ALIASED_LINE_WIDTH_RANGE => write!(fmt, "GetPName(ALIASED_LINE_WIDTH_RANGE)"),
            GetPName::ARRAY_BUFFER_BINDING => write!(fmt, "GetPName(ARRAY_BUFFER_BINDING)"),
            GetPName::BLEND => write!(fmt, "GetPName(BLEND)"),
            GetPName::BLEND_COLOR => write!(fmt, "GetPName(BLEND_COLOR)"),
            GetPName::BLEND_DST => write!(fmt, "GetPName(BLEND_DST)"),
            GetPName::BLEND_DST_ALPHA => write!(fmt, "GetPName(BLEND_DST_ALPHA)"),
            GetPName::BLEND_DST_RGB => write!(fmt, "GetPName(BLEND_DST_RGB)"),
            GetPName::BLEND_EQUATION_ALPHA => write!(fmt, "GetPName(BLEND_EQUATION_ALPHA)"),
            GetPName::BLEND_EQUATION_RGB => write!(fmt, "GetPName(BLEND_EQUATION_RGB)"),
            GetPName::BLEND_SRC => write!(fmt, "GetPName(BLEND_SRC)"),
            GetPName::BLEND_SRC_ALPHA => write!(fmt, "GetPName(BLEND_SRC_ALPHA)"),
            GetPName::BLEND_SRC_RGB => write!(fmt, "GetPName(BLEND_SRC_RGB)"),
            GetPName::COLOR_CLEAR_VALUE => write!(fmt, "GetPName(COLOR_CLEAR_VALUE)"),
            GetPName::COLOR_LOGIC_OP => write!(fmt, "GetPName(COLOR_LOGIC_OP)"),
            GetPName::COLOR_WRITEMASK => write!(fmt, "GetPName(COLOR_WRITEMASK)"),
            GetPName::COMPRESSED_TEXTURE_FORMATS => write!(fmt, "GetPName(COMPRESSED_TEXTURE_FORMATS)"),
            GetPName::CONTEXT_FLAGS => write!(fmt, "GetPName(CONTEXT_FLAGS)"),
            GetPName::CULL_FACE => write!(fmt, "GetPName(CULL_FACE)"),
            GetPName::CULL_FACE_MODE => write!(fmt, "GetPName(CULL_FACE_MODE)"),
            GetPName::CURRENT_PROGRAM => write!(fmt, "GetPName(CURRENT_PROGRAM)"),
            GetPName::DEBUG_GROUP_STACK_DEPTH => write!(fmt, "GetPName(DEBUG_GROUP_STACK_DEPTH)"),
            GetPName::DEPTH_CLEAR_VALUE => write!(fmt, "GetPName(DEPTH_CLEAR_VALUE)"),
            GetPName::DEPTH_FUNC => write!(fmt, "GetPName(DEPTH_FUNC)"),
            GetPName::DEPTH_RANGE => write!(fmt, "GetPName(DEPTH_RANGE)"),
            GetPName::DEPTH_TEST => write!(fmt, "GetPName(DEPTH_TEST)"),
            GetPName::DEPTH_WRITEMASK => write!(fmt, "GetPName(DEPTH_WRITEMASK)"),
            GetPName::DISPATCH_INDIRECT_BUFFER_BINDING => write!(fmt, "GetPName(DISPATCH_INDIRECT_BUFFER_BINDING)"),
            GetPName::DITHER => write!(fmt, "GetPName(DITHER)"),
            GetPName::DOUBLEBUFFER => write!(fmt, "GetPName(DOUBLEBUFFER)"),
            GetPName::DRAW_BUFFER => write!(fmt, "GetPName(DRAW_BUFFER)"),
            GetPName::DRAW_FRAMEBUFFER_BINDING => write!(fmt, "GetPName(DRAW_FRAMEBUFFER_BINDING)"),
            GetPName::ELEMENT_ARRAY_BUFFER_BINDING => write!(fmt, "GetPName(ELEMENT_ARRAY_BUFFER_BINDING)"),
            GetPName::FRAGMENT_SHADER_DERIVATIVE_HINT => write!(fmt, "GetPName(FRAGMENT_SHADER_DERIVATIVE_HINT)"),
            GetPName::FRONT_FACE => write!(fmt, "GetPName(FRONT_FACE)"),
            GetPName::IMPLEMENTATION_COLOR_READ_FORMAT => write!(fmt, "GetPName(IMPLEMENTATION_COLOR_READ_FORMAT)"),
            GetPName::IMPLEMENTATION_COLOR_READ_TYPE => write!(fmt, "GetPName(IMPLEMENTATION_COLOR_READ_TYPE)"),
            GetPName::LAYER_PROVOKING_VERTEX => write!(fmt, "GetPName(LAYER_PROVOKING_VERTEX)"),
            GetPName::LINE_SMOOTH => write!(fmt, "GetPName(LINE_SMOOTH)"),
            GetPName::LINE_SMOOTH_HINT => write!(fmt, "GetPName(LINE_SMOOTH_HINT)"),
            GetPName::LINE_WIDTH => write!(fmt, "GetPName(LINE_WIDTH)"),
            GetPName::LINE_WIDTH_GRANULARITY => write!(fmt, "GetPName(LINE_WIDTH_GRANULARITY)"),
            GetPName::LINE_WIDTH_RANGE => write!(fmt, "GetPName(LINE_WIDTH_RANGE)"),
            GetPName::LOGIC_OP_MODE => write!(fmt, "GetPName(LOGIC_OP_MODE)"),
            GetPName::MAJOR_VERSION => write!(fmt, "GetPName(MAJOR_VERSION)"),
            GetPName::MAX_3D_TEXTURE_SIZE => write!(fmt, "GetPName(MAX_3D_TEXTURE_SIZE)"),
            GetPName::MAX_ARRAY_TEXTURE_LAYERS => write!(fmt, "GetPName(MAX_ARRAY_TEXTURE_LAYERS)"),
            GetPName::MAX_CLIP_DISTANCES => write!(fmt, "GetPName(MAX_CLIP_DISTANCES)"),
            GetPName::MAX_COLOR_TEXTURE_SAMPLES => write!(fmt, "GetPName(MAX_COLOR_TEXTURE_SAMPLES)"),
            GetPName::MAX_COMBINED_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_COMBINED_ATOMIC_COUNTERS)"),
            GetPName::MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS)"),
            GetPName::MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS)"),
            GetPName::MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS)"),
            GetPName::MAX_COMBINED_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_COMBINED_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_COMBINED_TEXTURE_IMAGE_UNITS => write!(fmt, "GetPName(MAX_COMBINED_TEXTURE_IMAGE_UNITS)"),
            GetPName::MAX_COMBINED_UNIFORM_BLOCKS => write!(fmt, "GetPName(MAX_COMBINED_UNIFORM_BLOCKS)"),
            GetPName::MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS)"),
            GetPName::MAX_COMPUTE_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_COMPUTE_ATOMIC_COUNTERS)"),
            GetPName::MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS => write!(fmt, "GetPName(MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS)"),
            GetPName::MAX_COMPUTE_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_COMPUTE_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_COMPUTE_TEXTURE_IMAGE_UNITS => write!(fmt, "GetPName(MAX_COMPUTE_TEXTURE_IMAGE_UNITS)"),
            GetPName::MAX_COMPUTE_UNIFORM_BLOCKS => write!(fmt, "GetPName(MAX_COMPUTE_UNIFORM_BLOCKS)"),
            GetPName::MAX_COMPUTE_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_COMPUTE_UNIFORM_COMPONENTS)"),
            GetPName::MAX_COMPUTE_WORK_GROUP_COUNT => write!(fmt, "GetPName(MAX_COMPUTE_WORK_GROUP_COUNT)"),
            GetPName::MAX_COMPUTE_WORK_GROUP_INVOCATIONS => write!(fmt, "GetPName(MAX_COMPUTE_WORK_GROUP_INVOCATIONS)"),
            GetPName::MAX_COMPUTE_WORK_GROUP_SIZE => write!(fmt, "GetPName(MAX_COMPUTE_WORK_GROUP_SIZE)"),
            GetPName::MAX_CUBE_MAP_TEXTURE_SIZE => write!(fmt, "GetPName(MAX_CUBE_MAP_TEXTURE_SIZE)"),
            GetPName::MAX_DEBUG_GROUP_STACK_DEPTH => write!(fmt, "GetPName(MAX_DEBUG_GROUP_STACK_DEPTH)"),
            GetPName::MAX_DEPTH_TEXTURE_SAMPLES => write!(fmt, "GetPName(MAX_DEPTH_TEXTURE_SAMPLES)"),
            GetPName::MAX_DRAW_BUFFERS => write!(fmt, "GetPName(MAX_DRAW_BUFFERS)"),
            GetPName::MAX_DUAL_SOURCE_DRAW_BUFFERS => write!(fmt, "GetPName(MAX_DUAL_SOURCE_DRAW_BUFFERS)"),
            GetPName::MAX_ELEMENTS_INDICES => write!(fmt, "GetPName(MAX_ELEMENTS_INDICES)"),
            GetPName::MAX_ELEMENTS_VERTICES => write!(fmt, "GetPName(MAX_ELEMENTS_VERTICES)"),
            GetPName::MAX_ELEMENT_INDEX => write!(fmt, "GetPName(MAX_ELEMENT_INDEX)"),
            GetPName::MAX_FRAGMENT_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_FRAGMENT_ATOMIC_COUNTERS)"),
            GetPName::MAX_FRAGMENT_INPUT_COMPONENTS => write!(fmt, "GetPName(MAX_FRAGMENT_INPUT_COMPONENTS)"),
            GetPName::MAX_FRAGMENT_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_FRAGMENT_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_FRAGMENT_UNIFORM_BLOCKS => write!(fmt, "GetPName(MAX_FRAGMENT_UNIFORM_BLOCKS)"),
            GetPName::MAX_FRAGMENT_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_FRAGMENT_UNIFORM_COMPONENTS)"),
            GetPName::MAX_FRAGMENT_UNIFORM_VECTORS => write!(fmt, "GetPName(MAX_FRAGMENT_UNIFORM_VECTORS)"),
            GetPName::MAX_FRAMEBUFFER_HEIGHT => write!(fmt, "GetPName(MAX_FRAMEBUFFER_HEIGHT)"),
            GetPName::MAX_FRAMEBUFFER_LAYERS => write!(fmt, "GetPName(MAX_FRAMEBUFFER_LAYERS)"),
            GetPName::MAX_FRAMEBUFFER_SAMPLES => write!(fmt, "GetPName(MAX_FRAMEBUFFER_SAMPLES)"),
            GetPName::MAX_FRAMEBUFFER_WIDTH => write!(fmt, "GetPName(MAX_FRAMEBUFFER_WIDTH)"),
            GetPName::MAX_GEOMETRY_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_GEOMETRY_ATOMIC_COUNTERS)"),
            GetPName::MAX_GEOMETRY_INPUT_COMPONENTS => write!(fmt, "GetPName(MAX_GEOMETRY_INPUT_COMPONENTS)"),
            GetPName::MAX_GEOMETRY_OUTPUT_COMPONENTS => write!(fmt, "GetPName(MAX_GEOMETRY_OUTPUT_COMPONENTS)"),
            GetPName::MAX_GEOMETRY_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_GEOMETRY_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_GEOMETRY_TEXTURE_IMAGE_UNITS => write!(fmt, "GetPName(MAX_GEOMETRY_TEXTURE_IMAGE_UNITS)"),
            GetPName::MAX_GEOMETRY_UNIFORM_BLOCKS => write!(fmt, "GetPName(MAX_GEOMETRY_UNIFORM_BLOCKS)"),
            GetPName::MAX_GEOMETRY_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_GEOMETRY_UNIFORM_COMPONENTS)"),
            GetPName::MAX_INTEGER_SAMPLES => write!(fmt, "GetPName(MAX_INTEGER_SAMPLES)"),
            GetPName::MAX_LABEL_LENGTH => write!(fmt, "GetPName(MAX_LABEL_LENGTH)"),
            GetPName::MAX_PROGRAM_TEXEL_OFFSET => write!(fmt, "GetPName(MAX_PROGRAM_TEXEL_OFFSET)"),
            GetPName::MAX_RECTANGLE_TEXTURE_SIZE => write!(fmt, "GetPName(MAX_RECTANGLE_TEXTURE_SIZE)"),
            GetPName::MAX_RENDERBUFFER_SIZE => write!(fmt, "GetPName(MAX_RENDERBUFFER_SIZE)"),
            GetPName::MAX_SAMPLE_MASK_WORDS => write!(fmt, "GetPName(MAX_SAMPLE_MASK_WORDS)"),
            GetPName::MAX_SERVER_WAIT_TIMEOUT => write!(fmt, "GetPName(MAX_SERVER_WAIT_TIMEOUT)"),
            GetPName::MAX_SHADER_STORAGE_BUFFER_BINDINGS => write!(fmt, "GetPName(MAX_SHADER_STORAGE_BUFFER_BINDINGS)"),
            GetPName::MAX_TESS_CONTROL_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_TESS_CONTROL_ATOMIC_COUNTERS)"),
            GetPName::MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_TESS_EVALUATION_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_TESS_EVALUATION_ATOMIC_COUNTERS)"),
            GetPName::MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_TEXTURE_BUFFER_SIZE => write!(fmt, "GetPName(MAX_TEXTURE_BUFFER_SIZE)"),
            GetPName::MAX_TEXTURE_IMAGE_UNITS => write!(fmt, "GetPName(MAX_TEXTURE_IMAGE_UNITS)"),
            GetPName::MAX_TEXTURE_LOD_BIAS => write!(fmt, "GetPName(MAX_TEXTURE_LOD_BIAS)"),
            GetPName::MAX_TEXTURE_SIZE => write!(fmt, "GetPName(MAX_TEXTURE_SIZE)"),
            GetPName::MAX_UNIFORM_BLOCK_SIZE => write!(fmt, "GetPName(MAX_UNIFORM_BLOCK_SIZE)"),
            GetPName::MAX_UNIFORM_BUFFER_BINDINGS => write!(fmt, "GetPName(MAX_UNIFORM_BUFFER_BINDINGS)"),
            GetPName::MAX_UNIFORM_LOCATIONS => write!(fmt, "GetPName(MAX_UNIFORM_LOCATIONS)"),
            GetPName::MAX_VARYING_COMPONENTS => write!(fmt, "GetPName(MAX_VARYING_COMPONENTS)"),
            GetPName::MAX_VARYING_FLOATS => write!(fmt, "GetPName(MAX_VARYING_FLOATS)"),
            GetPName::MAX_VARYING_VECTORS => write!(fmt, "GetPName(MAX_VARYING_VECTORS)"),
            GetPName::MAX_VERTEX_ATOMIC_COUNTERS => write!(fmt, "GetPName(MAX_VERTEX_ATOMIC_COUNTERS)"),
            GetPName::MAX_VERTEX_ATTRIBS => write!(fmt, "GetPName(MAX_VERTEX_ATTRIBS)"),
            GetPName::MAX_VERTEX_ATTRIB_BINDINGS => write!(fmt, "GetPName(MAX_VERTEX_ATTRIB_BINDINGS)"),
            GetPName::MAX_VERTEX_ATTRIB_RELATIVE_OFFSET => write!(fmt, "GetPName(MAX_VERTEX_ATTRIB_RELATIVE_OFFSET)"),
            GetPName::MAX_VERTEX_OUTPUT_COMPONENTS => write!(fmt, "GetPName(MAX_VERTEX_OUTPUT_COMPONENTS)"),
            GetPName::MAX_VERTEX_SHADER_STORAGE_BLOCKS => write!(fmt, "GetPName(MAX_VERTEX_SHADER_STORAGE_BLOCKS)"),
            GetPName::MAX_VERTEX_TEXTURE_IMAGE_UNITS => write!(fmt, "GetPName(MAX_VERTEX_TEXTURE_IMAGE_UNITS)"),
            GetPName::MAX_VERTEX_UNIFORM_BLOCKS => write!(fmt, "GetPName(MAX_VERTEX_UNIFORM_BLOCKS)"),
            GetPName::MAX_VERTEX_UNIFORM_COMPONENTS => write!(fmt, "GetPName(MAX_VERTEX_UNIFORM_COMPONENTS)"),
            GetPName::MAX_VERTEX_UNIFORM_VECTORS => write!(fmt, "GetPName(MAX_VERTEX_UNIFORM_VECTORS)"),
            GetPName::MAX_VIEWPORTS => write!(fmt, "GetPName(MAX_VIEWPORTS)"),
            GetPName::MAX_VIEWPORT_DIMS => write!(fmt, "GetPName(MAX_VIEWPORT_DIMS)"),
            GetPName::MINOR_VERSION => write!(fmt, "GetPName(MINOR_VERSION)"),
            GetPName::MIN_MAP_BUFFER_ALIGNMENT => write!(fmt, "GetPName(MIN_MAP_BUFFER_ALIGNMENT)"),
            GetPName::MIN_PROGRAM_TEXEL_OFFSET => write!(fmt, "GetPName(MIN_PROGRAM_TEXEL_OFFSET)"),
            GetPName::NUM_COMPRESSED_TEXTURE_FORMATS => write!(fmt, "GetPName(NUM_COMPRESSED_TEXTURE_FORMATS)"),
            GetPName::NUM_EXTENSIONS => write!(fmt, "GetPName(NUM_EXTENSIONS)"),
            GetPName::NUM_PROGRAM_BINARY_FORMATS => write!(fmt, "GetPName(NUM_PROGRAM_BINARY_FORMATS)"),
            GetPName::NUM_SHADER_BINARY_FORMATS => write!(fmt, "GetPName(NUM_SHADER_BINARY_FORMATS)"),
            GetPName::PACK_ALIGNMENT => write!(fmt, "GetPName(PACK_ALIGNMENT)"),
            GetPName::PACK_IMAGE_HEIGHT => write!(fmt, "GetPName(PACK_IMAGE_HEIGHT)"),
            GetPName::PACK_LSB_FIRST => write!(fmt, "GetPName(PACK_LSB_FIRST)"),
            GetPName::PACK_ROW_LENGTH => write!(fmt, "GetPName(PACK_ROW_LENGTH)"),
            GetPName::PACK_SKIP_IMAGES => write!(fmt, "GetPName(PACK_SKIP_IMAGES)"),
            GetPName::PACK_SKIP_PIXELS => write!(fmt, "GetPName(PACK_SKIP_PIXELS)"),
            GetPName::PACK_SKIP_ROWS => write!(fmt, "GetPName(PACK_SKIP_ROWS)"),
            GetPName::PACK_SWAP_BYTES => write!(fmt, "GetPName(PACK_SWAP_BYTES)"),
            GetPName::PIXEL_PACK_BUFFER_BINDING => write!(fmt, "GetPName(PIXEL_PACK_BUFFER_BINDING)"),
            GetPName::PIXEL_UNPACK_BUFFER_BINDING => write!(fmt, "GetPName(PIXEL_UNPACK_BUFFER_BINDING)"),
            GetPName::POINT_FADE_THRESHOLD_SIZE => write!(fmt, "GetPName(POINT_FADE_THRESHOLD_SIZE)"),
            GetPName::POINT_SIZE => write!(fmt, "GetPName(POINT_SIZE)"),
            GetPName::POINT_SIZE_GRANULARITY => write!(fmt, "GetPName(POINT_SIZE_GRANULARITY)"),
            GetPName::POINT_SIZE_RANGE => write!(fmt, "GetPName(POINT_SIZE_RANGE)"),
            GetPName::POLYGON_MODE => write!(fmt, "GetPName(POLYGON_MODE)"),
            GetPName::POLYGON_OFFSET_FACTOR => write!(fmt, "GetPName(POLYGON_OFFSET_FACTOR)"),
            GetPName::POLYGON_OFFSET_FILL => write!(fmt, "GetPName(POLYGON_OFFSET_FILL)"),
            GetPName::POLYGON_OFFSET_LINE => write!(fmt, "GetPName(POLYGON_OFFSET_LINE)"),
            GetPName::POLYGON_OFFSET_POINT => write!(fmt, "GetPName(POLYGON_OFFSET_POINT)"),
            GetPName::POLYGON_OFFSET_UNITS => write!(fmt, "GetPName(POLYGON_OFFSET_UNITS)"),
            GetPName::POLYGON_SMOOTH => write!(fmt, "GetPName(POLYGON_SMOOTH)"),
            GetPName::POLYGON_SMOOTH_HINT => write!(fmt, "GetPName(POLYGON_SMOOTH_HINT)"),
            GetPName::PRIMITIVE_RESTART_INDEX => write!(fmt, "GetPName(PRIMITIVE_RESTART_INDEX)"),
            GetPName::PROGRAM_BINARY_FORMATS => write!(fmt, "GetPName(PROGRAM_BINARY_FORMATS)"),
            GetPName::PROGRAM_PIPELINE_BINDING => write!(fmt, "GetPName(PROGRAM_PIPELINE_BINDING)"),
            GetPName::PROGRAM_POINT_SIZE => write!(fmt, "GetPName(PROGRAM_POINT_SIZE)"),
            GetPName::PROVOKING_VERTEX => write!(fmt, "GetPName(PROVOKING_VERTEX)"),
            GetPName::READ_BUFFER => write!(fmt, "GetPName(READ_BUFFER)"),
            GetPName::READ_FRAMEBUFFER_BINDING => write!(fmt, "GetPName(READ_FRAMEBUFFER_BINDING)"),
            GetPName::RENDERBUFFER_BINDING => write!(fmt, "GetPName(RENDERBUFFER_BINDING)"),
            GetPName::SAMPLER_BINDING => write!(fmt, "GetPName(SAMPLER_BINDING)"),
            GetPName::SAMPLES => write!(fmt, "GetPName(SAMPLES)"),
            GetPName::SAMPLE_BUFFERS => write!(fmt, "GetPName(SAMPLE_BUFFERS)"),
            GetPName::SAMPLE_COVERAGE_INVERT => write!(fmt, "GetPName(SAMPLE_COVERAGE_INVERT)"),
            GetPName::SAMPLE_COVERAGE_VALUE => write!(fmt, "GetPName(SAMPLE_COVERAGE_VALUE)"),
            GetPName::SCISSOR_BOX => write!(fmt, "GetPName(SCISSOR_BOX)"),
            GetPName::SCISSOR_TEST => write!(fmt, "GetPName(SCISSOR_TEST)"),
            GetPName::SHADER_COMPILER => write!(fmt, "GetPName(SHADER_COMPILER)"),
            GetPName::SHADER_STORAGE_BUFFER_BINDING => write!(fmt, "GetPName(SHADER_STORAGE_BUFFER_BINDING)"),
            GetPName::SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT => write!(fmt, "GetPName(SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT)"),
            GetPName::SHADER_STORAGE_BUFFER_SIZE => write!(fmt, "GetPName(SHADER_STORAGE_BUFFER_SIZE)"),
            GetPName::SHADER_STORAGE_BUFFER_START => write!(fmt, "GetPName(SHADER_STORAGE_BUFFER_START)"),
            GetPName::SMOOTH_LINE_WIDTH_GRANULARITY => write!(fmt, "GetPName(SMOOTH_LINE_WIDTH_GRANULARITY)"),
            GetPName::SMOOTH_LINE_WIDTH_RANGE => write!(fmt, "GetPName(SMOOTH_LINE_WIDTH_RANGE)"),
            GetPName::SMOOTH_POINT_SIZE_GRANULARITY => write!(fmt, "GetPName(SMOOTH_POINT_SIZE_GRANULARITY)"),
            GetPName::SMOOTH_POINT_SIZE_RANGE => write!(fmt, "GetPName(SMOOTH_POINT_SIZE_RANGE)"),
            GetPName::STENCIL_BACK_FAIL => write!(fmt, "GetPName(STENCIL_BACK_FAIL)"),
            GetPName::STENCIL_BACK_FUNC => write!(fmt, "GetPName(STENCIL_BACK_FUNC)"),
            GetPName::STENCIL_BACK_PASS_DEPTH_FAIL => write!(fmt, "GetPName(STENCIL_BACK_PASS_DEPTH_FAIL)"),
            GetPName::STENCIL_BACK_PASS_DEPTH_PASS => write!(fmt, "GetPName(STENCIL_BACK_PASS_DEPTH_PASS)"),
            GetPName::STENCIL_BACK_REF => write!(fmt, "GetPName(STENCIL_BACK_REF)"),
            GetPName::STENCIL_BACK_VALUE_MASK => write!(fmt, "GetPName(STENCIL_BACK_VALUE_MASK)"),
            GetPName::STENCIL_BACK_WRITEMASK => write!(fmt, "GetPName(STENCIL_BACK_WRITEMASK)"),
            GetPName::STENCIL_CLEAR_VALUE => write!(fmt, "GetPName(STENCIL_CLEAR_VALUE)"),
            GetPName::STENCIL_FAIL => write!(fmt, "GetPName(STENCIL_FAIL)"),
            GetPName::STENCIL_FUNC => write!(fmt, "GetPName(STENCIL_FUNC)"),
            GetPName::STENCIL_PASS_DEPTH_FAIL => write!(fmt, "GetPName(STENCIL_PASS_DEPTH_FAIL)"),
            GetPName::STENCIL_PASS_DEPTH_PASS => write!(fmt, "GetPName(STENCIL_PASS_DEPTH_PASS)"),
            GetPName::STENCIL_REF => write!(fmt, "GetPName(STENCIL_REF)"),
            GetPName::STENCIL_TEST => write!(fmt, "GetPName(STENCIL_TEST)"),
            GetPName::STENCIL_VALUE_MASK => write!(fmt, "GetPName(STENCIL_VALUE_MASK)"),
            GetPName::STENCIL_WRITEMASK => write!(fmt, "GetPName(STENCIL_WRITEMASK)"),
            GetPName::STEREO => write!(fmt, "GetPName(STEREO)"),
            GetPName::SUBPIXEL_BITS => write!(fmt, "GetPName(SUBPIXEL_BITS)"),
            GetPName::TEXTURE_1D => write!(fmt, "GetPName(TEXTURE_1D)"),
            GetPName::TEXTURE_2D => write!(fmt, "GetPName(TEXTURE_2D)"),
            GetPName::TEXTURE_BINDING_1D => write!(fmt, "GetPName(TEXTURE_BINDING_1D)"),
            GetPName::TEXTURE_BINDING_1D_ARRAY => write!(fmt, "GetPName(TEXTURE_BINDING_1D_ARRAY)"),
            GetPName::TEXTURE_BINDING_2D => write!(fmt, "GetPName(TEXTURE_BINDING_2D)"),
            GetPName::TEXTURE_BINDING_2D_ARRAY => write!(fmt, "GetPName(TEXTURE_BINDING_2D_ARRAY)"),
            GetPName::TEXTURE_BINDING_2D_MULTISAMPLE => write!(fmt, "GetPName(TEXTURE_BINDING_2D_MULTISAMPLE)"),
            GetPName::TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY => write!(fmt, "GetPName(TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY)"),
            GetPName::TEXTURE_BINDING_3D => write!(fmt, "GetPName(TEXTURE_BINDING_3D)"),
            GetPName::TEXTURE_BINDING_BUFFER => write!(fmt, "GetPName(TEXTURE_BINDING_BUFFER)"),
            GetPName::TEXTURE_BINDING_CUBE_MAP => write!(fmt, "GetPName(TEXTURE_BINDING_CUBE_MAP)"),
            GetPName::TEXTURE_BINDING_RECTANGLE => write!(fmt, "GetPName(TEXTURE_BINDING_RECTANGLE)"),
            GetPName::TEXTURE_BUFFER_OFFSET_ALIGNMENT => write!(fmt, "GetPName(TEXTURE_BUFFER_OFFSET_ALIGNMENT)"),
            GetPName::TEXTURE_COMPRESSION_HINT => write!(fmt, "GetPName(TEXTURE_COMPRESSION_HINT)"),
            GetPName::TIMESTAMP => write!(fmt, "GetPName(TIMESTAMP)"),
            GetPName::TRANSFORM_FEEDBACK_BUFFER_BINDING => write!(fmt, "GetPName(TRANSFORM_FEEDBACK_BUFFER_BINDING)"),
            GetPName::TRANSFORM_FEEDBACK_BUFFER_SIZE => write!(fmt, "GetPName(TRANSFORM_FEEDBACK_BUFFER_SIZE)"),
            GetPName::TRANSFORM_FEEDBACK_BUFFER_START => write!(fmt, "GetPName(TRANSFORM_FEEDBACK_BUFFER_START)"),
            GetPName::UNIFORM_BUFFER_BINDING => write!(fmt, "GetPName(UNIFORM_BUFFER_BINDING)"),
            GetPName::UNIFORM_BUFFER_OFFSET_ALIGNMENT => write!(fmt, "GetPName(UNIFORM_BUFFER_OFFSET_ALIGNMENT)"),
            GetPName::UNIFORM_BUFFER_SIZE => write!(fmt, "GetPName(UNIFORM_BUFFER_SIZE)"),
            GetPName::UNIFORM_BUFFER_START => write!(fmt, "GetPName(UNIFORM_BUFFER_START)"),
            GetPName::UNPACK_ALIGNMENT => write!(fmt, "GetPName(UNPACK_ALIGNMENT)"),
            GetPName::UNPACK_IMAGE_HEIGHT => write!(fmt, "GetPName(UNPACK_IMAGE_HEIGHT)"),
            GetPName::UNPACK_LSB_FIRST => write!(fmt, "GetPName(UNPACK_LSB_FIRST)"),
            GetPName::UNPACK_ROW_LENGTH => write!(fmt, "GetPName(UNPACK_ROW_LENGTH)"),
            GetPName::UNPACK_SKIP_IMAGES => write!(fmt, "GetPName(UNPACK_SKIP_IMAGES)"),
            GetPName::UNPACK_SKIP_PIXELS => write!(fmt, "GetPName(UNPACK_SKIP_PIXELS)"),
            GetPName::UNPACK_SKIP_ROWS => write!(fmt, "GetPName(UNPACK_SKIP_ROWS)"),
            GetPName::UNPACK_SWAP_BYTES => write!(fmt, "GetPName(UNPACK_SWAP_BYTES)"),
            GetPName::VERTEX_ARRAY => write!(fmt, "GetPName(VERTEX_ARRAY)"),
            GetPName::VERTEX_ARRAY_BINDING => write!(fmt, "GetPName(VERTEX_ARRAY_BINDING)"),
            GetPName::VERTEX_BINDING_DIVISOR => write!(fmt, "GetPName(VERTEX_BINDING_DIVISOR)"),
            GetPName::VERTEX_BINDING_OFFSET => write!(fmt, "GetPName(VERTEX_BINDING_OFFSET)"),
            GetPName::VERTEX_BINDING_STRIDE => write!(fmt, "GetPName(VERTEX_BINDING_STRIDE)"),
            GetPName::VIEWPORT => write!(fmt, "GetPName(VIEWPORT)"),
            GetPName::VIEWPORT_BOUNDS_RANGE => write!(fmt, "GetPName(VIEWPORT_BOUNDS_RANGE)"),
            GetPName::VIEWPORT_INDEX_PROVOKING_VERTEX => write!(fmt, "GetPName(VIEWPORT_INDEX_PROVOKING_VERTEX)"),
            GetPName::VIEWPORT_SUBPIXEL_BITS => write!(fmt, "GetPName(VIEWPORT_SUBPIXEL_BITS)"),
            _ => write!(fmt, "GetPName({})", self.0),
        }
    }
}

impl_enum_traits!(GetPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetPixelMap(pub types::GLenum);

impl GetPixelMap {
}

impl ::std::fmt::Debug for GetPixelMap {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "GetPixelMap({})", self.0),
        }
    }
}

impl_enum_traits!(GetPixelMap);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetPointervPName(pub types::GLenum);

impl GetPointervPName {
    pub const DEBUG_CALLBACK_FUNCTION: GetPointervPName = GetPointervPName(super::DEBUG_CALLBACK_FUNCTION);
    pub const DEBUG_CALLBACK_USER_PARAM: GetPointervPName = GetPointervPName(super::DEBUG_CALLBACK_USER_PARAM);
}

impl ::std::fmt::Debug for GetPointervPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GetPointervPName::DEBUG_CALLBACK_FUNCTION => write!(fmt, "GetPointervPName(DEBUG_CALLBACK_FUNCTION)"),
            GetPointervPName::DEBUG_CALLBACK_USER_PARAM => write!(fmt, "GetPointervPName(DEBUG_CALLBACK_USER_PARAM)"),
            _ => write!(fmt, "GetPointervPName({})", self.0),
        }
    }
}

impl_enum_traits!(GetPointervPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GetTextureParameter(pub types::GLenum);

impl GetTextureParameter {
    pub const TEXTURE_ALPHA_SIZE: GetTextureParameter = GetTextureParameter(super::TEXTURE_ALPHA_SIZE);
    pub const TEXTURE_BLUE_SIZE: GetTextureParameter = GetTextureParameter(super::TEXTURE_BLUE_SIZE);
    pub const TEXTURE_BORDER_COLOR: GetTextureParameter = GetTextureParameter(super::TEXTURE_BORDER_COLOR);
    pub const TEXTURE_GREEN_SIZE: GetTextureParameter = GetTextureParameter(super::TEXTURE_GREEN_SIZE);
    pub const TEXTURE_HEIGHT: GetTextureParameter = GetTextureParameter(super::TEXTURE_HEIGHT);
    pub const TEXTURE_INTERNAL_FORMAT: GetTextureParameter = GetTextureParameter(super::TEXTURE_INTERNAL_FORMAT);
    pub const TEXTURE_MAG_FILTER: GetTextureParameter = GetTextureParameter(super::TEXTURE_MAG_FILTER);
    pub const TEXTURE_MIN_FILTER: GetTextureParameter = GetTextureParameter(super::TEXTURE_MIN_FILTER);
    pub const TEXTURE_RED_SIZE: GetTextureParameter = GetTextureParameter(super::TEXTURE_RED_SIZE);
    pub const TEXTURE_WIDTH: GetTextureParameter = GetTextureParameter(super::TEXTURE_WIDTH);
    pub const TEXTURE_WRAP_S: GetTextureParameter = GetTextureParameter(super::TEXTURE_WRAP_S);
    pub const TEXTURE_WRAP_T: GetTextureParameter = GetTextureParameter(super::TEXTURE_WRAP_T);
}

impl ::std::fmt::Debug for GetTextureParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GetTextureParameter::TEXTURE_ALPHA_SIZE => write!(fmt, "GetTextureParameter(TEXTURE_ALPHA_SIZE)"),
            GetTextureParameter::TEXTURE_BLUE_SIZE => write!(fmt, "GetTextureParameter(TEXTURE_BLUE_SIZE)"),
            GetTextureParameter::TEXTURE_BORDER_COLOR => write!(fmt, "GetTextureParameter(TEXTURE_BORDER_COLOR)"),
            GetTextureParameter::TEXTURE_GREEN_SIZE => write!(fmt, "GetTextureParameter(TEXTURE_GREEN_SIZE)"),
            GetTextureParameter::TEXTURE_HEIGHT => write!(fmt, "GetTextureParameter(TEXTURE_HEIGHT)"),
            GetTextureParameter::TEXTURE_INTERNAL_FORMAT => write!(fmt, "GetTextureParameter(TEXTURE_INTERNAL_FORMAT)"),
            GetTextureParameter::TEXTURE_MAG_FILTER => write!(fmt, "GetTextureParameter(TEXTURE_MAG_FILTER)"),
            GetTextureParameter::TEXTURE_MIN_FILTER => write!(fmt, "GetTextureParameter(TEXTURE_MIN_FILTER)"),
            GetTextureParameter::TEXTURE_RED_SIZE => write!(fmt, "GetTextureParameter(TEXTURE_RED_SIZE)"),
            GetTextureParameter::TEXTURE_WIDTH => write!(fmt, "GetTextureParameter(TEXTURE_WIDTH)"),
            GetTextureParameter::TEXTURE_WRAP_S => write!(fmt, "GetTextureParameter(TEXTURE_WRAP_S)"),
            GetTextureParameter::TEXTURE_WRAP_T => write!(fmt, "GetTextureParameter(TEXTURE_WRAP_T)"),
            _ => write!(fmt, "GetTextureParameter({})", self.0),
        }
    }
}

impl_enum_traits!(GetTextureParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct GraphicsResetStatus(pub types::GLenum);

impl GraphicsResetStatus {
    pub const NO_ERROR: GraphicsResetStatus = GraphicsResetStatus(super::NO_ERROR);
    pub const GUILTY_CONTEXT_RESET: GraphicsResetStatus = GraphicsResetStatus(super::GUILTY_CONTEXT_RESET);
    pub const INNOCENT_CONTEXT_RESET: GraphicsResetStatus = GraphicsResetStatus(super::INNOCENT_CONTEXT_RESET);
    pub const UNKNOWN_CONTEXT_RESET: GraphicsResetStatus = GraphicsResetStatus(super::UNKNOWN_CONTEXT_RESET);
}

impl ::std::fmt::Debug for GraphicsResetStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GraphicsResetStatus::GUILTY_CONTEXT_RESET => write!(fmt, "GraphicsResetStatus(GUILTY_CONTEXT_RESET)"),
            GraphicsResetStatus::INNOCENT_CONTEXT_RESET => write!(fmt, "GraphicsResetStatus(INNOCENT_CONTEXT_RESET)"),
            GraphicsResetStatus::NO_ERROR => write!(fmt, "GraphicsResetStatus(NO_ERROR)"),
            GraphicsResetStatus::UNKNOWN_CONTEXT_RESET => write!(fmt, "GraphicsResetStatus(UNKNOWN_CONTEXT_RESET)"),
            _ => write!(fmt, "GraphicsResetStatus({})", self.0),
        }
    }
}

impl_enum_traits!(GraphicsResetStatus);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct HintMode(pub types::GLenum);

impl HintMode {
    pub const DONT_CARE: HintMode = HintMode(super::DONT_CARE);
    pub const FASTEST: HintMode = HintMode(super::FASTEST);
    pub const NICEST: HintMode = HintMode(super::NICEST);
}

impl ::std::fmt::Debug for HintMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HintMode::DONT_CARE => write!(fmt, "HintMode(DONT_CARE)"),
            HintMode::FASTEST => write!(fmt, "HintMode(FASTEST)"),
            HintMode::NICEST => write!(fmt, "HintMode(NICEST)"),
            _ => write!(fmt, "HintMode({})", self.0),
        }
    }
}

impl_enum_traits!(HintMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct HintTarget(pub types::GLenum);

impl HintTarget {
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: HintTarget = HintTarget(super::FRAGMENT_SHADER_DERIVATIVE_HINT);
    pub const LINE_SMOOTH_HINT: HintTarget = HintTarget(super::LINE_SMOOTH_HINT);
    pub const POLYGON_SMOOTH_HINT: HintTarget = HintTarget(super::POLYGON_SMOOTH_HINT);
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: HintTarget = HintTarget(super::PROGRAM_BINARY_RETRIEVABLE_HINT);
    pub const TEXTURE_COMPRESSION_HINT: HintTarget = HintTarget(super::TEXTURE_COMPRESSION_HINT);
}

impl ::std::fmt::Debug for HintTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HintTarget::FRAGMENT_SHADER_DERIVATIVE_HINT => write!(fmt, "HintTarget(FRAGMENT_SHADER_DERIVATIVE_HINT)"),
            HintTarget::LINE_SMOOTH_HINT => write!(fmt, "HintTarget(LINE_SMOOTH_HINT)"),
            HintTarget::POLYGON_SMOOTH_HINT => write!(fmt, "HintTarget(POLYGON_SMOOTH_HINT)"),
            HintTarget::PROGRAM_BINARY_RETRIEVABLE_HINT => write!(fmt, "HintTarget(PROGRAM_BINARY_RETRIEVABLE_HINT)"),
            HintTarget::TEXTURE_COMPRESSION_HINT => write!(fmt, "HintTarget(TEXTURE_COMPRESSION_HINT)"),
            _ => write!(fmt, "HintTarget({})", self.0),
        }
    }
}

impl_enum_traits!(HintTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct HistogramTargetEXT(pub types::GLenum);

impl HistogramTargetEXT {
}

impl ::std::fmt::Debug for HistogramTargetEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "HistogramTargetEXT({})", self.0),
        }
    }
}

impl_enum_traits!(HistogramTargetEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct IndexPointerType(pub types::GLenum);

impl IndexPointerType {
    pub const DOUBLE: IndexPointerType = IndexPointerType(super::DOUBLE);
    pub const FLOAT: IndexPointerType = IndexPointerType(super::FLOAT);
    pub const INT: IndexPointerType = IndexPointerType(super::INT);
    pub const SHORT: IndexPointerType = IndexPointerType(super::SHORT);
}

impl ::std::fmt::Debug for IndexPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            IndexPointerType::DOUBLE => write!(fmt, "IndexPointerType(DOUBLE)"),
            IndexPointerType::FLOAT => write!(fmt, "IndexPointerType(FLOAT)"),
            IndexPointerType::INT => write!(fmt, "IndexPointerType(INT)"),
            IndexPointerType::SHORT => write!(fmt, "IndexPointerType(SHORT)"),
            _ => write!(fmt, "IndexPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(IndexPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct InterleavedArrayFormat(pub types::GLenum);

impl InterleavedArrayFormat {
}

impl ::std::fmt::Debug for InterleavedArrayFormat {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "InterleavedArrayFormat({})", self.0),
        }
    }
}

impl_enum_traits!(InterleavedArrayFormat);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct InternalFormat(pub types::GLenum);

impl InternalFormat {
    pub const RED: InternalFormat = InternalFormat(super::RED);
    pub const R8: InternalFormat = InternalFormat(super::R8);
    pub const R8_SNORM: InternalFormat = InternalFormat(super::R8_SNORM);
    pub const R16: InternalFormat = InternalFormat(super::R16);
    pub const R16_SNORM: InternalFormat = InternalFormat(super::R16_SNORM);
    pub const R16F: InternalFormat = InternalFormat(super::R16F);
    pub const R32F: InternalFormat = InternalFormat(super::R32F);
    pub const R8I: InternalFormat = InternalFormat(super::R8I);
    pub const R16I: InternalFormat = InternalFormat(super::R16I);
    pub const R32I: InternalFormat = InternalFormat(super::R32I);
    pub const R8UI: InternalFormat = InternalFormat(super::R8UI);
    pub const R16UI: InternalFormat = InternalFormat(super::R16UI);
    pub const R32UI: InternalFormat = InternalFormat(super::R32UI);
    pub const RG: InternalFormat = InternalFormat(super::RG);
    pub const RG8: InternalFormat = InternalFormat(super::RG8);
    pub const RG8_SNORM: InternalFormat = InternalFormat(super::RG8_SNORM);
    pub const RG16: InternalFormat = InternalFormat(super::RG16);
    pub const RG16_SNORM: InternalFormat = InternalFormat(super::RG16_SNORM);
    pub const RG16F: InternalFormat = InternalFormat(super::RG16F);
    pub const RG32F: InternalFormat = InternalFormat(super::RG32F);
    pub const RG8I: InternalFormat = InternalFormat(super::RG8I);
    pub const RG16I: InternalFormat = InternalFormat(super::RG16I);
    pub const RG32I: InternalFormat = InternalFormat(super::RG32I);
    pub const RG8UI: InternalFormat = InternalFormat(super::RG8UI);
    pub const RG16UI: InternalFormat = InternalFormat(super::RG16UI);
    pub const RG32UI: InternalFormat = InternalFormat(super::RG32UI);
    pub const RGB: InternalFormat = InternalFormat(super::RGB);
    pub const RGB4: InternalFormat = InternalFormat(super::RGB4);
    pub const RGB5: InternalFormat = InternalFormat(super::RGB5);
    pub const RGB8: InternalFormat = InternalFormat(super::RGB8);
    pub const RGB8_SNORM: InternalFormat = InternalFormat(super::RGB8_SNORM);
    pub const RGB10: InternalFormat = InternalFormat(super::RGB10);
    pub const RGB12: InternalFormat = InternalFormat(super::RGB12);
    pub const RGB16: InternalFormat = InternalFormat(super::RGB16);
    pub const RGB16F: InternalFormat = InternalFormat(super::RGB16F);
    pub const RGB16_SNORM: InternalFormat = InternalFormat(super::RGB16_SNORM);
    pub const RGB8I: InternalFormat = InternalFormat(super::RGB8I);
    pub const RGB16I: InternalFormat = InternalFormat(super::RGB16I);
    pub const RGB32I: InternalFormat = InternalFormat(super::RGB32I);
    pub const RGB8UI: InternalFormat = InternalFormat(super::RGB8UI);
    pub const RGB16UI: InternalFormat = InternalFormat(super::RGB16UI);
    pub const RGB32UI: InternalFormat = InternalFormat(super::RGB32UI);
    pub const SRGB: InternalFormat = InternalFormat(super::SRGB);
    pub const SRGB_ALPHA: InternalFormat = InternalFormat(super::SRGB_ALPHA);
    pub const SRGB8: InternalFormat = InternalFormat(super::SRGB8);
    pub const SRGB8_ALPHA8: InternalFormat = InternalFormat(super::SRGB8_ALPHA8);
    pub const R3_G3_B2: InternalFormat = InternalFormat(super::R3_G3_B2);
    pub const R11F_G11F_B10F: InternalFormat = InternalFormat(super::R11F_G11F_B10F);
    pub const RGB9_E5: InternalFormat = InternalFormat(super::RGB9_E5);
    pub const RGBA: InternalFormat = InternalFormat(super::RGBA);
    pub const RGBA4: InternalFormat = InternalFormat(super::RGBA4);
    pub const RGB5_A1: InternalFormat = InternalFormat(super::RGB5_A1);
    pub const RGBA8: InternalFormat = InternalFormat(super::RGBA8);
    pub const RGBA8_SNORM: InternalFormat = InternalFormat(super::RGBA8_SNORM);
    pub const RGB10_A2: InternalFormat = InternalFormat(super::RGB10_A2);
    pub const RGBA12: InternalFormat = InternalFormat(super::RGBA12);
    pub const RGBA16: InternalFormat = InternalFormat(super::RGBA16);
    pub const RGBA16F: InternalFormat = InternalFormat(super::RGBA16F);
    pub const RGBA32F: InternalFormat = InternalFormat(super::RGBA32F);
    pub const RGBA8I: InternalFormat = InternalFormat(super::RGBA8I);
    pub const RGBA16I: InternalFormat = InternalFormat(super::RGBA16I);
    pub const RGBA32I: InternalFormat = InternalFormat(super::RGBA32I);
    pub const RGBA8UI: InternalFormat = InternalFormat(super::RGBA8UI);
    pub const RGBA16UI: InternalFormat = InternalFormat(super::RGBA16UI);
    pub const RGBA32UI: InternalFormat = InternalFormat(super::RGBA32UI);
    pub const RGB10_A2UI: InternalFormat = InternalFormat(super::RGB10_A2UI);
    pub const DEPTH_COMPONENT: InternalFormat = InternalFormat(super::DEPTH_COMPONENT);
    pub const DEPTH_COMPONENT16: InternalFormat = InternalFormat(super::DEPTH_COMPONENT16);
    pub const DEPTH_COMPONENT32F: InternalFormat = InternalFormat(super::DEPTH_COMPONENT32F);
    pub const DEPTH_STENCIL: InternalFormat = InternalFormat(super::DEPTH_STENCIL);
    pub const DEPTH24_STENCIL8: InternalFormat = InternalFormat(super::DEPTH24_STENCIL8);
    pub const DEPTH32F_STENCIL8: InternalFormat = InternalFormat(super::DEPTH32F_STENCIL8);
    pub const COMPRESSED_RED: InternalFormat = InternalFormat(super::COMPRESSED_RED);
    pub const COMPRESSED_RG: InternalFormat = InternalFormat(super::COMPRESSED_RG);
    pub const COMPRESSED_RGB: InternalFormat = InternalFormat(super::COMPRESSED_RGB);
    pub const COMPRESSED_RGBA: InternalFormat = InternalFormat(super::COMPRESSED_RGBA);
    pub const COMPRESSED_SRGB: InternalFormat = InternalFormat(super::COMPRESSED_SRGB);
    pub const COMPRESSED_SRGB_ALPHA: InternalFormat = InternalFormat(super::COMPRESSED_SRGB_ALPHA);
    pub const COMPRESSED_RED_RGTC1: InternalFormat = InternalFormat(super::COMPRESSED_RED_RGTC1);
    pub const COMPRESSED_SIGNED_RED_RGTC1: InternalFormat = InternalFormat(super::COMPRESSED_SIGNED_RED_RGTC1);
    pub const COMPRESSED_R11_EAC: InternalFormat = InternalFormat(super::COMPRESSED_R11_EAC);
    pub const COMPRESSED_SIGNED_R11_EAC: InternalFormat = InternalFormat(super::COMPRESSED_SIGNED_R11_EAC);
    pub const COMPRESSED_RG_RGTC2: InternalFormat = InternalFormat(super::COMPRESSED_RG_RGTC2);
    pub const COMPRESSED_SIGNED_RG_RGTC2: InternalFormat = InternalFormat(super::COMPRESSED_SIGNED_RG_RGTC2);
    pub const COMPRESSED_RGBA_BPTC_UNORM: InternalFormat = InternalFormat(super::COMPRESSED_RGBA_BPTC_UNORM);
    pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: InternalFormat = InternalFormat(super::COMPRESSED_SRGB_ALPHA_BPTC_UNORM);
    pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: InternalFormat = InternalFormat(super::COMPRESSED_RGB_BPTC_SIGNED_FLOAT);
    pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: InternalFormat = InternalFormat(super::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT);
    pub const COMPRESSED_RGB8_ETC2: InternalFormat = InternalFormat(super::COMPRESSED_RGB8_ETC2);
    pub const COMPRESSED_SRGB8_ETC2: InternalFormat = InternalFormat(super::COMPRESSED_SRGB8_ETC2);
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: InternalFormat = InternalFormat(super::COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2);
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: InternalFormat = InternalFormat(super::COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2);
    pub const COMPRESSED_RGBA8_ETC2_EAC: InternalFormat = InternalFormat(super::COMPRESSED_RGBA8_ETC2_EAC);
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: InternalFormat = InternalFormat(super::COMPRESSED_SRGB8_ALPHA8_ETC2_EAC);
    pub const COMPRESSED_RG11_EAC: InternalFormat = InternalFormat(super::COMPRESSED_RG11_EAC);
    pub const COMPRESSED_SIGNED_RG11_EAC: InternalFormat = InternalFormat(super::COMPRESSED_SIGNED_RG11_EAC);
}

impl ::std::fmt::Debug for InternalFormat {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            InternalFormat::COMPRESSED_R11_EAC => write!(fmt, "InternalFormat(COMPRESSED_R11_EAC)"),
            InternalFormat::COMPRESSED_RED => write!(fmt, "InternalFormat(COMPRESSED_RED)"),
            InternalFormat::COMPRESSED_RED_RGTC1 => write!(fmt, "InternalFormat(COMPRESSED_RED_RGTC1)"),
            InternalFormat::COMPRESSED_RG => write!(fmt, "InternalFormat(COMPRESSED_RG)"),
            InternalFormat::COMPRESSED_RG11_EAC => write!(fmt, "InternalFormat(COMPRESSED_RG11_EAC)"),
            InternalFormat::COMPRESSED_RGB => write!(fmt, "InternalFormat(COMPRESSED_RGB)"),
            InternalFormat::COMPRESSED_RGB8_ETC2 => write!(fmt, "InternalFormat(COMPRESSED_RGB8_ETC2)"),
            InternalFormat::COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 => write!(fmt, "InternalFormat(COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2)"),
            InternalFormat::COMPRESSED_RGBA => write!(fmt, "InternalFormat(COMPRESSED_RGBA)"),
            InternalFormat::COMPRESSED_RGBA8_ETC2_EAC => write!(fmt, "InternalFormat(COMPRESSED_RGBA8_ETC2_EAC)"),
            InternalFormat::COMPRESSED_RGBA_BPTC_UNORM => write!(fmt, "InternalFormat(COMPRESSED_RGBA_BPTC_UNORM)"),
            InternalFormat::COMPRESSED_RGB_BPTC_SIGNED_FLOAT => write!(fmt, "InternalFormat(COMPRESSED_RGB_BPTC_SIGNED_FLOAT)"),
            InternalFormat::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT => write!(fmt, "InternalFormat(COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT)"),
            InternalFormat::COMPRESSED_RG_RGTC2 => write!(fmt, "InternalFormat(COMPRESSED_RG_RGTC2)"),
            InternalFormat::COMPRESSED_SIGNED_R11_EAC => write!(fmt, "InternalFormat(COMPRESSED_SIGNED_R11_EAC)"),
            InternalFormat::COMPRESSED_SIGNED_RED_RGTC1 => write!(fmt, "InternalFormat(COMPRESSED_SIGNED_RED_RGTC1)"),
            InternalFormat::COMPRESSED_SIGNED_RG11_EAC => write!(fmt, "InternalFormat(COMPRESSED_SIGNED_RG11_EAC)"),
            InternalFormat::COMPRESSED_SIGNED_RG_RGTC2 => write!(fmt, "InternalFormat(COMPRESSED_SIGNED_RG_RGTC2)"),
            InternalFormat::COMPRESSED_SRGB => write!(fmt, "InternalFormat(COMPRESSED_SRGB)"),
            InternalFormat::COMPRESSED_SRGB8_ALPHA8_ETC2_EAC => write!(fmt, "InternalFormat(COMPRESSED_SRGB8_ALPHA8_ETC2_EAC)"),
            InternalFormat::COMPRESSED_SRGB8_ETC2 => write!(fmt, "InternalFormat(COMPRESSED_SRGB8_ETC2)"),
            InternalFormat::COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 => write!(fmt, "InternalFormat(COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2)"),
            InternalFormat::COMPRESSED_SRGB_ALPHA => write!(fmt, "InternalFormat(COMPRESSED_SRGB_ALPHA)"),
            InternalFormat::COMPRESSED_SRGB_ALPHA_BPTC_UNORM => write!(fmt, "InternalFormat(COMPRESSED_SRGB_ALPHA_BPTC_UNORM)"),
            InternalFormat::DEPTH24_STENCIL8 => write!(fmt, "InternalFormat(DEPTH24_STENCIL8)"),
            InternalFormat::DEPTH32F_STENCIL8 => write!(fmt, "InternalFormat(DEPTH32F_STENCIL8)"),
            InternalFormat::DEPTH_COMPONENT => write!(fmt, "InternalFormat(DEPTH_COMPONENT)"),
            InternalFormat::DEPTH_COMPONENT16 => write!(fmt, "InternalFormat(DEPTH_COMPONENT16)"),
            InternalFormat::DEPTH_COMPONENT32F => write!(fmt, "InternalFormat(DEPTH_COMPONENT32F)"),
            InternalFormat::DEPTH_STENCIL => write!(fmt, "InternalFormat(DEPTH_STENCIL)"),
            InternalFormat::R11F_G11F_B10F => write!(fmt, "InternalFormat(R11F_G11F_B10F)"),
            InternalFormat::R16 => write!(fmt, "InternalFormat(R16)"),
            InternalFormat::R16F => write!(fmt, "InternalFormat(R16F)"),
            InternalFormat::R16I => write!(fmt, "InternalFormat(R16I)"),
            InternalFormat::R16UI => write!(fmt, "InternalFormat(R16UI)"),
            InternalFormat::R16_SNORM => write!(fmt, "InternalFormat(R16_SNORM)"),
            InternalFormat::R32F => write!(fmt, "InternalFormat(R32F)"),
            InternalFormat::R32I => write!(fmt, "InternalFormat(R32I)"),
            InternalFormat::R32UI => write!(fmt, "InternalFormat(R32UI)"),
            InternalFormat::R3_G3_B2 => write!(fmt, "InternalFormat(R3_G3_B2)"),
            InternalFormat::R8 => write!(fmt, "InternalFormat(R8)"),
            InternalFormat::R8I => write!(fmt, "InternalFormat(R8I)"),
            InternalFormat::R8UI => write!(fmt, "InternalFormat(R8UI)"),
            InternalFormat::R8_SNORM => write!(fmt, "InternalFormat(R8_SNORM)"),
            InternalFormat::RED => write!(fmt, "InternalFormat(RED)"),
            InternalFormat::RG => write!(fmt, "InternalFormat(RG)"),
            InternalFormat::RG16 => write!(fmt, "InternalFormat(RG16)"),
            InternalFormat::RG16F => write!(fmt, "InternalFormat(RG16F)"),
            InternalFormat::RG16I => write!(fmt, "InternalFormat(RG16I)"),
            InternalFormat::RG16UI => write!(fmt, "InternalFormat(RG16UI)"),
            InternalFormat::RG16_SNORM => write!(fmt, "InternalFormat(RG16_SNORM)"),
            InternalFormat::RG32F => write!(fmt, "InternalFormat(RG32F)"),
            InternalFormat::RG32I => write!(fmt, "InternalFormat(RG32I)"),
            InternalFormat::RG32UI => write!(fmt, "InternalFormat(RG32UI)"),
            InternalFormat::RG8 => write!(fmt, "InternalFormat(RG8)"),
            InternalFormat::RG8I => write!(fmt, "InternalFormat(RG8I)"),
            InternalFormat::RG8UI => write!(fmt, "InternalFormat(RG8UI)"),
            InternalFormat::RG8_SNORM => write!(fmt, "InternalFormat(RG8_SNORM)"),
            InternalFormat::RGB => write!(fmt, "InternalFormat(RGB)"),
            InternalFormat::RGB10 => write!(fmt, "InternalFormat(RGB10)"),
            InternalFormat::RGB10_A2 => write!(fmt, "InternalFormat(RGB10_A2)"),
            InternalFormat::RGB10_A2UI => write!(fmt, "InternalFormat(RGB10_A2UI)"),
            InternalFormat::RGB12 => write!(fmt, "InternalFormat(RGB12)"),
            InternalFormat::RGB16 => write!(fmt, "InternalFormat(RGB16)"),
            InternalFormat::RGB16F => write!(fmt, "InternalFormat(RGB16F)"),
            InternalFormat::RGB16I => write!(fmt, "InternalFormat(RGB16I)"),
            InternalFormat::RGB16UI => write!(fmt, "InternalFormat(RGB16UI)"),
            InternalFormat::RGB16_SNORM => write!(fmt, "InternalFormat(RGB16_SNORM)"),
            InternalFormat::RGB32I => write!(fmt, "InternalFormat(RGB32I)"),
            InternalFormat::RGB32UI => write!(fmt, "InternalFormat(RGB32UI)"),
            InternalFormat::RGB4 => write!(fmt, "InternalFormat(RGB4)"),
            InternalFormat::RGB5 => write!(fmt, "InternalFormat(RGB5)"),
            InternalFormat::RGB5_A1 => write!(fmt, "InternalFormat(RGB5_A1)"),
            InternalFormat::RGB8 => write!(fmt, "InternalFormat(RGB8)"),
            InternalFormat::RGB8I => write!(fmt, "InternalFormat(RGB8I)"),
            InternalFormat::RGB8UI => write!(fmt, "InternalFormat(RGB8UI)"),
            InternalFormat::RGB8_SNORM => write!(fmt, "InternalFormat(RGB8_SNORM)"),
            InternalFormat::RGB9_E5 => write!(fmt, "InternalFormat(RGB9_E5)"),
            InternalFormat::RGBA => write!(fmt, "InternalFormat(RGBA)"),
            InternalFormat::RGBA12 => write!(fmt, "InternalFormat(RGBA12)"),
            InternalFormat::RGBA16 => write!(fmt, "InternalFormat(RGBA16)"),
            InternalFormat::RGBA16F => write!(fmt, "InternalFormat(RGBA16F)"),
            InternalFormat::RGBA16I => write!(fmt, "InternalFormat(RGBA16I)"),
            InternalFormat::RGBA16UI => write!(fmt, "InternalFormat(RGBA16UI)"),
            InternalFormat::RGBA32F => write!(fmt, "InternalFormat(RGBA32F)"),
            InternalFormat::RGBA32I => write!(fmt, "InternalFormat(RGBA32I)"),
            InternalFormat::RGBA32UI => write!(fmt, "InternalFormat(RGBA32UI)"),
            InternalFormat::RGBA4 => write!(fmt, "InternalFormat(RGBA4)"),
            InternalFormat::RGBA8 => write!(fmt, "InternalFormat(RGBA8)"),
            InternalFormat::RGBA8I => write!(fmt, "InternalFormat(RGBA8I)"),
            InternalFormat::RGBA8UI => write!(fmt, "InternalFormat(RGBA8UI)"),
            InternalFormat::RGBA8_SNORM => write!(fmt, "InternalFormat(RGBA8_SNORM)"),
            InternalFormat::SRGB => write!(fmt, "InternalFormat(SRGB)"),
            InternalFormat::SRGB8 => write!(fmt, "InternalFormat(SRGB8)"),
            InternalFormat::SRGB8_ALPHA8 => write!(fmt, "InternalFormat(SRGB8_ALPHA8)"),
            InternalFormat::SRGB_ALPHA => write!(fmt, "InternalFormat(SRGB_ALPHA)"),
            _ => write!(fmt, "InternalFormat({})", self.0),
        }
    }
}

impl_enum_traits!(InternalFormat);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct InternalFormatPName(pub types::GLenum);

impl InternalFormatPName {
    pub const NUM_SAMPLE_COUNTS: InternalFormatPName = InternalFormatPName(super::NUM_SAMPLE_COUNTS);
    pub const SAMPLES: InternalFormatPName = InternalFormatPName(super::SAMPLES);
    pub const INTERNALFORMAT_SUPPORTED: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_SUPPORTED);
    pub const INTERNALFORMAT_PREFERRED: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_PREFERRED);
    pub const INTERNALFORMAT_RED_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_RED_SIZE);
    pub const INTERNALFORMAT_GREEN_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_GREEN_SIZE);
    pub const INTERNALFORMAT_BLUE_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_BLUE_SIZE);
    pub const INTERNALFORMAT_ALPHA_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_ALPHA_SIZE);
    pub const INTERNALFORMAT_DEPTH_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_DEPTH_SIZE);
    pub const INTERNALFORMAT_STENCIL_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_STENCIL_SIZE);
    pub const INTERNALFORMAT_SHARED_SIZE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_SHARED_SIZE);
    pub const INTERNALFORMAT_RED_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_RED_TYPE);
    pub const INTERNALFORMAT_GREEN_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_GREEN_TYPE);
    pub const INTERNALFORMAT_BLUE_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_BLUE_TYPE);
    pub const INTERNALFORMAT_ALPHA_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_ALPHA_TYPE);
    pub const INTERNALFORMAT_DEPTH_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_DEPTH_TYPE);
    pub const INTERNALFORMAT_STENCIL_TYPE: InternalFormatPName = InternalFormatPName(super::INTERNALFORMAT_STENCIL_TYPE);
    pub const MAX_WIDTH: InternalFormatPName = InternalFormatPName(super::MAX_WIDTH);
    pub const MAX_HEIGHT: InternalFormatPName = InternalFormatPName(super::MAX_HEIGHT);
    pub const MAX_DEPTH: InternalFormatPName = InternalFormatPName(super::MAX_DEPTH);
    pub const MAX_LAYERS: InternalFormatPName = InternalFormatPName(super::MAX_LAYERS);
    pub const COLOR_COMPONENTS: InternalFormatPName = InternalFormatPName(super::COLOR_COMPONENTS);
    pub const COLOR_RENDERABLE: InternalFormatPName = InternalFormatPName(super::COLOR_RENDERABLE);
    pub const DEPTH_RENDERABLE: InternalFormatPName = InternalFormatPName(super::DEPTH_RENDERABLE);
    pub const STENCIL_RENDERABLE: InternalFormatPName = InternalFormatPName(super::STENCIL_RENDERABLE);
    pub const FRAMEBUFFER_RENDERABLE: InternalFormatPName = InternalFormatPName(super::FRAMEBUFFER_RENDERABLE);
    pub const FRAMEBUFFER_RENDERABLE_LAYERED: InternalFormatPName = InternalFormatPName(super::FRAMEBUFFER_RENDERABLE_LAYERED);
    pub const FRAMEBUFFER_BLEND: InternalFormatPName = InternalFormatPName(super::FRAMEBUFFER_BLEND);
    pub const READ_PIXELS: InternalFormatPName = InternalFormatPName(super::READ_PIXELS);
    pub const READ_PIXELS_FORMAT: InternalFormatPName = InternalFormatPName(super::READ_PIXELS_FORMAT);
    pub const READ_PIXELS_TYPE: InternalFormatPName = InternalFormatPName(super::READ_PIXELS_TYPE);
    pub const TEXTURE_IMAGE_FORMAT: InternalFormatPName = InternalFormatPName(super::TEXTURE_IMAGE_FORMAT);
    pub const TEXTURE_IMAGE_TYPE: InternalFormatPName = InternalFormatPName(super::TEXTURE_IMAGE_TYPE);
    pub const GET_TEXTURE_IMAGE_FORMAT: InternalFormatPName = InternalFormatPName(super::GET_TEXTURE_IMAGE_FORMAT);
    pub const GET_TEXTURE_IMAGE_TYPE: InternalFormatPName = InternalFormatPName(super::GET_TEXTURE_IMAGE_TYPE);
    pub const MIPMAP: InternalFormatPName = InternalFormatPName(super::MIPMAP);
    pub const AUTO_GENERATE_MIPMAP: InternalFormatPName = InternalFormatPName(super::AUTO_GENERATE_MIPMAP);
    pub const COLOR_ENCODING: InternalFormatPName = InternalFormatPName(super::COLOR_ENCODING);
    pub const SRGB_READ: InternalFormatPName = InternalFormatPName(super::SRGB_READ);
    pub const SRGB_WRITE: InternalFormatPName = InternalFormatPName(super::SRGB_WRITE);
    pub const FILTER: InternalFormatPName = InternalFormatPName(super::FILTER);
    pub const VERTEX_TEXTURE: InternalFormatPName = InternalFormatPName(super::VERTEX_TEXTURE);
    pub const TESS_CONTROL_TEXTURE: InternalFormatPName = InternalFormatPName(super::TESS_CONTROL_TEXTURE);
    pub const TESS_EVALUATION_TEXTURE: InternalFormatPName = InternalFormatPName(super::TESS_EVALUATION_TEXTURE);
    pub const GEOMETRY_TEXTURE: InternalFormatPName = InternalFormatPName(super::GEOMETRY_TEXTURE);
    pub const FRAGMENT_TEXTURE: InternalFormatPName = InternalFormatPName(super::FRAGMENT_TEXTURE);
    pub const COMPUTE_TEXTURE: InternalFormatPName = InternalFormatPName(super::COMPUTE_TEXTURE);
    pub const TEXTURE_SHADOW: InternalFormatPName = InternalFormatPName(super::TEXTURE_SHADOW);
    pub const TEXTURE_GATHER: InternalFormatPName = InternalFormatPName(super::TEXTURE_GATHER);
    pub const TEXTURE_GATHER_SHADOW: InternalFormatPName = InternalFormatPName(super::TEXTURE_GATHER_SHADOW);
    pub const SHADER_IMAGE_LOAD: InternalFormatPName = InternalFormatPName(super::SHADER_IMAGE_LOAD);
    pub const SHADER_IMAGE_STORE: InternalFormatPName = InternalFormatPName(super::SHADER_IMAGE_STORE);
    pub const SHADER_IMAGE_ATOMIC: InternalFormatPName = InternalFormatPName(super::SHADER_IMAGE_ATOMIC);
    pub const IMAGE_TEXEL_SIZE: InternalFormatPName = InternalFormatPName(super::IMAGE_TEXEL_SIZE);
    pub const IMAGE_COMPATIBILITY_CLASS: InternalFormatPName = InternalFormatPName(super::IMAGE_COMPATIBILITY_CLASS);
    pub const IMAGE_PIXEL_FORMAT: InternalFormatPName = InternalFormatPName(super::IMAGE_PIXEL_FORMAT);
    pub const IMAGE_PIXEL_TYPE: InternalFormatPName = InternalFormatPName(super::IMAGE_PIXEL_TYPE);
    pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: InternalFormatPName = InternalFormatPName(super::IMAGE_FORMAT_COMPATIBILITY_TYPE);
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: InternalFormatPName = InternalFormatPName(super::SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST);
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: InternalFormatPName = InternalFormatPName(super::SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST);
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: InternalFormatPName = InternalFormatPName(super::SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE);
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: InternalFormatPName = InternalFormatPName(super::SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE);
    pub const TEXTURE_COMPRESSED: InternalFormatPName = InternalFormatPName(super::TEXTURE_COMPRESSED);
    pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: InternalFormatPName = InternalFormatPName(super::TEXTURE_COMPRESSED_BLOCK_WIDTH);
    pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: InternalFormatPName = InternalFormatPName(super::TEXTURE_COMPRESSED_BLOCK_HEIGHT);
    pub const TEXTURE_COMPRESSED_BLOCK_SIZE: InternalFormatPName = InternalFormatPName(super::TEXTURE_COMPRESSED_BLOCK_SIZE);
    pub const CLEAR_BUFFER: InternalFormatPName = InternalFormatPName(super::CLEAR_BUFFER);
    pub const TEXTURE_VIEW: InternalFormatPName = InternalFormatPName(super::TEXTURE_VIEW);
    pub const VIEW_COMPATIBILITY_CLASS: InternalFormatPName = InternalFormatPName(super::VIEW_COMPATIBILITY_CLASS);
    pub const CLEAR_TEXTURE: InternalFormatPName = InternalFormatPName(super::CLEAR_TEXTURE);
}

impl ::std::fmt::Debug for InternalFormatPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            InternalFormatPName::AUTO_GENERATE_MIPMAP => write!(fmt, "InternalFormatPName(AUTO_GENERATE_MIPMAP)"),
            InternalFormatPName::CLEAR_BUFFER => write!(fmt, "InternalFormatPName(CLEAR_BUFFER)"),
            InternalFormatPName::CLEAR_TEXTURE => write!(fmt, "InternalFormatPName(CLEAR_TEXTURE)"),
            InternalFormatPName::COLOR_COMPONENTS => write!(fmt, "InternalFormatPName(COLOR_COMPONENTS)"),
            InternalFormatPName::COLOR_ENCODING => write!(fmt, "InternalFormatPName(COLOR_ENCODING)"),
            InternalFormatPName::COLOR_RENDERABLE => write!(fmt, "InternalFormatPName(COLOR_RENDERABLE)"),
            InternalFormatPName::COMPUTE_TEXTURE => write!(fmt, "InternalFormatPName(COMPUTE_TEXTURE)"),
            InternalFormatPName::DEPTH_RENDERABLE => write!(fmt, "InternalFormatPName(DEPTH_RENDERABLE)"),
            InternalFormatPName::FILTER => write!(fmt, "InternalFormatPName(FILTER)"),
            InternalFormatPName::FRAGMENT_TEXTURE => write!(fmt, "InternalFormatPName(FRAGMENT_TEXTURE)"),
            InternalFormatPName::FRAMEBUFFER_BLEND => write!(fmt, "InternalFormatPName(FRAMEBUFFER_BLEND)"),
            InternalFormatPName::FRAMEBUFFER_RENDERABLE => write!(fmt, "InternalFormatPName(FRAMEBUFFER_RENDERABLE)"),
            InternalFormatPName::FRAMEBUFFER_RENDERABLE_LAYERED => write!(fmt, "InternalFormatPName(FRAMEBUFFER_RENDERABLE_LAYERED)"),
            InternalFormatPName::GEOMETRY_TEXTURE => write!(fmt, "InternalFormatPName(GEOMETRY_TEXTURE)"),
            InternalFormatPName::GET_TEXTURE_IMAGE_FORMAT => write!(fmt, "InternalFormatPName(GET_TEXTURE_IMAGE_FORMAT)"),
            InternalFormatPName::GET_TEXTURE_IMAGE_TYPE => write!(fmt, "InternalFormatPName(GET_TEXTURE_IMAGE_TYPE)"),
            InternalFormatPName::IMAGE_COMPATIBILITY_CLASS => write!(fmt, "InternalFormatPName(IMAGE_COMPATIBILITY_CLASS)"),
            InternalFormatPName::IMAGE_FORMAT_COMPATIBILITY_TYPE => write!(fmt, "InternalFormatPName(IMAGE_FORMAT_COMPATIBILITY_TYPE)"),
            InternalFormatPName::IMAGE_PIXEL_FORMAT => write!(fmt, "InternalFormatPName(IMAGE_PIXEL_FORMAT)"),
            InternalFormatPName::IMAGE_PIXEL_TYPE => write!(fmt, "InternalFormatPName(IMAGE_PIXEL_TYPE)"),
            InternalFormatPName::IMAGE_TEXEL_SIZE => write!(fmt, "InternalFormatPName(IMAGE_TEXEL_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_ALPHA_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_ALPHA_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_ALPHA_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_ALPHA_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_BLUE_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_BLUE_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_BLUE_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_BLUE_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_DEPTH_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_DEPTH_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_DEPTH_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_DEPTH_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_GREEN_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_GREEN_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_GREEN_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_GREEN_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_PREFERRED => write!(fmt, "InternalFormatPName(INTERNALFORMAT_PREFERRED)"),
            InternalFormatPName::INTERNALFORMAT_RED_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_RED_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_RED_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_RED_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_SHARED_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_SHARED_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_STENCIL_SIZE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_STENCIL_SIZE)"),
            InternalFormatPName::INTERNALFORMAT_STENCIL_TYPE => write!(fmt, "InternalFormatPName(INTERNALFORMAT_STENCIL_TYPE)"),
            InternalFormatPName::INTERNALFORMAT_SUPPORTED => write!(fmt, "InternalFormatPName(INTERNALFORMAT_SUPPORTED)"),
            InternalFormatPName::MAX_DEPTH => write!(fmt, "InternalFormatPName(MAX_DEPTH)"),
            InternalFormatPName::MAX_HEIGHT => write!(fmt, "InternalFormatPName(MAX_HEIGHT)"),
            InternalFormatPName::MAX_LAYERS => write!(fmt, "InternalFormatPName(MAX_LAYERS)"),
            InternalFormatPName::MAX_WIDTH => write!(fmt, "InternalFormatPName(MAX_WIDTH)"),
            InternalFormatPName::MIPMAP => write!(fmt, "InternalFormatPName(MIPMAP)"),
            InternalFormatPName::NUM_SAMPLE_COUNTS => write!(fmt, "InternalFormatPName(NUM_SAMPLE_COUNTS)"),
            InternalFormatPName::READ_PIXELS => write!(fmt, "InternalFormatPName(READ_PIXELS)"),
            InternalFormatPName::READ_PIXELS_FORMAT => write!(fmt, "InternalFormatPName(READ_PIXELS_FORMAT)"),
            InternalFormatPName::READ_PIXELS_TYPE => write!(fmt, "InternalFormatPName(READ_PIXELS_TYPE)"),
            InternalFormatPName::SAMPLES => write!(fmt, "InternalFormatPName(SAMPLES)"),
            InternalFormatPName::SHADER_IMAGE_ATOMIC => write!(fmt, "InternalFormatPName(SHADER_IMAGE_ATOMIC)"),
            InternalFormatPName::SHADER_IMAGE_LOAD => write!(fmt, "InternalFormatPName(SHADER_IMAGE_LOAD)"),
            InternalFormatPName::SHADER_IMAGE_STORE => write!(fmt, "InternalFormatPName(SHADER_IMAGE_STORE)"),
            InternalFormatPName::SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST => write!(fmt, "InternalFormatPName(SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST)"),
            InternalFormatPName::SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE => write!(fmt, "InternalFormatPName(SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE)"),
            InternalFormatPName::SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST => write!(fmt, "InternalFormatPName(SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST)"),
            InternalFormatPName::SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE => write!(fmt, "InternalFormatPName(SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE)"),
            InternalFormatPName::SRGB_READ => write!(fmt, "InternalFormatPName(SRGB_READ)"),
            InternalFormatPName::SRGB_WRITE => write!(fmt, "InternalFormatPName(SRGB_WRITE)"),
            InternalFormatPName::STENCIL_RENDERABLE => write!(fmt, "InternalFormatPName(STENCIL_RENDERABLE)"),
            InternalFormatPName::TESS_CONTROL_TEXTURE => write!(fmt, "InternalFormatPName(TESS_CONTROL_TEXTURE)"),
            InternalFormatPName::TESS_EVALUATION_TEXTURE => write!(fmt, "InternalFormatPName(TESS_EVALUATION_TEXTURE)"),
            InternalFormatPName::TEXTURE_COMPRESSED => write!(fmt, "InternalFormatPName(TEXTURE_COMPRESSED)"),
            InternalFormatPName::TEXTURE_COMPRESSED_BLOCK_HEIGHT => write!(fmt, "InternalFormatPName(TEXTURE_COMPRESSED_BLOCK_HEIGHT)"),
            InternalFormatPName::TEXTURE_COMPRESSED_BLOCK_SIZE => write!(fmt, "InternalFormatPName(TEXTURE_COMPRESSED_BLOCK_SIZE)"),
            InternalFormatPName::TEXTURE_COMPRESSED_BLOCK_WIDTH => write!(fmt, "InternalFormatPName(TEXTURE_COMPRESSED_BLOCK_WIDTH)"),
            InternalFormatPName::TEXTURE_GATHER => write!(fmt, "InternalFormatPName(TEXTURE_GATHER)"),
            InternalFormatPName::TEXTURE_GATHER_SHADOW => write!(fmt, "InternalFormatPName(TEXTURE_GATHER_SHADOW)"),
            InternalFormatPName::TEXTURE_IMAGE_FORMAT => write!(fmt, "InternalFormatPName(TEXTURE_IMAGE_FORMAT)"),
            InternalFormatPName::TEXTURE_IMAGE_TYPE => write!(fmt, "InternalFormatPName(TEXTURE_IMAGE_TYPE)"),
            InternalFormatPName::TEXTURE_SHADOW => write!(fmt, "InternalFormatPName(TEXTURE_SHADOW)"),
            InternalFormatPName::TEXTURE_VIEW => write!(fmt, "InternalFormatPName(TEXTURE_VIEW)"),
            InternalFormatPName::VERTEX_TEXTURE => write!(fmt, "InternalFormatPName(VERTEX_TEXTURE)"),
            InternalFormatPName::VIEW_COMPATIBILITY_CLASS => write!(fmt, "InternalFormatPName(VIEW_COMPATIBILITY_CLASS)"),
            _ => write!(fmt, "InternalFormatPName({})", self.0),
        }
    }
}

impl_enum_traits!(InternalFormatPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightEnvModeSGIX(pub types::GLenum);

impl LightEnvModeSGIX {
    pub const REPLACE: LightEnvModeSGIX = LightEnvModeSGIX(super::REPLACE);
}

impl ::std::fmt::Debug for LightEnvModeSGIX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            LightEnvModeSGIX::REPLACE => write!(fmt, "LightEnvModeSGIX(REPLACE)"),
            _ => write!(fmt, "LightEnvModeSGIX({})", self.0),
        }
    }
}

impl_enum_traits!(LightEnvModeSGIX);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightEnvParameterSGIX(pub types::GLenum);

impl LightEnvParameterSGIX {
}

impl ::std::fmt::Debug for LightEnvParameterSGIX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "LightEnvParameterSGIX({})", self.0),
        }
    }
}

impl_enum_traits!(LightEnvParameterSGIX);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightModelColorControl(pub types::GLenum);

impl LightModelColorControl {
}

impl ::std::fmt::Debug for LightModelColorControl {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "LightModelColorControl({})", self.0),
        }
    }
}

impl_enum_traits!(LightModelColorControl);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightModelParameter(pub types::GLenum);

impl LightModelParameter {
}

impl ::std::fmt::Debug for LightModelParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "LightModelParameter({})", self.0),
        }
    }
}

impl_enum_traits!(LightModelParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightName(pub types::GLenum);

impl LightName {
}

impl ::std::fmt::Debug for LightName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "LightName({})", self.0),
        }
    }
}

impl_enum_traits!(LightName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LightParameter(pub types::GLenum);

impl LightParameter {
}

impl ::std::fmt::Debug for LightParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "LightParameter({})", self.0),
        }
    }
}

impl_enum_traits!(LightParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ListMode(pub types::GLenum);

impl ListMode {
}

impl ::std::fmt::Debug for ListMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ListMode({})", self.0),
        }
    }
}

impl_enum_traits!(ListMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ListNameType(pub types::GLenum);

impl ListNameType {
    pub const BYTE: ListNameType = ListNameType(super::BYTE);
    pub const FLOAT: ListNameType = ListNameType(super::FLOAT);
    pub const INT: ListNameType = ListNameType(super::INT);
    pub const SHORT: ListNameType = ListNameType(super::SHORT);
    pub const UNSIGNED_BYTE: ListNameType = ListNameType(super::UNSIGNED_BYTE);
    pub const UNSIGNED_INT: ListNameType = ListNameType(super::UNSIGNED_INT);
    pub const UNSIGNED_SHORT: ListNameType = ListNameType(super::UNSIGNED_SHORT);
}

impl ::std::fmt::Debug for ListNameType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ListNameType::BYTE => write!(fmt, "ListNameType(BYTE)"),
            ListNameType::FLOAT => write!(fmt, "ListNameType(FLOAT)"),
            ListNameType::INT => write!(fmt, "ListNameType(INT)"),
            ListNameType::SHORT => write!(fmt, "ListNameType(SHORT)"),
            ListNameType::UNSIGNED_BYTE => write!(fmt, "ListNameType(UNSIGNED_BYTE)"),
            ListNameType::UNSIGNED_INT => write!(fmt, "ListNameType(UNSIGNED_INT)"),
            ListNameType::UNSIGNED_SHORT => write!(fmt, "ListNameType(UNSIGNED_SHORT)"),
            _ => write!(fmt, "ListNameType({})", self.0),
        }
    }
}

impl_enum_traits!(ListNameType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ListParameterName(pub types::GLenum);

impl ListParameterName {
}

impl ::std::fmt::Debug for ListParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ListParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(ListParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LogicOp(pub types::GLenum);

impl LogicOp {
    pub const AND: LogicOp = LogicOp(super::AND);
    pub const AND_INVERTED: LogicOp = LogicOp(super::AND_INVERTED);
    pub const AND_REVERSE: LogicOp = LogicOp(super::AND_REVERSE);
    pub const CLEAR: LogicOp = LogicOp(super::CLEAR);
    pub const COPY: LogicOp = LogicOp(super::COPY);
    pub const COPY_INVERTED: LogicOp = LogicOp(super::COPY_INVERTED);
    pub const EQUIV: LogicOp = LogicOp(super::EQUIV);
    pub const INVERT: LogicOp = LogicOp(super::INVERT);
    pub const NAND: LogicOp = LogicOp(super::NAND);
    pub const NOOP: LogicOp = LogicOp(super::NOOP);
    pub const NOR: LogicOp = LogicOp(super::NOR);
    pub const OR: LogicOp = LogicOp(super::OR);
    pub const OR_INVERTED: LogicOp = LogicOp(super::OR_INVERTED);
    pub const OR_REVERSE: LogicOp = LogicOp(super::OR_REVERSE);
    pub const SET: LogicOp = LogicOp(super::SET);
    pub const XOR: LogicOp = LogicOp(super::XOR);
}

impl ::std::fmt::Debug for LogicOp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            LogicOp::AND => write!(fmt, "LogicOp(AND)"),
            LogicOp::AND_INVERTED => write!(fmt, "LogicOp(AND_INVERTED)"),
            LogicOp::AND_REVERSE => write!(fmt, "LogicOp(AND_REVERSE)"),
            LogicOp::CLEAR => write!(fmt, "LogicOp(CLEAR)"),
            LogicOp::COPY => write!(fmt, "LogicOp(COPY)"),
            LogicOp::COPY_INVERTED => write!(fmt, "LogicOp(COPY_INVERTED)"),
            LogicOp::EQUIV => write!(fmt, "LogicOp(EQUIV)"),
            LogicOp::INVERT => write!(fmt, "LogicOp(INVERT)"),
            LogicOp::NAND => write!(fmt, "LogicOp(NAND)"),
            LogicOp::NOOP => write!(fmt, "LogicOp(NOOP)"),
            LogicOp::NOR => write!(fmt, "LogicOp(NOR)"),
            LogicOp::OR => write!(fmt, "LogicOp(OR)"),
            LogicOp::OR_INVERTED => write!(fmt, "LogicOp(OR_INVERTED)"),
            LogicOp::OR_REVERSE => write!(fmt, "LogicOp(OR_REVERSE)"),
            LogicOp::SET => write!(fmt, "LogicOp(SET)"),
            LogicOp::XOR => write!(fmt, "LogicOp(XOR)"),
            _ => write!(fmt, "LogicOp({})", self.0),
        }
    }
}

impl_enum_traits!(LogicOp);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MapBufferUsageMask(pub types::GLenum);

impl MapBufferUsageMask {
    pub const CLIENT_STORAGE_BIT: MapBufferUsageMask = MapBufferUsageMask(super::CLIENT_STORAGE_BIT);
    pub const DYNAMIC_STORAGE_BIT: MapBufferUsageMask = MapBufferUsageMask(super::DYNAMIC_STORAGE_BIT);
    pub const MAP_COHERENT_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_COHERENT_BIT);
    pub const MAP_FLUSH_EXPLICIT_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_FLUSH_EXPLICIT_BIT);
    pub const MAP_INVALIDATE_BUFFER_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_INVALIDATE_BUFFER_BIT);
    pub const MAP_INVALIDATE_RANGE_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_INVALIDATE_RANGE_BIT);
    pub const MAP_PERSISTENT_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_PERSISTENT_BIT);
    pub const MAP_READ_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_READ_BIT);
    pub const MAP_UNSYNCHRONIZED_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_UNSYNCHRONIZED_BIT);
    pub const MAP_WRITE_BIT: MapBufferUsageMask = MapBufferUsageMask(super::MAP_WRITE_BIT);
    pub const Empty: MapBufferUsageMask = MapBufferUsageMask(0);
}

impl ::std::fmt::Debug for MapBufferUsageMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MapBufferUsageMask::CLIENT_STORAGE_BIT => write!(fmt, "MapBufferUsageMask(CLIENT_STORAGE_BIT)"),
            MapBufferUsageMask::DYNAMIC_STORAGE_BIT => write!(fmt, "MapBufferUsageMask(DYNAMIC_STORAGE_BIT)"),
            MapBufferUsageMask::MAP_COHERENT_BIT => write!(fmt, "MapBufferUsageMask(MAP_COHERENT_BIT)"),
            MapBufferUsageMask::MAP_FLUSH_EXPLICIT_BIT => write!(fmt, "MapBufferUsageMask(MAP_FLUSH_EXPLICIT_BIT)"),
            MapBufferUsageMask::MAP_INVALIDATE_BUFFER_BIT => write!(fmt, "MapBufferUsageMask(MAP_INVALIDATE_BUFFER_BIT)"),
            MapBufferUsageMask::MAP_INVALIDATE_RANGE_BIT => write!(fmt, "MapBufferUsageMask(MAP_INVALIDATE_RANGE_BIT)"),
            MapBufferUsageMask::MAP_PERSISTENT_BIT => write!(fmt, "MapBufferUsageMask(MAP_PERSISTENT_BIT)"),
            MapBufferUsageMask::MAP_READ_BIT => write!(fmt, "MapBufferUsageMask(MAP_READ_BIT)"),
            MapBufferUsageMask::MAP_UNSYNCHRONIZED_BIT => write!(fmt, "MapBufferUsageMask(MAP_UNSYNCHRONIZED_BIT)"),
            MapBufferUsageMask::MAP_WRITE_BIT => write!(fmt, "MapBufferUsageMask(MAP_WRITE_BIT)"),
            _ => write!(fmt, "MapBufferUsageMask({})", self.0),
        }
    }
}

impl_enum_traits!(MapBufferUsageMask);

impl_enum_bitmask_traits!(MapBufferUsageMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MapQuery(pub types::GLenum);

impl MapQuery {
}

impl ::std::fmt::Debug for MapQuery {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MapQuery({})", self.0),
        }
    }
}

impl_enum_traits!(MapQuery);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MapTarget(pub types::GLenum);

impl MapTarget {
}

impl ::std::fmt::Debug for MapTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MapTarget({})", self.0),
        }
    }
}

impl_enum_traits!(MapTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MapTextureFormatINTEL(pub types::GLenum);

impl MapTextureFormatINTEL {
}

impl ::std::fmt::Debug for MapTextureFormatINTEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MapTextureFormatINTEL({})", self.0),
        }
    }
}

impl_enum_traits!(MapTextureFormatINTEL);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MaterialFace(pub types::GLenum);

impl MaterialFace {
    pub const BACK: MaterialFace = MaterialFace(super::BACK);
    pub const FRONT: MaterialFace = MaterialFace(super::FRONT);
    pub const FRONT_AND_BACK: MaterialFace = MaterialFace(super::FRONT_AND_BACK);
}

impl ::std::fmt::Debug for MaterialFace {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MaterialFace::BACK => write!(fmt, "MaterialFace(BACK)"),
            MaterialFace::FRONT => write!(fmt, "MaterialFace(FRONT)"),
            MaterialFace::FRONT_AND_BACK => write!(fmt, "MaterialFace(FRONT_AND_BACK)"),
            _ => write!(fmt, "MaterialFace({})", self.0),
        }
    }
}

impl_enum_traits!(MaterialFace);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MaterialParameter(pub types::GLenum);

impl MaterialParameter {
}

impl ::std::fmt::Debug for MaterialParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MaterialParameter({})", self.0),
        }
    }
}

impl_enum_traits!(MaterialParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MatrixMode(pub types::GLenum);

impl MatrixMode {
    pub const TEXTURE: MatrixMode = MatrixMode(super::TEXTURE);
}

impl ::std::fmt::Debug for MatrixMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MatrixMode::TEXTURE => write!(fmt, "MatrixMode(TEXTURE)"),
            _ => write!(fmt, "MatrixMode({})", self.0),
        }
    }
}

impl_enum_traits!(MatrixMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MemoryBarrierMask(pub types::GLenum);

impl MemoryBarrierMask {
    pub const ALL_BARRIER_BITS: MemoryBarrierMask = MemoryBarrierMask(super::ALL_BARRIER_BITS);
    pub const ATOMIC_COUNTER_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::ATOMIC_COUNTER_BARRIER_BIT);
    pub const BUFFER_UPDATE_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::BUFFER_UPDATE_BARRIER_BIT);
    pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::CLIENT_MAPPED_BUFFER_BARRIER_BIT);
    pub const COMMAND_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::COMMAND_BARRIER_BIT);
    pub const ELEMENT_ARRAY_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::ELEMENT_ARRAY_BARRIER_BIT);
    pub const FRAMEBUFFER_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::FRAMEBUFFER_BARRIER_BIT);
    pub const PIXEL_BUFFER_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::PIXEL_BUFFER_BARRIER_BIT);
    pub const QUERY_BUFFER_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::QUERY_BUFFER_BARRIER_BIT);
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::SHADER_IMAGE_ACCESS_BARRIER_BIT);
    pub const SHADER_STORAGE_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::SHADER_STORAGE_BARRIER_BIT);
    pub const TEXTURE_FETCH_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::TEXTURE_FETCH_BARRIER_BIT);
    pub const TEXTURE_UPDATE_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::TEXTURE_UPDATE_BARRIER_BIT);
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::TRANSFORM_FEEDBACK_BARRIER_BIT);
    pub const UNIFORM_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::UNIFORM_BARRIER_BIT);
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: MemoryBarrierMask = MemoryBarrierMask(super::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
    pub const Empty: MemoryBarrierMask = MemoryBarrierMask(0);
}

impl ::std::fmt::Debug for MemoryBarrierMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MemoryBarrierMask::ALL_BARRIER_BITS => write!(fmt, "MemoryBarrierMask(ALL_BARRIER_BITS)"),
            MemoryBarrierMask::ATOMIC_COUNTER_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(ATOMIC_COUNTER_BARRIER_BIT)"),
            MemoryBarrierMask::BUFFER_UPDATE_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(BUFFER_UPDATE_BARRIER_BIT)"),
            MemoryBarrierMask::CLIENT_MAPPED_BUFFER_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(CLIENT_MAPPED_BUFFER_BARRIER_BIT)"),
            MemoryBarrierMask::COMMAND_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(COMMAND_BARRIER_BIT)"),
            MemoryBarrierMask::ELEMENT_ARRAY_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(ELEMENT_ARRAY_BARRIER_BIT)"),
            MemoryBarrierMask::FRAMEBUFFER_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(FRAMEBUFFER_BARRIER_BIT)"),
            MemoryBarrierMask::PIXEL_BUFFER_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(PIXEL_BUFFER_BARRIER_BIT)"),
            MemoryBarrierMask::QUERY_BUFFER_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(QUERY_BUFFER_BARRIER_BIT)"),
            MemoryBarrierMask::SHADER_IMAGE_ACCESS_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(SHADER_IMAGE_ACCESS_BARRIER_BIT)"),
            MemoryBarrierMask::SHADER_STORAGE_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(SHADER_STORAGE_BARRIER_BIT)"),
            MemoryBarrierMask::TEXTURE_FETCH_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(TEXTURE_FETCH_BARRIER_BIT)"),
            MemoryBarrierMask::TEXTURE_UPDATE_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(TEXTURE_UPDATE_BARRIER_BIT)"),
            MemoryBarrierMask::TRANSFORM_FEEDBACK_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(TRANSFORM_FEEDBACK_BARRIER_BIT)"),
            MemoryBarrierMask::UNIFORM_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(UNIFORM_BARRIER_BIT)"),
            MemoryBarrierMask::VERTEX_ATTRIB_ARRAY_BARRIER_BIT => write!(fmt, "MemoryBarrierMask(VERTEX_ATTRIB_ARRAY_BARRIER_BIT)"),
            _ => write!(fmt, "MemoryBarrierMask({})", self.0),
        }
    }
}

impl_enum_traits!(MemoryBarrierMask);

impl_enum_bitmask_traits!(MemoryBarrierMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MemoryObjectParameterName(pub types::GLenum);

impl MemoryObjectParameterName {
}

impl ::std::fmt::Debug for MemoryObjectParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MemoryObjectParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(MemoryObjectParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MeshMode1(pub types::GLenum);

impl MeshMode1 {
    pub const LINE: MeshMode1 = MeshMode1(super::LINE);
    pub const POINT: MeshMode1 = MeshMode1(super::POINT);
}

impl ::std::fmt::Debug for MeshMode1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MeshMode1::LINE => write!(fmt, "MeshMode1(LINE)"),
            MeshMode1::POINT => write!(fmt, "MeshMode1(POINT)"),
            _ => write!(fmt, "MeshMode1({})", self.0),
        }
    }
}

impl_enum_traits!(MeshMode1);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MeshMode2(pub types::GLenum);

impl MeshMode2 {
    pub const FILL: MeshMode2 = MeshMode2(super::FILL);
    pub const LINE: MeshMode2 = MeshMode2(super::LINE);
    pub const POINT: MeshMode2 = MeshMode2(super::POINT);
}

impl ::std::fmt::Debug for MeshMode2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MeshMode2::FILL => write!(fmt, "MeshMode2(FILL)"),
            MeshMode2::LINE => write!(fmt, "MeshMode2(LINE)"),
            MeshMode2::POINT => write!(fmt, "MeshMode2(POINT)"),
            _ => write!(fmt, "MeshMode2({})", self.0),
        }
    }
}

impl_enum_traits!(MeshMode2);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MinmaxTargetEXT(pub types::GLenum);

impl MinmaxTargetEXT {
}

impl ::std::fmt::Debug for MinmaxTargetEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "MinmaxTargetEXT({})", self.0),
        }
    }
}

impl_enum_traits!(MinmaxTargetEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct NormalPointerType(pub types::GLenum);

impl NormalPointerType {
    pub const BYTE: NormalPointerType = NormalPointerType(super::BYTE);
    pub const DOUBLE: NormalPointerType = NormalPointerType(super::DOUBLE);
    pub const FLOAT: NormalPointerType = NormalPointerType(super::FLOAT);
    pub const INT: NormalPointerType = NormalPointerType(super::INT);
    pub const SHORT: NormalPointerType = NormalPointerType(super::SHORT);
}

impl ::std::fmt::Debug for NormalPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NormalPointerType::BYTE => write!(fmt, "NormalPointerType(BYTE)"),
            NormalPointerType::DOUBLE => write!(fmt, "NormalPointerType(DOUBLE)"),
            NormalPointerType::FLOAT => write!(fmt, "NormalPointerType(FLOAT)"),
            NormalPointerType::INT => write!(fmt, "NormalPointerType(INT)"),
            NormalPointerType::SHORT => write!(fmt, "NormalPointerType(SHORT)"),
            _ => write!(fmt, "NormalPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(NormalPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ObjectIdentifier(pub types::GLenum);

impl ObjectIdentifier {
    pub const BUFFER: ObjectIdentifier = ObjectIdentifier(super::BUFFER);
    pub const SHADER: ObjectIdentifier = ObjectIdentifier(super::SHADER);
    pub const PROGRAM: ObjectIdentifier = ObjectIdentifier(super::PROGRAM);
    pub const VERTEX_ARRAY: ObjectIdentifier = ObjectIdentifier(super::VERTEX_ARRAY);
    pub const QUERY: ObjectIdentifier = ObjectIdentifier(super::QUERY);
    pub const PROGRAM_PIPELINE: ObjectIdentifier = ObjectIdentifier(super::PROGRAM_PIPELINE);
    pub const TRANSFORM_FEEDBACK: ObjectIdentifier = ObjectIdentifier(super::TRANSFORM_FEEDBACK);
    pub const SAMPLER: ObjectIdentifier = ObjectIdentifier(super::SAMPLER);
    pub const TEXTURE: ObjectIdentifier = ObjectIdentifier(super::TEXTURE);
    pub const RENDERBUFFER: ObjectIdentifier = ObjectIdentifier(super::RENDERBUFFER);
    pub const FRAMEBUFFER: ObjectIdentifier = ObjectIdentifier(super::FRAMEBUFFER);
}

impl ::std::fmt::Debug for ObjectIdentifier {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ObjectIdentifier::BUFFER => write!(fmt, "ObjectIdentifier(BUFFER)"),
            ObjectIdentifier::FRAMEBUFFER => write!(fmt, "ObjectIdentifier(FRAMEBUFFER)"),
            ObjectIdentifier::PROGRAM => write!(fmt, "ObjectIdentifier(PROGRAM)"),
            ObjectIdentifier::PROGRAM_PIPELINE => write!(fmt, "ObjectIdentifier(PROGRAM_PIPELINE)"),
            ObjectIdentifier::QUERY => write!(fmt, "ObjectIdentifier(QUERY)"),
            ObjectIdentifier::RENDERBUFFER => write!(fmt, "ObjectIdentifier(RENDERBUFFER)"),
            ObjectIdentifier::SAMPLER => write!(fmt, "ObjectIdentifier(SAMPLER)"),
            ObjectIdentifier::SHADER => write!(fmt, "ObjectIdentifier(SHADER)"),
            ObjectIdentifier::TEXTURE => write!(fmt, "ObjectIdentifier(TEXTURE)"),
            ObjectIdentifier::TRANSFORM_FEEDBACK => write!(fmt, "ObjectIdentifier(TRANSFORM_FEEDBACK)"),
            ObjectIdentifier::VERTEX_ARRAY => write!(fmt, "ObjectIdentifier(VERTEX_ARRAY)"),
            _ => write!(fmt, "ObjectIdentifier({})", self.0),
        }
    }
}

impl_enum_traits!(ObjectIdentifier);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct OcclusionQueryEventMaskAMD(pub types::GLenum);

impl OcclusionQueryEventMaskAMD {
    pub const Empty: OcclusionQueryEventMaskAMD = OcclusionQueryEventMaskAMD(0);
}

impl ::std::fmt::Debug for OcclusionQueryEventMaskAMD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "OcclusionQueryEventMaskAMD({})", self.0),
        }
    }
}

impl_enum_traits!(OcclusionQueryEventMaskAMD);

impl_enum_bitmask_traits!(OcclusionQueryEventMaskAMD);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PatchParameterName(pub types::GLenum);

impl PatchParameterName {
    pub const PATCH_VERTICES: PatchParameterName = PatchParameterName(super::PATCH_VERTICES);
    pub const PATCH_DEFAULT_OUTER_LEVEL: PatchParameterName = PatchParameterName(super::PATCH_DEFAULT_OUTER_LEVEL);
    pub const PATCH_DEFAULT_INNER_LEVEL: PatchParameterName = PatchParameterName(super::PATCH_DEFAULT_INNER_LEVEL);
}

impl ::std::fmt::Debug for PatchParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PatchParameterName::PATCH_DEFAULT_INNER_LEVEL => write!(fmt, "PatchParameterName(PATCH_DEFAULT_INNER_LEVEL)"),
            PatchParameterName::PATCH_DEFAULT_OUTER_LEVEL => write!(fmt, "PatchParameterName(PATCH_DEFAULT_OUTER_LEVEL)"),
            PatchParameterName::PATCH_VERTICES => write!(fmt, "PatchParameterName(PATCH_VERTICES)"),
            _ => write!(fmt, "PatchParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(PatchParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathColor(pub types::GLenum);

impl PathColor {
}

impl ::std::fmt::Debug for PathColor {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathColor({})", self.0),
        }
    }
}

impl_enum_traits!(PathColor);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathCoverMode(pub types::GLenum);

impl PathCoverMode {
}

impl ::std::fmt::Debug for PathCoverMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathCoverMode({})", self.0),
        }
    }
}

impl_enum_traits!(PathCoverMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathElementType(pub types::GLenum);

impl PathElementType {
}

impl ::std::fmt::Debug for PathElementType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathElementType({})", self.0),
        }
    }
}

impl_enum_traits!(PathElementType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathFillMode(pub types::GLenum);

impl PathFillMode {
    pub const INVERT: PathFillMode = PathFillMode(super::INVERT);
}

impl ::std::fmt::Debug for PathFillMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PathFillMode::INVERT => write!(fmt, "PathFillMode(INVERT)"),
            _ => write!(fmt, "PathFillMode({})", self.0),
        }
    }
}

impl_enum_traits!(PathFillMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathFontStyle(pub types::GLenum);

impl PathFontStyle {
    pub const NONE: PathFontStyle = PathFontStyle(super::NONE);
}

impl ::std::fmt::Debug for PathFontStyle {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PathFontStyle::NONE => write!(fmt, "PathFontStyle(NONE)"),
            _ => write!(fmt, "PathFontStyle({})", self.0),
        }
    }
}

impl_enum_traits!(PathFontStyle);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathFontTarget(pub types::GLenum);

impl PathFontTarget {
}

impl ::std::fmt::Debug for PathFontTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathFontTarget({})", self.0),
        }
    }
}

impl_enum_traits!(PathFontTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathGenMode(pub types::GLenum);

impl PathGenMode {
    pub const NONE: PathGenMode = PathGenMode(super::NONE);
}

impl ::std::fmt::Debug for PathGenMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PathGenMode::NONE => write!(fmt, "PathGenMode(NONE)"),
            _ => write!(fmt, "PathGenMode({})", self.0),
        }
    }
}

impl_enum_traits!(PathGenMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathHandleMissingGlyphs(pub types::GLenum);

impl PathHandleMissingGlyphs {
}

impl ::std::fmt::Debug for PathHandleMissingGlyphs {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathHandleMissingGlyphs({})", self.0),
        }
    }
}

impl_enum_traits!(PathHandleMissingGlyphs);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathListMode(pub types::GLenum);

impl PathListMode {
}

impl ::std::fmt::Debug for PathListMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathListMode({})", self.0),
        }
    }
}

impl_enum_traits!(PathListMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathMetricMask(pub types::GLenum);

impl PathMetricMask {
}

impl ::std::fmt::Debug for PathMetricMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathMetricMask({})", self.0),
        }
    }
}

impl_enum_traits!(PathMetricMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathParameter(pub types::GLenum);

impl PathParameter {
}

impl ::std::fmt::Debug for PathParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathParameter({})", self.0),
        }
    }
}

impl_enum_traits!(PathParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathStringFormat(pub types::GLenum);

impl PathStringFormat {
}

impl ::std::fmt::Debug for PathStringFormat {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PathStringFormat({})", self.0),
        }
    }
}

impl_enum_traits!(PathStringFormat);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PathTransformType(pub types::GLenum);

impl PathTransformType {
    pub const NONE: PathTransformType = PathTransformType(super::NONE);
}

impl ::std::fmt::Debug for PathTransformType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PathTransformType::NONE => write!(fmt, "PathTransformType(NONE)"),
            _ => write!(fmt, "PathTransformType({})", self.0),
        }
    }
}

impl_enum_traits!(PathTransformType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PipelineParameterName(pub types::GLenum);

impl PipelineParameterName {
    pub const ACTIVE_PROGRAM: PipelineParameterName = PipelineParameterName(super::ACTIVE_PROGRAM);
    pub const VERTEX_SHADER: PipelineParameterName = PipelineParameterName(super::VERTEX_SHADER);
    pub const TESS_CONTROL_SHADER: PipelineParameterName = PipelineParameterName(super::TESS_CONTROL_SHADER);
    pub const TESS_EVALUATION_SHADER: PipelineParameterName = PipelineParameterName(super::TESS_EVALUATION_SHADER);
    pub const GEOMETRY_SHADER: PipelineParameterName = PipelineParameterName(super::GEOMETRY_SHADER);
    pub const FRAGMENT_SHADER: PipelineParameterName = PipelineParameterName(super::FRAGMENT_SHADER);
    pub const INFO_LOG_LENGTH: PipelineParameterName = PipelineParameterName(super::INFO_LOG_LENGTH);
}

impl ::std::fmt::Debug for PipelineParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PipelineParameterName::ACTIVE_PROGRAM => write!(fmt, "PipelineParameterName(ACTIVE_PROGRAM)"),
            PipelineParameterName::FRAGMENT_SHADER => write!(fmt, "PipelineParameterName(FRAGMENT_SHADER)"),
            PipelineParameterName::GEOMETRY_SHADER => write!(fmt, "PipelineParameterName(GEOMETRY_SHADER)"),
            PipelineParameterName::INFO_LOG_LENGTH => write!(fmt, "PipelineParameterName(INFO_LOG_LENGTH)"),
            PipelineParameterName::TESS_CONTROL_SHADER => write!(fmt, "PipelineParameterName(TESS_CONTROL_SHADER)"),
            PipelineParameterName::TESS_EVALUATION_SHADER => write!(fmt, "PipelineParameterName(TESS_EVALUATION_SHADER)"),
            PipelineParameterName::VERTEX_SHADER => write!(fmt, "PipelineParameterName(VERTEX_SHADER)"),
            _ => write!(fmt, "PipelineParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(PipelineParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelCopyType(pub types::GLenum);

impl PixelCopyType {
    pub const COLOR: PixelCopyType = PixelCopyType(super::COLOR);
    pub const DEPTH: PixelCopyType = PixelCopyType(super::DEPTH);
    pub const STENCIL: PixelCopyType = PixelCopyType(super::STENCIL);
}

impl ::std::fmt::Debug for PixelCopyType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PixelCopyType::COLOR => write!(fmt, "PixelCopyType(COLOR)"),
            PixelCopyType::DEPTH => write!(fmt, "PixelCopyType(DEPTH)"),
            PixelCopyType::STENCIL => write!(fmt, "PixelCopyType(STENCIL)"),
            _ => write!(fmt, "PixelCopyType({})", self.0),
        }
    }
}

impl_enum_traits!(PixelCopyType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelFormat(pub types::GLenum);

impl PixelFormat {
    pub const ALPHA: PixelFormat = PixelFormat(super::ALPHA);
    pub const BGR: PixelFormat = PixelFormat(super::BGR);
    pub const BGR_INTEGER: PixelFormat = PixelFormat(super::BGR_INTEGER);
    pub const BGRA: PixelFormat = PixelFormat(super::BGRA);
    pub const BGRA_INTEGER: PixelFormat = PixelFormat(super::BGRA_INTEGER);
    pub const BLUE: PixelFormat = PixelFormat(super::BLUE);
    pub const BLUE_INTEGER: PixelFormat = PixelFormat(super::BLUE_INTEGER);
    pub const DEPTH_COMPONENT: PixelFormat = PixelFormat(super::DEPTH_COMPONENT);
    pub const DEPTH_STENCIL: PixelFormat = PixelFormat(super::DEPTH_STENCIL);
    pub const GREEN: PixelFormat = PixelFormat(super::GREEN);
    pub const GREEN_INTEGER: PixelFormat = PixelFormat(super::GREEN_INTEGER);
    pub const RED: PixelFormat = PixelFormat(super::RED);
    pub const RED_INTEGER: PixelFormat = PixelFormat(super::RED_INTEGER);
    pub const RG: PixelFormat = PixelFormat(super::RG);
    pub const RG_INTEGER: PixelFormat = PixelFormat(super::RG_INTEGER);
    pub const RGB: PixelFormat = PixelFormat(super::RGB);
    pub const RGB_INTEGER: PixelFormat = PixelFormat(super::RGB_INTEGER);
    pub const RGBA: PixelFormat = PixelFormat(super::RGBA);
    pub const RGBA_INTEGER: PixelFormat = PixelFormat(super::RGBA_INTEGER);
    pub const STENCIL_INDEX: PixelFormat = PixelFormat(super::STENCIL_INDEX);
    pub const UNSIGNED_INT: PixelFormat = PixelFormat(super::UNSIGNED_INT);
    pub const UNSIGNED_SHORT: PixelFormat = PixelFormat(super::UNSIGNED_SHORT);
}

impl ::std::fmt::Debug for PixelFormat {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PixelFormat::ALPHA => write!(fmt, "PixelFormat(ALPHA)"),
            PixelFormat::BGR => write!(fmt, "PixelFormat(BGR)"),
            PixelFormat::BGRA => write!(fmt, "PixelFormat(BGRA)"),
            PixelFormat::BGRA_INTEGER => write!(fmt, "PixelFormat(BGRA_INTEGER)"),
            PixelFormat::BGR_INTEGER => write!(fmt, "PixelFormat(BGR_INTEGER)"),
            PixelFormat::BLUE => write!(fmt, "PixelFormat(BLUE)"),
            PixelFormat::BLUE_INTEGER => write!(fmt, "PixelFormat(BLUE_INTEGER)"),
            PixelFormat::DEPTH_COMPONENT => write!(fmt, "PixelFormat(DEPTH_COMPONENT)"),
            PixelFormat::DEPTH_STENCIL => write!(fmt, "PixelFormat(DEPTH_STENCIL)"),
            PixelFormat::GREEN => write!(fmt, "PixelFormat(GREEN)"),
            PixelFormat::GREEN_INTEGER => write!(fmt, "PixelFormat(GREEN_INTEGER)"),
            PixelFormat::RED => write!(fmt, "PixelFormat(RED)"),
            PixelFormat::RED_INTEGER => write!(fmt, "PixelFormat(RED_INTEGER)"),
            PixelFormat::RG => write!(fmt, "PixelFormat(RG)"),
            PixelFormat::RGB => write!(fmt, "PixelFormat(RGB)"),
            PixelFormat::RGBA => write!(fmt, "PixelFormat(RGBA)"),
            PixelFormat::RGBA_INTEGER => write!(fmt, "PixelFormat(RGBA_INTEGER)"),
            PixelFormat::RGB_INTEGER => write!(fmt, "PixelFormat(RGB_INTEGER)"),
            PixelFormat::RG_INTEGER => write!(fmt, "PixelFormat(RG_INTEGER)"),
            PixelFormat::STENCIL_INDEX => write!(fmt, "PixelFormat(STENCIL_INDEX)"),
            PixelFormat::UNSIGNED_INT => write!(fmt, "PixelFormat(UNSIGNED_INT)"),
            PixelFormat::UNSIGNED_SHORT => write!(fmt, "PixelFormat(UNSIGNED_SHORT)"),
            _ => write!(fmt, "PixelFormat({})", self.0),
        }
    }
}

impl_enum_traits!(PixelFormat);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelMap(pub types::GLenum);

impl PixelMap {
}

impl ::std::fmt::Debug for PixelMap {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PixelMap({})", self.0),
        }
    }
}

impl_enum_traits!(PixelMap);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelStoreParameter(pub types::GLenum);

impl PixelStoreParameter {
    pub const PACK_ALIGNMENT: PixelStoreParameter = PixelStoreParameter(super::PACK_ALIGNMENT);
    pub const PACK_IMAGE_HEIGHT: PixelStoreParameter = PixelStoreParameter(super::PACK_IMAGE_HEIGHT);
    pub const PACK_LSB_FIRST: PixelStoreParameter = PixelStoreParameter(super::PACK_LSB_FIRST);
    pub const PACK_ROW_LENGTH: PixelStoreParameter = PixelStoreParameter(super::PACK_ROW_LENGTH);
    pub const PACK_SKIP_IMAGES: PixelStoreParameter = PixelStoreParameter(super::PACK_SKIP_IMAGES);
    pub const PACK_SKIP_PIXELS: PixelStoreParameter = PixelStoreParameter(super::PACK_SKIP_PIXELS);
    pub const PACK_SKIP_ROWS: PixelStoreParameter = PixelStoreParameter(super::PACK_SKIP_ROWS);
    pub const PACK_SWAP_BYTES: PixelStoreParameter = PixelStoreParameter(super::PACK_SWAP_BYTES);
    pub const UNPACK_ALIGNMENT: PixelStoreParameter = PixelStoreParameter(super::UNPACK_ALIGNMENT);
    pub const UNPACK_IMAGE_HEIGHT: PixelStoreParameter = PixelStoreParameter(super::UNPACK_IMAGE_HEIGHT);
    pub const UNPACK_LSB_FIRST: PixelStoreParameter = PixelStoreParameter(super::UNPACK_LSB_FIRST);
    pub const UNPACK_ROW_LENGTH: PixelStoreParameter = PixelStoreParameter(super::UNPACK_ROW_LENGTH);
    pub const UNPACK_SKIP_IMAGES: PixelStoreParameter = PixelStoreParameter(super::UNPACK_SKIP_IMAGES);
    pub const UNPACK_SKIP_PIXELS: PixelStoreParameter = PixelStoreParameter(super::UNPACK_SKIP_PIXELS);
    pub const UNPACK_SKIP_ROWS: PixelStoreParameter = PixelStoreParameter(super::UNPACK_SKIP_ROWS);
    pub const UNPACK_SWAP_BYTES: PixelStoreParameter = PixelStoreParameter(super::UNPACK_SWAP_BYTES);
}

impl ::std::fmt::Debug for PixelStoreParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PixelStoreParameter::PACK_ALIGNMENT => write!(fmt, "PixelStoreParameter(PACK_ALIGNMENT)"),
            PixelStoreParameter::PACK_IMAGE_HEIGHT => write!(fmt, "PixelStoreParameter(PACK_IMAGE_HEIGHT)"),
            PixelStoreParameter::PACK_LSB_FIRST => write!(fmt, "PixelStoreParameter(PACK_LSB_FIRST)"),
            PixelStoreParameter::PACK_ROW_LENGTH => write!(fmt, "PixelStoreParameter(PACK_ROW_LENGTH)"),
            PixelStoreParameter::PACK_SKIP_IMAGES => write!(fmt, "PixelStoreParameter(PACK_SKIP_IMAGES)"),
            PixelStoreParameter::PACK_SKIP_PIXELS => write!(fmt, "PixelStoreParameter(PACK_SKIP_PIXELS)"),
            PixelStoreParameter::PACK_SKIP_ROWS => write!(fmt, "PixelStoreParameter(PACK_SKIP_ROWS)"),
            PixelStoreParameter::PACK_SWAP_BYTES => write!(fmt, "PixelStoreParameter(PACK_SWAP_BYTES)"),
            PixelStoreParameter::UNPACK_ALIGNMENT => write!(fmt, "PixelStoreParameter(UNPACK_ALIGNMENT)"),
            PixelStoreParameter::UNPACK_IMAGE_HEIGHT => write!(fmt, "PixelStoreParameter(UNPACK_IMAGE_HEIGHT)"),
            PixelStoreParameter::UNPACK_LSB_FIRST => write!(fmt, "PixelStoreParameter(UNPACK_LSB_FIRST)"),
            PixelStoreParameter::UNPACK_ROW_LENGTH => write!(fmt, "PixelStoreParameter(UNPACK_ROW_LENGTH)"),
            PixelStoreParameter::UNPACK_SKIP_IMAGES => write!(fmt, "PixelStoreParameter(UNPACK_SKIP_IMAGES)"),
            PixelStoreParameter::UNPACK_SKIP_PIXELS => write!(fmt, "PixelStoreParameter(UNPACK_SKIP_PIXELS)"),
            PixelStoreParameter::UNPACK_SKIP_ROWS => write!(fmt, "PixelStoreParameter(UNPACK_SKIP_ROWS)"),
            PixelStoreParameter::UNPACK_SWAP_BYTES => write!(fmt, "PixelStoreParameter(UNPACK_SWAP_BYTES)"),
            _ => write!(fmt, "PixelStoreParameter({})", self.0),
        }
    }
}

impl_enum_traits!(PixelStoreParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelStoreResampleMode(pub types::GLenum);

impl PixelStoreResampleMode {
}

impl ::std::fmt::Debug for PixelStoreResampleMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PixelStoreResampleMode({})", self.0),
        }
    }
}

impl_enum_traits!(PixelStoreResampleMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelStoreSubsampleRate(pub types::GLenum);

impl PixelStoreSubsampleRate {
}

impl ::std::fmt::Debug for PixelStoreSubsampleRate {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PixelStoreSubsampleRate({})", self.0),
        }
    }
}

impl_enum_traits!(PixelStoreSubsampleRate);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelTexGenMode(pub types::GLenum);

impl PixelTexGenMode {
    pub const NONE: PixelTexGenMode = PixelTexGenMode(super::NONE);
    pub const RGB: PixelTexGenMode = PixelTexGenMode(super::RGB);
    pub const RGBA: PixelTexGenMode = PixelTexGenMode(super::RGBA);
}

impl ::std::fmt::Debug for PixelTexGenMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PixelTexGenMode::NONE => write!(fmt, "PixelTexGenMode(NONE)"),
            PixelTexGenMode::RGB => write!(fmt, "PixelTexGenMode(RGB)"),
            PixelTexGenMode::RGBA => write!(fmt, "PixelTexGenMode(RGBA)"),
            _ => write!(fmt, "PixelTexGenMode({})", self.0),
        }
    }
}

impl_enum_traits!(PixelTexGenMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelTexGenParameterNameSGIS(pub types::GLenum);

impl PixelTexGenParameterNameSGIS {
}

impl ::std::fmt::Debug for PixelTexGenParameterNameSGIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PixelTexGenParameterNameSGIS({})", self.0),
        }
    }
}

impl_enum_traits!(PixelTexGenParameterNameSGIS);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelTransferParameter(pub types::GLenum);

impl PixelTransferParameter {
}

impl ::std::fmt::Debug for PixelTransferParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "PixelTransferParameter({})", self.0),
        }
    }
}

impl_enum_traits!(PixelTransferParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PixelType(pub types::GLenum);

impl PixelType {
    pub const BYTE: PixelType = PixelType(super::BYTE);
    pub const FLOAT: PixelType = PixelType(super::FLOAT);
    pub const INT: PixelType = PixelType(super::INT);
    pub const SHORT: PixelType = PixelType(super::SHORT);
    pub const UNSIGNED_BYTE: PixelType = PixelType(super::UNSIGNED_BYTE);
    pub const UNSIGNED_BYTE_3_3_2: PixelType = PixelType(super::UNSIGNED_BYTE_3_3_2);
    pub const UNSIGNED_INT: PixelType = PixelType(super::UNSIGNED_INT);
    pub const UNSIGNED_INT_10_10_10_2: PixelType = PixelType(super::UNSIGNED_INT_10_10_10_2);
    pub const UNSIGNED_INT_8_8_8_8: PixelType = PixelType(super::UNSIGNED_INT_8_8_8_8);
    pub const UNSIGNED_SHORT: PixelType = PixelType(super::UNSIGNED_SHORT);
    pub const UNSIGNED_SHORT_4_4_4_4: PixelType = PixelType(super::UNSIGNED_SHORT_4_4_4_4);
    pub const UNSIGNED_SHORT_5_5_5_1: PixelType = PixelType(super::UNSIGNED_SHORT_5_5_5_1);
}

impl ::std::fmt::Debug for PixelType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PixelType::BYTE => write!(fmt, "PixelType(BYTE)"),
            PixelType::FLOAT => write!(fmt, "PixelType(FLOAT)"),
            PixelType::INT => write!(fmt, "PixelType(INT)"),
            PixelType::SHORT => write!(fmt, "PixelType(SHORT)"),
            PixelType::UNSIGNED_BYTE => write!(fmt, "PixelType(UNSIGNED_BYTE)"),
            PixelType::UNSIGNED_BYTE_3_3_2 => write!(fmt, "PixelType(UNSIGNED_BYTE_3_3_2)"),
            PixelType::UNSIGNED_INT => write!(fmt, "PixelType(UNSIGNED_INT)"),
            PixelType::UNSIGNED_INT_10_10_10_2 => write!(fmt, "PixelType(UNSIGNED_INT_10_10_10_2)"),
            PixelType::UNSIGNED_INT_8_8_8_8 => write!(fmt, "PixelType(UNSIGNED_INT_8_8_8_8)"),
            PixelType::UNSIGNED_SHORT => write!(fmt, "PixelType(UNSIGNED_SHORT)"),
            PixelType::UNSIGNED_SHORT_4_4_4_4 => write!(fmt, "PixelType(UNSIGNED_SHORT_4_4_4_4)"),
            PixelType::UNSIGNED_SHORT_5_5_5_1 => write!(fmt, "PixelType(UNSIGNED_SHORT_5_5_5_1)"),
            _ => write!(fmt, "PixelType({})", self.0),
        }
    }
}

impl_enum_traits!(PixelType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PointParameterNameSGIS(pub types::GLenum);

impl PointParameterNameSGIS {
    pub const POINT_FADE_THRESHOLD_SIZE: PointParameterNameSGIS = PointParameterNameSGIS(super::POINT_FADE_THRESHOLD_SIZE);
}

impl ::std::fmt::Debug for PointParameterNameSGIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PointParameterNameSGIS::POINT_FADE_THRESHOLD_SIZE => write!(fmt, "PointParameterNameSGIS(POINT_FADE_THRESHOLD_SIZE)"),
            _ => write!(fmt, "PointParameterNameSGIS({})", self.0),
        }
    }
}

impl_enum_traits!(PointParameterNameSGIS);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PolygonMode(pub types::GLenum);

impl PolygonMode {
    pub const FILL: PolygonMode = PolygonMode(super::FILL);
    pub const LINE: PolygonMode = PolygonMode(super::LINE);
    pub const POINT: PolygonMode = PolygonMode(super::POINT);
}

impl ::std::fmt::Debug for PolygonMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PolygonMode::FILL => write!(fmt, "PolygonMode(FILL)"),
            PolygonMode::LINE => write!(fmt, "PolygonMode(LINE)"),
            PolygonMode::POINT => write!(fmt, "PolygonMode(POINT)"),
            _ => write!(fmt, "PolygonMode({})", self.0),
        }
    }
}

impl_enum_traits!(PolygonMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PrecisionType(pub types::GLenum);

impl PrecisionType {
    pub const LOW_FLOAT: PrecisionType = PrecisionType(super::LOW_FLOAT);
    pub const MEDIUM_FLOAT: PrecisionType = PrecisionType(super::MEDIUM_FLOAT);
    pub const HIGH_FLOAT: PrecisionType = PrecisionType(super::HIGH_FLOAT);
    pub const LOW_INT: PrecisionType = PrecisionType(super::LOW_INT);
    pub const MEDIUM_INT: PrecisionType = PrecisionType(super::MEDIUM_INT);
    pub const HIGH_INT: PrecisionType = PrecisionType(super::HIGH_INT);
}

impl ::std::fmt::Debug for PrecisionType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PrecisionType::HIGH_FLOAT => write!(fmt, "PrecisionType(HIGH_FLOAT)"),
            PrecisionType::HIGH_INT => write!(fmt, "PrecisionType(HIGH_INT)"),
            PrecisionType::LOW_FLOAT => write!(fmt, "PrecisionType(LOW_FLOAT)"),
            PrecisionType::LOW_INT => write!(fmt, "PrecisionType(LOW_INT)"),
            PrecisionType::MEDIUM_FLOAT => write!(fmt, "PrecisionType(MEDIUM_FLOAT)"),
            PrecisionType::MEDIUM_INT => write!(fmt, "PrecisionType(MEDIUM_INT)"),
            _ => write!(fmt, "PrecisionType({})", self.0),
        }
    }
}

impl_enum_traits!(PrecisionType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PrimitiveType(pub types::GLenum);

impl PrimitiveType {
    pub const LINES: PrimitiveType = PrimitiveType(super::LINES);
    pub const LINES_ADJACENCY: PrimitiveType = PrimitiveType(super::LINES_ADJACENCY);
    pub const LINE_LOOP: PrimitiveType = PrimitiveType(super::LINE_LOOP);
    pub const LINE_STRIP: PrimitiveType = PrimitiveType(super::LINE_STRIP);
    pub const LINE_STRIP_ADJACENCY: PrimitiveType = PrimitiveType(super::LINE_STRIP_ADJACENCY);
    pub const PATCHES: PrimitiveType = PrimitiveType(super::PATCHES);
    pub const POINTS: PrimitiveType = PrimitiveType(super::POINTS);
    pub const QUADS: PrimitiveType = PrimitiveType(super::QUADS);
    pub const TRIANGLES: PrimitiveType = PrimitiveType(super::TRIANGLES);
    pub const TRIANGLES_ADJACENCY: PrimitiveType = PrimitiveType(super::TRIANGLES_ADJACENCY);
    pub const TRIANGLE_FAN: PrimitiveType = PrimitiveType(super::TRIANGLE_FAN);
    pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType(super::TRIANGLE_STRIP);
    pub const TRIANGLE_STRIP_ADJACENCY: PrimitiveType = PrimitiveType(super::TRIANGLE_STRIP_ADJACENCY);
}

impl ::std::fmt::Debug for PrimitiveType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PrimitiveType::LINES => write!(fmt, "PrimitiveType(LINES)"),
            PrimitiveType::LINES_ADJACENCY => write!(fmt, "PrimitiveType(LINES_ADJACENCY)"),
            PrimitiveType::LINE_LOOP => write!(fmt, "PrimitiveType(LINE_LOOP)"),
            PrimitiveType::LINE_STRIP => write!(fmt, "PrimitiveType(LINE_STRIP)"),
            PrimitiveType::LINE_STRIP_ADJACENCY => write!(fmt, "PrimitiveType(LINE_STRIP_ADJACENCY)"),
            PrimitiveType::PATCHES => write!(fmt, "PrimitiveType(PATCHES)"),
            PrimitiveType::POINTS => write!(fmt, "PrimitiveType(POINTS)"),
            PrimitiveType::QUADS => write!(fmt, "PrimitiveType(QUADS)"),
            PrimitiveType::TRIANGLES => write!(fmt, "PrimitiveType(TRIANGLES)"),
            PrimitiveType::TRIANGLES_ADJACENCY => write!(fmt, "PrimitiveType(TRIANGLES_ADJACENCY)"),
            PrimitiveType::TRIANGLE_FAN => write!(fmt, "PrimitiveType(TRIANGLE_FAN)"),
            PrimitiveType::TRIANGLE_STRIP => write!(fmt, "PrimitiveType(TRIANGLE_STRIP)"),
            PrimitiveType::TRIANGLE_STRIP_ADJACENCY => write!(fmt, "PrimitiveType(TRIANGLE_STRIP_ADJACENCY)"),
            _ => write!(fmt, "PrimitiveType({})", self.0),
        }
    }
}

impl_enum_traits!(PrimitiveType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProgramInterface(pub types::GLenum);

impl ProgramInterface {
    pub const UNIFORM: ProgramInterface = ProgramInterface(super::UNIFORM);
    pub const UNIFORM_BLOCK: ProgramInterface = ProgramInterface(super::UNIFORM_BLOCK);
    pub const PROGRAM_INPUT: ProgramInterface = ProgramInterface(super::PROGRAM_INPUT);
    pub const PROGRAM_OUTPUT: ProgramInterface = ProgramInterface(super::PROGRAM_OUTPUT);
    pub const VERTEX_SUBROUTINE: ProgramInterface = ProgramInterface(super::VERTEX_SUBROUTINE);
    pub const TESS_CONTROL_SUBROUTINE: ProgramInterface = ProgramInterface(super::TESS_CONTROL_SUBROUTINE);
    pub const TESS_EVALUATION_SUBROUTINE: ProgramInterface = ProgramInterface(super::TESS_EVALUATION_SUBROUTINE);
    pub const GEOMETRY_SUBROUTINE: ProgramInterface = ProgramInterface(super::GEOMETRY_SUBROUTINE);
    pub const FRAGMENT_SUBROUTINE: ProgramInterface = ProgramInterface(super::FRAGMENT_SUBROUTINE);
    pub const COMPUTE_SUBROUTINE: ProgramInterface = ProgramInterface(super::COMPUTE_SUBROUTINE);
    pub const VERTEX_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::VERTEX_SUBROUTINE_UNIFORM);
    pub const TESS_CONTROL_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::TESS_CONTROL_SUBROUTINE_UNIFORM);
    pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::TESS_EVALUATION_SUBROUTINE_UNIFORM);
    pub const GEOMETRY_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::GEOMETRY_SUBROUTINE_UNIFORM);
    pub const FRAGMENT_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::FRAGMENT_SUBROUTINE_UNIFORM);
    pub const COMPUTE_SUBROUTINE_UNIFORM: ProgramInterface = ProgramInterface(super::COMPUTE_SUBROUTINE_UNIFORM);
    pub const TRANSFORM_FEEDBACK_VARYING: ProgramInterface = ProgramInterface(super::TRANSFORM_FEEDBACK_VARYING);
    pub const TRANSFORM_FEEDBACK_BUFFER: ProgramInterface = ProgramInterface(super::TRANSFORM_FEEDBACK_BUFFER);
    pub const BUFFER_VARIABLE: ProgramInterface = ProgramInterface(super::BUFFER_VARIABLE);
    pub const SHADER_STORAGE_BLOCK: ProgramInterface = ProgramInterface(super::SHADER_STORAGE_BLOCK);
}

impl ::std::fmt::Debug for ProgramInterface {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ProgramInterface::BUFFER_VARIABLE => write!(fmt, "ProgramInterface(BUFFER_VARIABLE)"),
            ProgramInterface::COMPUTE_SUBROUTINE => write!(fmt, "ProgramInterface(COMPUTE_SUBROUTINE)"),
            ProgramInterface::COMPUTE_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(COMPUTE_SUBROUTINE_UNIFORM)"),
            ProgramInterface::FRAGMENT_SUBROUTINE => write!(fmt, "ProgramInterface(FRAGMENT_SUBROUTINE)"),
            ProgramInterface::FRAGMENT_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(FRAGMENT_SUBROUTINE_UNIFORM)"),
            ProgramInterface::GEOMETRY_SUBROUTINE => write!(fmt, "ProgramInterface(GEOMETRY_SUBROUTINE)"),
            ProgramInterface::GEOMETRY_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(GEOMETRY_SUBROUTINE_UNIFORM)"),
            ProgramInterface::PROGRAM_INPUT => write!(fmt, "ProgramInterface(PROGRAM_INPUT)"),
            ProgramInterface::PROGRAM_OUTPUT => write!(fmt, "ProgramInterface(PROGRAM_OUTPUT)"),
            ProgramInterface::SHADER_STORAGE_BLOCK => write!(fmt, "ProgramInterface(SHADER_STORAGE_BLOCK)"),
            ProgramInterface::TESS_CONTROL_SUBROUTINE => write!(fmt, "ProgramInterface(TESS_CONTROL_SUBROUTINE)"),
            ProgramInterface::TESS_CONTROL_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(TESS_CONTROL_SUBROUTINE_UNIFORM)"),
            ProgramInterface::TESS_EVALUATION_SUBROUTINE => write!(fmt, "ProgramInterface(TESS_EVALUATION_SUBROUTINE)"),
            ProgramInterface::TESS_EVALUATION_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(TESS_EVALUATION_SUBROUTINE_UNIFORM)"),
            ProgramInterface::TRANSFORM_FEEDBACK_BUFFER => write!(fmt, "ProgramInterface(TRANSFORM_FEEDBACK_BUFFER)"),
            ProgramInterface::TRANSFORM_FEEDBACK_VARYING => write!(fmt, "ProgramInterface(TRANSFORM_FEEDBACK_VARYING)"),
            ProgramInterface::UNIFORM => write!(fmt, "ProgramInterface(UNIFORM)"),
            ProgramInterface::UNIFORM_BLOCK => write!(fmt, "ProgramInterface(UNIFORM_BLOCK)"),
            ProgramInterface::VERTEX_SUBROUTINE => write!(fmt, "ProgramInterface(VERTEX_SUBROUTINE)"),
            ProgramInterface::VERTEX_SUBROUTINE_UNIFORM => write!(fmt, "ProgramInterface(VERTEX_SUBROUTINE_UNIFORM)"),
            _ => write!(fmt, "ProgramInterface({})", self.0),
        }
    }
}

impl_enum_traits!(ProgramInterface);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProgramInterfacePName(pub types::GLenum);

impl ProgramInterfacePName {
    pub const ACTIVE_RESOURCES: ProgramInterfacePName = ProgramInterfacePName(super::ACTIVE_RESOURCES);
    pub const MAX_NAME_LENGTH: ProgramInterfacePName = ProgramInterfacePName(super::MAX_NAME_LENGTH);
    pub const MAX_NUM_ACTIVE_VARIABLES: ProgramInterfacePName = ProgramInterfacePName(super::MAX_NUM_ACTIVE_VARIABLES);
    pub const MAX_NUM_COMPATIBLE_SUBROUTINES: ProgramInterfacePName = ProgramInterfacePName(super::MAX_NUM_COMPATIBLE_SUBROUTINES);
}

impl ::std::fmt::Debug for ProgramInterfacePName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ProgramInterfacePName::ACTIVE_RESOURCES => write!(fmt, "ProgramInterfacePName(ACTIVE_RESOURCES)"),
            ProgramInterfacePName::MAX_NAME_LENGTH => write!(fmt, "ProgramInterfacePName(MAX_NAME_LENGTH)"),
            ProgramInterfacePName::MAX_NUM_ACTIVE_VARIABLES => write!(fmt, "ProgramInterfacePName(MAX_NUM_ACTIVE_VARIABLES)"),
            ProgramInterfacePName::MAX_NUM_COMPATIBLE_SUBROUTINES => write!(fmt, "ProgramInterfacePName(MAX_NUM_COMPATIBLE_SUBROUTINES)"),
            _ => write!(fmt, "ProgramInterfacePName({})", self.0),
        }
    }
}

impl_enum_traits!(ProgramInterfacePName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProgramParameterPName(pub types::GLenum);

impl ProgramParameterPName {
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: ProgramParameterPName = ProgramParameterPName(super::PROGRAM_BINARY_RETRIEVABLE_HINT);
    pub const PROGRAM_SEPARABLE: ProgramParameterPName = ProgramParameterPName(super::PROGRAM_SEPARABLE);
}

impl ::std::fmt::Debug for ProgramParameterPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ProgramParameterPName::PROGRAM_BINARY_RETRIEVABLE_HINT => write!(fmt, "ProgramParameterPName(PROGRAM_BINARY_RETRIEVABLE_HINT)"),
            ProgramParameterPName::PROGRAM_SEPARABLE => write!(fmt, "ProgramParameterPName(PROGRAM_SEPARABLE)"),
            _ => write!(fmt, "ProgramParameterPName({})", self.0),
        }
    }
}

impl_enum_traits!(ProgramParameterPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProgramPropertyARB(pub types::GLenum);

impl ProgramPropertyARB {
    pub const DELETE_STATUS: ProgramPropertyARB = ProgramPropertyARB(super::DELETE_STATUS);
    pub const LINK_STATUS: ProgramPropertyARB = ProgramPropertyARB(super::LINK_STATUS);
    pub const VALIDATE_STATUS: ProgramPropertyARB = ProgramPropertyARB(super::VALIDATE_STATUS);
    pub const INFO_LOG_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::INFO_LOG_LENGTH);
    pub const ATTACHED_SHADERS: ProgramPropertyARB = ProgramPropertyARB(super::ATTACHED_SHADERS);
    pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_ATOMIC_COUNTER_BUFFERS);
    pub const ACTIVE_ATTRIBUTES: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_ATTRIBUTES);
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    pub const ACTIVE_UNIFORMS: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_UNIFORMS);
    pub const ACTIVE_UNIFORM_BLOCKS: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_UNIFORM_BLOCKS);
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH);
    pub const ACTIVE_UNIFORM_MAX_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::ACTIVE_UNIFORM_MAX_LENGTH);
    pub const COMPUTE_WORK_GROUP_SIZE: ProgramPropertyARB = ProgramPropertyARB(super::COMPUTE_WORK_GROUP_SIZE);
    pub const PROGRAM_BINARY_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::PROGRAM_BINARY_LENGTH);
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: ProgramPropertyARB = ProgramPropertyARB(super::TRANSFORM_FEEDBACK_BUFFER_MODE);
    pub const TRANSFORM_FEEDBACK_VARYINGS: ProgramPropertyARB = ProgramPropertyARB(super::TRANSFORM_FEEDBACK_VARYINGS);
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: ProgramPropertyARB = ProgramPropertyARB(super::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH);
    pub const GEOMETRY_VERTICES_OUT: ProgramPropertyARB = ProgramPropertyARB(super::GEOMETRY_VERTICES_OUT);
    pub const GEOMETRY_INPUT_TYPE: ProgramPropertyARB = ProgramPropertyARB(super::GEOMETRY_INPUT_TYPE);
    pub const GEOMETRY_OUTPUT_TYPE: ProgramPropertyARB = ProgramPropertyARB(super::GEOMETRY_OUTPUT_TYPE);
}

impl ::std::fmt::Debug for ProgramPropertyARB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ProgramPropertyARB::ACTIVE_ATOMIC_COUNTER_BUFFERS => write!(fmt, "ProgramPropertyARB(ACTIVE_ATOMIC_COUNTER_BUFFERS)"),
            ProgramPropertyARB::ACTIVE_ATTRIBUTES => write!(fmt, "ProgramPropertyARB(ACTIVE_ATTRIBUTES)"),
            ProgramPropertyARB::ACTIVE_ATTRIBUTE_MAX_LENGTH => write!(fmt, "ProgramPropertyARB(ACTIVE_ATTRIBUTE_MAX_LENGTH)"),
            ProgramPropertyARB::ACTIVE_UNIFORMS => write!(fmt, "ProgramPropertyARB(ACTIVE_UNIFORMS)"),
            ProgramPropertyARB::ACTIVE_UNIFORM_BLOCKS => write!(fmt, "ProgramPropertyARB(ACTIVE_UNIFORM_BLOCKS)"),
            ProgramPropertyARB::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH => write!(fmt, "ProgramPropertyARB(ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH)"),
            ProgramPropertyARB::ACTIVE_UNIFORM_MAX_LENGTH => write!(fmt, "ProgramPropertyARB(ACTIVE_UNIFORM_MAX_LENGTH)"),
            ProgramPropertyARB::ATTACHED_SHADERS => write!(fmt, "ProgramPropertyARB(ATTACHED_SHADERS)"),
            ProgramPropertyARB::COMPUTE_WORK_GROUP_SIZE => write!(fmt, "ProgramPropertyARB(COMPUTE_WORK_GROUP_SIZE)"),
            ProgramPropertyARB::DELETE_STATUS => write!(fmt, "ProgramPropertyARB(DELETE_STATUS)"),
            ProgramPropertyARB::GEOMETRY_INPUT_TYPE => write!(fmt, "ProgramPropertyARB(GEOMETRY_INPUT_TYPE)"),
            ProgramPropertyARB::GEOMETRY_OUTPUT_TYPE => write!(fmt, "ProgramPropertyARB(GEOMETRY_OUTPUT_TYPE)"),
            ProgramPropertyARB::GEOMETRY_VERTICES_OUT => write!(fmt, "ProgramPropertyARB(GEOMETRY_VERTICES_OUT)"),
            ProgramPropertyARB::INFO_LOG_LENGTH => write!(fmt, "ProgramPropertyARB(INFO_LOG_LENGTH)"),
            ProgramPropertyARB::LINK_STATUS => write!(fmt, "ProgramPropertyARB(LINK_STATUS)"),
            ProgramPropertyARB::PROGRAM_BINARY_LENGTH => write!(fmt, "ProgramPropertyARB(PROGRAM_BINARY_LENGTH)"),
            ProgramPropertyARB::TRANSFORM_FEEDBACK_BUFFER_MODE => write!(fmt, "ProgramPropertyARB(TRANSFORM_FEEDBACK_BUFFER_MODE)"),
            ProgramPropertyARB::TRANSFORM_FEEDBACK_VARYINGS => write!(fmt, "ProgramPropertyARB(TRANSFORM_FEEDBACK_VARYINGS)"),
            ProgramPropertyARB::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH => write!(fmt, "ProgramPropertyARB(TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH)"),
            ProgramPropertyARB::VALIDATE_STATUS => write!(fmt, "ProgramPropertyARB(VALIDATE_STATUS)"),
            _ => write!(fmt, "ProgramPropertyARB({})", self.0),
        }
    }
}

impl_enum_traits!(ProgramPropertyARB);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProgramStagePName(pub types::GLenum);

impl ProgramStagePName {
    pub const ACTIVE_SUBROUTINE_UNIFORMS: ProgramStagePName = ProgramStagePName(super::ACTIVE_SUBROUTINE_UNIFORMS);
    pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: ProgramStagePName = ProgramStagePName(super::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS);
    pub const ACTIVE_SUBROUTINES: ProgramStagePName = ProgramStagePName(super::ACTIVE_SUBROUTINES);
    pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: ProgramStagePName = ProgramStagePName(super::ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH);
    pub const ACTIVE_SUBROUTINE_MAX_LENGTH: ProgramStagePName = ProgramStagePName(super::ACTIVE_SUBROUTINE_MAX_LENGTH);
}

impl ::std::fmt::Debug for ProgramStagePName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ProgramStagePName::ACTIVE_SUBROUTINES => write!(fmt, "ProgramStagePName(ACTIVE_SUBROUTINES)"),
            ProgramStagePName::ACTIVE_SUBROUTINE_MAX_LENGTH => write!(fmt, "ProgramStagePName(ACTIVE_SUBROUTINE_MAX_LENGTH)"),
            ProgramStagePName::ACTIVE_SUBROUTINE_UNIFORMS => write!(fmt, "ProgramStagePName(ACTIVE_SUBROUTINE_UNIFORMS)"),
            ProgramStagePName::ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS => write!(fmt, "ProgramStagePName(ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS)"),
            ProgramStagePName::ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH => write!(fmt, "ProgramStagePName(ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH)"),
            _ => write!(fmt, "ProgramStagePName({})", self.0),
        }
    }
}

impl_enum_traits!(ProgramStagePName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct QueryCounterTarget(pub types::GLenum);

impl QueryCounterTarget {
    pub const TIMESTAMP: QueryCounterTarget = QueryCounterTarget(super::TIMESTAMP);
}

impl ::std::fmt::Debug for QueryCounterTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            QueryCounterTarget::TIMESTAMP => write!(fmt, "QueryCounterTarget(TIMESTAMP)"),
            _ => write!(fmt, "QueryCounterTarget({})", self.0),
        }
    }
}

impl_enum_traits!(QueryCounterTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct QueryObjectParameterName(pub types::GLenum);

impl QueryObjectParameterName {
    pub const QUERY_RESULT_AVAILABLE: QueryObjectParameterName = QueryObjectParameterName(super::QUERY_RESULT_AVAILABLE);
    pub const QUERY_RESULT: QueryObjectParameterName = QueryObjectParameterName(super::QUERY_RESULT);
    pub const QUERY_RESULT_NO_WAIT: QueryObjectParameterName = QueryObjectParameterName(super::QUERY_RESULT_NO_WAIT);
    pub const QUERY_TARGET: QueryObjectParameterName = QueryObjectParameterName(super::QUERY_TARGET);
}

impl ::std::fmt::Debug for QueryObjectParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            QueryObjectParameterName::QUERY_RESULT => write!(fmt, "QueryObjectParameterName(QUERY_RESULT)"),
            QueryObjectParameterName::QUERY_RESULT_AVAILABLE => write!(fmt, "QueryObjectParameterName(QUERY_RESULT_AVAILABLE)"),
            QueryObjectParameterName::QUERY_RESULT_NO_WAIT => write!(fmt, "QueryObjectParameterName(QUERY_RESULT_NO_WAIT)"),
            QueryObjectParameterName::QUERY_TARGET => write!(fmt, "QueryObjectParameterName(QUERY_TARGET)"),
            _ => write!(fmt, "QueryObjectParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(QueryObjectParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct QueryParameterName(pub types::GLenum);

impl QueryParameterName {
    pub const CURRENT_QUERY: QueryParameterName = QueryParameterName(super::CURRENT_QUERY);
    pub const QUERY_COUNTER_BITS: QueryParameterName = QueryParameterName(super::QUERY_COUNTER_BITS);
}

impl ::std::fmt::Debug for QueryParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            QueryParameterName::CURRENT_QUERY => write!(fmt, "QueryParameterName(CURRENT_QUERY)"),
            QueryParameterName::QUERY_COUNTER_BITS => write!(fmt, "QueryParameterName(QUERY_COUNTER_BITS)"),
            _ => write!(fmt, "QueryParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(QueryParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct QueryTarget(pub types::GLenum);

impl QueryTarget {
    pub const SAMPLES_PASSED: QueryTarget = QueryTarget(super::SAMPLES_PASSED);
    pub const ANY_SAMPLES_PASSED: QueryTarget = QueryTarget(super::ANY_SAMPLES_PASSED);
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: QueryTarget = QueryTarget(super::ANY_SAMPLES_PASSED_CONSERVATIVE);
    pub const PRIMITIVES_GENERATED: QueryTarget = QueryTarget(super::PRIMITIVES_GENERATED);
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: QueryTarget = QueryTarget(super::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN);
    pub const TIME_ELAPSED: QueryTarget = QueryTarget(super::TIME_ELAPSED);
}

impl ::std::fmt::Debug for QueryTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            QueryTarget::ANY_SAMPLES_PASSED => write!(fmt, "QueryTarget(ANY_SAMPLES_PASSED)"),
            QueryTarget::ANY_SAMPLES_PASSED_CONSERVATIVE => write!(fmt, "QueryTarget(ANY_SAMPLES_PASSED_CONSERVATIVE)"),
            QueryTarget::PRIMITIVES_GENERATED => write!(fmt, "QueryTarget(PRIMITIVES_GENERATED)"),
            QueryTarget::SAMPLES_PASSED => write!(fmt, "QueryTarget(SAMPLES_PASSED)"),
            QueryTarget::TIME_ELAPSED => write!(fmt, "QueryTarget(TIME_ELAPSED)"),
            QueryTarget::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN => write!(fmt, "QueryTarget(TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN)"),
            _ => write!(fmt, "QueryTarget({})", self.0),
        }
    }
}

impl_enum_traits!(QueryTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ReadBufferMode(pub types::GLenum);

impl ReadBufferMode {
    pub const BACK: ReadBufferMode = ReadBufferMode(super::BACK);
    pub const BACK_LEFT: ReadBufferMode = ReadBufferMode(super::BACK_LEFT);
    pub const BACK_RIGHT: ReadBufferMode = ReadBufferMode(super::BACK_RIGHT);
    pub const FRONT: ReadBufferMode = ReadBufferMode(super::FRONT);
    pub const FRONT_LEFT: ReadBufferMode = ReadBufferMode(super::FRONT_LEFT);
    pub const FRONT_RIGHT: ReadBufferMode = ReadBufferMode(super::FRONT_RIGHT);
    pub const LEFT: ReadBufferMode = ReadBufferMode(super::LEFT);
    pub const RIGHT: ReadBufferMode = ReadBufferMode(super::RIGHT);
}

impl ::std::fmt::Debug for ReadBufferMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ReadBufferMode::BACK => write!(fmt, "ReadBufferMode(BACK)"),
            ReadBufferMode::BACK_LEFT => write!(fmt, "ReadBufferMode(BACK_LEFT)"),
            ReadBufferMode::BACK_RIGHT => write!(fmt, "ReadBufferMode(BACK_RIGHT)"),
            ReadBufferMode::FRONT => write!(fmt, "ReadBufferMode(FRONT)"),
            ReadBufferMode::FRONT_LEFT => write!(fmt, "ReadBufferMode(FRONT_LEFT)"),
            ReadBufferMode::FRONT_RIGHT => write!(fmt, "ReadBufferMode(FRONT_RIGHT)"),
            ReadBufferMode::LEFT => write!(fmt, "ReadBufferMode(LEFT)"),
            ReadBufferMode::RIGHT => write!(fmt, "ReadBufferMode(RIGHT)"),
            _ => write!(fmt, "ReadBufferMode({})", self.0),
        }
    }
}

impl_enum_traits!(ReadBufferMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct RenderbufferParameterName(pub types::GLenum);

impl RenderbufferParameterName {
    pub const RENDERBUFFER_WIDTH: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_WIDTH);
    pub const RENDERBUFFER_HEIGHT: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_HEIGHT);
    pub const RENDERBUFFER_INTERNAL_FORMAT: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_INTERNAL_FORMAT);
    pub const RENDERBUFFER_SAMPLES: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_SAMPLES);
    pub const RENDERBUFFER_RED_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_RED_SIZE);
    pub const RENDERBUFFER_GREEN_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_GREEN_SIZE);
    pub const RENDERBUFFER_BLUE_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_BLUE_SIZE);
    pub const RENDERBUFFER_ALPHA_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_ALPHA_SIZE);
    pub const RENDERBUFFER_DEPTH_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_DEPTH_SIZE);
    pub const RENDERBUFFER_STENCIL_SIZE: RenderbufferParameterName = RenderbufferParameterName(super::RENDERBUFFER_STENCIL_SIZE);
}

impl ::std::fmt::Debug for RenderbufferParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RenderbufferParameterName::RENDERBUFFER_ALPHA_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_ALPHA_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_BLUE_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_BLUE_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_DEPTH_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_DEPTH_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_GREEN_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_GREEN_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_HEIGHT => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_HEIGHT)"),
            RenderbufferParameterName::RENDERBUFFER_INTERNAL_FORMAT => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_INTERNAL_FORMAT)"),
            RenderbufferParameterName::RENDERBUFFER_RED_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_RED_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_SAMPLES => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_SAMPLES)"),
            RenderbufferParameterName::RENDERBUFFER_STENCIL_SIZE => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_STENCIL_SIZE)"),
            RenderbufferParameterName::RENDERBUFFER_WIDTH => write!(fmt, "RenderbufferParameterName(RENDERBUFFER_WIDTH)"),
            _ => write!(fmt, "RenderbufferParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(RenderbufferParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct RenderbufferTarget(pub types::GLenum);

impl RenderbufferTarget {
    pub const RENDERBUFFER: RenderbufferTarget = RenderbufferTarget(super::RENDERBUFFER);
}

impl ::std::fmt::Debug for RenderbufferTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RenderbufferTarget::RENDERBUFFER => write!(fmt, "RenderbufferTarget(RENDERBUFFER)"),
            _ => write!(fmt, "RenderbufferTarget({})", self.0),
        }
    }
}

impl_enum_traits!(RenderbufferTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct RenderingMode(pub types::GLenum);

impl RenderingMode {
}

impl ::std::fmt::Debug for RenderingMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "RenderingMode({})", self.0),
        }
    }
}

impl_enum_traits!(RenderingMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SamplePatternSGIS(pub types::GLenum);

impl SamplePatternSGIS {
}

impl ::std::fmt::Debug for SamplePatternSGIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "SamplePatternSGIS({})", self.0),
        }
    }
}

impl_enum_traits!(SamplePatternSGIS);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SamplerParameterName(pub types::GLenum);

impl SamplerParameterName {
    pub const TEXTURE_WRAP_S: SamplerParameterName = SamplerParameterName(super::TEXTURE_WRAP_S);
    pub const TEXTURE_WRAP_T: SamplerParameterName = SamplerParameterName(super::TEXTURE_WRAP_T);
    pub const TEXTURE_WRAP_R: SamplerParameterName = SamplerParameterName(super::TEXTURE_WRAP_R);
    pub const TEXTURE_MIN_FILTER: SamplerParameterName = SamplerParameterName(super::TEXTURE_MIN_FILTER);
    pub const TEXTURE_MAG_FILTER: SamplerParameterName = SamplerParameterName(super::TEXTURE_MAG_FILTER);
    pub const TEXTURE_BORDER_COLOR: SamplerParameterName = SamplerParameterName(super::TEXTURE_BORDER_COLOR);
    pub const TEXTURE_MIN_LOD: SamplerParameterName = SamplerParameterName(super::TEXTURE_MIN_LOD);
    pub const TEXTURE_MAX_LOD: SamplerParameterName = SamplerParameterName(super::TEXTURE_MAX_LOD);
    pub const TEXTURE_COMPARE_MODE: SamplerParameterName = SamplerParameterName(super::TEXTURE_COMPARE_MODE);
    pub const TEXTURE_COMPARE_FUNC: SamplerParameterName = SamplerParameterName(super::TEXTURE_COMPARE_FUNC);
}

impl ::std::fmt::Debug for SamplerParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SamplerParameterName::TEXTURE_BORDER_COLOR => write!(fmt, "SamplerParameterName(TEXTURE_BORDER_COLOR)"),
            SamplerParameterName::TEXTURE_COMPARE_FUNC => write!(fmt, "SamplerParameterName(TEXTURE_COMPARE_FUNC)"),
            SamplerParameterName::TEXTURE_COMPARE_MODE => write!(fmt, "SamplerParameterName(TEXTURE_COMPARE_MODE)"),
            SamplerParameterName::TEXTURE_MAG_FILTER => write!(fmt, "SamplerParameterName(TEXTURE_MAG_FILTER)"),
            SamplerParameterName::TEXTURE_MAX_LOD => write!(fmt, "SamplerParameterName(TEXTURE_MAX_LOD)"),
            SamplerParameterName::TEXTURE_MIN_FILTER => write!(fmt, "SamplerParameterName(TEXTURE_MIN_FILTER)"),
            SamplerParameterName::TEXTURE_MIN_LOD => write!(fmt, "SamplerParameterName(TEXTURE_MIN_LOD)"),
            SamplerParameterName::TEXTURE_WRAP_R => write!(fmt, "SamplerParameterName(TEXTURE_WRAP_R)"),
            SamplerParameterName::TEXTURE_WRAP_S => write!(fmt, "SamplerParameterName(TEXTURE_WRAP_S)"),
            SamplerParameterName::TEXTURE_WRAP_T => write!(fmt, "SamplerParameterName(TEXTURE_WRAP_T)"),
            _ => write!(fmt, "SamplerParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(SamplerParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SemaphoreParameterName(pub types::GLenum);

impl SemaphoreParameterName {
}

impl ::std::fmt::Debug for SemaphoreParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "SemaphoreParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(SemaphoreParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SeparableTargetEXT(pub types::GLenum);

impl SeparableTargetEXT {
}

impl ::std::fmt::Debug for SeparableTargetEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "SeparableTargetEXT({})", self.0),
        }
    }
}

impl_enum_traits!(SeparableTargetEXT);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ShaderParameterName(pub types::GLenum);

impl ShaderParameterName {
    pub const SHADER_TYPE: ShaderParameterName = ShaderParameterName(super::SHADER_TYPE);
    pub const DELETE_STATUS: ShaderParameterName = ShaderParameterName(super::DELETE_STATUS);
    pub const COMPILE_STATUS: ShaderParameterName = ShaderParameterName(super::COMPILE_STATUS);
    pub const INFO_LOG_LENGTH: ShaderParameterName = ShaderParameterName(super::INFO_LOG_LENGTH);
    pub const SHADER_SOURCE_LENGTH: ShaderParameterName = ShaderParameterName(super::SHADER_SOURCE_LENGTH);
}

impl ::std::fmt::Debug for ShaderParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ShaderParameterName::COMPILE_STATUS => write!(fmt, "ShaderParameterName(COMPILE_STATUS)"),
            ShaderParameterName::DELETE_STATUS => write!(fmt, "ShaderParameterName(DELETE_STATUS)"),
            ShaderParameterName::INFO_LOG_LENGTH => write!(fmt, "ShaderParameterName(INFO_LOG_LENGTH)"),
            ShaderParameterName::SHADER_SOURCE_LENGTH => write!(fmt, "ShaderParameterName(SHADER_SOURCE_LENGTH)"),
            ShaderParameterName::SHADER_TYPE => write!(fmt, "ShaderParameterName(SHADER_TYPE)"),
            _ => write!(fmt, "ShaderParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(ShaderParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ShaderType(pub types::GLenum);

impl ShaderType {
    pub const COMPUTE_SHADER: ShaderType = ShaderType(super::COMPUTE_SHADER);
    pub const VERTEX_SHADER: ShaderType = ShaderType(super::VERTEX_SHADER);
    pub const TESS_CONTROL_SHADER: ShaderType = ShaderType(super::TESS_CONTROL_SHADER);
    pub const TESS_EVALUATION_SHADER: ShaderType = ShaderType(super::TESS_EVALUATION_SHADER);
    pub const GEOMETRY_SHADER: ShaderType = ShaderType(super::GEOMETRY_SHADER);
    pub const FRAGMENT_SHADER: ShaderType = ShaderType(super::FRAGMENT_SHADER);
}

impl ::std::fmt::Debug for ShaderType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ShaderType::COMPUTE_SHADER => write!(fmt, "ShaderType(COMPUTE_SHADER)"),
            ShaderType::FRAGMENT_SHADER => write!(fmt, "ShaderType(FRAGMENT_SHADER)"),
            ShaderType::GEOMETRY_SHADER => write!(fmt, "ShaderType(GEOMETRY_SHADER)"),
            ShaderType::TESS_CONTROL_SHADER => write!(fmt, "ShaderType(TESS_CONTROL_SHADER)"),
            ShaderType::TESS_EVALUATION_SHADER => write!(fmt, "ShaderType(TESS_EVALUATION_SHADER)"),
            ShaderType::VERTEX_SHADER => write!(fmt, "ShaderType(VERTEX_SHADER)"),
            _ => write!(fmt, "ShaderType({})", self.0),
        }
    }
}

impl_enum_traits!(ShaderType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ShadingModel(pub types::GLenum);

impl ShadingModel {
}

impl ::std::fmt::Debug for ShadingModel {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "ShadingModel({})", self.0),
        }
    }
}

impl_enum_traits!(ShadingModel);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct StencilFaceDirection(pub types::GLenum);

impl StencilFaceDirection {
    pub const FRONT: StencilFaceDirection = StencilFaceDirection(super::FRONT);
    pub const BACK: StencilFaceDirection = StencilFaceDirection(super::BACK);
    pub const FRONT_AND_BACK: StencilFaceDirection = StencilFaceDirection(super::FRONT_AND_BACK);
}

impl ::std::fmt::Debug for StencilFaceDirection {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StencilFaceDirection::BACK => write!(fmt, "StencilFaceDirection(BACK)"),
            StencilFaceDirection::FRONT => write!(fmt, "StencilFaceDirection(FRONT)"),
            StencilFaceDirection::FRONT_AND_BACK => write!(fmt, "StencilFaceDirection(FRONT_AND_BACK)"),
            _ => write!(fmt, "StencilFaceDirection({})", self.0),
        }
    }
}

impl_enum_traits!(StencilFaceDirection);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct StencilFunction(pub types::GLenum);

impl StencilFunction {
    pub const ALWAYS: StencilFunction = StencilFunction(super::ALWAYS);
    pub const EQUAL: StencilFunction = StencilFunction(super::EQUAL);
    pub const GEQUAL: StencilFunction = StencilFunction(super::GEQUAL);
    pub const GREATER: StencilFunction = StencilFunction(super::GREATER);
    pub const LEQUAL: StencilFunction = StencilFunction(super::LEQUAL);
    pub const LESS: StencilFunction = StencilFunction(super::LESS);
    pub const NEVER: StencilFunction = StencilFunction(super::NEVER);
    pub const NOTEQUAL: StencilFunction = StencilFunction(super::NOTEQUAL);
}

impl ::std::fmt::Debug for StencilFunction {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StencilFunction::ALWAYS => write!(fmt, "StencilFunction(ALWAYS)"),
            StencilFunction::EQUAL => write!(fmt, "StencilFunction(EQUAL)"),
            StencilFunction::GEQUAL => write!(fmt, "StencilFunction(GEQUAL)"),
            StencilFunction::GREATER => write!(fmt, "StencilFunction(GREATER)"),
            StencilFunction::LEQUAL => write!(fmt, "StencilFunction(LEQUAL)"),
            StencilFunction::LESS => write!(fmt, "StencilFunction(LESS)"),
            StencilFunction::NEVER => write!(fmt, "StencilFunction(NEVER)"),
            StencilFunction::NOTEQUAL => write!(fmt, "StencilFunction(NOTEQUAL)"),
            _ => write!(fmt, "StencilFunction({})", self.0),
        }
    }
}

impl_enum_traits!(StencilFunction);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct StencilOp(pub types::GLenum);

impl StencilOp {
    pub const DECR: StencilOp = StencilOp(super::DECR);
    pub const DECR_WRAP: StencilOp = StencilOp(super::DECR_WRAP);
    pub const INCR: StencilOp = StencilOp(super::INCR);
    pub const INCR_WRAP: StencilOp = StencilOp(super::INCR_WRAP);
    pub const INVERT: StencilOp = StencilOp(super::INVERT);
    pub const KEEP: StencilOp = StencilOp(super::KEEP);
    pub const REPLACE: StencilOp = StencilOp(super::REPLACE);
    pub const ZERO: StencilOp = StencilOp(super::ZERO);
}

impl ::std::fmt::Debug for StencilOp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StencilOp::DECR => write!(fmt, "StencilOp(DECR)"),
            StencilOp::DECR_WRAP => write!(fmt, "StencilOp(DECR_WRAP)"),
            StencilOp::INCR => write!(fmt, "StencilOp(INCR)"),
            StencilOp::INCR_WRAP => write!(fmt, "StencilOp(INCR_WRAP)"),
            StencilOp::INVERT => write!(fmt, "StencilOp(INVERT)"),
            StencilOp::KEEP => write!(fmt, "StencilOp(KEEP)"),
            StencilOp::REPLACE => write!(fmt, "StencilOp(REPLACE)"),
            StencilOp::ZERO => write!(fmt, "StencilOp(ZERO)"),
            _ => write!(fmt, "StencilOp({})", self.0),
        }
    }
}

impl_enum_traits!(StencilOp);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct StringName(pub types::GLenum);

impl StringName {
    pub const EXTENSIONS: StringName = StringName(super::EXTENSIONS);
    pub const RENDERER: StringName = StringName(super::RENDERER);
    pub const VENDOR: StringName = StringName(super::VENDOR);
    pub const VERSION: StringName = StringName(super::VERSION);
    pub const SHADING_LANGUAGE_VERSION: StringName = StringName(super::SHADING_LANGUAGE_VERSION);
}

impl ::std::fmt::Debug for StringName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StringName::EXTENSIONS => write!(fmt, "StringName(EXTENSIONS)"),
            StringName::RENDERER => write!(fmt, "StringName(RENDERER)"),
            StringName::SHADING_LANGUAGE_VERSION => write!(fmt, "StringName(SHADING_LANGUAGE_VERSION)"),
            StringName::VENDOR => write!(fmt, "StringName(VENDOR)"),
            StringName::VERSION => write!(fmt, "StringName(VERSION)"),
            _ => write!(fmt, "StringName({})", self.0),
        }
    }
}

impl_enum_traits!(StringName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SubroutineParameterName(pub types::GLenum);

impl SubroutineParameterName {
    pub const NUM_COMPATIBLE_SUBROUTINES: SubroutineParameterName = SubroutineParameterName(super::NUM_COMPATIBLE_SUBROUTINES);
    pub const COMPATIBLE_SUBROUTINES: SubroutineParameterName = SubroutineParameterName(super::COMPATIBLE_SUBROUTINES);
    pub const UNIFORM_SIZE: SubroutineParameterName = SubroutineParameterName(super::UNIFORM_SIZE);
    pub const UNIFORM_NAME_LENGTH: SubroutineParameterName = SubroutineParameterName(super::UNIFORM_NAME_LENGTH);
}

impl ::std::fmt::Debug for SubroutineParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SubroutineParameterName::COMPATIBLE_SUBROUTINES => write!(fmt, "SubroutineParameterName(COMPATIBLE_SUBROUTINES)"),
            SubroutineParameterName::NUM_COMPATIBLE_SUBROUTINES => write!(fmt, "SubroutineParameterName(NUM_COMPATIBLE_SUBROUTINES)"),
            SubroutineParameterName::UNIFORM_NAME_LENGTH => write!(fmt, "SubroutineParameterName(UNIFORM_NAME_LENGTH)"),
            SubroutineParameterName::UNIFORM_SIZE => write!(fmt, "SubroutineParameterName(UNIFORM_SIZE)"),
            _ => write!(fmt, "SubroutineParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(SubroutineParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SyncCondition(pub types::GLenum);

impl SyncCondition {
    pub const SYNC_GPU_COMMANDS_COMPLETE: SyncCondition = SyncCondition(super::SYNC_GPU_COMMANDS_COMPLETE);
}

impl ::std::fmt::Debug for SyncCondition {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SyncCondition::SYNC_GPU_COMMANDS_COMPLETE => write!(fmt, "SyncCondition(SYNC_GPU_COMMANDS_COMPLETE)"),
            _ => write!(fmt, "SyncCondition({})", self.0),
        }
    }
}

impl_enum_traits!(SyncCondition);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SyncObjectMask(pub types::GLenum);

impl SyncObjectMask {
    pub const SYNC_FLUSH_COMMANDS_BIT: SyncObjectMask = SyncObjectMask(super::SYNC_FLUSH_COMMANDS_BIT);
    pub const Empty: SyncObjectMask = SyncObjectMask(0);
}

impl ::std::fmt::Debug for SyncObjectMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SyncObjectMask::SYNC_FLUSH_COMMANDS_BIT => write!(fmt, "SyncObjectMask(SYNC_FLUSH_COMMANDS_BIT)"),
            _ => write!(fmt, "SyncObjectMask({})", self.0),
        }
    }
}

impl_enum_traits!(SyncObjectMask);

impl_enum_bitmask_traits!(SyncObjectMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SyncParameterName(pub types::GLenum);

impl SyncParameterName {
    pub const OBJECT_TYPE: SyncParameterName = SyncParameterName(super::OBJECT_TYPE);
    pub const SYNC_STATUS: SyncParameterName = SyncParameterName(super::SYNC_STATUS);
    pub const SYNC_CONDITION: SyncParameterName = SyncParameterName(super::SYNC_CONDITION);
    pub const SYNC_FLAGS: SyncParameterName = SyncParameterName(super::SYNC_FLAGS);
}

impl ::std::fmt::Debug for SyncParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SyncParameterName::OBJECT_TYPE => write!(fmt, "SyncParameterName(OBJECT_TYPE)"),
            SyncParameterName::SYNC_CONDITION => write!(fmt, "SyncParameterName(SYNC_CONDITION)"),
            SyncParameterName::SYNC_FLAGS => write!(fmt, "SyncParameterName(SYNC_FLAGS)"),
            SyncParameterName::SYNC_STATUS => write!(fmt, "SyncParameterName(SYNC_STATUS)"),
            _ => write!(fmt, "SyncParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(SyncParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SyncStatus(pub types::GLenum);

impl SyncStatus {
    pub const ALREADY_SIGNALED: SyncStatus = SyncStatus(super::ALREADY_SIGNALED);
    pub const TIMEOUT_EXPIRED: SyncStatus = SyncStatus(super::TIMEOUT_EXPIRED);
    pub const CONDITION_SATISFIED: SyncStatus = SyncStatus(super::CONDITION_SATISFIED);
    pub const WAIT_FAILED: SyncStatus = SyncStatus(super::WAIT_FAILED);
}

impl ::std::fmt::Debug for SyncStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SyncStatus::ALREADY_SIGNALED => write!(fmt, "SyncStatus(ALREADY_SIGNALED)"),
            SyncStatus::CONDITION_SATISFIED => write!(fmt, "SyncStatus(CONDITION_SATISFIED)"),
            SyncStatus::TIMEOUT_EXPIRED => write!(fmt, "SyncStatus(TIMEOUT_EXPIRED)"),
            SyncStatus::WAIT_FAILED => write!(fmt, "SyncStatus(WAIT_FAILED)"),
            _ => write!(fmt, "SyncStatus({})", self.0),
        }
    }
}

impl_enum_traits!(SyncStatus);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TexCoordPointerType(pub types::GLenum);

impl TexCoordPointerType {
    pub const DOUBLE: TexCoordPointerType = TexCoordPointerType(super::DOUBLE);
    pub const FLOAT: TexCoordPointerType = TexCoordPointerType(super::FLOAT);
    pub const INT: TexCoordPointerType = TexCoordPointerType(super::INT);
    pub const SHORT: TexCoordPointerType = TexCoordPointerType(super::SHORT);
}

impl ::std::fmt::Debug for TexCoordPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TexCoordPointerType::DOUBLE => write!(fmt, "TexCoordPointerType(DOUBLE)"),
            TexCoordPointerType::FLOAT => write!(fmt, "TexCoordPointerType(FLOAT)"),
            TexCoordPointerType::INT => write!(fmt, "TexCoordPointerType(INT)"),
            TexCoordPointerType::SHORT => write!(fmt, "TexCoordPointerType(SHORT)"),
            _ => write!(fmt, "TexCoordPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(TexCoordPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureCoordName(pub types::GLenum);

impl TextureCoordName {
}

impl ::std::fmt::Debug for TextureCoordName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureCoordName({})", self.0),
        }
    }
}

impl_enum_traits!(TextureCoordName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureEnvMode(pub types::GLenum);

impl TextureEnvMode {
    pub const BLEND: TextureEnvMode = TextureEnvMode(super::BLEND);
}

impl ::std::fmt::Debug for TextureEnvMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureEnvMode::BLEND => write!(fmt, "TextureEnvMode(BLEND)"),
            _ => write!(fmt, "TextureEnvMode({})", self.0),
        }
    }
}

impl_enum_traits!(TextureEnvMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureEnvParameter(pub types::GLenum);

impl TextureEnvParameter {
}

impl ::std::fmt::Debug for TextureEnvParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureEnvParameter({})", self.0),
        }
    }
}

impl_enum_traits!(TextureEnvParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureEnvTarget(pub types::GLenum);

impl TextureEnvTarget {
}

impl ::std::fmt::Debug for TextureEnvTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureEnvTarget({})", self.0),
        }
    }
}

impl_enum_traits!(TextureEnvTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureFilterFuncSGIS(pub types::GLenum);

impl TextureFilterFuncSGIS {
}

impl ::std::fmt::Debug for TextureFilterFuncSGIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureFilterFuncSGIS({})", self.0),
        }
    }
}

impl_enum_traits!(TextureFilterFuncSGIS);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureGenMode(pub types::GLenum);

impl TextureGenMode {
}

impl ::std::fmt::Debug for TextureGenMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureGenMode({})", self.0),
        }
    }
}

impl_enum_traits!(TextureGenMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureGenParameter(pub types::GLenum);

impl TextureGenParameter {
}

impl ::std::fmt::Debug for TextureGenParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureGenParameter({})", self.0),
        }
    }
}

impl_enum_traits!(TextureGenParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureLayout(pub types::GLenum);

impl TextureLayout {
}

impl ::std::fmt::Debug for TextureLayout {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureLayout({})", self.0),
        }
    }
}

impl_enum_traits!(TextureLayout);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureMagFilter(pub types::GLenum);

impl TextureMagFilter {
    pub const LINEAR: TextureMagFilter = TextureMagFilter(super::LINEAR);
    pub const NEAREST: TextureMagFilter = TextureMagFilter(super::NEAREST);
}

impl ::std::fmt::Debug for TextureMagFilter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureMagFilter::LINEAR => write!(fmt, "TextureMagFilter(LINEAR)"),
            TextureMagFilter::NEAREST => write!(fmt, "TextureMagFilter(NEAREST)"),
            _ => write!(fmt, "TextureMagFilter({})", self.0),
        }
    }
}

impl_enum_traits!(TextureMagFilter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureMinFilter(pub types::GLenum);

impl TextureMinFilter {
    pub const LINEAR: TextureMinFilter = TextureMinFilter(super::LINEAR);
    pub const LINEAR_MIPMAP_LINEAR: TextureMinFilter = TextureMinFilter(super::LINEAR_MIPMAP_LINEAR);
    pub const LINEAR_MIPMAP_NEAREST: TextureMinFilter = TextureMinFilter(super::LINEAR_MIPMAP_NEAREST);
    pub const NEAREST: TextureMinFilter = TextureMinFilter(super::NEAREST);
    pub const NEAREST_MIPMAP_LINEAR: TextureMinFilter = TextureMinFilter(super::NEAREST_MIPMAP_LINEAR);
    pub const NEAREST_MIPMAP_NEAREST: TextureMinFilter = TextureMinFilter(super::NEAREST_MIPMAP_NEAREST);
}

impl ::std::fmt::Debug for TextureMinFilter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureMinFilter::LINEAR => write!(fmt, "TextureMinFilter(LINEAR)"),
            TextureMinFilter::LINEAR_MIPMAP_LINEAR => write!(fmt, "TextureMinFilter(LINEAR_MIPMAP_LINEAR)"),
            TextureMinFilter::LINEAR_MIPMAP_NEAREST => write!(fmt, "TextureMinFilter(LINEAR_MIPMAP_NEAREST)"),
            TextureMinFilter::NEAREST => write!(fmt, "TextureMinFilter(NEAREST)"),
            TextureMinFilter::NEAREST_MIPMAP_LINEAR => write!(fmt, "TextureMinFilter(NEAREST_MIPMAP_LINEAR)"),
            TextureMinFilter::NEAREST_MIPMAP_NEAREST => write!(fmt, "TextureMinFilter(NEAREST_MIPMAP_NEAREST)"),
            _ => write!(fmt, "TextureMinFilter({})", self.0),
        }
    }
}

impl_enum_traits!(TextureMinFilter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureParameterName(pub types::GLenum);

impl TextureParameterName {
    pub const TEXTURE_BORDER_COLOR: TextureParameterName = TextureParameterName(super::TEXTURE_BORDER_COLOR);
    pub const TEXTURE_MAG_FILTER: TextureParameterName = TextureParameterName(super::TEXTURE_MAG_FILTER);
    pub const TEXTURE_MIN_FILTER: TextureParameterName = TextureParameterName(super::TEXTURE_MIN_FILTER);
    pub const TEXTURE_WRAP_R: TextureParameterName = TextureParameterName(super::TEXTURE_WRAP_R);
    pub const TEXTURE_WRAP_S: TextureParameterName = TextureParameterName(super::TEXTURE_WRAP_S);
    pub const TEXTURE_WRAP_T: TextureParameterName = TextureParameterName(super::TEXTURE_WRAP_T);
    pub const TEXTURE_BASE_LEVEL: TextureParameterName = TextureParameterName(super::TEXTURE_BASE_LEVEL);
    pub const TEXTURE_COMPARE_MODE: TextureParameterName = TextureParameterName(super::TEXTURE_COMPARE_MODE);
    pub const TEXTURE_COMPARE_FUNC: TextureParameterName = TextureParameterName(super::TEXTURE_COMPARE_FUNC);
    pub const TEXTURE_LOD_BIAS: TextureParameterName = TextureParameterName(super::TEXTURE_LOD_BIAS);
    pub const TEXTURE_MIN_LOD: TextureParameterName = TextureParameterName(super::TEXTURE_MIN_LOD);
    pub const TEXTURE_MAX_LOD: TextureParameterName = TextureParameterName(super::TEXTURE_MAX_LOD);
    pub const TEXTURE_MAX_LEVEL: TextureParameterName = TextureParameterName(super::TEXTURE_MAX_LEVEL);
    pub const TEXTURE_SWIZZLE_R: TextureParameterName = TextureParameterName(super::TEXTURE_SWIZZLE_R);
    pub const TEXTURE_SWIZZLE_G: TextureParameterName = TextureParameterName(super::TEXTURE_SWIZZLE_G);
    pub const TEXTURE_SWIZZLE_B: TextureParameterName = TextureParameterName(super::TEXTURE_SWIZZLE_B);
    pub const TEXTURE_SWIZZLE_A: TextureParameterName = TextureParameterName(super::TEXTURE_SWIZZLE_A);
    pub const TEXTURE_SWIZZLE_RGBA: TextureParameterName = TextureParameterName(super::TEXTURE_SWIZZLE_RGBA);
    pub const DEPTH_STENCIL_TEXTURE_MODE: TextureParameterName = TextureParameterName(super::DEPTH_STENCIL_TEXTURE_MODE);
    pub const TEXTURE_ALPHA_SIZE: TextureParameterName = TextureParameterName(super::TEXTURE_ALPHA_SIZE);
    pub const TEXTURE_BLUE_SIZE: TextureParameterName = TextureParameterName(super::TEXTURE_BLUE_SIZE);
    pub const TEXTURE_GREEN_SIZE: TextureParameterName = TextureParameterName(super::TEXTURE_GREEN_SIZE);
    pub const TEXTURE_HEIGHT: TextureParameterName = TextureParameterName(super::TEXTURE_HEIGHT);
    pub const TEXTURE_INTERNAL_FORMAT: TextureParameterName = TextureParameterName(super::TEXTURE_INTERNAL_FORMAT);
    pub const TEXTURE_RED_SIZE: TextureParameterName = TextureParameterName(super::TEXTURE_RED_SIZE);
    pub const TEXTURE_WIDTH: TextureParameterName = TextureParameterName(super::TEXTURE_WIDTH);
}

impl ::std::fmt::Debug for TextureParameterName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureParameterName::DEPTH_STENCIL_TEXTURE_MODE => write!(fmt, "TextureParameterName(DEPTH_STENCIL_TEXTURE_MODE)"),
            TextureParameterName::TEXTURE_ALPHA_SIZE => write!(fmt, "TextureParameterName(TEXTURE_ALPHA_SIZE)"),
            TextureParameterName::TEXTURE_BASE_LEVEL => write!(fmt, "TextureParameterName(TEXTURE_BASE_LEVEL)"),
            TextureParameterName::TEXTURE_BLUE_SIZE => write!(fmt, "TextureParameterName(TEXTURE_BLUE_SIZE)"),
            TextureParameterName::TEXTURE_BORDER_COLOR => write!(fmt, "TextureParameterName(TEXTURE_BORDER_COLOR)"),
            TextureParameterName::TEXTURE_COMPARE_FUNC => write!(fmt, "TextureParameterName(TEXTURE_COMPARE_FUNC)"),
            TextureParameterName::TEXTURE_COMPARE_MODE => write!(fmt, "TextureParameterName(TEXTURE_COMPARE_MODE)"),
            TextureParameterName::TEXTURE_GREEN_SIZE => write!(fmt, "TextureParameterName(TEXTURE_GREEN_SIZE)"),
            TextureParameterName::TEXTURE_HEIGHT => write!(fmt, "TextureParameterName(TEXTURE_HEIGHT)"),
            TextureParameterName::TEXTURE_INTERNAL_FORMAT => write!(fmt, "TextureParameterName(TEXTURE_INTERNAL_FORMAT)"),
            TextureParameterName::TEXTURE_LOD_BIAS => write!(fmt, "TextureParameterName(TEXTURE_LOD_BIAS)"),
            TextureParameterName::TEXTURE_MAG_FILTER => write!(fmt, "TextureParameterName(TEXTURE_MAG_FILTER)"),
            TextureParameterName::TEXTURE_MAX_LEVEL => write!(fmt, "TextureParameterName(TEXTURE_MAX_LEVEL)"),
            TextureParameterName::TEXTURE_MAX_LOD => write!(fmt, "TextureParameterName(TEXTURE_MAX_LOD)"),
            TextureParameterName::TEXTURE_MIN_FILTER => write!(fmt, "TextureParameterName(TEXTURE_MIN_FILTER)"),
            TextureParameterName::TEXTURE_MIN_LOD => write!(fmt, "TextureParameterName(TEXTURE_MIN_LOD)"),
            TextureParameterName::TEXTURE_RED_SIZE => write!(fmt, "TextureParameterName(TEXTURE_RED_SIZE)"),
            TextureParameterName::TEXTURE_SWIZZLE_A => write!(fmt, "TextureParameterName(TEXTURE_SWIZZLE_A)"),
            TextureParameterName::TEXTURE_SWIZZLE_B => write!(fmt, "TextureParameterName(TEXTURE_SWIZZLE_B)"),
            TextureParameterName::TEXTURE_SWIZZLE_G => write!(fmt, "TextureParameterName(TEXTURE_SWIZZLE_G)"),
            TextureParameterName::TEXTURE_SWIZZLE_R => write!(fmt, "TextureParameterName(TEXTURE_SWIZZLE_R)"),
            TextureParameterName::TEXTURE_SWIZZLE_RGBA => write!(fmt, "TextureParameterName(TEXTURE_SWIZZLE_RGBA)"),
            TextureParameterName::TEXTURE_WIDTH => write!(fmt, "TextureParameterName(TEXTURE_WIDTH)"),
            TextureParameterName::TEXTURE_WRAP_R => write!(fmt, "TextureParameterName(TEXTURE_WRAP_R)"),
            TextureParameterName::TEXTURE_WRAP_S => write!(fmt, "TextureParameterName(TEXTURE_WRAP_S)"),
            TextureParameterName::TEXTURE_WRAP_T => write!(fmt, "TextureParameterName(TEXTURE_WRAP_T)"),
            _ => write!(fmt, "TextureParameterName({})", self.0),
        }
    }
}

impl_enum_traits!(TextureParameterName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureStorageMaskAMD(pub types::GLenum);

impl TextureStorageMaskAMD {
    pub const Empty: TextureStorageMaskAMD = TextureStorageMaskAMD(0);
}

impl ::std::fmt::Debug for TextureStorageMaskAMD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            _ => write!(fmt, "TextureStorageMaskAMD({})", self.0),
        }
    }
}

impl_enum_traits!(TextureStorageMaskAMD);

impl_enum_bitmask_traits!(TextureStorageMaskAMD);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureTarget(pub types::GLenum);

impl TextureTarget {
    pub const PROXY_TEXTURE_1D: TextureTarget = TextureTarget(super::PROXY_TEXTURE_1D);
    pub const PROXY_TEXTURE_1D_ARRAY: TextureTarget = TextureTarget(super::PROXY_TEXTURE_1D_ARRAY);
    pub const PROXY_TEXTURE_2D: TextureTarget = TextureTarget(super::PROXY_TEXTURE_2D);
    pub const PROXY_TEXTURE_2D_ARRAY: TextureTarget = TextureTarget(super::PROXY_TEXTURE_2D_ARRAY);
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: TextureTarget = TextureTarget(super::PROXY_TEXTURE_2D_MULTISAMPLE);
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: TextureTarget = TextureTarget(super::PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY);
    pub const PROXY_TEXTURE_3D: TextureTarget = TextureTarget(super::PROXY_TEXTURE_3D);
    pub const PROXY_TEXTURE_CUBE_MAP: TextureTarget = TextureTarget(super::PROXY_TEXTURE_CUBE_MAP);
    pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: TextureTarget = TextureTarget(super::PROXY_TEXTURE_CUBE_MAP_ARRAY);
    pub const PROXY_TEXTURE_RECTANGLE: TextureTarget = TextureTarget(super::PROXY_TEXTURE_RECTANGLE);
    pub const TEXTURE_1D: TextureTarget = TextureTarget(super::TEXTURE_1D);
    pub const TEXTURE_2D: TextureTarget = TextureTarget(super::TEXTURE_2D);
    pub const TEXTURE_3D: TextureTarget = TextureTarget(super::TEXTURE_3D);
    pub const TEXTURE_RECTANGLE: TextureTarget = TextureTarget(super::TEXTURE_RECTANGLE);
    pub const TEXTURE_CUBE_MAP: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP);
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_POSITIVE_X);
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_NEGATIVE_X);
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_POSITIVE_Y);
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_NEGATIVE_Y);
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_POSITIVE_Z);
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_NEGATIVE_Z);
    pub const TEXTURE_CUBE_MAP_ARRAY: TextureTarget = TextureTarget(super::TEXTURE_CUBE_MAP_ARRAY);
    pub const TEXTURE_1D_ARRAY: TextureTarget = TextureTarget(super::TEXTURE_1D_ARRAY);
    pub const TEXTURE_2D_ARRAY: TextureTarget = TextureTarget(super::TEXTURE_2D_ARRAY);
    pub const TEXTURE_2D_MULTISAMPLE: TextureTarget = TextureTarget(super::TEXTURE_2D_MULTISAMPLE);
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: TextureTarget = TextureTarget(super::TEXTURE_2D_MULTISAMPLE_ARRAY);
}

impl ::std::fmt::Debug for TextureTarget {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureTarget::PROXY_TEXTURE_1D => write!(fmt, "TextureTarget(PROXY_TEXTURE_1D)"),
            TextureTarget::PROXY_TEXTURE_1D_ARRAY => write!(fmt, "TextureTarget(PROXY_TEXTURE_1D_ARRAY)"),
            TextureTarget::PROXY_TEXTURE_2D => write!(fmt, "TextureTarget(PROXY_TEXTURE_2D)"),
            TextureTarget::PROXY_TEXTURE_2D_ARRAY => write!(fmt, "TextureTarget(PROXY_TEXTURE_2D_ARRAY)"),
            TextureTarget::PROXY_TEXTURE_2D_MULTISAMPLE => write!(fmt, "TextureTarget(PROXY_TEXTURE_2D_MULTISAMPLE)"),
            TextureTarget::PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY => write!(fmt, "TextureTarget(PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY)"),
            TextureTarget::PROXY_TEXTURE_3D => write!(fmt, "TextureTarget(PROXY_TEXTURE_3D)"),
            TextureTarget::PROXY_TEXTURE_CUBE_MAP => write!(fmt, "TextureTarget(PROXY_TEXTURE_CUBE_MAP)"),
            TextureTarget::PROXY_TEXTURE_CUBE_MAP_ARRAY => write!(fmt, "TextureTarget(PROXY_TEXTURE_CUBE_MAP_ARRAY)"),
            TextureTarget::PROXY_TEXTURE_RECTANGLE => write!(fmt, "TextureTarget(PROXY_TEXTURE_RECTANGLE)"),
            TextureTarget::TEXTURE_1D => write!(fmt, "TextureTarget(TEXTURE_1D)"),
            TextureTarget::TEXTURE_1D_ARRAY => write!(fmt, "TextureTarget(TEXTURE_1D_ARRAY)"),
            TextureTarget::TEXTURE_2D => write!(fmt, "TextureTarget(TEXTURE_2D)"),
            TextureTarget::TEXTURE_2D_ARRAY => write!(fmt, "TextureTarget(TEXTURE_2D_ARRAY)"),
            TextureTarget::TEXTURE_2D_MULTISAMPLE => write!(fmt, "TextureTarget(TEXTURE_2D_MULTISAMPLE)"),
            TextureTarget::TEXTURE_2D_MULTISAMPLE_ARRAY => write!(fmt, "TextureTarget(TEXTURE_2D_MULTISAMPLE_ARRAY)"),
            TextureTarget::TEXTURE_3D => write!(fmt, "TextureTarget(TEXTURE_3D)"),
            TextureTarget::TEXTURE_CUBE_MAP => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP)"),
            TextureTarget::TEXTURE_CUBE_MAP_ARRAY => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_ARRAY)"),
            TextureTarget::TEXTURE_CUBE_MAP_NEGATIVE_X => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_NEGATIVE_X)"),
            TextureTarget::TEXTURE_CUBE_MAP_NEGATIVE_Y => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_NEGATIVE_Y)"),
            TextureTarget::TEXTURE_CUBE_MAP_NEGATIVE_Z => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_NEGATIVE_Z)"),
            TextureTarget::TEXTURE_CUBE_MAP_POSITIVE_X => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_POSITIVE_X)"),
            TextureTarget::TEXTURE_CUBE_MAP_POSITIVE_Y => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_POSITIVE_Y)"),
            TextureTarget::TEXTURE_CUBE_MAP_POSITIVE_Z => write!(fmt, "TextureTarget(TEXTURE_CUBE_MAP_POSITIVE_Z)"),
            TextureTarget::TEXTURE_RECTANGLE => write!(fmt, "TextureTarget(TEXTURE_RECTANGLE)"),
            _ => write!(fmt, "TextureTarget({})", self.0),
        }
    }
}

impl_enum_traits!(TextureTarget);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureUnit(pub types::GLenum);

impl TextureUnit {
    pub const TEXTURE0: TextureUnit = TextureUnit(super::TEXTURE0);
    pub const TEXTURE1: TextureUnit = TextureUnit(super::TEXTURE1);
    pub const TEXTURE2: TextureUnit = TextureUnit(super::TEXTURE2);
    pub const TEXTURE3: TextureUnit = TextureUnit(super::TEXTURE3);
    pub const TEXTURE4: TextureUnit = TextureUnit(super::TEXTURE4);
    pub const TEXTURE5: TextureUnit = TextureUnit(super::TEXTURE5);
    pub const TEXTURE6: TextureUnit = TextureUnit(super::TEXTURE6);
    pub const TEXTURE7: TextureUnit = TextureUnit(super::TEXTURE7);
    pub const TEXTURE8: TextureUnit = TextureUnit(super::TEXTURE8);
    pub const TEXTURE9: TextureUnit = TextureUnit(super::TEXTURE9);
    pub const TEXTURE10: TextureUnit = TextureUnit(super::TEXTURE10);
    pub const TEXTURE11: TextureUnit = TextureUnit(super::TEXTURE11);
    pub const TEXTURE12: TextureUnit = TextureUnit(super::TEXTURE12);
    pub const TEXTURE13: TextureUnit = TextureUnit(super::TEXTURE13);
    pub const TEXTURE14: TextureUnit = TextureUnit(super::TEXTURE14);
    pub const TEXTURE15: TextureUnit = TextureUnit(super::TEXTURE15);
    pub const TEXTURE16: TextureUnit = TextureUnit(super::TEXTURE16);
    pub const TEXTURE17: TextureUnit = TextureUnit(super::TEXTURE17);
    pub const TEXTURE18: TextureUnit = TextureUnit(super::TEXTURE18);
    pub const TEXTURE19: TextureUnit = TextureUnit(super::TEXTURE19);
    pub const TEXTURE20: TextureUnit = TextureUnit(super::TEXTURE20);
    pub const TEXTURE21: TextureUnit = TextureUnit(super::TEXTURE21);
    pub const TEXTURE22: TextureUnit = TextureUnit(super::TEXTURE22);
    pub const TEXTURE23: TextureUnit = TextureUnit(super::TEXTURE23);
    pub const TEXTURE24: TextureUnit = TextureUnit(super::TEXTURE24);
    pub const TEXTURE25: TextureUnit = TextureUnit(super::TEXTURE25);
    pub const TEXTURE26: TextureUnit = TextureUnit(super::TEXTURE26);
    pub const TEXTURE27: TextureUnit = TextureUnit(super::TEXTURE27);
    pub const TEXTURE28: TextureUnit = TextureUnit(super::TEXTURE28);
    pub const TEXTURE29: TextureUnit = TextureUnit(super::TEXTURE29);
    pub const TEXTURE30: TextureUnit = TextureUnit(super::TEXTURE30);
    pub const TEXTURE31: TextureUnit = TextureUnit(super::TEXTURE31);
}

impl ::std::fmt::Debug for TextureUnit {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureUnit::TEXTURE0 => write!(fmt, "TextureUnit(TEXTURE0)"),
            TextureUnit::TEXTURE1 => write!(fmt, "TextureUnit(TEXTURE1)"),
            TextureUnit::TEXTURE10 => write!(fmt, "TextureUnit(TEXTURE10)"),
            TextureUnit::TEXTURE11 => write!(fmt, "TextureUnit(TEXTURE11)"),
            TextureUnit::TEXTURE12 => write!(fmt, "TextureUnit(TEXTURE12)"),
            TextureUnit::TEXTURE13 => write!(fmt, "TextureUnit(TEXTURE13)"),
            TextureUnit::TEXTURE14 => write!(fmt, "TextureUnit(TEXTURE14)"),
            TextureUnit::TEXTURE15 => write!(fmt, "TextureUnit(TEXTURE15)"),
            TextureUnit::TEXTURE16 => write!(fmt, "TextureUnit(TEXTURE16)"),
            TextureUnit::TEXTURE17 => write!(fmt, "TextureUnit(TEXTURE17)"),
            TextureUnit::TEXTURE18 => write!(fmt, "TextureUnit(TEXTURE18)"),
            TextureUnit::TEXTURE19 => write!(fmt, "TextureUnit(TEXTURE19)"),
            TextureUnit::TEXTURE2 => write!(fmt, "TextureUnit(TEXTURE2)"),
            TextureUnit::TEXTURE20 => write!(fmt, "TextureUnit(TEXTURE20)"),
            TextureUnit::TEXTURE21 => write!(fmt, "TextureUnit(TEXTURE21)"),
            TextureUnit::TEXTURE22 => write!(fmt, "TextureUnit(TEXTURE22)"),
            TextureUnit::TEXTURE23 => write!(fmt, "TextureUnit(TEXTURE23)"),
            TextureUnit::TEXTURE24 => write!(fmt, "TextureUnit(TEXTURE24)"),
            TextureUnit::TEXTURE25 => write!(fmt, "TextureUnit(TEXTURE25)"),
            TextureUnit::TEXTURE26 => write!(fmt, "TextureUnit(TEXTURE26)"),
            TextureUnit::TEXTURE27 => write!(fmt, "TextureUnit(TEXTURE27)"),
            TextureUnit::TEXTURE28 => write!(fmt, "TextureUnit(TEXTURE28)"),
            TextureUnit::TEXTURE29 => write!(fmt, "TextureUnit(TEXTURE29)"),
            TextureUnit::TEXTURE3 => write!(fmt, "TextureUnit(TEXTURE3)"),
            TextureUnit::TEXTURE30 => write!(fmt, "TextureUnit(TEXTURE30)"),
            TextureUnit::TEXTURE31 => write!(fmt, "TextureUnit(TEXTURE31)"),
            TextureUnit::TEXTURE4 => write!(fmt, "TextureUnit(TEXTURE4)"),
            TextureUnit::TEXTURE5 => write!(fmt, "TextureUnit(TEXTURE5)"),
            TextureUnit::TEXTURE6 => write!(fmt, "TextureUnit(TEXTURE6)"),
            TextureUnit::TEXTURE7 => write!(fmt, "TextureUnit(TEXTURE7)"),
            TextureUnit::TEXTURE8 => write!(fmt, "TextureUnit(TEXTURE8)"),
            TextureUnit::TEXTURE9 => write!(fmt, "TextureUnit(TEXTURE9)"),
            _ => write!(fmt, "TextureUnit({})", self.0),
        }
    }
}

impl_enum_traits!(TextureUnit);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextureWrapMode(pub types::GLenum);

impl TextureWrapMode {
    pub const CLAMP_TO_BORDER: TextureWrapMode = TextureWrapMode(super::CLAMP_TO_BORDER);
    pub const CLAMP_TO_EDGE: TextureWrapMode = TextureWrapMode(super::CLAMP_TO_EDGE);
    pub const REPEAT: TextureWrapMode = TextureWrapMode(super::REPEAT);
}

impl ::std::fmt::Debug for TextureWrapMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TextureWrapMode::CLAMP_TO_BORDER => write!(fmt, "TextureWrapMode(CLAMP_TO_BORDER)"),
            TextureWrapMode::CLAMP_TO_EDGE => write!(fmt, "TextureWrapMode(CLAMP_TO_EDGE)"),
            TextureWrapMode::REPEAT => write!(fmt, "TextureWrapMode(REPEAT)"),
            _ => write!(fmt, "TextureWrapMode({})", self.0),
        }
    }
}

impl_enum_traits!(TextureWrapMode);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TransformFeedbackPName(pub types::GLenum);

impl TransformFeedbackPName {
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: TransformFeedbackPName = TransformFeedbackPName(super::TRANSFORM_FEEDBACK_BUFFER_BINDING);
    pub const TRANSFORM_FEEDBACK_BUFFER_START: TransformFeedbackPName = TransformFeedbackPName(super::TRANSFORM_FEEDBACK_BUFFER_START);
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: TransformFeedbackPName = TransformFeedbackPName(super::TRANSFORM_FEEDBACK_BUFFER_SIZE);
    pub const TRANSFORM_FEEDBACK_PAUSED: TransformFeedbackPName = TransformFeedbackPName(super::TRANSFORM_FEEDBACK_PAUSED);
    pub const TRANSFORM_FEEDBACK_ACTIVE: TransformFeedbackPName = TransformFeedbackPName(super::TRANSFORM_FEEDBACK_ACTIVE);
}

impl ::std::fmt::Debug for TransformFeedbackPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TransformFeedbackPName::TRANSFORM_FEEDBACK_ACTIVE => write!(fmt, "TransformFeedbackPName(TRANSFORM_FEEDBACK_ACTIVE)"),
            TransformFeedbackPName::TRANSFORM_FEEDBACK_BUFFER_BINDING => write!(fmt, "TransformFeedbackPName(TRANSFORM_FEEDBACK_BUFFER_BINDING)"),
            TransformFeedbackPName::TRANSFORM_FEEDBACK_BUFFER_SIZE => write!(fmt, "TransformFeedbackPName(TRANSFORM_FEEDBACK_BUFFER_SIZE)"),
            TransformFeedbackPName::TRANSFORM_FEEDBACK_BUFFER_START => write!(fmt, "TransformFeedbackPName(TRANSFORM_FEEDBACK_BUFFER_START)"),
            TransformFeedbackPName::TRANSFORM_FEEDBACK_PAUSED => write!(fmt, "TransformFeedbackPName(TRANSFORM_FEEDBACK_PAUSED)"),
            _ => write!(fmt, "TransformFeedbackPName({})", self.0),
        }
    }
}

impl_enum_traits!(TransformFeedbackPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypeEnum(pub types::GLenum);

impl TypeEnum {
    pub const QUERY_WAIT: TypeEnum = TypeEnum(super::QUERY_WAIT);
    pub const QUERY_NO_WAIT: TypeEnum = TypeEnum(super::QUERY_NO_WAIT);
    pub const QUERY_BY_REGION_WAIT: TypeEnum = TypeEnum(super::QUERY_BY_REGION_WAIT);
    pub const QUERY_BY_REGION_NO_WAIT: TypeEnum = TypeEnum(super::QUERY_BY_REGION_NO_WAIT);
}

impl ::std::fmt::Debug for TypeEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TypeEnum::QUERY_BY_REGION_NO_WAIT => write!(fmt, "TypeEnum(QUERY_BY_REGION_NO_WAIT)"),
            TypeEnum::QUERY_BY_REGION_WAIT => write!(fmt, "TypeEnum(QUERY_BY_REGION_WAIT)"),
            TypeEnum::QUERY_NO_WAIT => write!(fmt, "TypeEnum(QUERY_NO_WAIT)"),
            TypeEnum::QUERY_WAIT => write!(fmt, "TypeEnum(QUERY_WAIT)"),
            _ => write!(fmt, "TypeEnum({})", self.0),
        }
    }
}

impl_enum_traits!(TypeEnum);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct UniformBlockPName(pub types::GLenum);

impl UniformBlockPName {
    pub const UNIFORM_BLOCK_BINDING: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_BINDING);
    pub const UNIFORM_BLOCK_DATA_SIZE: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_DATA_SIZE);
    pub const UNIFORM_BLOCK_NAME_LENGTH: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_NAME_LENGTH);
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_ACTIVE_UNIFORMS);
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES);
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER);
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER);
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER);
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER);
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER);
    pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: UniformBlockPName = UniformBlockPName(super::UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER);
}

impl ::std::fmt::Debug for UniformBlockPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            UniformBlockPName::UNIFORM_BLOCK_ACTIVE_UNIFORMS => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_ACTIVE_UNIFORMS)"),
            UniformBlockPName::UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES)"),
            UniformBlockPName::UNIFORM_BLOCK_BINDING => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_BINDING)"),
            UniformBlockPName::UNIFORM_BLOCK_DATA_SIZE => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_DATA_SIZE)"),
            UniformBlockPName::UNIFORM_BLOCK_NAME_LENGTH => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_NAME_LENGTH)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER)"),
            UniformBlockPName::UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER => write!(fmt, "UniformBlockPName(UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER)"),
            _ => write!(fmt, "UniformBlockPName({})", self.0),
        }
    }
}

impl_enum_traits!(UniformBlockPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct UniformPName(pub types::GLenum);

impl UniformPName {
    pub const UNIFORM_TYPE: UniformPName = UniformPName(super::UNIFORM_TYPE);
    pub const UNIFORM_SIZE: UniformPName = UniformPName(super::UNIFORM_SIZE);
    pub const UNIFORM_NAME_LENGTH: UniformPName = UniformPName(super::UNIFORM_NAME_LENGTH);
    pub const UNIFORM_BLOCK_INDEX: UniformPName = UniformPName(super::UNIFORM_BLOCK_INDEX);
    pub const UNIFORM_OFFSET: UniformPName = UniformPName(super::UNIFORM_OFFSET);
    pub const UNIFORM_ARRAY_STRIDE: UniformPName = UniformPName(super::UNIFORM_ARRAY_STRIDE);
    pub const UNIFORM_MATRIX_STRIDE: UniformPName = UniformPName(super::UNIFORM_MATRIX_STRIDE);
    pub const UNIFORM_IS_ROW_MAJOR: UniformPName = UniformPName(super::UNIFORM_IS_ROW_MAJOR);
    pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: UniformPName = UniformPName(super::UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX);
}

impl ::std::fmt::Debug for UniformPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            UniformPName::UNIFORM_ARRAY_STRIDE => write!(fmt, "UniformPName(UNIFORM_ARRAY_STRIDE)"),
            UniformPName::UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX => write!(fmt, "UniformPName(UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX)"),
            UniformPName::UNIFORM_BLOCK_INDEX => write!(fmt, "UniformPName(UNIFORM_BLOCK_INDEX)"),
            UniformPName::UNIFORM_IS_ROW_MAJOR => write!(fmt, "UniformPName(UNIFORM_IS_ROW_MAJOR)"),
            UniformPName::UNIFORM_MATRIX_STRIDE => write!(fmt, "UniformPName(UNIFORM_MATRIX_STRIDE)"),
            UniformPName::UNIFORM_NAME_LENGTH => write!(fmt, "UniformPName(UNIFORM_NAME_LENGTH)"),
            UniformPName::UNIFORM_OFFSET => write!(fmt, "UniformPName(UNIFORM_OFFSET)"),
            UniformPName::UNIFORM_SIZE => write!(fmt, "UniformPName(UNIFORM_SIZE)"),
            UniformPName::UNIFORM_TYPE => write!(fmt, "UniformPName(UNIFORM_TYPE)"),
            _ => write!(fmt, "UniformPName({})", self.0),
        }
    }
}

impl_enum_traits!(UniformPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct UseProgramStageMask(pub types::GLenum);

impl UseProgramStageMask {
    pub const VERTEX_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::VERTEX_SHADER_BIT);
    pub const FRAGMENT_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::FRAGMENT_SHADER_BIT);
    pub const GEOMETRY_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::GEOMETRY_SHADER_BIT);
    pub const TESS_CONTROL_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::TESS_CONTROL_SHADER_BIT);
    pub const TESS_EVALUATION_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::TESS_EVALUATION_SHADER_BIT);
    pub const COMPUTE_SHADER_BIT: UseProgramStageMask = UseProgramStageMask(super::COMPUTE_SHADER_BIT);
    pub const ALL_SHADER_BITS: UseProgramStageMask = UseProgramStageMask(super::ALL_SHADER_BITS);
    pub const Empty: UseProgramStageMask = UseProgramStageMask(0);
}

impl ::std::fmt::Debug for UseProgramStageMask {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            UseProgramStageMask::ALL_SHADER_BITS => write!(fmt, "UseProgramStageMask(ALL_SHADER_BITS)"),
            UseProgramStageMask::COMPUTE_SHADER_BIT => write!(fmt, "UseProgramStageMask(COMPUTE_SHADER_BIT)"),
            UseProgramStageMask::FRAGMENT_SHADER_BIT => write!(fmt, "UseProgramStageMask(FRAGMENT_SHADER_BIT)"),
            UseProgramStageMask::GEOMETRY_SHADER_BIT => write!(fmt, "UseProgramStageMask(GEOMETRY_SHADER_BIT)"),
            UseProgramStageMask::TESS_CONTROL_SHADER_BIT => write!(fmt, "UseProgramStageMask(TESS_CONTROL_SHADER_BIT)"),
            UseProgramStageMask::TESS_EVALUATION_SHADER_BIT => write!(fmt, "UseProgramStageMask(TESS_EVALUATION_SHADER_BIT)"),
            UseProgramStageMask::VERTEX_SHADER_BIT => write!(fmt, "UseProgramStageMask(VERTEX_SHADER_BIT)"),
            _ => write!(fmt, "UseProgramStageMask({})", self.0),
        }
    }
}

impl_enum_traits!(UseProgramStageMask);

impl_enum_bitmask_traits!(UseProgramStageMask);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexArrayPName(pub types::GLenum);

impl VertexArrayPName {
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_ENABLED);
    pub const VERTEX_ATTRIB_ARRAY_SIZE: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_SIZE);
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_STRIDE);
    pub const VERTEX_ATTRIB_ARRAY_TYPE: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_TYPE);
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_NORMALIZED);
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_INTEGER);
    pub const VERTEX_ATTRIB_ARRAY_LONG: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_LONG);
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_ARRAY_DIVISOR);
    pub const VERTEX_ATTRIB_RELATIVE_OFFSET: VertexArrayPName = VertexArrayPName(super::VERTEX_ATTRIB_RELATIVE_OFFSET);
}

impl ::std::fmt::Debug for VertexArrayPName {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_DIVISOR => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_DIVISOR)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_ENABLED => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_ENABLED)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_INTEGER => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_INTEGER)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_LONG => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_LONG)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_NORMALIZED => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_NORMALIZED)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_SIZE => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_SIZE)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_STRIDE => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_STRIDE)"),
            VertexArrayPName::VERTEX_ATTRIB_ARRAY_TYPE => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_ARRAY_TYPE)"),
            VertexArrayPName::VERTEX_ATTRIB_RELATIVE_OFFSET => write!(fmt, "VertexArrayPName(VERTEX_ATTRIB_RELATIVE_OFFSET)"),
            _ => write!(fmt, "VertexArrayPName({})", self.0),
        }
    }
}

impl_enum_traits!(VertexArrayPName);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexAttribEnum(pub types::GLenum);

impl VertexAttribEnum {
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING);
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_ENABLED);
    pub const VERTEX_ATTRIB_ARRAY_SIZE: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_SIZE);
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_STRIDE);
    pub const VERTEX_ATTRIB_ARRAY_TYPE: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_TYPE);
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_NORMALIZED);
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_INTEGER);
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: VertexAttribEnum = VertexAttribEnum(super::VERTEX_ATTRIB_ARRAY_DIVISOR);
    pub const CURRENT_VERTEX_ATTRIB: VertexAttribEnum = VertexAttribEnum(super::CURRENT_VERTEX_ATTRIB);
}

impl ::std::fmt::Debug for VertexAttribEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexAttribEnum::CURRENT_VERTEX_ATTRIB => write!(fmt, "VertexAttribEnum(CURRENT_VERTEX_ATTRIB)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_BUFFER_BINDING => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_BUFFER_BINDING)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_DIVISOR => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_DIVISOR)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_ENABLED => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_ENABLED)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_INTEGER => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_INTEGER)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_NORMALIZED => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_NORMALIZED)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_SIZE => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_SIZE)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_STRIDE => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_STRIDE)"),
            VertexAttribEnum::VERTEX_ATTRIB_ARRAY_TYPE => write!(fmt, "VertexAttribEnum(VERTEX_ATTRIB_ARRAY_TYPE)"),
            _ => write!(fmt, "VertexAttribEnum({})", self.0),
        }
    }
}

impl_enum_traits!(VertexAttribEnum);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexAttribPointerType(pub types::GLenum);

impl VertexAttribPointerType {
    pub const BYTE: VertexAttribPointerType = VertexAttribPointerType(super::BYTE);
    pub const UNSIGNED_BYTE: VertexAttribPointerType = VertexAttribPointerType(super::UNSIGNED_BYTE);
    pub const SHORT: VertexAttribPointerType = VertexAttribPointerType(super::SHORT);
    pub const UNSIGNED_SHORT: VertexAttribPointerType = VertexAttribPointerType(super::UNSIGNED_SHORT);
    pub const INT: VertexAttribPointerType = VertexAttribPointerType(super::INT);
    pub const UNSIGNED_INT: VertexAttribPointerType = VertexAttribPointerType(super::UNSIGNED_INT);
    pub const FLOAT: VertexAttribPointerType = VertexAttribPointerType(super::FLOAT);
    pub const DOUBLE: VertexAttribPointerType = VertexAttribPointerType(super::DOUBLE);
    pub const HALF_FLOAT: VertexAttribPointerType = VertexAttribPointerType(super::HALF_FLOAT);
    pub const FIXED: VertexAttribPointerType = VertexAttribPointerType(super::FIXED);
    pub const INT_2_10_10_10_REV: VertexAttribPointerType = VertexAttribPointerType(super::INT_2_10_10_10_REV);
    pub const UNSIGNED_INT_2_10_10_10_REV: VertexAttribPointerType = VertexAttribPointerType(super::UNSIGNED_INT_2_10_10_10_REV);
    pub const UNSIGNED_INT_10F_11F_11F_REV: VertexAttribPointerType = VertexAttribPointerType(super::UNSIGNED_INT_10F_11F_11F_REV);
}

impl ::std::fmt::Debug for VertexAttribPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexAttribPointerType::BYTE => write!(fmt, "VertexAttribPointerType(BYTE)"),
            VertexAttribPointerType::DOUBLE => write!(fmt, "VertexAttribPointerType(DOUBLE)"),
            VertexAttribPointerType::FIXED => write!(fmt, "VertexAttribPointerType(FIXED)"),
            VertexAttribPointerType::FLOAT => write!(fmt, "VertexAttribPointerType(FLOAT)"),
            VertexAttribPointerType::HALF_FLOAT => write!(fmt, "VertexAttribPointerType(HALF_FLOAT)"),
            VertexAttribPointerType::INT => write!(fmt, "VertexAttribPointerType(INT)"),
            VertexAttribPointerType::INT_2_10_10_10_REV => write!(fmt, "VertexAttribPointerType(INT_2_10_10_10_REV)"),
            VertexAttribPointerType::SHORT => write!(fmt, "VertexAttribPointerType(SHORT)"),
            VertexAttribPointerType::UNSIGNED_BYTE => write!(fmt, "VertexAttribPointerType(UNSIGNED_BYTE)"),
            VertexAttribPointerType::UNSIGNED_INT => write!(fmt, "VertexAttribPointerType(UNSIGNED_INT)"),
            VertexAttribPointerType::UNSIGNED_INT_10F_11F_11F_REV => write!(fmt, "VertexAttribPointerType(UNSIGNED_INT_10F_11F_11F_REV)"),
            VertexAttribPointerType::UNSIGNED_INT_2_10_10_10_REV => write!(fmt, "VertexAttribPointerType(UNSIGNED_INT_2_10_10_10_REV)"),
            VertexAttribPointerType::UNSIGNED_SHORT => write!(fmt, "VertexAttribPointerType(UNSIGNED_SHORT)"),
            _ => write!(fmt, "VertexAttribPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(VertexAttribPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexAttribType(pub types::GLenum);

impl VertexAttribType {
    pub const BYTE: VertexAttribType = VertexAttribType(super::BYTE);
    pub const SHORT: VertexAttribType = VertexAttribType(super::SHORT);
    pub const INT: VertexAttribType = VertexAttribType(super::INT);
    pub const FIXED: VertexAttribType = VertexAttribType(super::FIXED);
    pub const FLOAT: VertexAttribType = VertexAttribType(super::FLOAT);
    pub const HALF_FLOAT: VertexAttribType = VertexAttribType(super::HALF_FLOAT);
    pub const DOUBLE: VertexAttribType = VertexAttribType(super::DOUBLE);
    pub const UNSIGNED_BYTE: VertexAttribType = VertexAttribType(super::UNSIGNED_BYTE);
    pub const UNSIGNED_SHORT: VertexAttribType = VertexAttribType(super::UNSIGNED_SHORT);
    pub const UNSIGNED_INT: VertexAttribType = VertexAttribType(super::UNSIGNED_INT);
    pub const INT_2_10_10_10_REV: VertexAttribType = VertexAttribType(super::INT_2_10_10_10_REV);
    pub const UNSIGNED_INT_2_10_10_10_REV: VertexAttribType = VertexAttribType(super::UNSIGNED_INT_2_10_10_10_REV);
    pub const UNSIGNED_INT_10F_11F_11F_REV: VertexAttribType = VertexAttribType(super::UNSIGNED_INT_10F_11F_11F_REV);
}

impl ::std::fmt::Debug for VertexAttribType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexAttribType::BYTE => write!(fmt, "VertexAttribType(BYTE)"),
            VertexAttribType::DOUBLE => write!(fmt, "VertexAttribType(DOUBLE)"),
            VertexAttribType::FIXED => write!(fmt, "VertexAttribType(FIXED)"),
            VertexAttribType::FLOAT => write!(fmt, "VertexAttribType(FLOAT)"),
            VertexAttribType::HALF_FLOAT => write!(fmt, "VertexAttribType(HALF_FLOAT)"),
            VertexAttribType::INT => write!(fmt, "VertexAttribType(INT)"),
            VertexAttribType::INT_2_10_10_10_REV => write!(fmt, "VertexAttribType(INT_2_10_10_10_REV)"),
            VertexAttribType::SHORT => write!(fmt, "VertexAttribType(SHORT)"),
            VertexAttribType::UNSIGNED_BYTE => write!(fmt, "VertexAttribType(UNSIGNED_BYTE)"),
            VertexAttribType::UNSIGNED_INT => write!(fmt, "VertexAttribType(UNSIGNED_INT)"),
            VertexAttribType::UNSIGNED_INT_10F_11F_11F_REV => write!(fmt, "VertexAttribType(UNSIGNED_INT_10F_11F_11F_REV)"),
            VertexAttribType::UNSIGNED_INT_2_10_10_10_REV => write!(fmt, "VertexAttribType(UNSIGNED_INT_2_10_10_10_REV)"),
            VertexAttribType::UNSIGNED_SHORT => write!(fmt, "VertexAttribType(UNSIGNED_SHORT)"),
            _ => write!(fmt, "VertexAttribType({})", self.0),
        }
    }
}

impl_enum_traits!(VertexAttribType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexBufferObjectParameter(pub types::GLenum);

impl VertexBufferObjectParameter {
    pub const BUFFER_ACCESS: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_ACCESS);
    pub const BUFFER_ACCESS_FLAGS: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_ACCESS_FLAGS);
    pub const BUFFER_IMMUTABLE_STORAGE: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_IMMUTABLE_STORAGE);
    pub const BUFFER_MAPPED: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_MAPPED);
    pub const BUFFER_MAP_LENGTH: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_MAP_LENGTH);
    pub const BUFFER_MAP_OFFSET: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_MAP_OFFSET);
    pub const BUFFER_SIZE: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_SIZE);
    pub const BUFFER_STORAGE_FLAGS: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_STORAGE_FLAGS);
    pub const BUFFER_USAGE: VertexBufferObjectParameter = VertexBufferObjectParameter(super::BUFFER_USAGE);
}

impl ::std::fmt::Debug for VertexBufferObjectParameter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexBufferObjectParameter::BUFFER_ACCESS => write!(fmt, "VertexBufferObjectParameter(BUFFER_ACCESS)"),
            VertexBufferObjectParameter::BUFFER_ACCESS_FLAGS => write!(fmt, "VertexBufferObjectParameter(BUFFER_ACCESS_FLAGS)"),
            VertexBufferObjectParameter::BUFFER_IMMUTABLE_STORAGE => write!(fmt, "VertexBufferObjectParameter(BUFFER_IMMUTABLE_STORAGE)"),
            VertexBufferObjectParameter::BUFFER_MAPPED => write!(fmt, "VertexBufferObjectParameter(BUFFER_MAPPED)"),
            VertexBufferObjectParameter::BUFFER_MAP_LENGTH => write!(fmt, "VertexBufferObjectParameter(BUFFER_MAP_LENGTH)"),
            VertexBufferObjectParameter::BUFFER_MAP_OFFSET => write!(fmt, "VertexBufferObjectParameter(BUFFER_MAP_OFFSET)"),
            VertexBufferObjectParameter::BUFFER_SIZE => write!(fmt, "VertexBufferObjectParameter(BUFFER_SIZE)"),
            VertexBufferObjectParameter::BUFFER_STORAGE_FLAGS => write!(fmt, "VertexBufferObjectParameter(BUFFER_STORAGE_FLAGS)"),
            VertexBufferObjectParameter::BUFFER_USAGE => write!(fmt, "VertexBufferObjectParameter(BUFFER_USAGE)"),
            _ => write!(fmt, "VertexBufferObjectParameter({})", self.0),
        }
    }
}

impl_enum_traits!(VertexBufferObjectParameter);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexBufferObjectUsage(pub types::GLenum);

impl VertexBufferObjectUsage {
    pub const STREAM_DRAW: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STREAM_DRAW);
    pub const STREAM_READ: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STREAM_READ);
    pub const STREAM_COPY: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STREAM_COPY);
    pub const STATIC_DRAW: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STATIC_DRAW);
    pub const STATIC_READ: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STATIC_READ);
    pub const STATIC_COPY: VertexBufferObjectUsage = VertexBufferObjectUsage(super::STATIC_COPY);
    pub const DYNAMIC_DRAW: VertexBufferObjectUsage = VertexBufferObjectUsage(super::DYNAMIC_DRAW);
    pub const DYNAMIC_READ: VertexBufferObjectUsage = VertexBufferObjectUsage(super::DYNAMIC_READ);
    pub const DYNAMIC_COPY: VertexBufferObjectUsage = VertexBufferObjectUsage(super::DYNAMIC_COPY);
}

impl ::std::fmt::Debug for VertexBufferObjectUsage {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexBufferObjectUsage::DYNAMIC_COPY => write!(fmt, "VertexBufferObjectUsage(DYNAMIC_COPY)"),
            VertexBufferObjectUsage::DYNAMIC_DRAW => write!(fmt, "VertexBufferObjectUsage(DYNAMIC_DRAW)"),
            VertexBufferObjectUsage::DYNAMIC_READ => write!(fmt, "VertexBufferObjectUsage(DYNAMIC_READ)"),
            VertexBufferObjectUsage::STATIC_COPY => write!(fmt, "VertexBufferObjectUsage(STATIC_COPY)"),
            VertexBufferObjectUsage::STATIC_DRAW => write!(fmt, "VertexBufferObjectUsage(STATIC_DRAW)"),
            VertexBufferObjectUsage::STATIC_READ => write!(fmt, "VertexBufferObjectUsage(STATIC_READ)"),
            VertexBufferObjectUsage::STREAM_COPY => write!(fmt, "VertexBufferObjectUsage(STREAM_COPY)"),
            VertexBufferObjectUsage::STREAM_DRAW => write!(fmt, "VertexBufferObjectUsage(STREAM_DRAW)"),
            VertexBufferObjectUsage::STREAM_READ => write!(fmt, "VertexBufferObjectUsage(STREAM_READ)"),
            _ => write!(fmt, "VertexBufferObjectUsage({})", self.0),
        }
    }
}

impl_enum_traits!(VertexBufferObjectUsage);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexPointerType(pub types::GLenum);

impl VertexPointerType {
    pub const DOUBLE: VertexPointerType = VertexPointerType(super::DOUBLE);
    pub const FLOAT: VertexPointerType = VertexPointerType(super::FLOAT);
    pub const INT: VertexPointerType = VertexPointerType(super::INT);
    pub const SHORT: VertexPointerType = VertexPointerType(super::SHORT);
}

impl ::std::fmt::Debug for VertexPointerType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexPointerType::DOUBLE => write!(fmt, "VertexPointerType(DOUBLE)"),
            VertexPointerType::FLOAT => write!(fmt, "VertexPointerType(FLOAT)"),
            VertexPointerType::INT => write!(fmt, "VertexPointerType(INT)"),
            VertexPointerType::SHORT => write!(fmt, "VertexPointerType(SHORT)"),
            _ => write!(fmt, "VertexPointerType({})", self.0),
        }
    }
}

impl_enum_traits!(VertexPointerType);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VertexProvokingMode(pub types::GLenum);

impl VertexProvokingMode {
    pub const FIRST_VERTEX_CONVENTION: VertexProvokingMode = VertexProvokingMode(super::FIRST_VERTEX_CONVENTION);
    pub const LAST_VERTEX_CONVENTION: VertexProvokingMode = VertexProvokingMode(super::LAST_VERTEX_CONVENTION);
}

impl ::std::fmt::Debug for VertexProvokingMode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VertexProvokingMode::FIRST_VERTEX_CONVENTION => write!(fmt, "VertexProvokingMode(FIRST_VERTEX_CONVENTION)"),
            VertexProvokingMode::LAST_VERTEX_CONVENTION => write!(fmt, "VertexProvokingMode(LAST_VERTEX_CONVENTION)"),
            _ => write!(fmt, "VertexProvokingMode({})", self.0),
        }
    }
}

impl_enum_traits!(VertexProvokingMode);

}
