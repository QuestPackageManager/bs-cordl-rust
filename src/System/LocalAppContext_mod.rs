#[cfg(feature = "System+LocalAppContext")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalAppContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+LocalAppContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::LocalAppContext => "System"
    ."LocalAppContext"
);
#[cfg(feature = "System+LocalAppContext")]
impl std::ops::Deref for crate::System::LocalAppContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalAppContext")]
impl std::ops::DerefMut for crate::System::LocalAppContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+LocalAppContext")]
impl crate::System::LocalAppContext {
    #[cfg(feature = "System+LocalAppContext+__c")]
    pub type __c = crate::System::LocalAppContext___c;
    pub fn GetCachedSwitchValue(
        switchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        switchValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedSwitchValue", (switchName, switchValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedSwitchValueInternal(
        switchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        switchValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedSwitchValueInternal", (switchName, switchValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DisableCaching() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DisableCaching", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+LocalAppContext")]
impl quest_hook::libil2cpp::ObjectType for crate::System::LocalAppContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
