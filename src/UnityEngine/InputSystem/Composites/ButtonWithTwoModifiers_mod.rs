#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct ButtonWithTwoModifiers {
    __cordl_parent: crate::UnityEngine::InputSystem::InputBindingComposite_1<f32>,
    pub modifier1: i32,
    pub modifier2: i32,
    pub button: i32,
    pub overrideModifiersNeedToBePressedFirst: bool,
    pub modifiersOrder:
        crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Composites";
    const CLASS_NAME: &'static str = "ButtonWithTwoModifiers";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers {
    type Target = crate::UnityEngine::InputSystem::InputBindingComposite_1<f32>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
impl crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers {
    #[cfg(feature = "UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder")]
    pub type ModifiersOrder =
        crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder;
    pub fn EvaluateMagnitude(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputBindingCompositeContext,
                    >), f32, 1usize>("EvaluateMagnitude")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EvaluateMagnitude",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinishSetup(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputBindingCompositeContext,
                    >), quest_hook::libil2cpp::Void, 1usize>("FinishSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinishSetup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn ModifiersArePressed(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputBindingCompositeContext,
                    >), bool, 1usize>("ModifiersArePressed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ModifiersArePressed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadValue(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputBindingCompositeContext,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::InputSystem::InputBindingCompositeContext,
                    >), f32, 1usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ButtonWithTwoModifiers_ModifiersOrder {
    #[default]
    Default = 0i32,
    Ordered = 1i32,
    Unordered = 2i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Composites";
    const CLASS_NAME: &'static str = "ButtonWithTwoModifiers/ModifiersOrder";
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
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder
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
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder
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
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+Composites+ButtonWithTwoModifiers+ModifiersOrder"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::Composites::ButtonWithTwoModifiers_ModifiersOrder
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
