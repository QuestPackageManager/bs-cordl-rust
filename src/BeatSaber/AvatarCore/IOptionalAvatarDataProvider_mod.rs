#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IOptionalAvatarDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IOptionalAvatarDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::IOptionalAvatarDataProvider => "BeatSaber.AvatarCore"
    ."IOptionalAvatarDataProvider"
);
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
    pub fn get_playbackDelaySeconds(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_playbackDelaySeconds", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_dataDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_dataDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_dataDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetDisplayContext(
        &mut self,
        avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDisplayContext", (avatarDisplayContext))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        > = __cordl_object.invoke("get_currentData", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
