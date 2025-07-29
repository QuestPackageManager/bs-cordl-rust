#[cfg(feature = "cordl_class_BeatmapLevelsPromoModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _promotedBeatmapLevelPacksSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _updatedBeatmapLevelPacksSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _promotedBeatmapLevelsSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _updatedBeatmapLevelsSet: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatmapLevelsPromoModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelsPromoModel";
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
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoModel")]
impl crate::GlobalNamespace::BeatmapLevelsPromoModel {
    pub fn IsBeatmapLevelPackPromoted(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >),
                        bool,
                        1usize,
                    >("IsBeatmapLevelPackPromoted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBeatmapLevelPackPromoted", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapLevelPack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelPackUpdated(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >),
                        bool,
                        1usize,
                    >("IsBeatmapLevelPackUpdated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBeatmapLevelPackUpdated", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapLevelPack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelPromoted(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevel,
                        >),
                        bool,
                        1usize,
                    >("IsBeatmapLevelPromoted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBeatmapLevelPromoted", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBeatmapLevelUpdated(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevel,
                        >),
                        bool,
                        1usize,
                    >("IsBeatmapLevelUpdated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsBeatmapLevelUpdated", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        promoDataSO: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (promoDataSO))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        promoDataSO: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelsPromoDataSO,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (promoDataSO))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapLevelsPromoModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsPromoModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
