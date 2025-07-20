#[cfg(feature = "ISonyPSPremiumHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ISonyPSPremiumHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISonyPSPremiumHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyPSPremiumHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyPSPremiumHelper";
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
#[cfg(feature = "ISonyPSPremiumHelper")]
impl std::ops::Deref for crate::GlobalNamespace::ISonyPSPremiumHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::ISonyPSPremiumHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl crate::GlobalNamespace::ISonyPSPremiumHelper {
    #[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
    pub type DisplayJoinPremiumDialogResult = crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult;
    #[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
    pub type GetPremiumStatusResult = crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult;
    pub fn DisplayJoinPremiumDialogAsync(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult,
                            >,
                        >,
                        1usize,
                    >("DisplayJoinPremiumDialogAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DisplayJoinPremiumDialogAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (token))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPremiumStatusAsync(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult,
                            >,
                        >,
                        1usize,
                    >("GetPremiumStatusAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetPremiumStatusAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (token))? };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyPremiumFeature(
        &mut self,
        isSpectator: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("NotifyPremiumFeature")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "NotifyPremiumFeature", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isSpectator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISonyPSPremiumHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ISonyPSPremiumHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyPSPremiumHelper/DisplayJoinPremiumDialogResult";
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
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
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
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
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
#[cfg(feature = "ISonyPSPremiumHelper+DisplayJoinPremiumDialogResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyPSPremiumHelper_DisplayJoinPremiumDialogResult {
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
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ISonyPSPremiumHelper_GetPremiumStatusResult {
    #[default]
    Authorized = 0i32,
    Failed = 2i32,
    Unauthorized = 1i32,
}
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISonyPSPremiumHelper/GetPremiumStatusResult";
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
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult {
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
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult {
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
#[cfg(feature = "ISonyPSPremiumHelper+GetPremiumStatusResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ISonyPSPremiumHelper_GetPremiumStatusResult {
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
