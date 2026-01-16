#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshInfo {
    pub vertexCount: i32,
    pub vertexData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::Text::TextCoreVertex>,
    >,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub vertices:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    pub normals:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    pub tangents:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    pub vertexBufferSize: i32,
    pub uvs0:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    pub uvs2:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>>,
    pub colors32:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>>,
    pub triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub vertexDataLayout: crate::UnityEngine::TextCore::Text::VertexDataLayout,
    pub applySDF: bool,
    pub glyphRenderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::Text::MeshInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "MeshInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::TextCore::Text::MeshInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::TextCore::Text::MeshInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::TextCore::Text::MeshInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::TextCore::Text::MeshInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+MeshInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::TextCore::Text::MeshInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+MeshInfo")]
impl crate::UnityEngine::TextCore::Text::MeshInfo {
    pub fn Clear(
        &mut self,
        uploadChanges: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (uploadChanges))? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearUnusedVertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearUnusedVertices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearUnusedVertices",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ResizeMeshInfo(
        &mut self,
        _cordl_size: i32,
        isIMGUI: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, bool), quest_hook::libil2cpp::Void, 2usize>(
                        "ResizeMeshInfo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResizeMeshInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_size, isIMGUI))? };
        Ok(__cordl_ret.into())
    }
    pub fn SortGeometry(
        &mut self,
        order: crate::UnityEngine::TextCore::Text::VertexSortingOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::TextCore::Text::VertexSortingOrder),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SortGeometry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SortGeometry", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (order))? };
        Ok(__cordl_ret.into())
    }
    pub fn SwapVertexData(
        &mut self,
        src: i32,
        dst: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "SwapVertexData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SwapVertexData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
        layout: crate::UnityEngine::TextCore::Text::VertexDataLayout,
        isIMGUI: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::TextCore::Text::VertexDataLayout,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_size, layout, isIMGUI))? };
        Ok(__cordl_ret.into())
    }
}
