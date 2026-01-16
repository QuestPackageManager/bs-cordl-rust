#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+BpmEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmEventConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _songFrequency: i32,
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+BpmEventConverter")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapDataLoaderVersion4";
    const CLASS_NAME: &'static str = "BpmEventConverter";
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
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    pub fn Convert(
        &mut self,
        bpmData: quest_hook::libil2cpp::Gc<crate::BeatmapLevelSaveDataVersion4::BpmData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatmapLevelSaveDataVersion4::BpmData,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        1usize,
                    >("Convert")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Convert",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData> =
            unsafe { cordl_method_info.invoke_unchecked(self, (bpmData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        songFrequency: i32,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (songFrequency, bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        songFrequency: i32,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BpmTimeProcessor>,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (songFrequency, bpmTimeProcessor))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapDataLoaderVersion4+BpmEventConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
