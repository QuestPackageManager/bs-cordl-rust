#[cfg(feature = "UnityEngine+RectTransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct RectTransformUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::RectTransformUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "RectTransformUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::Deref for crate::UnityEngine::RectTransformUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::RectTransformUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl crate::UnityEngine::RectTransformUtility {
    pub fn FlipLayoutAxes(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        keepPositioning: bool,
        recursive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FlipLayoutAxes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FlipLayoutAxes", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (rect, keepPositioning, recursive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlipLayoutOnAxis(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        axis: i32,
        keepPositioning: bool,
        recursive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            i32,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("FlipLayoutOnAxis")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FlipLayoutOnAxis", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (rect, axis, keepPositioning, recursive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransposed(
        input: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("GetTransposed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransposed", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustPoint(
        point: crate::UnityEngine::Vector2,
        elementTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                        ),
                        crate::UnityEngine::Vector2,
                        3usize,
                    >("PixelAdjustPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PixelAdjustPoint", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (point, elementTransform, canvas))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustPoint_Injected(
        point: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        elementTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("PixelAdjustPoint_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PixelAdjustPoint_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (point, elementTransform, canvas, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustRect(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                        ),
                        crate::UnityEngine::Rect,
                        2usize,
                    >("PixelAdjustRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PixelAdjustRect", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked((), (rectTransform, canvas))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PixelAdjustRect_Injected(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PixelAdjustRect_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PixelAdjustRect_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (rectTransform, canvas, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInRectangle(
        screenPoint: crate::UnityEngine::Vector2,
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector4,
                        ),
                        bool,
                        4usize,
                    >("PointInRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PointInRectangle", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (screenPoint, rect, cam, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInRectangle_Injected(
        screenPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        ),
                        bool,
                        4usize,
                    >("PointInRectangle_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PointInRectangle_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (screenPoint, rect, cam, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RectangleContainsScreenPoint_RectTransform_Vector2_Camera0(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        bool,
                        3usize,
                    >("RectangleContainsScreenPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RectangleContainsScreenPoint", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (rect, screenPoint, cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RectangleContainsScreenPoint_Vector4_1(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        offset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector4,
                        ),
                        bool,
                        4usize,
                    >("RectangleContainsScreenPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RectangleContainsScreenPoint", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (rect, screenPoint, cam, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToLocalPointInRectangle(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        localPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        ),
                        bool,
                        4usize,
                    >("ScreenPointToLocalPointInRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScreenPointToLocalPointInRectangle", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (rect, screenPoint, cam, localPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToRay(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        screenPos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector2,
                        ),
                        crate::UnityEngine::Ray,
                        2usize,
                    >("ScreenPointToRay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScreenPointToRay", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Ray = unsafe {
            method.invoke_unchecked((), (cam, screenPos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScreenPointToWorldPointInRectangle(
        rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        screenPoint: crate::UnityEngine::Vector2,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        ),
                        bool,
                        4usize,
                    >("ScreenPointToWorldPointInRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScreenPointToWorldPointInRectangle", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (rect, screenPoint, cam, worldPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WorldToScreenPoint(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        worldPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector3,
                        ),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("WorldToScreenPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WorldToScreenPoint", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (cam, worldPoint))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RectTransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
