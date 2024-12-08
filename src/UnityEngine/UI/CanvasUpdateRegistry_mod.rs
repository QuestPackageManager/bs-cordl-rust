#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct CanvasUpdateRegistry {
    __cordl_parent: crate::System::Object,
    pub m_PerformingLayoutUpdate: bool,
    pub m_PerformingGraphicUpdate: bool,
    pub m_CanvasUpdateProfilerStrings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_LayoutRebuildQueue: *mut crate::UnityEngine::UI::Collections::IndexedSet_1<
        *mut crate::UnityEngine::UI::ICanvasElement,
    >,
    pub m_GraphicRebuildQueue: *mut crate::UnityEngine::UI::Collections::IndexedSet_1<
        *mut crate::UnityEngine::UI::ICanvasElement,
    >,
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::CanvasUpdateRegistry =>
    "UnityEngine.UI"."CanvasUpdateRegistry"
);
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl std::ops::Deref for crate::UnityEngine::UI::CanvasUpdateRegistry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::CanvasUpdateRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl crate::UnityEngine::UI::CanvasUpdateRegistry {
    pub const m_CullingUpdateProfilerString: &'static str = "ClipperRegistry.Cull";
    pub fn CleanInvalidItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanInvalidItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalDisableCanvasElementForGraphicRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalDisableCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalDisableCanvasElementForLayoutRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalDisableCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalRegisterCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalRegisterCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ObjectValidForUpdate(
        &mut self,
        element: *mut crate::UnityEngine::UI::ICanvasElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ObjectValidForUpdate", (element))?;
        Ok(__cordl_ret)
    }
    pub fn PerformUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+CanvasUpdateRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::CanvasUpdateRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}