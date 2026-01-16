#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+EventBox")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBox {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub f: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    pub w: f32,
    pub d: crate::BeatmapSaveDataCommon::DistributionParamType,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+EventBox")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataVersion3::EventBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "EventBox";
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
#[cfg(feature = "BeatmapSaveDataVersion3+EventBox")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::EventBox {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBox")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::EventBox {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+EventBox")]
impl crate::BeatmapSaveDataVersion3::EventBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                indexFilter,
                beatDistributionParam,
                beatDistributionParamType,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
                        f32,
                        crate::BeatmapSaveDataCommon::DistributionParamType,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatDistributionParam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_beatDistributionParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_beatDistributionParam",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatDistributionParamType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::DistributionParamType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::BeatmapSaveDataCommon::DistributionParamType, 0usize>(
                        "get_beatDistributionParamType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_beatDistributionParamType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataCommon::DistributionParamType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_indexFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion3::IndexFilter,
                        >,
                        0usize,
                    >("get_indexFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_indexFilter", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::IndexFilter> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+EventBox")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::EventBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
