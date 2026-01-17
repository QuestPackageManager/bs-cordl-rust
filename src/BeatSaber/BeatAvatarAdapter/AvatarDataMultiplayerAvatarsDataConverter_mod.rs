#[cfg(
    feature = "cordl_class_BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter"
)]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct AvatarDataMultiplayerAvatarsDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarAdapter";
    const CLASS_NAME: &'static str = "AvatarDataMultiplayerAvatarsDataConverter";
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
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl std::ops::Deref
    for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl std::ops::DerefMut
    for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {
    pub fn CreateAvatarData(
        multiplayerAvatarsData: crate::GlobalNamespace::MultiplayerAvatarData,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::MultiplayerAvatarData),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarData,
                        >,
                        1usize,
                    >("CreateAvatarData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAvatarData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData> =
            unsafe { cordl_method_info.invoke_unchecked((), (multiplayerAvatarsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMultiplayerAvatarsData(
        avatarData: quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::BeatSaber::BeatAvatarSDK::AvatarData,
                        >),
                        crate::GlobalNamespace::MultiplayerAvatarData,
                        1usize,
                    >("CreateMultiplayerAvatarsData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMultiplayerAvatarsData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarData =
            unsafe { cordl_method_info.invoke_unchecked((), (avatarData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
