#[cfg(feature = "UnityEngine+UIElements+StyleSheet+ImportStruct")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StyleSheet_ImportStruct {
    pub styleSheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub mediaQueries: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet+ImportStruct")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSheet_ImportStruct
    => "UnityEngine.UIElements"."StyleSheet/ImportStruct"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheet+ImportStruct")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheet_ImportStruct {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet+ImportStruct")]
impl crate::UnityEngine::UIElements::StyleSheet_ImportStruct {}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleSheet {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_ImportedWithErrors: bool,
    pub m_ImportedWithWarnings: bool,
    pub m_Rules: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::StyleRule,
    >,
    pub m_ComplexSelectors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
    pub floats: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub dimensions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleSheets::Dimension,
    >,
    pub colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    pub strings: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub assets: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
    pub imports: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleSheet_ImportStruct,
    >,
    pub m_FlattenedImportedStyleSheets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::StyleSheet,
    >,
    pub m_ContentHash: i32,
    pub scalableImages: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleSheets::ScalableImage,
    >,
    pub orderedNameSelectors: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
    pub orderedTypeSelectors: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
    pub orderedClassSelectors: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::UnityEngine::UIElements::StyleComplexSelector,
    >,
    pub m_IsDefaultStyleSheet: bool,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSheet =>
    "UnityEngine.UIElements"."StyleSheet"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheet {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
impl crate::UnityEngine::UIElements::StyleSheet {
    #[cfg(feature = "UnityEngine+UIElements+StyleSheet+ImportStruct")]
    pub type ImportStruct = crate::UnityEngine::UIElements::StyleSheet_ImportStruct;
    pub fn CheckAccess<T>(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppArray<T>,
        _cordl_type: crate::UnityEngine::UIElements::StyleValueType,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("CheckAccess", (list, _cordl_type, handle))?;
        Ok(__cordl_ret)
    }
    pub fn FlattenImportedStyleSheetsRecursive_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlattenImportedStyleSheetsRecursive", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlattenImportedStyleSheetsRecursive_StyleSheet1(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlattenImportedStyleSheetsRecursive", (sheet))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadAssetReference(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("ReadAssetReference", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadColor(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("ReadColor", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadDimension(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::Dimension,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::Dimension = __cordl_object
            .invoke("ReadDimension", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadEnum(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadEnum", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFloat(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ReadFloat", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleValueFunction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleValueFunction = __cordl_object
            .invoke("ReadFunction", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFunctionName(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadFunctionName", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadKeyword(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleValueKeyword,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleValueKeyword = __cordl_object
            .invoke("ReadKeyword", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadMissingAssetReferenceUrl(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadMissingAssetReferenceUrl", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadResourcePath(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadResourcePath", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadScalableImage(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::ScalableImage,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::ScalableImage = __cordl_object
            .invoke("ReadScalableImage", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadString(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadString", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn ReadVariable(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadVariable", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn SetupReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupReferences", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryCheckAccess<T>(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppArray<T>,
        _cordl_type: crate::UnityEngine::UIElements::StyleValueType,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryCheckAccess", (list, _cordl_type, handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadAssetReference(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadAssetReference", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadColor(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryReadColor", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadDimension(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::Dimension,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadDimension", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadEnum(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryReadEnum", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadFloat(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryReadFloat", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadResourcePath(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadResourcePath", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadString(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryReadString", (handle, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryReadVariable(
        &mut self,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryReadVariable", (handle, value))?;
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
    pub fn get_complexSelectors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleComplexSelector,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleComplexSelector,
        > = __cordl_object.invoke("get_complexSelectors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentHash(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_contentHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flattenedRecursiveImports(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StyleSheet,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StyleSheet,
        > = __cordl_object.invoke("get_flattenedRecursiveImports", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_importedWithErrors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_importedWithErrors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_importedWithWarnings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_importedWithWarnings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDefaultStyleSheet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDefaultStyleSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleRule,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleRule,
        > = __cordl_object.invoke("get_rules", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_complexSelectors(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleComplexSelector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_complexSelectors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_contentHash(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentHash", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_importedWithErrors(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_importedWithErrors", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_importedWithWarnings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_importedWithWarnings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isDefaultStyleSheet(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDefaultStyleSheet", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rules(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleRule,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rules", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheet")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::StyleSheet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}