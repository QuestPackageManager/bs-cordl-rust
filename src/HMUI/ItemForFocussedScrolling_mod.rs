#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
#[repr(C)]
#[derive(Debug)]
pub struct ItemForFocussedScrolling {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ItemForFocussedScrolling => "HMUI"
    ."ItemForFocussedScrolling"
);
#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
impl std::ops::Deref for crate::HMUI::ItemForFocussedScrolling {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
impl std::ops::DerefMut for crate::HMUI::ItemForFocussedScrolling {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
impl crate::HMUI::ItemForFocussedScrolling {
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
#[cfg(feature = "HMUI+ItemForFocussedScrolling")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ItemForFocussedScrolling {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
