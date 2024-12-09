#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseCompositeField_3<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<TValueType>,
    pub m_Fields: *mut crate::System::Collections::Generic::List_1<TField>,
    pub m_ShouldUpdateDisplay: bool,
    pub m_ForceUpdateDisplay: bool,
    pub m_PropertyIndex: i32,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TField: std::marker::PhantomData<TField>,
    __cordl_phantom_TFieldValue: std::marker::PhantomData<TFieldValue>,
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseCompositeField_3 <
    TValueType, TField, TFieldValue > => "UnityEngine.UIElements"."BaseCompositeField`3"
    < TValueType, TField, TFieldValue >
);
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::BaseCompositeField_3<
    TValueType,
    TField,
    TFieldValue,
> {
    type Target = crate::UnityEngine::UIElements::BaseField_1<TValueType>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::BaseCompositeField_3<
    TValueType,
    TField,
    TFieldValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::BaseCompositeField_3<TValueType, TField, TFieldValue> {
    #[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
    pub type FieldDescription = crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
        TValueType,
        TField,
        TFieldValue,
    >;
    #[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+__c__DisplayClass24_0")]
    pub type __c__DisplayClass24_0 = crate::UnityEngine::UIElements::BaseCompositeField_3___c__DisplayClass24_0<
        TValueType,
        TField,
        TFieldValue,
    >;
    pub fn DescribeFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                TValueType,
                TField,
                TFieldValue,
            >,
        >,
    >
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                TValueType,
                TField,
                TFieldValue,
            >,
        > = __cordl_object.invoke("DescribeFields", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSpacer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("GetSpacer", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        label: *mut crate::System::String,
        fieldsByLine: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, fieldsByLine))?;
        Ok(__cordl_object)
    }
    pub fn OnViewDataReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: TValueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDisplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDisplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMixedValueContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMixedValueContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        label: *mut crate::System::String,
        fieldsByLine: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, fieldsByLine))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseCompositeField_3<
    TValueType,
    TField,
    TFieldValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BaseCompositeField_3_FieldDescription<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> {
    pub name: *mut crate::System::String,
    pub ussName: *mut crate::System::String,
    pub read: *mut crate::System::Func_2<TValueType, TFieldValue>,
    pub write: *mut crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
        TValueType,
        TField,
        TFieldValue,
    >,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TField: std::marker::PhantomData<TField>,
    __cordl_phantom_TFieldValue: std::marker::PhantomData<TFieldValue>,
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription < TValueType,
    TField, TFieldValue > => "UnityEngine.UIElements"
    ."BaseCompositeField`3/FieldDescription<TValueType,TField,TFieldValue>" < TValueType,
    TField, TFieldValue >
);
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    #[cfg(
        feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
    )]
    pub type WriteDelegate = crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
        TValueType,
        TField,
        TFieldValue,
    >;
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        ussName: *mut crate::System::String,
        read: *mut crate::System::Func_2<TValueType, TFieldValue>,
        write: *mut crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
            TValueType,
            TField,
            TFieldValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, ussName, read, write),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TField: std::marker::PhantomData<TField>,
    __cordl_phantom_TFieldValue: std::marker::PhantomData<TFieldValue>,
}
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate <
    TValueType, TField, TFieldValue > => "UnityEngine.UIElements"
    ."BaseCompositeField`3/FieldDescription/WriteDelegate" < TValueType, TField,
    TFieldValue >
);
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType,
    TField,
    TFieldValue,
> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType,
    TField,
    TFieldValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType,
    TField,
    TFieldValue,
> {
    pub fn Invoke(
        &mut self,
        val: quest_hook::libil2cpp::ByRefMut<TValueType>,
        fieldValue: TFieldValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (val, fieldValue))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription+WriteDelegate"
)]
impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType,
    TField,
    TFieldValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
