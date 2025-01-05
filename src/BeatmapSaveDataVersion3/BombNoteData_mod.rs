#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct BombNoteData {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem,
    >,
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::BombNoteData =>
    "BeatmapSaveDataVersion3"."BombNoteData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::BombNoteData {
    type Target = quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::BeatmapSaveDataItem,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::BombNoteData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
impl crate::BeatmapSaveDataVersion3::BombNoteData {
    pub fn New(
        beat: f32,
        line: i32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, line, layer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        line: i32,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, line, layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layer(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_line(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_line", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BombNoteData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::BombNoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
