#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct X509KeyUsageExtension {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509Extension,
    pub _keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    pub _status: crate::System::Security::Cryptography::AsnDecodeStatus,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension =>
    "System.Security.Cryptography.X509Certificates"."X509KeyUsageExtension"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509Extension;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    pub const friendlyName: &'static str = "Key Usage";
    pub const oid: &'static str = "2.5.29.15";
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
        encodedKeyUsage: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedKeyUsage, critical))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509KeyUsageFlags__cordl_bool2(
        &mut self,
        keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyUsages, critical))?;
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
    pub fn get_KeyUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags = __cordl_object
            .invoke("get_KeyUsages", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetValidFlags(
        &mut self,
        flags: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags = __cordl_object
            .invoke("GetValidFlags", (flags))?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_AsnEncodedData__cordl_bool1(
        encodedKeyUsage: *mut crate::System::Security::Cryptography::AsnEncodedData,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedKeyUsage, critical))?;
        Ok(__cordl_object)
    }
    pub fn New_X509KeyUsageFlags__cordl_bool2(
        keyUsages: crate::System::Security::Cryptography::X509Certificates::X509KeyUsageFlags,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyUsages, critical))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509KeyUsageExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509KeyUsageExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
