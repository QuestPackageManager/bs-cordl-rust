#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
#[repr(C)]
#[derive(Debug)]
pub struct DuplicateNodeError {
    __cordl_parent: crate::System::Exception,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::KdTree::DuplicateNodeError =>
    "UnityEngine.ProBuilder.KdTree"."DuplicateNodeError"
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::KdTree::DuplicateNodeError {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::KdTree::DuplicateNodeError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
impl crate::UnityEngine::ProBuilder::KdTree::DuplicateNodeError {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+DuplicateNodeError")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::DuplicateNodeError {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
