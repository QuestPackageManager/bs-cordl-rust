#[cfg(feature = "StartMiddleEndButtonsGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct StartMiddleEndButtonsGroup {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StartMiddleEndButtonsGroup =>
    ""."StartMiddleEndButtonsGroup"
);
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl std::ops::Deref for crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetLayoutHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayoutVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutVertical", ())?;
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
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ILayoutController>>
for crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ILayoutController> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StartMiddleEndButtonsGroup")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ILayoutController>>
for crate::GlobalNamespace::StartMiddleEndButtonsGroup {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ILayoutController> {
        unsafe { std::mem::transmute(self) }
    }
}
