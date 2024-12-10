#[cfg(feature = "TestInvitePlatformHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TestInvitePlatformHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TestInvitePlatformHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TestInvitePlatformHandler => ""
    ."TestInvitePlatformHandler"
);
#[cfg(feature = "TestInvitePlatformHandler")]
impl std::ops::Deref for crate::GlobalNamespace::TestInvitePlatformHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TestInvitePlatformHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::TestInvitePlatformHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TestInvitePlatformHandler")]
impl crate::GlobalNamespace::TestInvitePlatformHandler {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenInvitePanel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSupported", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TestInvitePlatformHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TestInvitePlatformHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TestInvitePlatformHandler")]
impl AsRef<crate::GlobalNamespace::IInvitePlatformHandler>
for crate::GlobalNamespace::TestInvitePlatformHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IInvitePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TestInvitePlatformHandler")]
impl AsMut<crate::GlobalNamespace::IInvitePlatformHandler>
for crate::GlobalNamespace::TestInvitePlatformHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IInvitePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
