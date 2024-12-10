#[cfg(feature = "PS5ActivityIdsModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PS5ActivityIdsModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _progressActivities: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::ProgressActivitySO,
    >,
    pub _competetiveActivities: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::CompetetiveActivitySO,
    >,
}
#[cfg(feature = "PS5ActivityIdsModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PS5ActivityIdsModelSO => ""
    ."PS5ActivityIdsModelSO"
);
#[cfg(feature = "PS5ActivityIdsModelSO")]
impl std::ops::Deref for crate::GlobalNamespace::PS5ActivityIdsModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PS5ActivityIdsModelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PS5ActivityIdsModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PS5ActivityIdsModelSO")]
impl crate::GlobalNamespace::PS5ActivityIdsModelSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_competetiveActivities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::CompetetiveActivitySO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::CompetetiveActivitySO,
            >,
        > = __cordl_object.invoke("get_competetiveActivities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_progressActivities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::ProgressActivitySO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::ProgressActivitySO,
            >,
        > = __cordl_object.invoke("get_progressActivities", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PS5ActivityIdsModelSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PS5ActivityIdsModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
