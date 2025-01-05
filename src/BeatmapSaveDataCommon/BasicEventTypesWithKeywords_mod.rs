#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEventTypesWithKeywords {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub d: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
            >,
        >,
    >,
}
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataCommon::BasicEventTypesWithKeywords => "BeatmapSaveDataCommon"
    ."BasicEventTypesWithKeywords"
);
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl std::ops::Deref for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl std::ops::DerefMut for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    #[cfg(
        feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
    )]
    pub type BasicEventTypesForKeyword = crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword;
    pub fn New(
        data: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                >,
            >,
        > = __cordl_object.invoke("get_data", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub e: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::BeatmapSaveDataCommon::BeatmapEventType,
        >,
    >,
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword =>
    "BeatmapSaveDataCommon"."BasicEventTypesWithKeywords/BasicEventTypesForKeyword"
);
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl std::ops::Deref
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl std::ops::DerefMut
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    pub fn New(
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::BeatmapSaveDataCommon::BeatmapEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyword, eventTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::BeatmapSaveDataCommon::BeatmapEventType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyword, eventTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::BeatmapSaveDataCommon::BeatmapEventType,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::BeatmapSaveDataCommon::BeatmapEventType,
            >,
        > = __cordl_object.invoke("get_eventTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_keyword", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
