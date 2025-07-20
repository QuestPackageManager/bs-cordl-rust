#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticAvatarVisualDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "StaticAvatarVisualDataProvider";
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
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    pub fn New(
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (avatarsData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::MultiplayerAvatarsData),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (avatarsData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::MultiplayerAvatarsData,
                0usize,
            >("get_avatarsData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "get_avatarsData", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
