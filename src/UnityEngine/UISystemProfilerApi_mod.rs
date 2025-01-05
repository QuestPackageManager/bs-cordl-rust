#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
#[repr(C)]
#[derive(Debug)]
pub struct UISystemProfilerApi {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UISystemProfilerApi =>
    "UnityEngine"."UISystemProfilerApi"
);
#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
impl std::ops::Deref for crate::UnityEngine::UISystemProfilerApi {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
impl std::ops::DerefMut for crate::UnityEngine::UISystemProfilerApi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
impl crate::UnityEngine::UISystemProfilerApi {
    #[cfg(feature = "UnityEngine+UISystemProfilerApi+SampleType")]
    pub type SampleType = crate::UnityEngine::UISystemProfilerApi_SampleType;
    pub fn AddMarker(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddMarker", (name, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample(
        _cordl_type: crate::UnityEngine::UISystemProfilerApi_SampleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginSample", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndSample(
        _cordl_type: crate::UnityEngine::UISystemProfilerApi_SampleType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndSample", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UISystemProfilerApi {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi+SampleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UISystemProfilerApi_SampleType {
    #[default]
    Layout = 0i32,
    Render = 1i32,
}
#[cfg(feature = "UnityEngine+UISystemProfilerApi+SampleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UISystemProfilerApi_SampleType =>
    "UnityEngine"."UISystemProfilerApi/SampleType"
);
