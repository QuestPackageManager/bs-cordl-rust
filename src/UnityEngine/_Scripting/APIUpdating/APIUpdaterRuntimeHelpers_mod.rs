#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct APIUpdaterRuntimeHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers =>
    "UnityEngine._Scripting.APIUpdating"."APIUpdaterRuntimeHelpers"
);
#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
impl std::ops::Deref
for crate::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
impl std::ops::DerefMut
for crate::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
impl crate::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers {}
#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
