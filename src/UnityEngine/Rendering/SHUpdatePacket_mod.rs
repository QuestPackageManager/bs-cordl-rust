#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct SHUpdatePacket {
    pub shr0: f32,
    pub shr1: f32,
    pub shr2: f32,
    pub shr3: f32,
    pub shr4: f32,
    pub shr5: f32,
    pub shr6: f32,
    pub shr7: f32,
    pub shr8: f32,
    pub shg0: f32,
    pub shg1: f32,
    pub shg2: f32,
    pub shg3: f32,
    pub shg4: f32,
    pub shg5: f32,
    pub shg6: f32,
    pub shg7: f32,
    pub shg8: f32,
    pub shb0: f32,
    pub shb1: f32,
    pub shb2: f32,
    pub shb3: f32,
    pub shb4: f32,
    pub shb5: f32,
    pub shb6: f32,
    pub shb7: f32,
    pub shb8: f32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::SHUpdatePacket {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SHUpdatePacket";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::SHUpdatePacket {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::SHUpdatePacket {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::SHUpdatePacket {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::SHUpdatePacket {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+SHUpdatePacket")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Rendering::SHUpdatePacket {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SHUpdatePacket")]
impl crate::UnityEngine::Rendering::SHUpdatePacket {}
