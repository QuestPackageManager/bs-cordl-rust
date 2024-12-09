#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoTlsSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _RemoteCertificateValidationCallback_k__BackingField: *mut crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
    pub _ClientCertificateSelectionCallback_k__BackingField: *mut crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback,
    pub _CertificateValidationTime_k__BackingField: crate::System::Nullable_1<
        crate::System::DateTime,
    >,
    pub _TrustAnchors_k__BackingField: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    pub _UserSettings_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _CertificateSearchPaths_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _SendCloseNotify_k__BackingField: bool,
    pub _ClientCertificateIssuers_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _DisallowUnauthenticatedCertificateRequest_k__BackingField: bool,
    pub _EnabledProtocols_k__BackingField: crate::System::Nullable_1<
        crate::Mono::Security::Interface::TlsProtocols,
    >,
    pub _EnabledCiphers_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::Mono::Security::Interface::CipherSuiteCode,
    >,
    pub cloned: bool,
    pub checkCertName: bool,
    pub checkCertRevocationStatus: bool,
    pub useServicePointManagerCallback: crate::System::Nullable_1<bool>,
    pub skipSystemValidators: bool,
    pub callbackNeedsChain: bool,
    pub certificateValidator: *mut crate::Mono::Security::Interface::ICertificateValidator,
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::MonoTlsSettings =>
    "Mono.Security.Interface"."MonoTlsSettings"
);
#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
impl std::ops::Deref for crate::Mono::Security::Interface::MonoTlsSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
impl std::ops::DerefMut for crate::Mono::Security::Interface::MonoTlsSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
impl crate::Mono::Security::Interface::MonoTlsSettings {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoTlsSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoTlsSettings = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloneWithValidator(
        &mut self,
        validator: *mut crate::Mono::Security::Interface::ICertificateValidator,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoTlsSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoTlsSettings = __cordl_object
            .invoke("CloneWithValidator", (validator))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_MonoTlsSettings1(
        other: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
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
    pub fn _ctor_MonoTlsSettings1(
        &mut self,
        other: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn get_CallbackNeedsCertificateChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CallbackNeedsCertificateChain", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateSearchPaths(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_CertificateSearchPaths", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateValidationTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTime>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTime> = __cordl_object
            .invoke("get_CertificateValidationTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateValidator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::ICertificateValidator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::ICertificateValidator = __cordl_object
            .invoke("get_CertificateValidator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientCertificateIssuers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ClientCertificateIssuers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientCertificateSelectionCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback = __cordl_object
            .invoke("get_ClientCertificateSelectionCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DisallowUnauthenticatedCertificateRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_DisallowUnauthenticatedCertificateRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnabledCiphers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::Mono::Security::Interface::CipherSuiteCode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::Mono::Security::Interface::CipherSuiteCode,
        > = __cordl_object.invoke("get_EnabledCiphers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnabledProtocols(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Mono::Security::Interface::TlsProtocols>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Mono::Security::Interface::TlsProtocols,
        > = __cordl_object.invoke("get_EnabledProtocols", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RemoteCertificateValidationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback = __cordl_object
            .invoke("get_RemoteCertificateValidationCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SendCloseNotify(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SendCloseNotify", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TrustAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection = __cordl_object
            .invoke("get_TrustAnchors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseServicePointManagerCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_UseServicePointManagerCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("get_UserSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CertificateSearchPaths(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateSearchPaths", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CertificateValidationTime(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateValidationTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientCertificateIssuers(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificateIssuers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ClientCertificateSelectionCallback(
        &mut self,
        value: *mut crate::Mono::Security::Interface::MonoLocalCertificateSelectionCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ClientCertificateSelectionCallback", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DisallowUnauthenticatedCertificateRequest(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DisallowUnauthenticatedCertificateRequest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EnabledCiphers(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::Mono::Security::Interface::CipherSuiteCode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnabledCiphers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EnabledProtocols(
        &mut self,
        value: crate::System::Nullable_1<crate::Mono::Security::Interface::TlsProtocols>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnabledProtocols", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RemoteCertificateValidationCallback(
        &mut self,
        value: *mut crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RemoteCertificateValidationCallback", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SendCloseNotify(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SendCloseNotify", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TrustAnchors(
        &mut self,
        value: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TrustAnchors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UseServicePointManagerCallback(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UseServicePointManagerCallback", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UserSettings(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserSettings", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::MonoTlsSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
