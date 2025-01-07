#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroupLists {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapEventDataBoxGroupListDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BeatmapEventDataBoxGroupList,
            >,
        >,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    pub _beatToTimeConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatToTimeConverter,
    >,
    pub _updateBeatmapDataOnInsert: bool,
    pub _lightEventConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLightEventConverter,
    >,
}
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventDataBoxGroupLists";
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
#[cfg(feature = "BeatmapEventDataBoxGroupLists")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventDataBoxGroupLists {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        > = __cordl_object.invoke("Insert", (groupId, beatmapEventDataBoxGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        updateBeatmapDataOnInsert: bool,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatmapData,
                    beatToTimeConverter,
                    updateBeatmapDataOnInsert,
                    lightEventConverter,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        groupId: i32,
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
            .invoke("Remove", (groupId, nodeToDelete))?;
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
    pub fn ToggleUpdateBeatmapDataOnInsert(
        &mut self,
        enableUpdateOnInsert: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ToggleUpdateBeatmapDataOnInsert", (enableUpdateOnInsert))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        updateBeatmapDataOnInsert: bool,
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
                (
                    beatmapData,
                    beatToTimeConverter,
                    updateBeatmapDataOnInsert,
                    lightEventConverter,
                ),
            )?;
        Ok(__cordl_ret.into())
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
