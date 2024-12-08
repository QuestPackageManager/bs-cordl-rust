#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultDragAndDropClient {
    __cordl_parent: crate::UnityEngine::UIElements::DragAndDropData,
    pub m_GenericData: *mut crate::System::Collections::Hashtable,
    pub m_DraggedInfoLabel: *mut crate::UnityEngine::UIElements::Label,
    pub m_VisualMode: crate::UnityEngine::UIElements::DragVisualMode,
    pub m_UnityObjectReferences: *mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::Object,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultDragAndDropClient => "UnityEngine.UIElements"
    ."DefaultDragAndDropClient"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultDragAndDropClient {
    type Target = crate::UnityEngine::UIElements::DragAndDropData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultDragAndDropClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
impl crate::UnityEngine::UIElements::DefaultDragAndDropClient {
    pub fn DragCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DragCleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGenericData(
        &mut self,
        key: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetGenericData", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_source(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_source", ())?;
        Ok(__cordl_ret)
    }
    pub fn AcceptDrag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AcceptDrag", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetVisualMode(
        &mut self,
        mode: crate::UnityEngine::UIElements::DragVisualMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisualMode", (mode))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn StartDrag(
        &mut self,
        args: crate::UnityEngine::UIElements::StartDragArgs,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartDrag", (args, pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn get_data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::DragAndDropData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::DragAndDropData = __cordl_object
            .invoke("get_data", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDragAndDropClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultDragAndDropClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
