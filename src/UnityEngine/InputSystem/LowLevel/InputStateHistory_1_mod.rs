#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct InputStateHistory_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputStateHistory`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.LowLevel",
                        "InputStateHistory`1",
                    )
                    .unwrap()
                    .make_generic::<(TValue)>()
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    type Target = crate::UnityEngine::InputSystem::LowLevel::InputStateHistory;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<
        TValue,
    >;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
    pub type Record = crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
        TValue,
    >;
    pub fn AddRecord(
        &mut self,
        record: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = __cordl_object.invoke("AddRecord", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
                crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
                    TValue,
                >,
            >,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
                    TValue,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString2(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object.into())
    }
    pub fn New_InputControl_1_1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (control))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Nullable_1_0(
        maxStateSizeInBytes: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxStateSizeInBytes))?;
        Ok(__cordl_object.into())
    }
    pub fn RecordStateChange(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
        value: TValue,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = __cordl_object.invoke("RecordStateChange", (control, value, _cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString2(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputControl_1_1(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (control))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Nullable_1_0(
        &mut self,
        maxStateSizeInBytes: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxStateSizeInBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (index, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputStateHistory_1_Enumerator<TValue: quest_hook::libil2cpp::Type> {
    pub m_History: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
    >,
    pub m_Index: i32,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "Enumerator";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.LowLevel",
                        "Enumerator",
                    )
                    .unwrap()
                    .make_generic::<(TValue)>()
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        history: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (history),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Enumerator<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputStateHistory_1_Record<TValue: quest_hook::libil2cpp::Type> {
    pub m_Owner: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
    >,
    pub m_IndexPlusOne: i32,
    pub m_Version: u32,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "Record";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.LowLevel",
                        "Record",
                    )
                    .unwrap()
                    .make_generic::<(TValue)>()
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
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
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    pub fn CheckValid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        record: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (record),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputStateHistory_1_Record0(
        &mut self,
        other: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeExtraMemoryPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUnsafeExtraMemoryPtr",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeExtraMemoryPtrUnchecked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUnsafeExtraMemoryPtrUnchecked",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeMemoryPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetUnsafeMemoryPtr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafeMemoryPtrUnchecked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetUnsafeMemoryPtrUnchecked",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReadValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject0(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
        >,
        index: i32,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (owner, index, header),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputStateHistory_1_i32_1(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (owner, index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl_1<TValue>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_control", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_header", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_index",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_owner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<TValue>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_owner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previous(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<
            TValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_previous", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_recordIndex(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_recordIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f64>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateHistory_1+Record")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1_Record<TValue>,
    > {
        todo!()
    }
}
