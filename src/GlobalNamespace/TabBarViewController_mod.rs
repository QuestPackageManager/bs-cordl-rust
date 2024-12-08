#[cfg(feature = "TabBarViewController+TabBarItem")]
#[repr(C)]
#[derive(Debug)]
pub struct TabBarViewController_TabBarItem {
    __cordl_parent: crate::System::Object,
    pub title: *mut crate::System::String,
    pub action: *mut crate::System::Action,
}
#[cfg(feature = "TabBarViewController+TabBarItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TabBarViewController_TabBarItem
    => ""."TabBarViewController/TabBarItem"
);
#[cfg(feature = "TabBarViewController+TabBarItem")]
impl std::ops::Deref for crate::GlobalNamespace::TabBarViewController_TabBarItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TabBarViewController+TabBarItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::TabBarViewController_TabBarItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TabBarViewController+TabBarItem")]
impl crate::GlobalNamespace::TabBarViewController_TabBarItem {
    pub fn New(
        title: *mut crate::System::String,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (title, action))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        title: *mut crate::System::String,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (title, action))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TabBarViewController+TabBarItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TabBarViewController_TabBarItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TabBarViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct TabBarViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _segmentedControll: *mut crate::HMUI::TextSegmentedControl,
    pub _contentSizeFilter: *mut crate::UnityEngine::UI::ContentSizeFitter,
    pub _labels: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _items: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TabBarViewController_TabBarItem,
    >,
    pub _shouldReloadData: bool,
}
#[cfg(feature = "TabBarViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TabBarViewController => ""."TabBarViewController"
);
#[cfg(feature = "TabBarViewController")]
impl std::ops::Deref for TabBarViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TabBarViewController")]
impl std::ops::DerefMut for TabBarViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TabBarViewController")]
impl TabBarViewController {
    #[cfg(feature = "TabBarViewController+TabBarItem")]
    pub type TabBarItem = crate::GlobalNamespace::TabBarViewController_TabBarItem;
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectCell", (segmentedControl, cellNumber))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectItem(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectItem", (index))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        items: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TabBarViewController_TabBarItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (items))?;
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
    pub fn get_selectedCellNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedCellNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sizeToFit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_sizeToFit", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sizeToFit(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sizeToFit", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TabBarViewController")]
impl quest_hook::libil2cpp::ObjectType for TabBarViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}