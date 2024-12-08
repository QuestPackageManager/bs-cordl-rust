#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
#[repr(C)]
#[derive(Debug)]
pub struct X500DistinguishedName {
    __cordl_parent: crate::System::Security::Cryptography::AsnEncodedData,
    pub name: *mut crate::System::String,
    pub canonEncoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X500DistinguishedName =>
    "System.Security.Cryptography.X509Certificates"."X500DistinguishedName"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName {
    type Target = crate::System::Security::Cryptography::AsnEncodedData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
impl crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName {
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn Decode(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X500DistinguishedNameFlags,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Decode", (flag))?;
        Ok(__cordl_ret)
    }
    pub fn Format(
        &mut self,
        multiLine: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Format", (multiLine))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        encodedDistinguishedName: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedDistinguishedName))?;
        Ok(__cordl_ret)
    }
    pub fn DecodeRawData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecodeRawData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        encodedDistinguishedName: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedDistinguishedName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X500DistinguishedName")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
