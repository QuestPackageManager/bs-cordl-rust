#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PackedMatrix {
    pub packed0: crate::Unity::Mathematics::float4,
    pub packed1: crate::Unity::Mathematics::float4,
    pub packed2: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::PackedMatrix {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "PackedMatrix";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::PackedMatrix {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::PackedMatrix {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::PackedMatrix {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::PackedMatrix {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PackedMatrix")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Rendering::PackedMatrix {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+PackedMatrix")]
impl crate::UnityEngine::Rendering::PackedMatrix {
    pub fn FromFloat4x4(
        m: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::PackedMatrix> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Mathematics::float4x4,
                        >),
                        crate::UnityEngine::Rendering::PackedMatrix,
                        1usize,
                    >("FromFloat4x4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromFloat4x4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::PackedMatrix =
            unsafe { cordl_method_info.invoke_unchecked((), (m))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromMatrix4x4(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::PackedMatrix> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                        crate::UnityEngine::Rendering::PackedMatrix,
                        1usize,
                    >("FromMatrix4x4")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromMatrix4x4", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::PackedMatrix =
            unsafe { cordl_method_info.invoke_unchecked((), (m))? };
        Ok(__cordl_ret.into())
    }
}
