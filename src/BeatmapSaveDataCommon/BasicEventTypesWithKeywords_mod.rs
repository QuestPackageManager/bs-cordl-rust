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
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
    const CLASS_NAME: &'static str = "BasicEventTypesWithKeywords";
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
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl std::ops::Deref for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords")]
impl std::ops::DerefMut for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (data))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_data")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_data", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
    const CLASS_NAME: &'static str = "BasicEventTypesWithKeywords/BasicEventTypesForKeyword";
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
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl std::ops::Deref
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapSaveDataCommon+BasicEventTypesWithKeywords+BasicEventTypesForKeyword"
)]
impl std::ops::DerefMut
for crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords_BasicEventTypesForKeyword {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::BeatmapSaveDataCommon::BeatmapEventType,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keyword, eventTypes))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::BeatmapSaveDataCommon::BeatmapEventType,
                            >,
                        >,
                        0usize,
                    >("get_eventTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_eventTypes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::BeatmapSaveDataCommon::BeatmapEventType,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_keyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_keyword", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
