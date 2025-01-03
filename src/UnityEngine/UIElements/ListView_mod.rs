#[cfg(feature = "UnityEngine+UIElements+ListView")]
#[repr(C)]
#[derive(Debug)]
pub struct ListView {
    __cordl_parent: crate::UnityEngine::UIElements::BaseListView,
    pub m_MakeItem: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
    >,
    pub m_BindItem: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<*mut crate::UnityEngine::UIElements::VisualElement, i32>,
    >,
    pub _unbindItem_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<*mut crate::UnityEngine::UIElements::VisualElement, i32>,
    >,
    pub _destroyItem_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ListView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ListView =>
    "UnityEngine.UIElements"."ListView"
);
#[cfg(feature = "UnityEngine+UIElements+ListView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ListView {
    type Target = crate::UnityEngine::UIElements::BaseListView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ListView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView")]
impl crate::UnityEngine::UIElements::ListView {
    #[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::ListView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::ListView_UxmlTraits;
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
}
#[cfg(feature = "UnityEngine+UIElements+ListView")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ListView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ListView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::ListView,
        *mut crate::UnityEngine::UIElements::ListView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ListView_UxmlFactory =>
    "UnityEngine.UIElements"."ListView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ListView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::ListView,
        *mut crate::UnityEngine::UIElements::ListView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ListView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
impl crate::UnityEngine::UIElements::ListView_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct ListView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseListView_UxmlTraits,
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ListView_UxmlTraits =>
    "UnityEngine.UIElements"."ListView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ListView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseListView_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ListView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
impl crate::UnityEngine::UIElements::ListView_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+ListView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
