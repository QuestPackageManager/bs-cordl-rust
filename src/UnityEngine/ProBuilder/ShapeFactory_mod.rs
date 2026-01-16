#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ShapeFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::ShapeFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ShapeFactory";
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
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ShapeFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ShapeFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl crate::UnityEngine::ProBuilder::ShapeFactory {
    pub fn Instantiate_PivotLocation0<T>(
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ProBuilder::PivotLocation),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        1usize,
                    >("Instantiate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Instantiate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = unsafe { cordl_method_info.invoke_unchecked((), (pivotType))? };
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Shape2(
        shape: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Shapes::Shape>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::Shapes::Shape,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        1usize,
                    >("Instantiate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Instantiate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = unsafe { cordl_method_info.invoke_unchecked((), (shape))? };
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Type_PivotLocation1(
        shapeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        pivotType: crate::UnityEngine::ProBuilder::PivotLocation,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            crate::UnityEngine::ProBuilder::PivotLocation,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        2usize,
                    >("Instantiate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Instantiate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = unsafe { cordl_method_info.invoke_unchecked((), (shapeType, pivotType))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+ShapeFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ShapeFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
