#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueTuple_1<T1: quest_hook::libil2cpp::Type> {
    pub Item1: T1,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Mono::ValueTuple_1<T1> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "ValueTuple`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("Mono", "ValueTuple`1")
                    .unwrap()
                    .make_generic::<(T1)>()
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
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::Mono::ValueTuple_1<T1> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::Mono::ValueTuple_1<T1> {
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
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::Mono::ValueTuple_1<T1> {
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
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::Mono::ValueTuple_1<T1> {
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
#[cfg(feature = "cordl_class_Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Mono::ValueTuple_1<T1> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> crate::Mono::ValueTuple_1<T1> {}
