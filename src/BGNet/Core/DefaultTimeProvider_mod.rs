#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::DefaultTimeProvider => "BGNet.Core"
    ."DefaultTimeProvider"
);
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl std::ops::Deref for crate::BGNet::Core::DefaultTimeProvider {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl std::ops::DerefMut for crate::BGNet::Core::DefaultTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl crate::BGNet::Core::DefaultTimeProvider {
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGNet+Core+DefaultTimeProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Core::DefaultTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
