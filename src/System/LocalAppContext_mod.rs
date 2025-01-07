#[cfg(feature = "System+LocalAppContext")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalAppContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+LocalAppContext")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::LocalAppContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "LocalAppContext";
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
