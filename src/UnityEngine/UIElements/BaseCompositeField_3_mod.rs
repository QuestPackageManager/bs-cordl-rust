#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseCompositeField_3<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<TValueType>,
    pub m_Fields: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<TField>,
    >,
    pub m_ShouldUpdateDisplay: bool,
    pub m_ForceUpdateDisplay: bool,
    pub m_PropertyIndex: i32,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TField: std::marker::PhantomData<TField>,
    __cordl_phantom_TFieldValue: std::marker::PhantomData<TFieldValue>,
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseCompositeField_3<
    TValueType,
    TField,
    TFieldValue,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseCompositeField`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements",
                        "BaseCompositeField`3",
                    )
                    .unwrap()
                    .make_generic::<(TValueType, TField, TFieldValue)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn DescribeFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                    TValueType,
                    TField,
                    TFieldValue,
                >,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                    TValueType,
                    TField,
                    TFieldValue,
                >,
            >,
        > = __cordl_object.invoke("DescribeFields", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpacer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetSpacer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fieldsByLine: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, fieldsByLine))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BaseCompositeField_3_FieldDescription<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ussName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub read: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValueType, TFieldValue>>,
    pub write: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
            TValueType,
            TField,
            TFieldValue,
        >,
    >,
    __cordl_phantom_TValueType: std::marker::PhantomData<TValueType>,
    __cordl_phantom_TField: std::marker::PhantomData<TField>,
    __cordl_phantom_TFieldValue: std::marker::PhantomData<TFieldValue>,
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseCompositeField`3/FieldDescription";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements",
                        "BaseCompositeField`3/FieldDescription",
                    )
                    .unwrap()
                    .make_generic::<(TValueType, TField, TFieldValue)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseCompositeField_3+FieldDescription")]
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
    TValueType,
    TField,
    TFieldValue,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ussName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        read: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValueType, TFieldValue>>,
        write: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
                TValueType,
                TField,
                TFieldValue,
            >,
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
        Ok(__cordl_ret.into())
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
unsafe impl<
    TValueType: quest_hook::libil2cpp::Type,
    TField: quest_hook::libil2cpp::Type,
    TFieldValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::FieldDescription_BaseCompositeField_3_WriteDelegate<
    TValueType,
    TField,
    TFieldValue,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseCompositeField`3/FieldDescription/WriteDelegate";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.UIElements",
                        "BaseCompositeField`3/FieldDescription/WriteDelegate",
                    )
                    .unwrap()
                    .make_generic::<(TValueType, TField, TFieldValue)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValueType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TField: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TFieldValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
