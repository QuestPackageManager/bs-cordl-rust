#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MovedFromAttribute {
    __cordl_parent: crate::System::Attribute,
    pub data: crate::UnityEngine::Scripting::APIUpdating::MovedFromAttributeData,
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Scripting::APIUpdating::MovedFromAttribute =>
    "UnityEngine.Scripting.APIUpdating"."MovedFromAttribute"
);
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl std::ops::Deref for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl std::ops::DerefMut
for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    pub fn _ctor__cordl_bool_String_String_String0(
        &mut self,
        autoUpdateAPI: bool,
        sourceNamespace: *mut crate::System::String,
        sourceAssembly: *mut crate::System::String,
        sourceClassName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (autoUpdateAPI, sourceNamespace, sourceAssembly, sourceClassName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        sourceNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sourceNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool_String_String_String0(
        autoUpdateAPI: bool,
        sourceNamespace: *mut crate::System::String,
        sourceAssembly: *mut crate::System::String,
        sourceClassName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (autoUpdateAPI, sourceNamespace, sourceAssembly, sourceClassName),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        sourceNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sourceNamespace))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
