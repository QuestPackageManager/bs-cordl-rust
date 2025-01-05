#[cfg(feature = "IReadonlyBeatmapLineData")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadonlyBeatmapLineData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IReadonlyBeatmapLineData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IReadonlyBeatmapLineData => ""
    ."IReadonlyBeatmapLineData"
);
#[cfg(feature = "IReadonlyBeatmapLineData")]
impl std::ops::Deref for crate::GlobalNamespace::IReadonlyBeatmapLineData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyBeatmapLineData")]
impl std::ops::DerefMut for crate::GlobalNamespace::IReadonlyBeatmapLineData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyBeatmapLineData")]
impl crate::GlobalNamespace::IReadonlyBeatmapLineData {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_beatmapObjectsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>,
            >,
        > = __cordl_object.invoke("get_beatmapObjectsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IReadonlyBeatmapLineData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IReadonlyBeatmapLineData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
