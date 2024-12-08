#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct FxEventsCollection {
    __cordl_parent: crate::System::Object,
    pub _il: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
    >,
    pub _fl: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::FxEventsCollection =>
    "BeatmapSaveDataVersion3"."FxEventsCollection"
);
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::FxEventsCollection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::FxEventsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
impl crate::BeatmapSaveDataVersion3::FxEventsCollection {
    pub fn get_intEventsList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
        > = __cordl_object.invoke("get_intEventsList", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_floatEventsList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
        > = __cordl_object.invoke("get_floatEventsList", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddEventAndGetIndex_FloatFxEventBaseData0(
        &mut self,
        e: *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AddEventAndGetIndex", (e))?;
        Ok(__cordl_ret)
    }
    pub fn AddEventAndGetIndex_IntFxEventBaseData1(
        &mut self,
        e: *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AddEventAndGetIndex", (e))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_List_1_1(
        &mut self,
        intFxEventBaseData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
        >,
        floatFxEventBaseData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (intFxEventBaseData, floatFxEventBaseData))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_List_1_1(
        intFxEventBaseData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::IntFxEventBaseData,
        >,
        floatFxEventBaseData: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FloatFxEventBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (intFxEventBaseData, floatFxEventBaseData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventsCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::FxEventsCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
