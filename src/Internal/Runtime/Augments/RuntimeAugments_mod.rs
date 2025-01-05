#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeAugments {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Runtime::Augments::RuntimeAugments =>
    "Internal.Runtime.Augments"."RuntimeAugments"
);
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::RuntimeAugments {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::DerefMut for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl crate::Internal::Runtime::Augments::RuntimeAugments {
    pub fn ReportUnhandledException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReportUnhandledException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Callbacks() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Internal::Runtime::Augments::ReflectionExecutionDomainCallbacks,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Callbacks", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
