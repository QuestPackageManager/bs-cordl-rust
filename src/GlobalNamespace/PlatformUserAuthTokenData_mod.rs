#[cfg(feature = "PlatformUserAuthTokenData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformUserAuthTokenData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _token_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _validPlatformEnvironment_k__BackingField: crate::GlobalNamespace::PlatformEnvironment,
}
#[cfg(feature = "PlatformUserAuthTokenData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformUserAuthTokenData => ""
    ."PlatformUserAuthTokenData"
);
#[cfg(feature = "PlatformUserAuthTokenData")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformUserAuthTokenData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformUserAuthTokenData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl crate::GlobalNamespace::PlatformUserAuthTokenData {
    pub fn New(
        token: *mut quest_hook::libil2cpp::Il2CppString,
        validPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token, validPlatformEnvironment))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        token: *mut quest_hook::libil2cpp::Il2CppString,
        validPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token, validPlatformEnvironment))?;
        Ok(__cordl_ret)
    }
    pub fn get_token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_token", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_validPlatformEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PlatformEnvironment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlatformEnvironment = __cordl_object
            .invoke("get_validPlatformEnvironment", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_token(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_token", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_validPlatformEnvironment(
        &mut self,
        value: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_validPlatformEnvironment", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformUserAuthTokenData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
