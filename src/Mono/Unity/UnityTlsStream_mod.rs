#[cfg(feature = "Mono+Unity+UnityTlsStream")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsStream {
    __cordl_parent: crate::Mono::Net::Security::MobileAuthenticatedStream,
}
#[cfg(feature = "Mono+Unity+UnityTlsStream")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::UnityTlsStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTlsStream";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        options: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MonoSslAuthenticationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsContext,
        > = __cordl_object.invoke("CreateContext", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_ret.into())
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
