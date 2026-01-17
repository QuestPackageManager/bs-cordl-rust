#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeGenerator")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ShapeGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeGenerator")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::ShapeGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ShapeGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ShapeGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeGenerator")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ShapeGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::ShapeType,
                            crate::UnityEngine::ProBuilder::PivotLocation,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        2usize,
                    >("CreateShape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShape", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> =
            unsafe { cordl_method_info.invoke_unchecked((), (shape, pivotType))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            f32,
                            f32,
                            f32,
                            i32,
                            bool,
                            bool,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        11usize,
                    >("GenerateArch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateArch", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
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
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ProBuilder::PivotLocation, f32, f32, i32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        4usize,
                    >("GenerateCone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCone", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked((), (pivotType, radius, height, subdivAxis))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCube(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        2usize,
                    >("GenerateCube")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCube", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> =
            unsafe { cordl_method_info.invoke_unchecked((), (pivotType, _cordl_size))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            f32,
                            f32,
                            f32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        7usize,
                    >("GenerateCurvedStair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCurvedStair", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    pivotType,
                    stairWidth,
                    height,
                    innerRadius,
                    circumference,
                    steps,
                    buildSides,
                ),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            i32,
                            f32,
                            f32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        6usize,
                    >("GenerateCylinder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateCylinder", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    pivotType,
                    axisDivisions,
                    radius,
                    height,
                    heightCuts,
                    smoothing,
                ),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            f32,
                            f32,
                            f32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        6usize,
                    >("GenerateDoor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateDoor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    pivotType,
                    totalWidth,
                    totalHeight,
                    ledgeHeight,
                    legWidth,
                    depth,
                ),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            i32,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        5usize,
                    >("GenerateIcosahedron")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateIcosahedron", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (pivotType, radius, subdivisions, weldVertices, manualUvs),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            f32,
                            f32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        6usize,
                    >("GeneratePipe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GeneratePipe", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    pivotType,
                    radius,
                    height,
                    thickness,
                    subdivAxis,
                    subdivHeight,
                ),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            f32,
                            f32,
                            i32,
                            i32,
                            crate::UnityEngine::ProBuilder::Axis,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        6usize,
                    >("GeneratePlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GeneratePlane", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (pivotType, width, height, widthCuts, heightCuts, axis))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrism(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
        _cordl_size: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        2usize,
                    >("GeneratePrism")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GeneratePrism", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> =
            unsafe { cordl_method_info.invoke_unchecked((), (pivotType, _cordl_size))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            crate::UnityEngine::Vector3,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        4usize,
                    >("GenerateStair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateStair", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked((), (pivotType, _cordl_size, steps, buildSides))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            i32,
                            f32,
                            f32,
                            f32,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        8usize,
                    >("GenerateStair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateStair", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
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
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ProBuilder::PivotLocation,
                            i32,
                            i32,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        9usize,
                    >("GenerateTorus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateTorus", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
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
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCirclePoints(
        segments: i32,
        radius: f32,
        circumference: f32,
        rotation: crate::UnityEngine::Quaternion,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, f32, f32, crate::UnityEngine::Quaternion, f32),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Vector3,
                            >,
                        >,
                        5usize,
                    >("GetCirclePoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCirclePoints", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (segments, radius, circumference, rotation, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubdivideIcosahedron(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                        >,
                        f32,
                    ), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >, 2usize>("SubdivideIcosahedron")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SubdivideIcosahedron",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (vertices, radius))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeGenerator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ShapeGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
