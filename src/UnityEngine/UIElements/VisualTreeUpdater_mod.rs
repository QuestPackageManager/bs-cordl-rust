#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeUpdater_UpdaterArray {
    __cordl_parent: crate::System::Object,
    pub m_VisualTreeUpdaters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::IVisualTreeUpdater,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray => "UnityEngine.UIElements"
    ."VisualTreeUpdater/UpdaterArray"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
impl crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Item_VisualTreeUpdatePhase0(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualTreeUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualTreeUpdater = __cordl_object
            .invoke("get_Item", (phase))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_1(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualTreeUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualTreeUpdater = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
        value: *mut crate::UnityEngine::UIElements::IVisualTreeUpdater,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (phase, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualTreeUpdater {
    __cordl_parent: crate::System::Object,
    pub m_Panel: *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
    pub m_UpdaterArray: *mut crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray,
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualTreeUpdater =>
    "UnityEngine.UIElements"."VisualTreeUpdater"
);
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualTreeUpdater {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualTreeUpdater {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
impl crate::UnityEngine::UIElements::VisualTreeUpdater {
    #[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater+UpdaterArray")]
    pub type UpdaterArray = crate::UnityEngine::UIElements::VisualTreeUpdater_UpdaterArray;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUpdater(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualTreeUpdater,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualTreeUpdater = __cordl_object
            .invoke("GetUpdater", (phase))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        panel: *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (panel))?;
        Ok(__cordl_object)
    }
    pub fn OnVersionChanged(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        versionChangeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVersionChanged", (ve, versionChangeType))?;
        Ok(__cordl_ret)
    }
    pub fn SetDefaultUpdaters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultUpdaters", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetUpdater<T>(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUpdater", (phase))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVisualTreePhase(
        &mut self,
        phase: crate::UnityEngine::UIElements::VisualTreeUpdatePhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisualTreePhase", (phase))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        panel: *mut crate::UnityEngine::UIElements::BaseVisualElementPanel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (panel))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualTreeUpdater")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualTreeUpdater {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
