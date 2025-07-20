#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IOptionalAvatarDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "IOptionalAvatarDataProvider";
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
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
impl crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
    pub fn SetDisplayContext(
        &mut self,
        avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::BeatSaber::AvatarCore::AvatarDisplayContext),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDisplayContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "SetDisplayContext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (avatarDisplayContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_dataDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::BeatSaber::AvatarCore::OptionalAvatarData,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_dataDidChangeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "add_dataDidChangeEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_currentData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        u32,
                        crate::BeatSaber::AvatarCore::OptionalAvatarData,
                    >,
                >,
                0usize,
            >("get_currentData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "get_currentData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::BeatSaber::AvatarCore::OptionalAvatarData,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_playbackDelaySeconds(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_playbackDelaySeconds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "get_playbackDelaySeconds",
                    0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_dataDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::BeatSaber::AvatarCore::OptionalAvatarData,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_dataDidChangeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider as
                    quest_hook::libil2cpp::Type > ::class(), "remove_dataDidChangeEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
