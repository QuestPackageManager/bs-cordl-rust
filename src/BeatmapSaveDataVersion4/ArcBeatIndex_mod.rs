#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct ArcBeatIndex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hb: f32,
    pub hi: i32,
    pub hr: i32,
    pub tb: f32,
    pub ti: i32,
    pub tr: i32,
    pub ai: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::ArcBeatIndex =>
    "BeatmapSaveDataVersion4"."ArcBeatIndex"
);
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl crate::BeatmapSaveDataVersion4::ArcBeatIndex {
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
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl AsRef<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn as_ref(&self) -> &crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl AsMut<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn as_mut(&mut self) -> &mut crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl AsRef<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ArcBeatIndex")]
impl AsMut<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::ArcBeatIndex {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
