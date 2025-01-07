#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenRequestSteam {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformUserId: i64,
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSteam {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GetXPlatformAccessTokenRequestSteam";
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
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl std::ops::Deref for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSteam {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl std::ops::DerefMut for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSteam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl crate::GlobalNamespace::GetXPlatformAccessTokenRequestSteam {
    pub fn New(
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformUserId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformToken, platformUserId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformUserId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformToken, platformUserId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSteam {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
