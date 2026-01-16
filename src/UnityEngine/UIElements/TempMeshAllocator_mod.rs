#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TempMeshAllocator {
    pub m_Handle: crate::System::Runtime::InteropServices::GCHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TempMeshAllocator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TempMeshAllocator";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::TempMeshAllocator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::TempMeshAllocator {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::TempMeshAllocator {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::TempMeshAllocator {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+TempMeshAllocator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TempMeshAllocator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TempMeshAllocator")]
impl crate::UnityEngine::UIElements::TempMeshAllocator {
    pub fn AllocateTempMesh(
        &mut self,
        vertexCount: i32,
        indexCount: i32,
        vertices: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        indices: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<u16>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeSlice_1<
                                    crate::UnityEngine::UIElements::Vertex,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeSlice_1<u16>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("AllocateTempMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateTempMesh", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (vertexCount, indexCount, vertices, indices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        handle: crate::System::Runtime::InteropServices::GCHandle,
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TempMeshAllocator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::Runtime::InteropServices::GCHandle,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::UIElements::TempMeshAllocator,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (handle, allocator))?
        };
        Ok(__cordl_ret.into())
    }
}
