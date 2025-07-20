#[cfg(feature = "BeatmapDataNoArrowsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataNoArrowsTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataNoArrowsTransform";
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
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IReadonlyBeatmapData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IReadonlyBeatmapData,
                        >,
                        1usize,
                    >("CreateTransformedData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateTransformedData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = unsafe { cordl_method_info.invoke_unchecked((), (beatmapData))? };
        Ok(__cordl_ret.into())
    }
    pub fn _CreateTransformedData_g__ProcessData_0_0(
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                        1usize,
                    >("<CreateTransformedData>g__ProcessData|0_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<CreateTransformedData>g__ProcessData|0_0", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = unsafe { cordl_method_info.invoke_unchecked((), (beatmapDataItem))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
