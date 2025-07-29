#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonoEventInfo {
    pub declaring_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub reflected_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub add_method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub remove_method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub raise_method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub attrs: crate::System::Reflection::EventAttributes,
    pub other_methods: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        >,
    >,
}
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Reflection::MonoEventInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "MonoEventInfo";
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
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Reflection::MonoEventInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Reflection::MonoEventInfo {
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
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Reflection::MonoEventInfo {
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
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Reflection::MonoEventInfo {
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
#[cfg(feature = "cordl_class_System+Reflection+MonoEventInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Reflection::MonoEventInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Reflection+MonoEventInfo")]
impl crate::System::Reflection::MonoEventInfo {}
