#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DuplicatePropertyNameHandling {
    #[default]
    Error = 2i32,
    Ignore = 1i32,
    Replace = 0i32,
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Linq";
    const CLASS_NAME: &'static str = "DuplicatePropertyNameHandling";
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling {
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling {
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
#[cfg(feature = "cordl_class_Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling {
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
