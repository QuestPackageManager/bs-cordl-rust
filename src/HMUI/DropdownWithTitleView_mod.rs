#[cfg(feature = "HMUI+DropdownWithTitleView")]
#[repr(C)]
#[derive(Debug)]
pub struct DropdownWithTitleView {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::HMUI::SimpleTextDropdown>,
    pub _rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _titleRectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
}
#[cfg(feature = "HMUI+DropdownWithTitleView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::DropdownWithTitleView => "HMUI"
    ."DropdownWithTitleView"
);
#[cfg(feature = "HMUI+DropdownWithTitleView")]
impl std::ops::Deref for crate::HMUI::DropdownWithTitleView {
    type Target = quest_hook::libil2cpp::Gc<crate::HMUI::SimpleTextDropdown>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+DropdownWithTitleView")]
impl std::ops::DerefMut for crate::HMUI::DropdownWithTitleView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+DropdownWithTitleView")]
impl crate::HMUI::DropdownWithTitleView {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshSize(
        &mut self,
        dataSource: quest_hook::libil2cpp::Gc<crate::HMUI::TableView_IDataSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshSize", (dataSource))?;
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
#[cfg(feature = "HMUI+DropdownWithTitleView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::DropdownWithTitleView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
