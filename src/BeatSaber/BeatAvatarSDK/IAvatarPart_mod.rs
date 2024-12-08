#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarPart {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::IAvatarPart =>
    "BeatSaber.BeatAvatarSDK"."IAvatarPart"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::IAvatarPart {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::IAvatarPart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
impl crate::BeatSaber::BeatAvatarSDK::IAvatarPart {
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localizationKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizationKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+IAvatarPart")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::BeatAvatarSDK::IAvatarPart {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
