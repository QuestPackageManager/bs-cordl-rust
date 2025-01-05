#[cfg(feature = "EssentialHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct EssentialHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "EssentialHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EssentialHelpers => ""
    ."EssentialHelpers"
);
#[cfg(feature = "EssentialHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::EssentialHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EssentialHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::EssentialHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EssentialHelpers")]
impl crate::GlobalNamespace::EssentialHelpers {
    pub fn GetOrAddComponent<T>(
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrAddComponent", (go))?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeDestroy(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SafeDestroy", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentTimeStamp() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentTimeStamp", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EssentialHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EssentialHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
