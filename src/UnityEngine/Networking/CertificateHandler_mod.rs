#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateHandler {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Networking::CertificateHandler =>
    "UnityEngine.Networking"."CertificateHandler"
);
#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
impl std::ops::Deref for crate::UnityEngine::Networking::CertificateHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
impl std::ops::DerefMut for crate::UnityEngine::Networking::CertificateHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
impl crate::UnityEngine::Networking::CertificateHandler {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificate(
        &mut self,
        certificateData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateCertificate", (certificateData))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateCertificateNative(
        &mut self,
        certificateData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateCertificateNative", (certificateData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Networking+CertificateHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::CertificateHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
