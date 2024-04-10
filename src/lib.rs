#![doc = include_str!("../README.md")]
#![allow(
   warnings,
   unused,
   non_snake_case,
   non_camel_case_types,
   non_upper_case_globals
)]
// MIT License
//
// Copyright (c) 2024 oberrich <oberrich.llvm@proton.me>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub mod ffi {
   // use vendored `generated.rs` for docs.rs
   #[cfg(feature = "docsrs")]
   include!("ffi/generated.rs");

   #[cfg(not(feature = "docsrs"))]
   include!(concat!(env!("OUT_DIR"), "\\generated.rs"));
}

pub mod ext {
   use crate::ffi::*;
   use std::arch::asm;

   #[macro_export]
   macro_rules! InitializeObjectAttributes {
      ($p:expr, $n:expr, $a:expr, $r:expr, $s:expr) => {{
         let _o = $p;
         _o.Length = ::std::mem::size_of::<ffi::OBJECT_ATTRIBUTES>() as u32;
         _o.RootDirectory = $r;
         _o.ObjectName = $n;
         _o.Attributes = $a;
         _o.SecurityDescriptor = $s;
         _o.SecurityQualityOfService = ::std::ptr::null_mut();
      }};
   }

   macro_rules! FIELD_OFFSET {
     ($_type:ty, $field:ident$(.$cfields:ident)*) => {{
         let obj = core::mem::MaybeUninit::<$_type>::uninit();
         let base = obj.as_ptr();
         unsafe { core::ptr::addr_of!((*base).$field$(.$cfields)*) as usize - base as usize }
     }};
 }

   #[inline]
   pub unsafe fn __readfsdword(offset: u32) -> usize {
      let out: usize;
      asm!(
          "mov {:e}, fs:[{:e}]",
          lateout(reg) out,
          in(reg) offset,
          options(nostack, pure, readonly),
      );
      out
   }

   #[inline]
   #[cfg(target_pointer_width = "64")]
   pub unsafe fn __readgsqword(offset: u32) -> usize {
      let out: usize;
      asm!(
          "mov {}, gs:[{:e}]",
          lateout(reg) out,
          in(reg) offset,
          options(nostack, pure, readonly),
      );
      out
   }

   #[inline]
   pub unsafe fn NtCurrentTeb() -> *mut TEB {
      let teb_offset = FIELD_OFFSET!(NT_TIB, Self_) as u32;
      #[cfg(target_arch = "x86_64")]
      {
         __readgsqword(teb_offset) as _
      }
      #[cfg(target_arch = "x86")]
      {
         __readfsdword(teb_offset) as _
      }
   }
}
