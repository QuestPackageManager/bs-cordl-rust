#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteData {
    __cordl_parent: crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    pub _time: f32,
    pub _lineIndex: i32,
    pub _lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
    pub _cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "NoteData";
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
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    type Target = crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        _cordl_type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, lineIndex, lineLayer, _cordl_type, cutDirection),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        _cordl_type: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
        cutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_time, lineIndex, lineLayer, _cordl_type, cutDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = __cordl_object
            .invoke("get_cutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = __cordl_object
            .invoke("get_lineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
