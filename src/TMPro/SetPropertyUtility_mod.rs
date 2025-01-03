#[cfg(feature = "TMPro+SetPropertyUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct SetPropertyUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::SetPropertyUtility => "TMPro"
    ."SetPropertyUtility"
);
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl std::ops::Deref for crate::TMPro::SetPropertyUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl std::ops::DerefMut for crate::TMPro::SetPropertyUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl crate::TMPro::SetPropertyUtility {
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
    pub fn SetEquatableStruct<T>(
        currentValue: quest_hook::libil2cpp::ByRefMut<T>,
        newValue: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEquatableStruct", (currentValue, newValue))?;
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
#[cfg(feature = "TMPro+SetPropertyUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::SetPropertyUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
