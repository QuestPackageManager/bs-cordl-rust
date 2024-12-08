#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeTypeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Header_k__BackingField: *mut crate::System::String,
    pub _IntermediateScriptingStructName_k__BackingField: *mut crate::System::String,
    pub _CodegenOptions_k__BackingField: crate::UnityEngine::Bindings::CodegenOptions,
}
#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Bindings::NativeTypeAttribute =>
    "UnityEngine.Bindings"."NativeTypeAttribute"
);
#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
impl std::ops::Deref for crate::UnityEngine::Bindings::NativeTypeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Bindings::NativeTypeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
impl crate::UnityEngine::Bindings::NativeTypeAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_CodegenOptions1(
        codegenOptions: crate::UnityEngine::Bindings::CodegenOptions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (codegenOptions))?;
        Ok(__cordl_object)
    }
    pub fn New_CodegenOptions_String3(
        codegenOptions: crate::UnityEngine::Bindings::CodegenOptions,
        intermediateStructName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (codegenOptions, intermediateStructName))?;
        Ok(__cordl_object)
    }
    pub fn New_String2(
        header: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CodegenOptions1(
        &mut self,
        codegenOptions: crate::UnityEngine::Bindings::CodegenOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (codegenOptions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CodegenOptions_String3(
        &mut self,
        codegenOptions: crate::UnityEngine::Bindings::CodegenOptions,
        intermediateStructName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (codegenOptions, intermediateStructName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String2(
        &mut self,
        header: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header))?;
        Ok(__cordl_ret)
    }
    pub fn set_CodegenOptions(
        &mut self,
        value: crate::UnityEngine::Bindings::CodegenOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CodegenOptions", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Header(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Header", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IntermediateScriptingStructName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IntermediateScriptingStructName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Bindings+NativeTypeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Bindings::NativeTypeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
