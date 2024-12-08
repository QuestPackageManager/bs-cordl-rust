#[cfg(feature = "Deeplink")]
#[repr(C)]
#[derive(Debug)]
pub struct Deeplink {
    __cordl_parent: crate::System::Object,
    pub Destination: *mut crate::System::String,
    pub LevelID: *mut crate::System::String,
    pub PackID: *mut crate::System::String,
    pub Difficulty: *mut crate::System::String,
    pub Characteristic: *mut crate::System::String,
    pub MultiplayerLobbyCode: *mut crate::System::String,
    pub MultiplayerSecret: *mut crate::System::String,
}
#[cfg(feature = "Deeplink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for Deeplink => ""."Deeplink"
);
#[cfg(feature = "Deeplink")]
impl std::ops::Deref for Deeplink {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Deeplink")]
impl std::ops::DerefMut for Deeplink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Deeplink")]
impl Deeplink {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Deeplink")]
impl quest_hook::libil2cpp::ObjectType for Deeplink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
