#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StylePropertyNameCollection {
    pub propertiesList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StylePropertyNameCollection";
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl crate::UnityEngine::UIElements::StylePropertyNameCollection {
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
    pub type Enumerator = crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_UIElements_StylePropertyName__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<UnityEngine.UIElements.StylePropertyName>.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StylePropertyName,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (list),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
> for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
> for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StylePropertyNameCollection_Enumerator {
    pub m_Enumerator: crate::System::Collections::Generic::List_1_Enumerator<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "Enumerator";
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
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
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    > {
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
        enumerator: crate::System::Collections::Generic::List_1_Enumerator<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (enumerator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyName = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
> for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
> for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
