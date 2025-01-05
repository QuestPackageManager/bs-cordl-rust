#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
#[repr(C)]
#[derive(Debug)]
pub struct RectangularVertexClipper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_WorldCorners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub m_CanvasCorners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
}
#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::RectangularVertexClipper =>
    "UnityEngine.UI"."RectangularVertexClipper"
);
#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
impl std::ops::Deref for crate::UnityEngine::UI::RectangularVertexClipper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
impl std::ops::DerefMut for crate::UnityEngine::UI::RectangularVertexClipper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
impl crate::UnityEngine::UI::RectangularVertexClipper {
    pub fn GetCanvasRect(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetCanvasRect", (t, c))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+UI+RectangularVertexClipper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::RectangularVertexClipper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
