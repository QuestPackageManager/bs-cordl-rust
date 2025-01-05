#[cfg(feature = "UnityEngine+MinAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MinAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub min: f32,
}
#[cfg(feature = "UnityEngine+MinAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MinAttribute => "UnityEngine"
    ."MinAttribute"
);
#[cfg(feature = "UnityEngine+MinAttribute")]
impl std::ops::Deref for crate::UnityEngine::MinAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+MinAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::MinAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+MinAttribute")]
impl crate::UnityEngine::MinAttribute {
    pub fn New(
        min: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (min))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        min: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (min))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+MinAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::MinAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
