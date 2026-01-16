#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBaseDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "FloatFxBaseDataConverter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    pub fn Convert(
        beat: f32,
        floatFxEvent: crate::BeatmapSaveDataVersion4::FloatFxEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, crate::BeatmapSaveDataVersion4::FloatFxEvent),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::FloatFxBaseData,
                        >,
                        2usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Convert",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData> =
            unsafe { cordl_method_info.invoke_unchecked((), (beat, floatFxEvent))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
