#[cfg(feature = "Mono+RuntimeMarshal")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeMarshal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+RuntimeMarshal")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeMarshal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeMarshal";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl std::ops::Deref for crate::Mono::RuntimeMarshal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl std::ops::DerefMut for crate::Mono::RuntimeMarshal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl crate::Mono::RuntimeMarshal {
    pub fn AsciHexDigitValue(c: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsciHexDigitValue", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeBlobArray(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeBlobArray", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeBlobSize(
        in_ptr: crate::System::IntPtr,
        out_ptr: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeBlobSize", (in_ptr, out_ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FreeAssemblyName(
        name: quest_hook::libil2cpp::ByRefMut<crate::Mono::MonoAssemblyName>,
        freeStruct: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FreeAssemblyName", (name, freeStruct))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarshalString(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::SafeStringMarshal> {
        let __cordl_ret: crate::Mono::SafeStringMarshal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarshalString", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn PtrToUtf8String(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PtrToUtf8String", (ptr))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::RuntimeMarshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
