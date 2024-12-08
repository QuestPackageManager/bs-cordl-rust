#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroupLists {
    __cordl_parent: crate::System::Object,
    pub _beatmapEventDataBoxGroupListDict: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::GlobalNamespace::BeatmapEventDataBoxGroupList,
    >,
    pub _beatmapData: *mut crate::GlobalNamespace::BeatmapData,
    pub _beatToTimeConverter: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    pub _updateBeatmapDataOnInsert: bool,
}
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEventDataBoxGroupLists
    => ""."BeatmapEventDataBoxGroupLists"
);
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
impl crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    pub fn Insert(
        &mut self,
        groupId: i32,
        beatmapEventDataBoxGroup: *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        > = __cordl_object.invoke("Insert", (groupId, beatmapEventDataBoxGroup))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapData: *mut crate::GlobalNamespace::BeatmapData,
        beatToTimeConverter: *mut crate::GlobalNamespace::IBeatToTimeConverter,
        updateBeatmapDataOnInsert: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beatmapData, beatToTimeConverter, updateBeatmapDataOnInsert),
            )?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        groupId: i32,
        nodeToDelete: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (groupId, nodeToDelete))?;
        Ok(__cordl_ret)
    }
    pub fn SyncWithBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncWithBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToggleUpdateBeatmapDataOnInsert(
        &mut self,
        enableUpdateOnInsert: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ToggleUpdateBeatmapDataOnInsert", (enableUpdateOnInsert))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beatmapData: *mut crate::GlobalNamespace::BeatmapData,
        beatToTimeConverter: *mut crate::GlobalNamespace::IBeatToTimeConverter,
        updateBeatmapDataOnInsert: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (beatmapData, beatToTimeConverter, updateBeatmapDataOnInsert),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
