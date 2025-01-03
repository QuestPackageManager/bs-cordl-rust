#[cfg(feature = "Mono+Security+Interface+TlsException")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsException {
    __cordl_parent: crate::System::Exception,
    pub alert: quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::Alert>,
}
#[cfg(feature = "Mono+Security+Interface+TlsException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::TlsException =>
    "Mono.Security.Interface"."TlsException"
);
#[cfg(feature = "Mono+Security+Interface+TlsException")]
impl std::ops::Deref for crate::Mono::Security::Interface::TlsException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+TlsException")]
impl std::ops::DerefMut for crate::Mono::Security::Interface::TlsException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+TlsException")]
impl crate::Mono::Security::Interface::TlsException {
    pub fn New_Alert0(
        alert: quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::Alert>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alert, message))?;
        Ok(__cordl_object.into())
    }
    pub fn New_AlertDescription1(
        description: crate::Mono::Security::Interface::AlertDescription,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (description, message))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Alert0(
        &mut self,
        alert: quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::Alert>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alert, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AlertDescription1(
        &mut self,
        description: crate::Mono::Security::Interface::AlertDescription,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (description, message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Interface+TlsException")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::TlsException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
