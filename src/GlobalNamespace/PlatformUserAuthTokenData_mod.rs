#[cfg(feature = "PlatformUserAuthTokenData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformUserAuthTokenData {
    __cordl_parent: crate::System::Object,
    pub _token_k__BackingField: *mut crate::System::String,
    pub _validPlatformEnvironment_k__BackingField: PlatformEnvironment,
}
#[cfg(feature = "PlatformUserAuthTokenData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlatformUserAuthTokenData => ""
    ."PlatformUserAuthTokenData"
);
#[cfg(feature = "PlatformUserAuthTokenData")]
impl std::ops::Deref for PlatformUserAuthTokenData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl std::ops::DerefMut for PlatformUserAuthTokenData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl PlatformUserAuthTokenData {
    pub fn _ctor(
        &mut self,
        token: *mut crate::System::String,
        validPlatformEnvironment: PlatformEnvironment,
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_token", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_validPlatformEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<PlatformEnvironment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: PlatformEnvironment = __cordl_object
            .invoke("get_validPlatformEnvironment", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_token(
        &mut self,
        value: *mut crate::System::String,
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
        value: PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_validPlatformEnvironment", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        token: *mut crate::System::String,
        validPlatformEnvironment: PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token, validPlatformEnvironment))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlatformUserAuthTokenData")]
impl quest_hook::libil2cpp::ObjectType for PlatformUserAuthTokenData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
