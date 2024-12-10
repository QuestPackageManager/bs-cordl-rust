#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshGenerationContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Painter2D: *mut crate::UnityEngine::UIElements::Painter2D,
    pub painter: *mut crate::UnityEngine::UIElements::IStylePainter,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MeshGenerationContext
    => "UnityEngine.UIElements"."MeshGenerationContext"
);
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MeshGenerationContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MeshGenerationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
impl crate::UnityEngine::UIElements::MeshGenerationContext {
    #[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext+MeshFlags")]
    pub type MeshFlags = crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags;
    pub fn New(
        painter: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStylePainter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (painter))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        painter: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStylePainter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (painter))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasPainter2D(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPainter2D", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_painter2D(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Painter2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Painter2D,
        > = __cordl_object.invoke("get_painter2D", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MeshGenerationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext+MeshFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshGenerationContext_MeshFlags {
    None = 0i32,
    SkipDynamicAtlas = 2i32,
    UVisDisplacement = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContext+MeshFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshGenerationContext_MeshFlags =>
    "UnityEngine.UIElements"."MeshGenerationContext/MeshFlags"
);
