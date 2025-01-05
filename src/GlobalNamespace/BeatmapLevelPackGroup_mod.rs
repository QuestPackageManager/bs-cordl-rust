#[cfg(feature = "BeatmapLevelPackGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPackGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _groupName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _beatmapLevelPacks_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelPackGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelPackGroup => ""
    ."BeatmapLevelPackGroup"
);
#[cfg(feature = "BeatmapLevelPackGroup")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelPackGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelPackGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPackGroup")]
impl crate::GlobalNamespace::BeatmapLevelPackGroup {
    pub fn New(
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        collections: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupName, collections))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateBeatmapLevelPacks(
        &mut self,
        beatmapLevelPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBeatmapLevelPacks", (beatmapLevelPacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        collections: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupName, collections))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        > = __cordl_object.invoke("get_beatmapLevelPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_groupName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_groupName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapLevelPacks(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapLevelPacks", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelPackGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelPackGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
