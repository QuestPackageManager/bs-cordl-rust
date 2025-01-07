#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Offset {
    pub m_Left: f32,
    pub m_Right: f32,
    pub m_Top: f32,
    pub m_Bottom: f32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::Text::Offset {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "Offset";
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
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::Offset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::Offset {
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
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::Offset {
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
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::Offset {
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
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::Offset {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
impl crate::UnityEngine::TextCore::Text::Offset {
    pub fn Equals(
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (left, right, top, bottom),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_left(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_left",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_right",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_top(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_top",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zero() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::Text::Offset,
    > {
        let __cordl_ret: crate::UnityEngine::TextCore::Text::Offset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::TextCore::Text::Offset,
        rhs: crate::UnityEngine::TextCore::Text::Offset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        a: crate::UnityEngine::TextCore::Text::Offset,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::Text::Offset> {
        let __cordl_ret: crate::UnityEngine::TextCore::Text::Offset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
