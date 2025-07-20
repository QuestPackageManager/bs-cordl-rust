#[cfg(feature = "BeatmapEventTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventTypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapEventTypeExtensions";
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
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl crate::GlobalNamespace::BeatmapEventTypeExtensions {
    pub fn IsCoreLightIntensityChangeEvent(
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BasicBeatmapEventType),
                        bool,
                        1usize,
                    >("IsCoreLightIntensityChangeEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCoreLightIntensityChangeEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (basicBeatmapEventType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEventTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
