#[cfg(feature = "UnityEngine+RangeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct RangeAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub min: f32,
    pub max: f32,
}
#[cfg(feature = "UnityEngine+RangeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RangeAttribute => "UnityEngine"
    ."RangeAttribute"
);
#[cfg(feature = "UnityEngine+RangeAttribute")]
impl std::ops::Deref for crate::UnityEngine::RangeAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RangeAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::RangeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RangeAttribute")]
impl crate::UnityEngine::RangeAttribute {
    pub fn New(
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (min, max))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (min, max))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RangeAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RangeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
