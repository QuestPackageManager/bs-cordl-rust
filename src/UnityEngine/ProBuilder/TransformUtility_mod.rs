#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::TransformUtility =>
    "UnityEngine.ProBuilder"."TransformUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::TransformUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::TransformUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl crate::UnityEngine::ProBuilder::TransformUtility {
    pub fn InverseTransformVertex(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InverseTransformVertex", (transform, vertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReparentChildren(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReparentChildren", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformVertex(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TransformVertex", (transform, vertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnparentChildren(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnparentChildren", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::TransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
