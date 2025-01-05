#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_UpdateRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LayoutRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
    pub m_LayoutQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_GraphicRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
    pub m_GraphicQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_UpdateRegistry => "TMPro"
    ."TMP_UpdateRegistry"
);
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl std::ops::Deref for crate::TMPro::TMP_UpdateRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl std::ops::DerefMut for crate::TMPro::TMP_UpdateRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl crate::TMPro::TMP_UpdateRegistry {
    pub fn InternalRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalRegisterCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalRegisterCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PerformUpdateForCanvasRendererObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformUpdateForCanvasRendererObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformUpdateForMeshRendererObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformUpdateForMeshRendererObjects", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCanvasElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCanvasElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterCanvasElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnRegisterCanvasElementForRebuild", (element))?;
        Ok(__cordl_ret.into())
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateRegistry>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateRegistry> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_UpdateRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
