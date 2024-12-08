#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
#[repr(C)]
#[derive(Debug)]
pub struct PublicKey {
    __cordl_parent: crate::System::Object,
    pub _keyValue: *mut crate::System::Security::Cryptography::AsnEncodedData,
    pub _params: *mut crate::System::Security::Cryptography::AsnEncodedData,
    pub _oid: *mut crate::System::Security::Cryptography::Oid,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::PublicKey =>
    "System.Security.Cryptography.X509Certificates"."PublicKey"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl crate::System::Security::Cryptography::X509Certificates::PublicKey {
    pub fn New(
        oid: *mut crate::System::Security::Cryptography::Oid,
        parameters: *mut crate::System::Security::Cryptography::AsnEncodedData,
        keyValue: *mut crate::System::Security::Cryptography::AsnEncodedData,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, parameters, keyValue))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        oid: *mut crate::System::Security::Cryptography::Oid,
        parameters: *mut crate::System::Security::Cryptography::AsnEncodedData,
        keyValue: *mut crate::System::Security::Cryptography::AsnEncodedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, parameters, keyValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_EncodedKeyValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::AsnEncodedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::AsnEncodedData = __cordl_object
            .invoke("get_EncodedKeyValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncodedParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::AsnEncodedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::AsnEncodedData = __cordl_object
            .invoke("get_EncodedParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Oid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::Cryptography::Oid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::Oid = __cordl_object
            .invoke("get_Oid", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+PublicKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::PublicKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
