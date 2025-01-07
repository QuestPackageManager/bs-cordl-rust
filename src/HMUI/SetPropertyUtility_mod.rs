#[cfg(feature = "HMUI+SetPropertyUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::SetPropertyUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "SetPropertyUtility";
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
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl std::ops::Deref for crate::HMUI::SetPropertyUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl std::ops::DerefMut for crate::HMUI::SetPropertyUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl crate::HMUI::SetPropertyUtility {
    pub fn SetClass<T>(
        currentValue: quest_hook::libil2cpp::ByRefMut<T>,
        newValue: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetClass", (currentValue, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColor(
        currentValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        newValue: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColor", (currentValue, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStruct<T>(
        currentValue: quest_hook::libil2cpp::ByRefMut<T>,
        newValue: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetStruct", (currentValue, newValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SetPropertyUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
