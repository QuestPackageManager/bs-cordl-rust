#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DllImportAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: *mut quest_hook::libil2cpp::Il2CppString,
    pub EntryPoint: *mut quest_hook::libil2cpp::Il2CppString,
    pub CharSet: crate::System::Runtime::InteropServices::CharSet,
    pub SetLastError: bool,
    pub ExactSpelling: bool,
    pub PreserveSig: bool,
    pub CallingConvention: crate::System::Runtime::InteropServices::CallingConvention,
    pub BestFitMapping: bool,
    pub ThrowOnUnmappableChar: bool,
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::DllImportAttribute =>
    "System.Runtime.InteropServices"."DllImportAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::DllImportAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::DllImportAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl crate::System::Runtime::InteropServices::DllImportAttribute {
    pub fn New_Il2CppString1(
        dllName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dllName))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_CharSet__cordl_bool__cordl_bool__cordl_bool_CallingConvention__cordl_bool__cordl_bool0(
        dllName: *mut quest_hook::libil2cpp::Il2CppString,
        entryPoint: *mut quest_hook::libil2cpp::Il2CppString,
        charSet: crate::System::Runtime::InteropServices::CharSet,
        exactSpelling: bool,
        setLastError: bool,
        preserveSig: bool,
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
        bestFitMapping: bool,
        throwOnUnmappableChar: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    dllName,
                    entryPoint,
                    charSet,
                    exactSpelling,
                    setLastError,
                    preserveSig,
                    callingConvention,
                    bestFitMapping,
                    throwOnUnmappableChar,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        dllName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dllName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_CharSet__cordl_bool__cordl_bool__cordl_bool_CallingConvention__cordl_bool__cordl_bool0(
        &mut self,
        dllName: *mut quest_hook::libil2cpp::Il2CppString,
        entryPoint: *mut quest_hook::libil2cpp::Il2CppString,
        charSet: crate::System::Runtime::InteropServices::CharSet,
        exactSpelling: bool,
        setLastError: bool,
        preserveSig: bool,
        callingConvention: crate::System::Runtime::InteropServices::CallingConvention,
        bestFitMapping: bool,
        throwOnUnmappableChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    dllName,
                    entryPoint,
                    charSet,
                    exactSpelling,
                    setLastError,
                    preserveSig,
                    callingConvention,
                    bestFitMapping,
                    throwOnUnmappableChar,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DllImportAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::DllImportAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
