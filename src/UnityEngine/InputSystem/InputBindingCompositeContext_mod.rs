#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputBindingCompositeContext {
    pub m_State: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionState>,
    pub m_BindingIndex: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputBindingCompositeContext";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext")]
impl crate::UnityEngine::InputSystem::InputBindingCompositeContext {
    #[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1")]
    pub type DefaultComparer_1<TValue: quest_hook::libil2cpp::Type> =
        crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>;
    #[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
    pub type PartBinding =
        crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding;
    pub fn EvaluateMagnitude(&mut self, partNumber: i32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), f32, 1usize>("EvaluateMagnitude")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EvaluateMagnitude",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPressTime(&mut self, partNumber: i32) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), f64, 1usize>("GetPressTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPressTime",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked(self, (partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsButton(&mut self, partNumber: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("ReadValueAsButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValueAsButton",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueAsObject(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("ReadValueAsObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadValueAsObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_ByRefMut1<TValue>(
        &mut self,
        partNumber: i32,
        sourceControl: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        >,
                    ), TValue, 2usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TValue =
            unsafe { cordl_method_info.invoke_unchecked(self, (partNumber, sourceControl))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_ByRefMut_TComparer3<TValue, TComparer>(
        &mut self,
        partNumber: i32,
        sourceControl: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputControl,
                            >,
                        >,
                        TComparer,
                    ), TValue, 3usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: TValue = unsafe {
            cordl_method_info.invoke_unchecked(self, (partNumber, sourceControl, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_Il2CppObject_i32_4(
        &mut self,
        partNumber: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (partNumber, buffer, bufferSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_TComparer2<TValue, TComparer>(
        &mut self,
        partNumber: i32,
        comparer: TComparer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TComparer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, TComparer), TValue, 2usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: TValue =
            unsafe { cordl_method_info.invoke_unchecked(self, (partNumber, comparer))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadValue_i32_0<TValue>(
        &mut self,
        partNumber: i32,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), TValue, 1usize>("ReadValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadValue",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TValue =
            unsafe { cordl_method_info.invoke_unchecked(self, (partNumber))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_controls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding,
                            >,
                        >,
                        0usize,
                    >("get_controls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_controls", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputBindingCompositeContext_DefaultComparer_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputBindingCompositeContext/DefaultComparer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.InputSystem",
                "InputBindingCompositeContext/DefaultComparer`1",
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
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
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
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
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
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
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
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
#[cfg(
    feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1"
)]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
{
    pub fn Compare(&mut self, x: TValue, y: TValue) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TValue, TValue), i32, 2usize>("Compare")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Compare",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    AsRef<crate::System::Collections::Generic::IComparer_1<TValue>>
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
{
    fn as_ref(&self) -> &crate::System::Collections::Generic::IComparer_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+DefaultComparer_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    AsMut<crate::System::Collections::Generic::IComparer_1<TValue>>
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_DefaultComparer_1<TValue>
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IComparer_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InputBindingCompositeContext_PartBinding {
    pub _part_k__BackingField: i32,
    pub _control_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputBindingCompositeContext/PartBinding";
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
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
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputBindingCompositeContext+PartBinding")]
impl crate::UnityEngine::InputSystem::InputBindingCompositeContext_PartBinding {
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >,
                        0usize,
                    >("get_control")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_control", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_part(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_part")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_part",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_control(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_control")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_control", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_part(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_part")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_part",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
