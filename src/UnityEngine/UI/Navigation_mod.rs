#[cfg(feature = "UnityEngine+UI+Navigation")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Navigation {
    pub m_Mode: crate::UnityEngine::UI::Navigation_Mode,
    pub m_WrapAround: bool,
    pub m_SelectOnUp: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    pub m_SelectOnDown: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    pub m_SelectOnLeft: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    pub m_SelectOnRight: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
}
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::Navigation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "Navigation";
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
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UI::Navigation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::UI::Navigation {
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
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UI::Navigation {
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
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UI::Navigation {
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
#[cfg(feature = "UnityEngine+UI+Navigation")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::UI::Navigation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation")]
impl crate::UnityEngine::UI::Navigation {
    #[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
    pub type Mode = crate::UnityEngine::UI::Navigation_Mode;
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::UI::Navigation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultNavigation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::Navigation,
    > {
        let __cordl_ret: crate::UnityEngine::UI::Navigation = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultNavigation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::Navigation_Mode> {
        let __cordl_ret: crate::UnityEngine::UI::Navigation_Mode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectOnDown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectOnDown",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectOnLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectOnLeft",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectOnRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectOnRight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectOnUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_selectOnUp",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wrapAround(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wrapAround",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mode(
        &mut self,
        value: crate::UnityEngine::UI::Navigation_Mode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectOnDown(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectOnDown",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectOnLeft(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectOnLeft",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectOnRight(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectOnRight",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectOnUp(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Selectable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_selectOnUp",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wrapAround(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wrapAround",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UI::Navigation>>
for crate::UnityEngine::UI::Navigation {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UI::Navigation> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UI::Navigation>>
for crate::UnityEngine::UI::Navigation {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UI::Navigation> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Navigation_Mode {
    #[default]
    Automatic = 3i32,
    Explicit = 4i32,
    Horizontal = 1i32,
    None = 0i32,
    Vertical = 2i32,
}
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::Navigation_Mode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "Mode";
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
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UI::Navigation_Mode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UI::Navigation_Mode {
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
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UI::Navigation_Mode {
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
#[cfg(feature = "UnityEngine+UI+Navigation+Mode")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UI::Navigation_Mode {
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
