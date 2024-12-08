#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
#[repr(C)]
#[derive(Debug)]
pub struct SpecialEventsForKeyword {
    __cordl_parent: crate::System::Object,
    pub _keyword: *mut crate::System::String,
    pub _specialEvents: *mut crate::System::Collections::Generic::List_1<
        crate::BeatmapSaveDataCommon::BeatmapEventType,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."SpecialEventsForKeyword"
);
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
impl std::ops::Deref
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
impl std::ops::DerefMut
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword {
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_keyword", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_specialEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        > = __cordl_object.invoke("get_specialEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        keyword: *mut crate::System::String,
        specialEvents: *mut crate::System::Collections::Generic::List_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyword, specialEvents))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyword: *mut crate::System::String,
        specialEvents: *mut crate::System::Collections::Generic::List_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyword, specialEvents))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventsForKeyword")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
