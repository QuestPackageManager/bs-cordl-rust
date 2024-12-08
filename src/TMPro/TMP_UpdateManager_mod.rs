#[cfg(feature = "TMPro+TMP_UpdateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_UpdateManager {
    __cordl_parent: crate::System::Object,
    pub m_LayoutQueueLookup: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub m_LayoutRebuildQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Text,
    >,
    pub m_GraphicQueueLookup: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub m_GraphicRebuildQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Text,
    >,
    pub m_InternalUpdateLookup: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub m_InternalUpdateQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Text,
    >,
    pub m_CullingUpdateLookup: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub m_CullingUpdateQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_Text,
    >,
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_UpdateManager => "TMPro"
    ."TMP_UpdateManager"
);
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::Deref for crate::TMPro::TMP_UpdateManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl std::ops::DerefMut for crate::TMPro::TMP_UpdateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl crate::TMPro::TMP_UpdateManager {
    pub fn DoRebuilds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoRebuilds", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterTextElementForCullingUpdate(
        &mut self,
        element: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForCullingUpdate", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterTextElementForGraphicRebuild(
        &mut self,
        element: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterTextElementForLayoutRebuild(
        &mut self,
        element: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegisterTextObjectForUpdate(
        &mut self,
        textObject: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRegisterTextObjectForUpdate", (textObject))?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnRegisterTextElementForGraphicRebuild(
        &mut self,
        element: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextElementForGraphicRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnRegisterTextElementForLayoutRebuild(
        &mut self,
        element: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextElementForLayoutRebuild", (element))?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnRegisterTextObjectForUpdate(
        &mut self,
        textObject: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalUnRegisterTextObjectForUpdate", (textObject))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnCameraPreCull(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCameraPreCull", ())?;
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
#[cfg(feature = "TMPro+TMP_UpdateManager")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_UpdateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
