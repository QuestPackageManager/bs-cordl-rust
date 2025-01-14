#[cfg(feature = "UnityEngine+ProBuilder+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::Math {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "Math";
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
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl crate::UnityEngine::ProBuilder::Math {
    pub const handleEpsilon: f32 = 0.0001f32;
    pub const k_FltCompareEpsilon: f32 = 0.0001f32;
    pub const k_FltEpsilon: f32 = 0.000000000000000000000000000000000000000000001f32;
    pub const phi: f32 = 1.618034f32;
    pub fn Abs(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("Abs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Abs", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Approx(a: f32, b: f32, delta: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), bool, 3usize>("Approx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Approx", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b, delta)) };
        Ok(__cordl_ret.into())
    }
    pub fn Approx2(
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2, f32),
                bool,
                3usize,
            >("Approx2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Approx2", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b, delta)) };
        Ok(__cordl_ret.into())
    }
    pub fn Approx3(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3, f32),
                bool,
                3usize,
            >("Approx3")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Approx3", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b, delta)) };
        Ok(__cordl_ret.into())
    }
    pub fn Approx4(
        a: crate::UnityEngine::Vector4,
        b: crate::UnityEngine::Vector4,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector4, crate::UnityEngine::Vector4, f32),
                bool,
                3usize,
            >("Approx4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Approx4", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b, delta)) };
        Ok(__cordl_ret.into())
    }
    pub fn ApproxC(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Color, crate::UnityEngine::Color, f32),
                bool,
                3usize,
            >("ApproxC")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApproxC", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (a, b, delta)) };
        Ok(__cordl_ret.into())
    }
    pub fn Average_IList_1_IList_1_0(
        array: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Vector2,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector2,
                2usize,
            >("Average")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Average", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (array, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Average_IList_1_IList_1_1(
        array: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Vector3,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("Average")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Average", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (array, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Average_IList_1_IList_1_2(
        array: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector4>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::Vector4,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector4,
                2usize,
            >("Average")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Average", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (array, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clamp(
        value: i32,
        lowerBound: i32,
        upperBound: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i32, 3usize>("Clamp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clamp", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (value, lowerBound, upperBound))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Cross(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Cross")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Cross", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (a, b, res))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DistancePointLineSegment_Vector2_Vector2_Vector2_0(
        point: crate::UnityEngine::Vector2,
        lineStart: crate::UnityEngine::Vector2,
        lineEnd: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                ),
                f32,
                3usize,
            >("DistancePointLineSegment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DistancePointLineSegment", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (point, lineStart, lineEnd))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DistancePointLineSegment_Vector3_Vector3_Vector3_1(
        point: crate::UnityEngine::Vector3,
        lineStart: crate::UnityEngine::Vector3,
        lineEnd: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                f32,
                3usize,
            >("DistancePointLineSegment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DistancePointLineSegment", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (point, lineStart, lineEnd))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DivideBy_Vector2_Vector2_0(
        v: crate::UnityEngine::Vector2,
        o: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2),
                crate::UnityEngine::Vector2,
                2usize,
            >("DivideBy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DivideBy", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v, o))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DivideBy_Vector3_Vector3_1(
        v: crate::UnityEngine::Vector3,
        o: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                2usize,
            >("DivideBy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DivideBy", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (v, o))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector2_0(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2),
                crate::UnityEngine::Vector2,
                1usize,
            >("EnsureUnitVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureUnitVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector3_1(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("EnsureUnitVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureUnitVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector4_2(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector4),
                crate::UnityEngine::Vector4,
                1usize,
            >("EnsureUnitVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EnsureUnitVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FixNaN(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector4),
                crate::UnityEngine::Vector4,
                1usize,
            >("FixNaN")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FixNaN", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBounds(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Bounds,
                2usize,
            >("GetBounds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBounds", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Bounds = unsafe {
            method.invoke_unchecked((), (positions, indices))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineSegmentIntersect_ByRefMut0(
        p0: crate::UnityEngine::Vector2,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        p3: crate::UnityEngine::Vector2,
        intersect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                ),
                bool,
                5usize,
            >("GetLineSegmentIntersect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLineSegmentIntersect", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (p0, p1, p2, p3, intersect))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineSegmentIntersect_Vector2_Vector2_Vector2_Vector2_1(
        p0: crate::UnityEngine::Vector2,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        p3: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                ),
                bool,
                4usize,
            >("GetLineSegmentIntersect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLineSegmentIntersect", 4usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (p0, p1, p2, p3)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNearestPointRayRay_Ray_Ray0(
        a: crate::UnityEngine::Ray,
        b: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Ray, crate::UnityEngine::Ray),
                crate::UnityEngine::Vector3,
                2usize,
            >("GetNearestPointRayRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNearestPointRayRay", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNearestPointRayRay_Vector3_Vector3_Vector3_Vector3_1(
        ao: crate::UnityEngine::Vector3,
        ad: crate::UnityEngine::Vector3,
        bo: crate::UnityEngine::Vector3,
        bd: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                crate::UnityEngine::Vector3,
                4usize,
            >("GetNearestPointRayRay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNearestPointRayRay", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (ao, ad, bo, bd))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvertScaleVector(
        scaleVector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("InvertScaleVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InvertScaleVector", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (scaleVector))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCardinalAxis(
        v: crate::UnityEngine::Vector3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3, f32),
                bool,
                2usize,
            >("IsCardinalAxis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsCardinalAxis", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (v, epsilon)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector2_1(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2),
                bool,
                1usize,
            >("IsNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNumber", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector3_2(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                bool,
                1usize,
            >("IsNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNumber", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector4_3(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector4),
                bool,
                1usize,
            >("IsNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNumber", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_f32_0(value: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNumber", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn LargestValue_Vector2_1(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2),
                f32,
                1usize,
            >("LargestValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LargestValue", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn LargestValue_Vector3_0(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                f32,
                1usize,
            >("LargestValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LargestValue", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn LargestVector2_IList_1_1(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector2,
                2usize,
            >("LargestVector2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LargestVector2", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LargestVector2_Il2CppArray0(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                >),
                crate::UnityEngine::Vector2,
                1usize,
            >("LargestVector2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LargestVector2", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeNonZero(value: f32, min: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32), f32, 2usize>("MakeNonZero")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MakeNonZero", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (value, min)) };
        Ok(__cordl_ret.into())
    }
    pub fn Max<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>),
                T,
                1usize,
            >("Max")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Max", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn Min<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>),
                T,
                1usize,
            >("Min")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Min", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (array)) };
        Ok(__cordl_ret.into())
    }
    pub fn NormalTangentBitangent(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Normal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                ),
                crate::UnityEngine::ProBuilder::Normal,
                2usize,
            >("NormalTangentBitangent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NormalTangentBitangent", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::ProBuilder::Normal = unsafe {
            method.invoke_unchecked((), (mesh, face))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Normal_IList_1_IList_1_1(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Vertex,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("Normal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Normal", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (vertices, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Normal_ProBuilderMesh_Face2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                ),
                crate::UnityEngine::Vector3,
                2usize,
            >("Normal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Normal", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (mesh, face))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Normal_Vector3_Vector3_Vector3_0(
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                crate::UnityEngine::Vector3,
                3usize,
            >("Normal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Normal", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (p0, p1, p2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Perpendicular(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2),
                crate::UnityEngine::Vector2,
                1usize,
            >("Perpendicular")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Perpendicular", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInCircumference(
        radius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, crate::UnityEngine::Vector2),
                crate::UnityEngine::Vector2,
                3usize,
            >("PointInCircumference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInCircumference", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (radius, angleInDegrees, origin))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInEllipseCircumference(
        xRadius: f32,
        yRadius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    f32,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                ),
                crate::UnityEngine::Vector2,
                5usize,
            >("PointInEllipseCircumference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInEllipseCircumference", 5usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (xRadius, yRadius, angleInDegrees, origin, tangent),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInEllipseCircumferenceWithConstantAngle(
        xRadius: f32,
        yRadius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    f32,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                ),
                crate::UnityEngine::Vector2,
                5usize,
            >("PointInEllipseCircumferenceWithConstantAngle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInEllipseCircumferenceWithConstantAngle", 5usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (xRadius, yRadius, angleInDegrees, origin, tangent),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Bounds2D_Vector2_1(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        polyBounds: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                    crate::UnityEngine::Vector2,
                ),
                bool,
                4usize,
            >("PointInPolygon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInPolygon", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (positions, polyBounds, edges, point))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Bounds2D_Vector2_2(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        polyBounds: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::ProBuilder::Edge,
                        >,
                    >,
                    crate::UnityEngine::Vector2,
                ),
                bool,
                4usize,
            >("PointInPolygon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInPolygon", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (positions, polyBounds, edges, point))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Vector2_0(
        polygon: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        point: crate::UnityEngine::Vector2,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    crate::UnityEngine::Vector2,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                bool,
                3usize,
            >("PointInPolygon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInPolygon", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (polygon, point, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PointInSphere(
        radius: f32,
        latitudeAngle: f32,
        longitudeAngle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32),
                crate::UnityEngine::Vector3,
                3usize,
            >("PointInSphere")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointInSphere", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (radius, latitudeAngle, longitudeAngle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PolygonArea(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                f32,
                2usize,
            >("PolygonArea")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PolygonArea", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (vertices, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RayIntersectsTriangle(
        InRay: crate::UnityEngine::Ray,
        InTriangleA: crate::UnityEngine::Vector3,
        InTriangleB: crate::UnityEngine::Vector3,
        InTriangleC: crate::UnityEngine::Vector3,
        OutDistance: quest_hook::libil2cpp::ByRefMut<f32>,
        OutPoint: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Ray,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                bool,
                6usize,
            >("RayIntersectsTriangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RayIntersectsTriangle", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (InRay, InTriangleA, InTriangleB, InTriangleC, OutDistance, OutPoint),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn RayIntersectsTriangle2(
        origin: crate::UnityEngine::Vector3,
        dir: crate::UnityEngine::Vector3,
        vert0: crate::UnityEngine::Vector3,
        vert1: crate::UnityEngine::Vector3,
        vert2: crate::UnityEngine::Vector3,
        distance: quest_hook::libil2cpp::ByRefMut<f32>,
        normal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                bool,
                7usize,
            >("RayIntersectsTriangle2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RayIntersectsTriangle2", 7usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (origin, dir, vert0, vert1, vert2, distance, normal),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersectsLineSegment_Vector2_Vector2_0(
        rect: crate::UnityEngine::Rect,
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Rect,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                ),
                bool,
                3usize,
            >("RectIntersectsLineSegment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RectIntersectsLineSegment", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (rect, a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersectsLineSegment_Vector3_Vector3_1(
        rect: crate::UnityEngine::Rect,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Rect,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                bool,
                3usize,
            >("RectIntersectsLineSegment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RectIntersectsLineSegment", 3usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (rect, a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReflectPoint(
        point: crate::UnityEngine::Vector2,
        lineStart: crate::UnityEngine::Vector2,
        lineEnd: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                ),
                crate::UnityEngine::Vector2,
                3usize,
            >("ReflectPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReflectPoint", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (point, lineStart, lineEnd))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RotateAroundPoint(
        v: crate::UnityEngine::Vector2,
        origin: crate::UnityEngine::Vector2,
        theta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2, f32),
                crate::UnityEngine::Vector2,
                3usize,
            >("RotateAroundPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RotateAroundPoint", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v, origin, theta))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScaleAroundPoint(
        v: crate::UnityEngine::Vector2,
        origin: crate::UnityEngine::Vector2,
        scale: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                ),
                crate::UnityEngine::Vector2,
                3usize,
            >("ScaleAroundPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScaleAroundPoint", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v, origin, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Secant(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), f32, 1usize>("Secant")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Secant", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (x)) };
        Ok(__cordl_ret.into())
    }
    pub fn Sign(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3),
                crate::UnityEngine::Vector3,
                1usize,
            >("Sign")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sign", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SignedAngle(
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2),
                f32,
                2usize,
            >("SignedAngle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SignedAngle", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn SmallestVector2_IList_1_1(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<i32>,
                    >,
                ),
                crate::UnityEngine::Vector2,
                2usize,
            >("SmallestVector2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SmallestVector2", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v, indexes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SmallestVector2_Il2CppArray0(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                >),
                crate::UnityEngine::Vector2,
                1usize,
            >("SmallestVector2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SmallestVector2", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked((), (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SqrDistance(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Vector3, crate::UnityEngine::Vector3),
                f32,
                2usize,
            >("SqrDistance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SqrDistance", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn SqrDistanceRayPoint(
        ray: crate::UnityEngine::Ray,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::Ray, crate::UnityEngine::Vector3),
                f32,
                2usize,
            >("SqrDistanceRayPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SqrDistanceRayPoint", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (ray, point)) };
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Subtract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Subtract", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (a, b, res))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sum(v: crate::UnityEngine::Vector3) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::UnityEngine::Vector3), f32, 1usize>("Sum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sum", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn TriangleArea(
        x: crate::UnityEngine::Vector3,
        y: crate::UnityEngine::Vector3,
        z: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                f32,
                3usize,
            >("TriangleArea")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TriangleArea", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (x, y, z)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
