#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ShapeGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ShapeGenerator =>
    "UnityEngine.ProBuilder"."ShapeGenerator"
);
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ShapeGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ShapeGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl crate::UnityEngine::ProBuilder::ShapeGenerator {
    pub fn CreateShape(
        shape: crate::UnityEngine::ProBuilder::ShapeType,
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateShape", (shape, pivotType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateArch(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        angle: f32,
        radius: f32,
        width: f32,
        depth: f32,
        radialCuts: i32,
        insideFaces: bool,
        outsideFaces: bool,
        frontFaces: bool,
        backFaces: bool,
        endCaps: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateArch",
                (
                    pivotType,
                    angle,
                    radius,
                    width,
                    depth,
                    radialCuts,
                    insideFaces,
                    outsideFaces,
                    frontFaces,
                    backFaces,
                    endCaps,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCone(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        radius: f32,
        height: f32,
        subdivAxis: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCone", (pivotType, radius, height, subdivAxis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCube(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCube", (pivotType, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCurvedStair(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        stairWidth: f32,
        height: f32,
        innerRadius: f32,
        circumference: f32,
        steps: i32,
        buildSides: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateCurvedStair",
                (
                    pivotType,
                    stairWidth,
                    height,
                    innerRadius,
                    circumference,
                    steps,
                    buildSides,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCylinder(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        axisDivisions: i32,
        radius: f32,
        height: f32,
        heightCuts: i32,
        smoothing: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateCylinder",
                (pivotType, axisDivisions, radius, height, heightCuts, smoothing),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateDoor(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        totalWidth: f32,
        totalHeight: f32,
        ledgeHeight: f32,
        legWidth: f32,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateDoor",
                (pivotType, totalWidth, totalHeight, ledgeHeight, legWidth, depth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateIcosahedron(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        radius: f32,
        subdivisions: i32,
        weldVertices: bool,
        manualUvs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateIcosahedron",
                (pivotType, radius, subdivisions, weldVertices, manualUvs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePipe(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        radius: f32,
        height: f32,
        thickness: f32,
        subdivAxis: i32,
        subdivHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GeneratePipe",
                (pivotType, radius, height, thickness, subdivAxis, subdivHeight),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePlane(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        width: f32,
        height: f32,
        widthCuts: i32,
        heightCuts: i32,
        axis: crate::UnityEngine::ProBuilder::Axis,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GeneratePlane",
                (pivotType, width, height, widthCuts, heightCuts, axis),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrism(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePrism", (pivotType, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStair_Vector3_i32__cordl_bool0(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        _cordl_size: crate::UnityEngine::Vector3,
        steps: i32,
        buildSides: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateStair", (pivotType, _cordl_size, steps, buildSides))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStair_i32_f32_f32_f32__cordl_bool__cordl_bool__cordl_bool1(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        steps: i32,
        width: f32,
        height: f32,
        depth: f32,
        sidesGoToFloor: bool,
        generateBack: bool,
        platformsOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateStair",
                (
                    pivotType,
                    steps,
                    width,
                    height,
                    depth,
                    sidesGoToFloor,
                    generateBack,
                    platformsOnly,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTorus(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        rows: i32,
        columns: i32,
        innerRadius: f32,
        outerRadius: f32,
        smooth: bool,
        horizontalCircumference: f32,
        verticalCircumference: f32,
        manualUvs: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateTorus",
                (
                    pivotType,
                    rows,
                    columns,
                    innerRadius,
                    outerRadius,
                    smooth,
                    horizontalCircumference,
                    verticalCircumference,
                    manualUvs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCirclePoints(
        segments: i32,
        radius: f32,
        circumference: f32,
        rotation: crate::UnityEngine::Quaternion,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCirclePoints",
                (segments, radius, circumference, rotation, offset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SubdivideIcosahedron(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubdivideIcosahedron", (vertices, radius))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::ShapeGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
