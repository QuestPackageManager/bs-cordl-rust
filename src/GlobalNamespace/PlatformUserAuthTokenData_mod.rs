#[cfg(feature = "PlatformUserAuthTokenData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformUserAuthTokenData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _token_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _validPlatformEnvironment_k__BackingField: crate::GlobalNamespace::PlatformEnvironment,
}
#[cfg(feature = "PlatformUserAuthTokenData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformUserAuthTokenData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformUserAuthTokenData";
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
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        validPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (token, validPlatformEnvironment))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        validPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (token, validPlatformEnvironment))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_token", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_validPlatformEnvironment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PlatformEnvironment> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlatformEnvironment = __cordl_object
            .invoke("get_validPlatformEnvironment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_token(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_token", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
