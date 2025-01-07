#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StyleFontDefinition {
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
    pub m_Value: crate::UnityEngine::UIElements::FontDefinition,
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleFontDefinition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StyleFontDefinition";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::StyleFontDefinition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StyleFontDefinition {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::StyleFontDefinition {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::StyleFontDefinition {
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
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleFontDefinition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
impl crate::UnityEngine::UIElements::StyleFontDefinition {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_StyleFontDefinition0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_FontDefinition0(
        &mut self,
        f: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StyleKeyword1(
        &mut self,
        f: crate::UnityEngine::UIElements::FontDefinition,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f, keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleKeyword> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleKeyword = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_keyword",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        f: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleFontDefinition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFontDefinition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (f))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFontDefinition>,
> for crate::UnityEngine::UIElements::StyleFontDefinition {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StyleFontDefinition,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleFontDefinition>,
> for crate::UnityEngine::UIElements::StyleFontDefinition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::StyleFontDefinition,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::FontDefinition,
    >,
> for crate::UnityEngine::UIElements::StyleFontDefinition {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::FontDefinition,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleFontDefinition")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::FontDefinition,
    >,
> for crate::UnityEngine::UIElements::StyleFontDefinition {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::FontDefinition,
    > {
        todo!()
    }
}
