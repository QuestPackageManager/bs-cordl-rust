#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
#[repr(C)]
#[derive(Debug)]
pub struct YogaNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ygNode: crate::System::IntPtr,
    pub _config: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaConfig>,
    pub _parent: quest_hook::libil2cpp::Gc<crate::System::WeakReference>,
    pub _children: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        >,
    >,
    pub _measureFunction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Yoga::MeasureFunction,
    >,
    pub _baselineFunction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Yoga::BaselineFunction,
    >,
    pub _data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaNode =>
    "UnityEngine.Yoga"."YogaNode"
);
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl std::ops::Deref for crate::UnityEngine::Yoga::YogaNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::YogaNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl crate::UnityEngine::Yoga::YogaNode {
    pub fn BaselineInternal(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BaselineInternal", (node, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLayout(
        &mut self,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayout", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyStyle(
        &mut self,
        srcNode: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyStyle", (srcNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkLayoutSeen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkLayoutSeen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MeasureInternal(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        widthMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MeasureInternal", (node, width, widthMode, height, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        config: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (config))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeasureFunction(
        &mut self,
        measureFunction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Yoga::MeasureFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeasureFunction", (measureFunction))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleMargin(
        &mut self,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStyleMargin", (edge, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStylePadding(
        &mut self,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStylePadding", (edge, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStylePosition(
        &mut self,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStylePosition", (edge, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        config: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (config))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasNewLayout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasNewLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBaselineDefined(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsBaselineDefined", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMeasureDefined(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMeasureDefined", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutBorderBottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutBorderBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutBorderLeft(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutBorderLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutBorderRight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutBorderRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutBorderTop(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutBorderTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutBottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutMarginBottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutMarginBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutMarginLeft(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutMarginLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutMarginRight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutMarginRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutMarginTop(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutMarginTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutPaddingBottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutPaddingBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutPaddingLeft(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutPaddingLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutPaddingRight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutPaddingRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutPaddingTop(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutPaddingTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutRight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LayoutY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_LayoutY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AlignContent(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AlignContent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AlignItems(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AlignItems", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AlignSelf(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AlignSelf", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BorderBottomWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BorderBottomWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BorderLeftWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BorderLeftWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BorderRightWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BorderRightWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BorderTopWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BorderTopWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Bottom(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Bottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Config(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Config", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Display(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaDisplay,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Display", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Flex(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Flex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FlexBasis(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FlexBasis", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FlexDirection(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaFlexDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FlexDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FlexGrow(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FlexGrow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FlexShrink(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FlexShrink", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Height(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Height", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_JustifyContent(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaJustify,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_JustifyContent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Left(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Left", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MarginBottom(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MarginBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MarginLeft(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MarginLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MarginRight(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MarginRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MarginTop(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MarginTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxHeight(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxWidth(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MinHeight(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MinWidth(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Overflow(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaOverflow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Overflow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PaddingBottom(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PaddingBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PaddingLeft(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PaddingLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PaddingRight(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PaddingRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PaddingTop(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PaddingTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PositionType(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaPositionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PositionType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Right(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Right", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Top(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Top", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Width(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Width", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Wrap(
        &mut self,
        value: crate::UnityEngine::Yoga::YogaWrap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Wrap", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::YogaNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    >,
> for crate::UnityEngine::Yoga::YogaNode {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    >,
> for crate::UnityEngine::Yoga::YogaNode {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::Yoga::YogaNode {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaNode")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::Yoga::YogaNode {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
