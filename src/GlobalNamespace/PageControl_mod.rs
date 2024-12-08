#[cfg(feature = "PageControl")]
#[repr(C)]
#[derive(Debug)]
pub struct PageControl {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _content: *mut crate::UnityEngine::RectTransform,
    pub _spacing: f32,
    pub _elementPrefab: *mut PageControlElement,
    pub _activeElements: *mut crate::System::Collections::Generic::List_1<
        *mut PageControlElement,
    >,
    pub _inactiveElements: *mut crate::System::Collections::Generic::Queue_1<
        *mut PageControlElement,
    >,
    pub _selectedPage: i32,
    pub _pagesCount: i32,
}
#[cfg(feature = "PageControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PageControl => ""."PageControl"
);
#[cfg(feature = "PageControl")]
impl std::ops::Deref for PageControl {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PageControl")]
impl std::ops::DerefMut for PageControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PageControl")]
impl PageControl {
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
    pub fn SetPagesCount(
        &mut self,
        pagesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPagesCount", (pagesCount))?;
        Ok(__cordl_ret)
    }
    pub fn SetVisible(
        &mut self,
        isVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisible", (isVisible))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectedPageIndex(
        &mut self,
        page: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedPageIndex", (page))?;
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
#[cfg(feature = "PageControl")]
impl quest_hook::libil2cpp::ObjectType for PageControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
