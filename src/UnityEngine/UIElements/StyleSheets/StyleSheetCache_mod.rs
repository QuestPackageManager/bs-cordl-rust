#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSheetCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSheetCache =>
    "UnityEngine.UIElements.StyleSheets"."StyleSheetCache"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache {
    #[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKey")]
    pub type SheetHandleKey = crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey;
    #[cfg(
        feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
    )]
    pub type SheetHandleKeyComparer = crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer;
    pub fn GetPropertyId(
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyId", (rule, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyIds_StyleRule1(
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyIds", (rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyIds_StyleSheet_i32_0(
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        ruleIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyIds", (sheet, ruleIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKey")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StyleSheetCache_SheetHandleKey {
    pub sheetInstanceID: i32,
    pub index: i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey =>
    "UnityEngine.UIElements.StyleSheets"."StyleSheetCache/SheetHandleKey"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKey")]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey {
    pub fn _ctor(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sheet, index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSheetCache_SheetHandleKeyComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer =>
    "UnityEngine.UIElements.StyleSheets"."StyleSheetCache/SheetHandleKeyComparer"
);
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    pub fn Equals(
        &mut self,
        x: crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
        y: crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        key: crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (key))?;
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
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    >,
>
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StyleSheetCache+SheetHandleKeyComparer"
)]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    >,
>
for crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKeyComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StyleSheets::StyleSheetCache_SheetHandleKey,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
