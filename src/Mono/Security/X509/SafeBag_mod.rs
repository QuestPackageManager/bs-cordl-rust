#[cfg(feature = "Mono+Security+X509+SafeBag")]
#[repr(C)]
#[derive(Debug)]
pub struct SafeBag {
    __cordl_parent: crate::System::Object,
    pub _bagOID: *mut crate::System::String,
    pub _asn1: *mut crate::Mono::Security::ASN1,
}
#[cfg(feature = "Mono+Security+X509+SafeBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::SafeBag =>
    "Mono.Security.X509"."SafeBag"
);
#[cfg(feature = "Mono+Security+X509+SafeBag")]
impl std::ops::Deref for crate::Mono::Security::X509::SafeBag {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+SafeBag")]
impl std::ops::DerefMut for crate::Mono::Security::X509::SafeBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+SafeBag")]
impl crate::Mono::Security::X509::SafeBag {
    pub fn get_BagOID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BagOID", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bagOID: *mut crate::System::String,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bagOID, asn1))?;
        Ok(__cordl_ret)
    }
    pub fn get_ASN1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::ASN1> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::ASN1 = __cordl_object
            .invoke("get_ASN1", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bagOID: *mut crate::System::String,
        asn1: *mut crate::Mono::Security::ASN1,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bagOID, asn1))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Security+X509+SafeBag")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::SafeBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
