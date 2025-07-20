#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveDataUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapDataLoaderVersion4::BeatmapLevelSaveDataUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveDataUtils";
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
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BeatmapLevelSaveDataUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BeatmapLevelSaveDataUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
impl crate::BeatmapDataLoaderVersion4::BeatmapLevelSaveDataUtils {
    pub fn MigrateBeatmapLevelSaveData(
        beatmapLevelSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("MigrateBeatmapLevelSaveData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MigrateBeatmapLevelSaveData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapLevelSaveData))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapLevelSaveDataUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BeatmapLevelSaveDataUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
