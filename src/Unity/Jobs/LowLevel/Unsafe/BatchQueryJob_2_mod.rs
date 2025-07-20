#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchQueryJob_2<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> {
    pub commands: crate::Unity::Collections::NativeArray_1<CommandT>,
    pub results: crate::Unity::Collections::NativeArray_1<ResultT>,
    __cordl_phantom_CommandT: std::marker::PhantomData<CommandT>,
    __cordl_phantom_ResultT: std::marker::PhantomData<ResultT>,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Jobs.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "BatchQueryJob`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Jobs.LowLevel.Unsafe",
                        "BatchQueryJob`2",
                    )
                    .unwrap()
                    .make_generic::<(CommandT, ResultT)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
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
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
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
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
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
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    pub fn _ctor(
        &mut self,
        commands: crate::Unity::Collections::NativeArray_1<CommandT>,
        results: crate::Unity::Collections::NativeArray_1<ResultT>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        CommandT: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        ResultT: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<CommandT>,
                            crate::Unity::Collections::NativeArray_1<ResultT>,
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
            method.invoke_unchecked(self, (commands, results))?
        };
        Ok(__cordl_ret.into())
    }
}
