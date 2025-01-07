#[cfg(feature = "System+Security+Cryptography+CspParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct CspParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ProviderType: i32,
    pub ProviderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub KeyContainerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub KeyNumber: i32,
    pub m_flags: i32,
}
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::CspParameters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography";
    const CLASS_NAME: &'static str = "CspParameters";
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
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
impl std::ops::Deref for crate::System::Security::Cryptography::CspParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::CspParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
impl crate::System::Security::Cryptography::CspParameters {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        dwTypeIn: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dwTypeIn))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Il2CppString_Il2CppString2(
        dwTypeIn: i32,
        strProviderNameIn: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        strContainerNameIn: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dwTypeIn, strProviderNameIn, strContainerNameIn))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Il2CppString_Il2CppString_CspProviderFlags3(
        providerType: i32,
        providerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyContainerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::System::Security::Cryptography::CspProviderFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (providerType, providerName, keyContainerName, flags),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        dwTypeIn: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dwTypeIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Il2CppString_Il2CppString2(
        &mut self,
        dwTypeIn: i32,
        strProviderNameIn: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        strContainerNameIn: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dwTypeIn, strProviderNameIn, strContainerNameIn))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Il2CppString_Il2CppString_CspProviderFlags3(
        &mut self,
        providerType: i32,
        providerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keyContainerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::System::Security::Cryptography::CspProviderFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (providerType, providerName, keyContainerName, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::CspProviderFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::CspProviderFlags = __cordl_object
            .invoke("get_Flags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Flags(
        &mut self,
        value: crate::System::Security::Cryptography::CspProviderFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Flags", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::CspParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
