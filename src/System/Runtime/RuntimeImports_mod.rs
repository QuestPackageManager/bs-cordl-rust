#[cfg(feature = "System+Runtime+RuntimeImports")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeImports {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::RuntimeImports =>
    "System.Runtime"."RuntimeImports"
);
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl std::ops::Deref for crate::System::Runtime::RuntimeImports {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl std::ops::DerefMut for crate::System::Runtime::RuntimeImports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl crate::System::Runtime::RuntimeImports {
    pub fn Memmove(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memmove", (dest, src, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Memmove_wbarrier(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: u32,
        type_handle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memmove_wbarrier", (dest, src, len, type_handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RhZeroMemory(
        b: quest_hook::libil2cpp::ByRefMut<u8>,
        byteLength: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RhZeroMemory", (b, byteLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroMemory(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byteLength: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroMemory", (p, byteLength))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+RuntimeImports")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::RuntimeImports {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
