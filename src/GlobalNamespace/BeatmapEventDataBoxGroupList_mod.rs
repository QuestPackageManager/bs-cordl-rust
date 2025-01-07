#[cfg(feature = "BeatmapEventDataBoxGroupList")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroupList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub updateBeatmapDataOnInsert: bool,
    pub _beatmapEventDataBoxGroupProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapEventDataBoxGroupProcessor,
    >,
    pub _sortedList: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SortedList_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventDataBoxGroup>,
        >,
    >,
    pub _groupId: i32,
    pub _beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    pub _beatToTimeConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatToTimeConverter,
    >,
    pub _lightEventConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLightEventConverter,
    >,
    pub _nonSyncedInsertsExist: bool,
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventDataBoxGroupList {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBoxGroupList";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroupList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventDataBoxGroupList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl crate::GlobalNamespace::BeatmapEventDataBoxGroupList {
    pub fn Insert(
        &mut self,
        beatmapEventDataBoxGroup: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventDataBoxGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapEventDataBoxGroup,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapEventDataBoxGroup,
                >,
            >,
        > = __cordl_object.invoke("Insert", (beatmapEventDataBoxGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        groupId: i32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (groupId, beatmapData, beatToTimeConverter, lightEventConverter),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn NoDomainReloadInit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NoDomainReloadInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        nodeToDelete: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapEventDataBoxGroup,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (nodeToDelete))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncWithBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncWithBeatmapData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (groupId, beatmapData, beatToTimeConverter, lightEventConverter),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventDataBoxGroupList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
