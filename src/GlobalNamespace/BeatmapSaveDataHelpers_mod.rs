#[cfg(feature = "BeatmapSaveDataHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveDataHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapSaveDataHelpers => ""."BeatmapSaveDataHelpers"
);
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl std::ops::Deref for BeatmapSaveDataHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl std::ops::DerefMut for BeatmapSaveDataHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl BeatmapSaveDataHelpers {
    #[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
    pub type VersionSerializedData = crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData;
    #[cfg(feature = "BeatmapSaveDataHelpers+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::BeatmapSaveDataHelpers___c__DisplayClass6_0;
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl quest_hook::libil2cpp::ObjectType for BeatmapSaveDataHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveDataHelpers_VersionSerializedData {
    __cordl_parent: crate::System::Object,
    pub _version: *mut crate::System::String,
    pub version: *mut crate::System::String,
}
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData => ""
    ."BeatmapSaveDataHelpers/VersionSerializedData"
);
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
impl crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_v(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_v", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
