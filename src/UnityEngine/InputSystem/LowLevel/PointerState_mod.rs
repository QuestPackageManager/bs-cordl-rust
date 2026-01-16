#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PointerState {
    pub pointerId: u32,
    pub position: crate::UnityEngine::Vector2,
    pub delta: crate::UnityEngine::Vector2,
    pub pressure: f32,
    pub radius: crate::UnityEngine::Vector2,
    pub buttons: u16,
    pub displayIndex: u16,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "PointerState";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+LowLevel+PointerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl crate::UnityEngine::InputSystem::LowLevel::PointerState {
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::FourCC, 0usize>(
                        "get_format",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_format",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_kFormat(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::FourCC,
                        0usize,
                    >("get_kFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_kFormat", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+PointerState")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo>
    for crate::UnityEngine::InputSystem::LowLevel::PointerState
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateTypeInfo {
        todo!()
    }
}
