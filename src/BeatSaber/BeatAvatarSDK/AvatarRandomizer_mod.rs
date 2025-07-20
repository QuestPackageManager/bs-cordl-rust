#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarRandomizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "AvatarRandomizer";
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    pub fn RandomizeAll(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarPartsModel: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::AvatarData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RandomizeAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::AvatarRandomizer as
                    quest_hook::libil2cpp::Type > ::class(), "RandomizeAll", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (avatarData, avatarPartsModel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RandomizeColors(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RandomizeColors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::AvatarRandomizer as
                    quest_hook::libil2cpp::Type > ::class(), "RandomizeColors", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (avatarData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RandomizeModels(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarPartsModel: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::AvatarData,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RandomizeModels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::AvatarRandomizer as
                    quest_hook::libil2cpp::Type > ::class(), "RandomizeModels", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (avatarData, avatarPartsModel))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
