#[cfg(feature = "HMUI+SetPropertyUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+SetPropertyUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SetPropertyUtility => "HMUI"
    ."SetPropertyUtility"
);
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
