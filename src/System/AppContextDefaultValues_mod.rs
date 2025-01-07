#[cfg(feature = "System+AppContextDefaultValues")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContextDefaultValues {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+AppContextDefaultValues")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::AppContextDefaultValues {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "AppContextDefaultValues";
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
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::Deref for crate::System::AppContextDefaultValues {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::DerefMut for crate::System::AppContextDefaultValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl crate::System::AppContextDefaultValues {
    pub fn PopulateDefaultValues() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateDefaultValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSwitchOverride(
        switchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        overrideValue: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSwitchOverride", (switchName, overrideValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppContextDefaultValues {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
