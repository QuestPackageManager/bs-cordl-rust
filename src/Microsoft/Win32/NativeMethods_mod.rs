#[cfg(feature = "Microsoft+Win32+NativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::Win32::NativeMethods =>
    "Microsoft.Win32"."NativeMethods"
);
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::Deref for crate::Microsoft::Win32::NativeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl std::ops::DerefMut for crate::Microsoft::Win32::NativeMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl crate::Microsoft::Win32::NativeMethods {}
#[cfg(feature = "Microsoft+Win32+NativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::Microsoft::Win32::NativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
