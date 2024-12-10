#[cfg(feature = "UnityEngine+UIElements+TreeView")]
#[repr(C)]
#[derive(Debug)]
pub struct TreeView {
    __cordl_parent: crate::UnityEngine::UIElements::BaseTreeView,
    pub m_MakeItem: *mut crate::System::Func_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_BindItem: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        i32,
    >,
    pub _unbindItem_k__BackingField: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        i32,
    >,
    pub _destroyItem_k__BackingField: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TreeView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeView =>
    "UnityEngine.UIElements"."TreeView"
);
#[cfg(feature = "UnityEngine+UIElements+TreeView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TreeView {
    type Target = crate::UnityEngine::UIElements::BaseTreeView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TreeView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView")]
impl crate::UnityEngine::UIElements::TreeView {
    #[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::TreeView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::TreeView_UxmlTraits;
    pub fn CreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        > = __cordl_object.invoke("CreateViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasValidDataAndBindings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValidDataAndBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Func_1_Action_2_1(
        makeItem: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
        bindItem: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (makeItem, bindItem))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Func_1_Action_2_1(
        &mut self,
        makeItem: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
        bindItem: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (makeItem, bindItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        > = __cordl_object.invoke("get_bindItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_destroyItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_destroyItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_makeItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_makeItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unbindItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        > = __cordl_object.invoke("get_unbindItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bindItem(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindItem", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_makeItem(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_makeItem", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::TreeView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct TreeView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::TreeView,
        *mut crate::UnityEngine::UIElements::TreeView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeView_UxmlFactory =>
    "UnityEngine.UIElements"."TreeView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TreeView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::TreeView,
        *mut crate::UnityEngine::UIElements::TreeView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TreeView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
impl crate::UnityEngine::UIElements::TreeView_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TreeView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct TreeView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseTreeView_UxmlTraits,
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TreeView_UxmlTraits =>
    "UnityEngine.UIElements"."TreeView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TreeView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseTreeView_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TreeView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
impl crate::UnityEngine::UIElements::TreeView_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+TreeView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TreeView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
