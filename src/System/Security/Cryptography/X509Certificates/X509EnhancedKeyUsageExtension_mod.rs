#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
#[repr(C)]
#[derive(Debug)]
pub struct X509EnhancedKeyUsageExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _enhKeyUsage: *mut crate::System::Security::Cryptography::OidCollection,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension =>
    "System.Security.Cryptography.X509Certificates"."X509EnhancedKeyUsageExtension"
);
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
impl crate::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension {
    pub fn CopyFrom(
        &mut self,
        asnEncodedData: *mut crate::System::Security::Cryptography::AsnEncodedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (asnEncodedData))?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        extension: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::AsnDecodeStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::AsnDecodeStatus = __cordl_object
            .invoke("Decode", (extension))?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Encode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_AsnEncodedData__cordl_bool1(
        encodedEnhancedKeyUsages: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedEnhancedKeyUsages, critical))?;
        Ok(__cordl_object)
    }
    pub fn New_OidCollection__cordl_bool2(
        enhancedKeyUsages: *mut crate::System::Security::Cryptography::OidCollection,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enhancedKeyUsages, critical))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
        multiLine: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (multiLine))?;
        Ok(__cordl_ret)
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
    pub fn _ctor_AsnEncodedData__cordl_bool1(
        &mut self,
        encodedEnhancedKeyUsages: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedEnhancedKeyUsages, critical))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OidCollection__cordl_bool2(
        &mut self,
        enhancedKeyUsages: *mut crate::System::Security::Cryptography::OidCollection,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (enhancedKeyUsages, critical))?;
        Ok(__cordl_ret)
    }
    pub fn get_EnhancedKeyUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::OidCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::OidCollection = __cordl_object
            .invoke("get_EnhancedKeyUsages", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Security+Cryptography+X509Certificates+X509EnhancedKeyUsageExtension"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509EnhancedKeyUsageExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}