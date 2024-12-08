#[cfg(feature = "Mono+Unity+UnityTlsStream")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsStream {
    __cordl_parent: crate::Mono::Net::Security::MobileAuthenticatedStream,
}
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTlsStream => "Mono.Unity"
    ."UnityTlsStream"
);
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsStream {
    type Target = crate::Mono::Net::Security::MobileAuthenticatedStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
impl crate::Mono::Unity::UnityTlsStream {
    pub fn CreateContext(
        &mut self,
        options: *mut crate::Mono::Net::Security::MonoSslAuthenticationOptions,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Net::Security::MobileTlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Net::Security::MobileTlsContext = __cordl_object
            .invoke("CreateContext", (options))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
        owner: *mut crate::System::Net::Security::SslStream,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
        owner: *mut crate::System::Net::Security::SslStream,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
