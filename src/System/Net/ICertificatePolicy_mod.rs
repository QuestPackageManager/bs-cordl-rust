#[cfg(feature = "System+Net+ICertificatePolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct ICertificatePolicy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+ICertificatePolicy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ICertificatePolicy => "System.Net"
    ."ICertificatePolicy"
);
#[cfg(feature = "System+Net+ICertificatePolicy")]
impl std::ops::Deref for crate::System::Net::ICertificatePolicy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ICertificatePolicy")]
impl std::ops::DerefMut for crate::System::Net::ICertificatePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ICertificatePolicy")]
impl crate::System::Net::ICertificatePolicy {
    pub fn CheckValidationResult(
        &mut self,
        srvPoint: *mut crate::System::Net::ServicePoint,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        request: *mut crate::System::Net::WebRequest,
        certificateProblem: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CheckValidationResult",
                (srvPoint, certificate, request, certificateProblem),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Net+ICertificatePolicy")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ICertificatePolicy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
