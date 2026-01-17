#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct XRView {
    pub projMatrix: crate::UnityEngine::Matrix4x4,
    pub viewMatrix: crate::UnityEngine::Matrix4x4,
    pub prevViewMatrix: crate::UnityEngine::Matrix4x4,
    pub viewport: crate::UnityEngine::Rect,
    pub occlusionMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub textureArraySlice: i32,
    pub eyeCenterUV: crate::UnityEngine::Vector2,
    pub isPrevViewMatrixValid: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Experimental::Rendering::XRView {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "XRView";
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Experimental::Rendering::XRView
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Experimental::Rendering::XRView
{
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Experimental::Rendering::XRView
{
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Experimental::Rendering::XRView {
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+XRView")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Experimental::Rendering::XRView
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+XRView")]
impl crate::UnityEngine::Experimental::Rendering::XRView {
    pub fn ComputeEyeCenterUV(
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Matrix4x4),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("ComputeEyeCenterUV")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeEyeCenterUV", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked((), (proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        projMatrix: crate::UnityEngine::Matrix4x4,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        prevViewMatrix: crate::UnityEngine::Matrix4x4,
        isPrevViewMatrixValid: bool,
        viewport: crate::UnityEngine::Rect,
        occlusionMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        textureArraySlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        bool,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    projMatrix,
                    viewMatrix,
                    prevViewMatrix,
                    isPrevViewMatrixValid,
                    viewport,
                    occlusionMesh,
                    textureArraySlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
