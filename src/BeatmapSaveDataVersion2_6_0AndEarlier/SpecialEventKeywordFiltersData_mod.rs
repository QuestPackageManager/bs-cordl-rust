#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
#[repr(C)]
#[derive(Debug)]
pub struct SpecialEventKeywordFiltersData {
    __cordl_parent: crate::System::Object,
    pub _keywords: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."SpecialEventKeywordFiltersData"
);
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
impl std::ops::Deref
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
impl std::ops::DerefMut
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData {
    pub fn New(
        keywords: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keywords))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        keywords: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keywords))?;
        Ok(__cordl_ret)
    }
    pub fn get_keywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventsForKeyword,
        > = __cordl_object.invoke("get_keywords", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SpecialEventKeywordFiltersData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
