#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancellationCallbackCoreWorkArguments {
    pub _currArrayFragment: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SparselyPopulatedArrayFragment_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::CancellationCallbackInfo>,
        >,
    >,
    pub _currArrayIndex: i32,
}
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationCallbackCoreWorkArguments";
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
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
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
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
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
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
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
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
impl crate::System::Threading::CancellationCallbackCoreWorkArguments {
    pub fn _ctor(
        &mut self,
        currArrayFragment: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::CancellationCallbackInfo,
                >,
            >,
        >,
        currArrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::SparselyPopulatedArrayFragment_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::CancellationCallbackInfo,
                                    >,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (currArrayFragment, currArrayIndex))?
        };
        Ok(__cordl_ret.into())
    }
}
