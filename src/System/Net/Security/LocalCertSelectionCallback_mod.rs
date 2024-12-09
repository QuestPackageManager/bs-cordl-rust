#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalCertSelectionCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Security::LocalCertSelectionCallback => "System.Net.Security"
    ."LocalCertSelectionCallback"
);
#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
impl std::ops::Deref for crate::System::Net::Security::LocalCertSelectionCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
impl std::ops::DerefMut for crate::System::Net::Security::LocalCertSelectionCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
impl crate::System::Net::Security::LocalCertSelectionCallback {
    pub fn Invoke(
        &mut self,
        targetHost: *mut quest_hook::libil2cpp::Il2CppString,
        localCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        remoteCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        acceptableIssuers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke(
                "Invoke",
                (targetHost, localCertificates, remoteCertificate, acceptableIssuers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Security+LocalCertSelectionCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Security::LocalCertSelectionCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
