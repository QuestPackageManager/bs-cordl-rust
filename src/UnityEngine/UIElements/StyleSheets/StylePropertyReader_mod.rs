#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyReader_GetCursorIdFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction =>
    "UnityEngine.UIElements.StyleSheets"."StylePropertyReader/GetCursorIdFunction"
);
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
impl crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction {
    pub fn Invoke(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        handle: crate::UnityEngine::UIElements::StyleValueHandle,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", (sheet, handle))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyReader {
    __cordl_parent: crate::System::Object,
    pub m_Values: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
    >,
    pub m_ValueCount: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_Resolver: *mut crate::UnityEngine::UIElements::StyleVariableResolver,
    pub m_Sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub m_Properties: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::StyleProperty,
    >,
    pub m_PropertyIds: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    >,
    pub m_CurrentValueIndex: i32,
    pub m_CurrentPropertyIndex: i32,
    pub _property_k__BackingField: *mut crate::UnityEngine::UIElements::StyleProperty,
    pub _propertyId_k__BackingField: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    pub _valueCount_k__BackingField: i32,
    pub _dpiScaling_k__BackingField: f32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::StylePropertyReader =>
    "UnityEngine.UIElements.StyleSheets"."StylePropertyReader"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
impl crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader {
    #[cfg(
        feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader+GetCursorIdFunction"
    )]
    pub type GetCursorIdFunction = crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader_GetCursorIdFunction;
    pub fn set_propertyId(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_propertyId", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_valueCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_valueCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadEnum(
        &mut self,
        enumType: crate::UnityEngine::UIElements::StyleSheets::StyleEnumType,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadEnum", (enumType, index))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue = __cordl_object
            .invoke("GetValue", (index))?;
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
    pub fn ReadColor(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("ReadColor", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadInt(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadInt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTranslate(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = __cordl_object
            .invoke("ReadTranslate", (index))?;
        Ok(__cordl_ret)
    }
    pub fn set_valueCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_valueCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTransformOrigin(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransformOrigin> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = __cordl_object
            .invoke("ReadTransformOrigin", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFloat(&mut self, index: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ReadFloat", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueType(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleValueType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleValueType = __cordl_object
            .invoke("GetValueType", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFont(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Font> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Font = __cordl_object
            .invoke("ReadFont", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackgroundPositionX(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = __cordl_object
            .invoke("ReadBackgroundPositionX", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadLength(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Length = __cordl_object
            .invoke("ReadLength", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTimeValue(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TimeValue> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TimeValue = __cordl_object
            .invoke("ReadTimeValue", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackgroundSize(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = __cordl_object
            .invoke("ReadBackgroundSize", (index))?;
        Ok(__cordl_ret)
    }
    pub fn LoadProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_propertyId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId = __cordl_object
            .invoke("get_propertyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_property(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::StyleProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_property", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetCurrentProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurrentProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadListTimeValue(
        &mut self,
        list: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::TimeValue,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadListTimeValue", (list, index))?;
        Ok(__cordl_ret)
    }
    pub fn get_dpiScaling(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_dpiScaling", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContext(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        selector: *mut crate::UnityEngine::UIElements::StyleComplexSelector,
        varContext: *mut crate::UnityEngine::UIElements::StyleVariableContext,
        dpiScaling: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContext", (sheet, selector, varContext, dpiScaling))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackgroundRepeat(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat = __cordl_object
            .invoke("ReadBackgroundRepeat", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackgroundPosition(
        &mut self,
        index: i32,
        keyword: crate::UnityEngine::UIElements::BackgroundPositionKeyword,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = __cordl_object
            .invoke("ReadBackgroundPosition", (index, keyword))?;
        Ok(__cordl_ret)
    }
    pub fn set_dpiScaling(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dpiScaling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReadScale(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = __cordl_object
            .invoke("ReadScale", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadListStylePropertyName(
        &mut self,
        list: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadListStylePropertyName", (list, index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsString(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadAsString", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadRotate(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = __cordl_object
            .invoke("ReadRotate", (index))?;
        Ok(__cordl_ret)
    }
    pub fn MoveNextProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId = __cordl_object
            .invoke("MoveNextProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadFontDefinition(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = __cordl_object
            .invoke("ReadFontDefinition", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCursor(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Cursor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Cursor = __cordl_object
            .invoke("ReadCursor", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackgroundPositionY(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = __cordl_object
            .invoke("ReadBackgroundPositionY", (index))?;
        Ok(__cordl_ret)
    }
    pub fn IsValueType(
        &mut self,
        index: i32,
        _cordl_type: crate::UnityEngine::UIElements::StyleValueType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValueType", (index, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTextShadow(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextShadow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = __cordl_object
            .invoke("ReadTextShadow", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadListEasingFunction(
        &mut self,
        list: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::EasingFunction,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadListEasingFunction", (list, index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBackground(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Background = __cordl_object
            .invoke("ReadBackground", (index))?;
        Ok(__cordl_ret)
    }
    pub fn SetInlineContext(
        &mut self,
        sheet: *mut crate::UnityEngine::UIElements::StyleSheet,
        properties: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::StyleProperty,
        >,
        propertyIds: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
        dpiScaling: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInlineContext", (sheet, properties, propertyIds, dpiScaling))?;
        Ok(__cordl_ret)
    }
    pub fn IsKeyword(
        &mut self,
        index: i32,
        keyword: crate::UnityEngine::UIElements::StyleValueKeyword,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyword", (index, keyword))?;
        Ok(__cordl_ret)
    }
    pub fn get_property(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::StyleProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::StyleProperty = __cordl_object
            .invoke("get_property", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+StylePropertyReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
