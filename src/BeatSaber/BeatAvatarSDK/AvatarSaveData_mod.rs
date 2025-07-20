#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub headTopId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub glassesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub facialHairId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub handsId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub clothesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub skinColorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub mouthId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub eyesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub headTopPrimaryColor: crate::UnityEngine::Color,
    pub headTopSecondaryColor: crate::UnityEngine::Color,
    pub glassesColor: crate::UnityEngine::Color,
    pub facialHairColor: crate::UnityEngine::Color,
    pub handsColor: crate::UnityEngine::Color,
    pub clothesPrimaryColor: crate::UnityEngine::Color,
    pub clothesSecondaryColor: crate::UnityEngine::Color,
    pub clothesDetailColor: crate::UnityEngine::Color,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "AvatarSaveData";
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::AvatarSaveData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::AvatarSaveData as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
