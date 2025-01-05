#[cfg(feature = "UnityEngine+_Scripting+APIUpdating+APIUpdaterRuntimeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct APIUpdaterRuntimeHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::UnityEngine::_Scripting::APIUpdating::APIUpdaterRuntimeHelpers {
    pub fn GetMovedFromAttributeDataForType(
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        assembly: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        nsp: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        klass: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMovedFromAttributeDataForType",
                (sourceType, assembly, nsp, klass),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObsoleteTypeRedirection(
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        assemblyName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        nsp: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        className: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetObsoleteTypeRedirection",
                (sourceType, assemblyName, nsp, className),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
