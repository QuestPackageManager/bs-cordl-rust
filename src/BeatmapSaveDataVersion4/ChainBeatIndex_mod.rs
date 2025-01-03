#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct ChainBeatIndex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hb: f32,
    pub hr: i32,
    pub tb: f32,
    pub tr: i32,
    pub i: i32,
    pub ci: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::ChainBeatIndex =>
    "BeatmapSaveDataVersion4"."ChainBeatIndex"
);
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl crate::BeatmapSaveDataVersion4::ChainBeatIndex {
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
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beat", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl AsRef<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn as_ref(&self) -> &crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl AsMut<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn as_mut(&mut self) -> &mut crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl AsRef<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ChainBeatIndex")]
impl AsMut<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::ChainBeatIndex {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
