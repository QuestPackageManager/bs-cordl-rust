#[cfg(feature = "UnityEngine+ProBuilder+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Math =>
    "UnityEngine.ProBuilder"."Math"
);
#[cfg(feature = "UnityEngine+ProBuilder+Math")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Math {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approx(a: f32, b: f32, delta: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approx", (a, b, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approx2(
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approx2", (a, b, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approx3(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approx3", (a, b, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approx4(
        a: crate::UnityEngine::Vector4,
        b: crate::UnityEngine::Vector4,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approx4", (a, b, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApproxC(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApproxC", (a, b, delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average_Gc_Gc0(
        array: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (array, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average_Gc_Gc1(
        array: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (array, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average_Gc_Gc2(
        array: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector4>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (array, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp(
        value: i32,
        lowerBound: i32,
        upperBound: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, lowerBound, upperBound))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cross(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cross", (a, b, res))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistancePointLineSegment_Vector2_Vector2_Vector2_0(
        point: crate::UnityEngine::Vector2,
        lineStart: crate::UnityEngine::Vector2,
        lineEnd: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistancePointLineSegment", (point, lineStart, lineEnd))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistancePointLineSegment_Vector3_Vector3_Vector3_1(
        point: crate::UnityEngine::Vector3,
        lineStart: crate::UnityEngine::Vector3,
        lineEnd: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistancePointLineSegment", (point, lineStart, lineEnd))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivideBy_Vector2_Vector2_0(
        v: crate::UnityEngine::Vector2,
        o: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivideBy", (v, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivideBy_Vector3_Vector3_1(
        v: crate::UnityEngine::Vector3,
        o: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivideBy", (v, o))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector2_0(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureUnitVector", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector3_1(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureUnitVector", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUnitVector_Vector4_2(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureUnitVector", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FixNaN(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FixNaN", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBounds(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indices: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_ret: crate::UnityEngine::Bounds = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBounds", (positions, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLineSegmentIntersect_ByRefMut0(
        p0: crate::UnityEngine::Vector2,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        p3: crate::UnityEngine::Vector2,
        intersect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLineSegmentIntersect", (p0, p1, p2, p3, intersect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLineSegmentIntersect_Vector2_Vector2_Vector2_Vector2_1(
        p0: crate::UnityEngine::Vector2,
        p1: crate::UnityEngine::Vector2,
        p2: crate::UnityEngine::Vector2,
        p3: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLineSegmentIntersect", (p0, p1, p2, p3))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNearestPointRayRay_Ray_Ray0(
        a: crate::UnityEngine::Ray,
        b: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNearestPointRayRay", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNearestPointRayRay_Vector3_Vector3_Vector3_Vector3_1(
        ao: crate::UnityEngine::Vector3,
        ad: crate::UnityEngine::Vector3,
        bo: crate::UnityEngine::Vector3,
        bd: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNearestPointRayRay", (ao, ad, bo, bd))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvertScaleVector(
        scaleVector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvertScaleVector", (scaleVector))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCardinalAxis(
        v: crate::UnityEngine::Vector3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCardinalAxis", (v, epsilon))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector2_1(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector3_2(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_Vector4_3(
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumber_f32_0(value: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumber", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn LargestValue_Vector2_1(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LargestValue", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn LargestValue_Vector3_0(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LargestValue", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn LargestVector2_Gc0(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LargestVector2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn LargestVector2_Gc1(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LargestVector2", (v, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeNonZero(value: f32, min: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeNonZero", (value, min))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalTangentBitangent(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Normal> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Normal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalTangentBitangent", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normal_Gc_Gc1(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normal", (vertices, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normal_Gc_Gc2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normal", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normal_Vector3_Vector3_Vector3_0(
        p0: crate::UnityEngine::Vector3,
        p1: crate::UnityEngine::Vector3,
        p2: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normal", (p0, p1, p2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Perpendicular(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Perpendicular", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInCircumference(
        radius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInCircumference", (radius, angleInDegrees, origin))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInEllipseCircumference(
        xRadius: f32,
        yRadius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PointInEllipseCircumference",
                (xRadius, yRadius, angleInDegrees, origin, tangent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInEllipseCircumferenceWithConstantAngle(
        xRadius: f32,
        yRadius: f32,
        angleInDegrees: f32,
        origin: crate::UnityEngine::Vector2,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PointInEllipseCircumferenceWithConstantAngle",
                (xRadius, yRadius, angleInDegrees, origin, tangent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Gc_Vector2_1(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        polyBounds: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInPolygon", (positions, polyBounds, edges, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Gc_Vector2_2(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        polyBounds: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Bounds2D>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInPolygon", (positions, polyBounds, edges, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInPolygon_Vector2_0(
        polygon: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        point: crate::UnityEngine::Vector2,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInPolygon", (polygon, point, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointInSphere(
        radius: f32,
        latitudeAngle: f32,
        longitudeAngle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointInSphere", (radius, latitudeAngle, longitudeAngle))?;
        Ok(__cordl_ret.into())
    }
    pub fn PolygonArea(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PolygonArea", (vertices, indexes))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RayIntersectsTriangle",
                (InRay, InTriangleA, InTriangleB, InTriangleC, OutDistance, OutPoint),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RayIntersectsTriangle2",
                (origin, dir, vert0, vert1, vert2, distance, normal),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersectsLineSegment_Vector2_Vector2_0(
        rect: crate::UnityEngine::Rect,
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectIntersectsLineSegment", (rect, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersectsLineSegment_Vector3_Vector3_1(
        rect: crate::UnityEngine::Rect,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectIntersectsLineSegment", (rect, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReflectPoint(
        point: crate::UnityEngine::Vector2,
        lineStart: crate::UnityEngine::Vector2,
        lineEnd: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReflectPoint", (point, lineStart, lineEnd))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateAroundPoint(
        v: crate::UnityEngine::Vector2,
        origin: crate::UnityEngine::Vector2,
        theta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateAroundPoint", (v, origin, theta))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScaleAroundPoint(
        v: crate::UnityEngine::Vector2,
        origin: crate::UnityEngine::Vector2,
        scale: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScaleAroundPoint", (v, origin, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn Secant(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Secant", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignedAngle(
        a: crate::UnityEngine::Vector2,
        b: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignedAngle", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn SmallestVector2_Gc0(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SmallestVector2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn SmallestVector2_Gc1(
        v: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SmallestVector2", (v, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqrDistance(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqrDistance", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqrDistanceRayPoint(
        ray: crate::UnityEngine::Ray,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqrDistanceRayPoint", (ray, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (a, b, res))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sum(v: crate::UnityEngine::Vector3) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sum", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriangleArea(
        x: crate::UnityEngine::Vector3,
        y: crate::UnityEngine::Vector3,
        z: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriangleArea", (x, y, z))?;
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
