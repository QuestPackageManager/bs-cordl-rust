#[cfg(feature = "System+Security+Cryptography+CspParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct CspParameters {
    __cordl_parent: crate::System::Object,
    pub ProviderType: i32,
    pub ProviderName: *mut crate::System::String,
    pub KeyContainerName: *mut crate::System::String,
    pub KeyNumber: i32,
    pub m_flags: i32,
}
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::CspParameters =>
    "System.Security.Cryptography"."CspParameters"
);
#[cfg(feature = "System+Security+Cryptography+CspParameters")]
impl std::ops::Deref for crate::System::Security::Cryptography::CspParameters {
    type Target = crate::System::Object;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(dwTypeIn: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dwTypeIn))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_String_String2(
        dwTypeIn: i32,
        strProviderNameIn: *mut crate::System::String,
        strContainerNameIn: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dwTypeIn, strProviderNameIn, strContainerNameIn))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_String_String_CspProviderFlags3(
        providerType: i32,
        providerName: *mut crate::System::String,
        keyContainerName: *mut crate::System::String,
        flags: crate::System::Security::Cryptography::CspProviderFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (providerType, providerName, keyContainerName, flags),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_String_String2(
        &mut self,
        dwTypeIn: i32,
        strProviderNameIn: *mut crate::System::String,
        strContainerNameIn: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dwTypeIn, strProviderNameIn, strContainerNameIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_String_String_CspProviderFlags3(
        &mut self,
        providerType: i32,
        providerName: *mut crate::System::String,
        keyContainerName: *mut crate::System::String,
        flags: crate::System::Security::Cryptography::CspProviderFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (providerType, providerName, keyContainerName, flags))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
