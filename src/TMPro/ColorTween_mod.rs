#[cfg(feature = "TMPro+ColorTween")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorTween {
    pub m_Target: quest_hook::libil2cpp::Gc<crate::TMPro::ColorTween_ColorTweenCallback>,
    pub m_StartColor: crate::UnityEngine::Color,
    pub m_TargetColor: crate::UnityEngine::Color,
    pub m_TweenMode: crate::TMPro::ColorTween_ColorTweenMode,
    pub m_Duration: f32,
    pub m_IgnoreTimeScale: bool,
}
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::ColorTween {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "ColorTween";
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
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::ColorTween {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::ColorTween {
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
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::ColorTween {
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
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::ColorTween {
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
#[cfg(feature = "TMPro+ColorTween")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::ColorTween {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl crate::TMPro::ColorTween {
    #[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
    pub type ColorTweenCallback = crate::TMPro::ColorTween_ColorTweenCallback;
    #[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
    pub type ColorTweenMode = crate::TMPro::ColorTween_ColorTweenMode;
    pub fn AddOnChangedCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Events::UnityAction_1<crate::UnityEngine::Color>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddOnChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "AddOnChangedCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("GetDuration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "GetDuration", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIgnoreTimescale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("GetIgnoreTimescale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "GetIgnoreTimescale", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TweenValue(
        &mut self,
        floatPercentage: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("TweenValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "TweenValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (floatPercentage))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidTarget(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ValidTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "ValidTarget", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_duration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "get_duration", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreTimeScale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_ignoreTimeScale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "get_ignoreTimeScale", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_startColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Color, 0usize>("get_startColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "get_startColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_targetColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Color, 0usize>("get_targetColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "get_targetColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tweenMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::ColorTween_ColorTweenMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::TMPro::ColorTween_ColorTweenMode,
                0usize,
            >("get_tweenMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "get_tweenMode", 0usize
                )
            });
        let __cordl_ret: crate::TMPro::ColorTween_ColorTweenMode = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_duration(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_duration")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "set_duration", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ignoreTimeScale(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ignoreTimeScale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "set_ignoreTimeScale", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_startColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_startColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "set_startColor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_targetColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_targetColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "set_targetColor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_tweenMode(
        &mut self,
        value: crate::TMPro::ColorTween_ColorTweenMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::TMPro::ColorTween_ColorTweenMode),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_tweenMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween as quest_hook::libil2cpp::Type > ::class(),
                    "set_tweenMode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl AsRef<crate::TMPro::ITweenValue> for crate::TMPro::ColorTween {
    fn as_ref(&self) -> &crate::TMPro::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "TMPro+ColorTween")]
impl AsMut<crate::TMPro::ITweenValue> for crate::TMPro::ColorTween {
    fn as_mut(&mut self) -> &mut crate::TMPro::ITweenValue {
        todo!()
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorTween_ColorTweenCallback {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<crate::UnityEngine::Color>,
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::ColorTween_ColorTweenCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "ColorTween/ColorTweenCallback";
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
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl std::ops::Deref for crate::TMPro::ColorTween_ColorTweenCallback {
    type Target = crate::UnityEngine::Events::UnityEvent_1<crate::UnityEngine::Color>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl std::ops::DerefMut for crate::TMPro::ColorTween_ColorTweenCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl crate::TMPro::ColorTween_ColorTweenCallback {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::TMPro::ColorTween_ColorTweenCallback as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::TMPro::ColorTween_ColorTweenCallback as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::ColorTween_ColorTweenCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorTween_ColorTweenMode {
    #[default]
    All = 0i32,
    Alpha = 2i32,
    RGB = 1i32,
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::ColorTween_ColorTweenMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "ColorTween/ColorTweenMode";
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
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::ColorTween_ColorTweenMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::TMPro::ColorTween_ColorTweenMode {
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
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::ColorTween_ColorTweenMode {
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
#[cfg(feature = "TMPro+ColorTween+ColorTweenMode")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::ColorTween_ColorTweenMode {
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
