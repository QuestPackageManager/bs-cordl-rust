#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TilingJob {
    pub lights: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::VisibleLight,
    >,
    pub reflectionProbes: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::VisibleReflectionProbe,
    >,
    pub tileRanges: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::Universal::InclusiveRange,
    >,
    pub itemsPerTile: i32,
    pub rangesPerItem: i32,
    pub worldToViews: crate::UnityEngine::Rendering::Universal::Fixed2_1<
        crate::Unity::Mathematics::float4x4,
    >,
    pub tileScale: crate::Unity::Mathematics::float2,
    pub tileScaleInv: crate::Unity::Mathematics::float2,
    pub viewPlaneBottoms: crate::UnityEngine::Rendering::Universal::Fixed2_1<f32>,
    pub viewPlaneTops: crate::UnityEngine::Rendering::Universal::Fixed2_1<f32>,
    pub viewToViewportScaleBiases: crate::UnityEngine::Rendering::Universal::Fixed2_1<
        crate::Unity::Mathematics::float4,
    >,
    pub tileCount: crate::Unity::Mathematics::int2,
    pub near: f32,
    pub isOrthographic: bool,
    pub m_TileYRange: crate::UnityEngine::Rendering::Universal::InclusiveRange,
    pub m_Offset: i32,
    pub m_ViewIndex: i32,
    pub m_CenterOffset: crate::Unity::Mathematics::float2,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::TilingJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "TilingJob";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::TilingJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::TilingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::TilingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::TilingJob {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+TilingJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::Universal::TilingJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+TilingJob")]
impl crate::UnityEngine::Rendering::Universal::TilingJob {
    pub fn EvaluateNearConic(
        near: f32,
        o: crate::Unity::Mathematics::float3,
        d: crate::Unity::Mathematics::float3,
        r: f32,
        u: crate::Unity::Mathematics::float3,
        v: crate::Unity::Mathematics::float3,
        theta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                        ),
                        crate::Unity::Mathematics::float3,
                        7usize,
                    >("EvaluateNearConic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EvaluateNearConic", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float3 = unsafe {
            cordl_method_info.invoke_unchecked((), (near, o, d, r, u, v, theta))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        jobIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (jobIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandOrthographic(
        &mut self,
        positionVS: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExpandOrthographic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpandOrthographic", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (positionVS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandRangeOrthographic(
        &mut self,
        range: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::InclusiveRange,
        >,
        xVS: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::InclusiveRange,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ExpandRangeOrthographic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpandRangeOrthographic", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (range, xVS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandY(
        &mut self,
        positionVS: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExpandY")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ExpandY",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (positionVS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearConicTangentTheta(
        o: crate::Unity::Mathematics::float2,
        d: crate::Unity::Mathematics::float2,
        r: f32,
        u: crate::Unity::Mathematics::float2,
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float2,
                            crate::Unity::Mathematics::float2,
                            f32,
                            crate::Unity::Mathematics::float2,
                            crate::Unity::Mathematics::float2,
                        ),
                        crate::Unity::Mathematics::float2,
                        5usize,
                    >("FindNearConicTangentTheta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindNearConicTangentTheta", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked((), (o, d, r, u, v))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearConicYTheta(
        near: f32,
        o: crate::Unity::Mathematics::float3,
        d: crate::Unity::Mathematics::float3,
        r: f32,
        u: crate::Unity::Mathematics::float3,
        v: crate::Unity::Mathematics::float3,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                        ),
                        crate::Unity::Mathematics::float2,
                        7usize,
                    >("FindNearConicYTheta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindNearConicYTheta", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked((), (near, o, d, r, u, v, y))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCircleClipPoints(
        circleCenter: crate::Unity::Mathematics::float3,
        circleNormal: crate::Unity::Mathematics::float3,
        circleRadius: f32,
        near: f32,
        p0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                        ),
                        bool,
                        6usize,
                    >("GetCircleClipPoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCircleClipPoints", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (circleCenter, circleNormal, circleRadius, near, p0, p1),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetConeSideTangentPoints(
        vertex: crate::Unity::Mathematics::float3,
        axis: crate::Unity::Mathematics::float3,
        cosHalfAngle: f32,
        circleRadius: f32,
        coneHeight: f32,
        range: f32,
        circleU: crate::Unity::Mathematics::float3,
        circleV: crate::Unity::Mathematics::float3,
        l1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        l2: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                            f32,
                            f32,
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("GetConeSideTangentPoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetConeSideTangentPoints", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        vertex,
                        axis,
                        cosHalfAngle,
                        circleRadius,
                        coneHeight,
                        range,
                        circleU,
                        circleV,
                        l1,
                        l2,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProjectedCircleHorizon(
        center: crate::Unity::Mathematics::float2,
        radius: f32,
        U: crate::Unity::Mathematics::float2,
        V: crate::Unity::Mathematics::float2,
        uv1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
        uv2: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float2,
                            f32,
                            crate::Unity::Mathematics::float2,
                            crate::Unity::Mathematics::float2,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float2,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float2,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("GetProjectedCircleHorizon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProjectedCircleHorizon", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (center, radius, U, V, uv1, uv2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereHorizon(
        center: crate::Unity::Mathematics::float2,
        radius: f32,
        near: f32,
        clipRadius: f32,
        p0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
        p1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float2,
                            f32,
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float2,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float2,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("GetSphereHorizon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSphereHorizon", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (center, radius, near, clipRadius, p0, p1))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereYPlaneHorizon(
        center: crate::Unity::Mathematics::float3,
        sphereRadius: f32,
        near: f32,
        clipRadius: f32,
        y: f32,
        left: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        right: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            f32,
                            f32,
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("GetSphereYPlaneHorizon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSphereYPlaneHorizon", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (center, sphereRadius, near, clipRadius, y, left, right),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectCircleYPlane(
        y: f32,
        circleCenter: crate::Unity::Mathematics::float3,
        circleNormal: crate::Unity::Mathematics::float3,
        circleU: crate::Unity::Mathematics::float3,
        circleV: crate::Unity::Mathematics::float3,
        circleRadius: f32,
        p1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        p2: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            f32,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            crate::Unity::Mathematics::float3,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Mathematics::float3,
                            >,
                        ),
                        bool,
                        8usize,
                    >("IntersectCircleYPlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntersectCircleYPlane", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        y,
                        circleCenter,
                        circleNormal,
                        circleU,
                        circleV,
                        circleRadius,
                        p1,
                        p2,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IntersectEllipseLine(
        a: f32,
        b: f32,
        line: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<f32, f32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (f32, f32, crate::Unity::Mathematics::float3),
                        crate::System::ValueTuple_2<f32, f32>,
                        3usize,
                    >("IntersectEllipseLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IntersectEllipseLine", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<f32, f32> = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TileLight(
        &mut self,
        lightIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TileLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TileLight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lightIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TileLightOrthographic(
        &mut self,
        lightIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TileLightOrthographic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TileLightOrthographic", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lightIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TileReflectionProbe(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TileReflectionProbe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TileReflectionProbe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewToTileSpace(
        &mut self,
        positionVS: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("ViewToTileSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ViewToTileSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (positionVS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ViewToTileSpaceOrthographic(
        &mut self,
        positionVS: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Mathematics::float3),
                        crate::Unity::Mathematics::float2,
                        1usize,
                    >("ViewToTileSpaceOrthographic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ViewToTileSpaceOrthographic", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Mathematics::float2 = unsafe {
            cordl_method_info.invoke_unchecked(self, (positionVS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _TileLightOrthographic_g__SpherePointIsValid_20_0(
        p: crate::Unity::Mathematics::float3,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("<TileLightOrthographic>g__SpherePointIsValid|20_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<TileLightOrthographic>g__SpherePointIsValid|20_0", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (p, _cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _TileLight_g__ConicPointIsValid_19_1(
        p: crate::Unity::Mathematics::float3,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("<TileLight>g__ConicPointIsValid|19_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<TileLight>g__ConicPointIsValid|19_1", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (p, _cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _TileLight_g__SpherePointIsValid_19_0(
        p: crate::Unity::Mathematics::float3,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Mathematics::float3,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("<TileLight>g__SpherePointIsValid|19_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<TileLight>g__SpherePointIsValid|19_0", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (p, _cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn square(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("square")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "square",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (x))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+TilingJob")]
impl AsRef<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::Universal::TilingJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+TilingJob")]
impl AsMut<crate::Unity::Jobs::IJobFor>
for crate::UnityEngine::Rendering::Universal::TilingJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobFor {
        todo!()
    }
}
