#[cfg(feature = "cordl_class_BeatmapDataCallbackWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataCallbackWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub aheadTime: f32,
    pub subtypeIdentifiers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "cordl_class_BeatmapDataCallbackWrapper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataCallbackWrapper";
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
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataCallbackWrapper")]
impl crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    pub fn CallCallback(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CallCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CallCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (beatmapData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aheadTime: f32,
        BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subtypeIdentifiers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (aheadTime, BasicBeatmapEventType, subtypeIdentifiers),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        aheadTime: f32,
        BasicBeatmapEventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subtypeIdentifiers: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f32,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
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
            cordl_method_info
                .invoke_unchecked(self, (aheadTime, BasicBeatmapEventType, subtypeIdentifiers))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapDataCallbackWrapper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapDataCallbackWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
