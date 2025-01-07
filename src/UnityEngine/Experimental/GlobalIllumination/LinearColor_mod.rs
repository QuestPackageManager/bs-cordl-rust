#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LinearColor {
    pub m_red: f32,
    pub m_green: f32,
    pub m_blue: f32,
    pub m_intensity: f32,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";
    const CLASS_NAME: &'static str = "LinearColor";
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
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
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LinearColor")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LinearColor {
    pub fn Black() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Black", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Convert(
        color: crate::UnityEngine::Color,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::GlobalIllumination::LinearColor,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::GlobalIllumination::LinearColor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (color, intensity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_blue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_blue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_green(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_green",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_red(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_red",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_blue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_blue",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_green(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_green",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_red(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_red",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
