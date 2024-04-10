#[allow(
   warnings,
   unused,
   non_snake_case,
   non_camel_case_types,
   non_upper_case_globals
)]
pub mod ffi {
   // use vendored `generated.rs` for docs.rs
   #[cfg(feature="docsrs")]
   include!("ffi/generated.rs");

   #[cfg(not(feature="docsrs"))]
   include!(concat!(env!("OUT_DIR"), "\\generated.rs"));

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
