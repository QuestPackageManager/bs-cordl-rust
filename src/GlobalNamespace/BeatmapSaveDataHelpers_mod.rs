#[cfg(feature = "BeatmapSaveDataHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveDataHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapSaveDataHelpers => ""
    ."BeatmapSaveDataHelpers"
);
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapSaveDataHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapSaveDataHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl crate::GlobalNamespace::BeatmapSaveDataHelpers {
    #[cfg(feature = "BeatmapSaveDataHelpers+VersionSerializedData")]
    pub type VersionSerializedData = crate::GlobalNamespace::BeatmapSaveDataHelpers_VersionSerializedData;
    pub fn GetVersion(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVersion", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVersionAsync(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::Version>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::Version>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVersionAsync", (data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapSaveDataHelpers {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: *mut quest_hook::libil2cpp::Il2CppString,
    pub version: *mut quest_hook::libil2cpp::Il2CppString,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn get_v(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_v", ())?;
        Ok(__cordl_ret.into())
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
