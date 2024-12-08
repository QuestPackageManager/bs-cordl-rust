#[cfg(feature = "UnityEngine+NativeClassAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeClassAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _QualifiedNativeName_k__BackingField: *mut crate::System::String,
    pub _Declaration_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+NativeClassAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NativeClassAttribute =>
    "UnityEngine"."NativeClassAttribute"
);
#[cfg(feature = "UnityEngine+NativeClassAttribute")]
impl std::ops::Deref for crate::UnityEngine::NativeClassAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NativeClassAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::NativeClassAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NativeClassAttribute")]
impl crate::UnityEngine::NativeClassAttribute {
    pub fn set_QualifiedNativeName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_QualifiedNativeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        qualifiedCppName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (qualifiedCppName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        qualifiedCppName: *mut crate::System::String,
        declaration: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (qualifiedCppName, declaration))?;
        Ok(__cordl_ret)
    }
    pub fn set_Declaration(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Declaration", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        qualifiedCppName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (qualifiedCppName))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        qualifiedCppName: *mut crate::System::String,
        declaration: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (qualifiedCppName, declaration))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+NativeClassAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::NativeClassAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
