#[cfg(feature = "System+LocalAppContextSwitches")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalAppContextSwitches {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+LocalAppContextSwitches")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::LocalAppContextSwitches => "System"
    ."LocalAppContextSwitches"
);
#[cfg(feature = "System+LocalAppContextSwitches")]
impl std::ops::Deref for crate::System::LocalAppContextSwitches {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalAppContextSwitches")]
impl std::ops::DerefMut for crate::System::LocalAppContextSwitches {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalAppContextSwitches")]
impl crate::System::LocalAppContextSwitches {
    pub fn get_AllowArbitraryTypeInstantiation() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AllowArbitraryTypeInstantiation", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+LocalAppContextSwitches")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalAppContextSwitches {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
