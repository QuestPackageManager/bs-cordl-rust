#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericDropdownMenu {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dropdownMenu: *mut crate::UnityEngine::UIElements::GenericDropdownMenu,
}
#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UIToolkitUtilities::GenericDropdownMenu
    => "BGLib.UIToolkitUtilities"."GenericDropdownMenu"
);
#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
impl std::ops::Deref for crate::BGLib::UIToolkitUtilities::GenericDropdownMenu {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
impl std::ops::DerefMut for crate::BGLib::UIToolkitUtilities::GenericDropdownMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
impl crate::BGLib::UIToolkitUtilities::GenericDropdownMenu {
    pub fn AddDisabledItem(
        &mut self,
        itemName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isChecked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDisabledItem", (itemName, isChecked))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddItem_Action0(
        &mut self,
        itemName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isChecked: bool,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddItem_Action_1_Il2CppObject1(
        &mut self,
        itemName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isChecked: bool,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSeparator(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSeparator", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn DropDown(
        &mut self,
        position: crate::UnityEngine::Rect,
        targetElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        anchored: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DropDown", (position, targetElement, anchored))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateItem(
        &mut self,
        itemName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isChecked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateItem", (itemName, isChecked))?;
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
#[cfg(feature = "BGLib+UIToolkitUtilities+GenericDropdownMenu")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UIToolkitUtilities::GenericDropdownMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
