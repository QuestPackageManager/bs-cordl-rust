#[cfg(feature = "Mono+Unity+Debug")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Unity+Debug")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::Debug => "Mono.Unity"."Debug"
);
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::Deref for crate::Mono::Unity::Debug {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::DerefMut for crate::Mono::Unity::Debug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl crate::Mono::Unity::Debug {
    pub fn CheckAndThrow_Gc_AlertDescription0(
        errorState: crate::Mono::Unity::UnityTls_unitytls_errorstate,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckAndThrow", (errorState, context, defaultAlert))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAndThrow_UnityTls_unitytls_x509verify_result_Gc_AlertDescription1(
        errorState: crate::Mono::Unity::UnityTls_unitytls_errorstate,
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckAndThrow", (errorState, verifyResult, context, defaultAlert))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::Debug {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
