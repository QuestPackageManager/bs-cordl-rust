#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StyleRotate {
    pub m_Value: crate::UnityEngine::UIElements::Rotate,
    pub m_Keyword: crate::UnityEngine::UIElements::StyleKeyword,
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::StyleRotate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "StyleRotate";
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
for crate::UnityEngine::UIElements::StyleRotate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StyleRotate {
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
for crate::UnityEngine::UIElements::StyleRotate {
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
for crate::UnityEngine::UIElements::StyleRotate {
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
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleRotate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
impl crate::UnityEngine::UIElements::StyleRotate {
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
    pub fn Equals_StyleRotate0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleRotate,
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Rotate_StyleKeyword1(
        &mut self,
        v: crate::UnityEngine::UIElements::Rotate,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, keyword),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_StyleKeyword0(
        &mut self,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyword),
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
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::StyleRotate,
        rhs: crate::UnityEngine::UIElements::StyleRotate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleRotate> {
        let __cordl_ret: crate::UnityEngine::UIElements::StyleRotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (keyword))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleRotate>>
for crate::UnityEngine::UIElements::StyleRotate {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleRotate> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleRotate>>
for crate::UnityEngine::UIElements::StyleRotate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::StyleRotate> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
impl AsRef<
    crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::UIElements::Rotate>,
> for crate::UnityEngine::UIElements::StyleRotate {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Rotate,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleRotate")]
impl AsMut<
    crate::UnityEngine::UIElements::IStyleValue_1<crate::UnityEngine::UIElements::Rotate>,
> for crate::UnityEngine::UIElements::StyleRotate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStyleValue_1<
        crate::UnityEngine::UIElements::Rotate,
    > {
        todo!()
    }
}
