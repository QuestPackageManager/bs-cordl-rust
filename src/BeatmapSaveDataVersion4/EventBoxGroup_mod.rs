#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBoxGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub t: crate::BeatmapSaveDataVersion4::EventBoxGroupType,
    pub g: i32,
    pub b: f32,
    pub e: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::EventBox>,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::EventBoxGroup =>
    "BeatmapSaveDataVersion4"."EventBoxGroup"
);
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl crate::BeatmapSaveDataVersion4::EventBoxGroup {
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
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl AsRef<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn as_ref(&self) -> &crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl AsMut<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn as_mut(&mut self) -> &mut crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl AsRef<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroup")]
impl AsMut<crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat>>
for crate::BeatmapSaveDataVersion4::EventBoxGroup {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<*mut crate::BeatmapSaveDataCommon::IBeat> {
        unsafe { std::mem::transmute(self) }
    }
}
