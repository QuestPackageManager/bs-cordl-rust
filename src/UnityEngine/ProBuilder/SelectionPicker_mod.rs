#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+SelectionPicker")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPicker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+SelectionPicker")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::SelectionPicker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "SelectionPicker";
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
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SelectionPicker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectionPicker {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl crate::UnityEngine::ProBuilder::SelectionPicker {
    pub fn PickEdgesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                >,
                            >,
                            crate::UnityEngine::ProBuilder::PickerOptions,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Edge,
                            >,
                        >,
                        5usize,
                    >("PickEdgesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickEdgesInRect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, rect, selectable, options, pixelsPerPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PickFace(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        mousePosition: crate::UnityEngine::Vector3,
        pickable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                        3usize,
                    >("PickFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickFace", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (camera, mousePosition, pickable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PickFacesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                >,
                            >,
                            crate::UnityEngine::ProBuilder::PickerOptions,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::Face,
                                >,
                            >,
                        >,
                        5usize,
                    >("PickFacesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickFacesInRect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, rect, selectable, options, pixelsPerPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PickVerticesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<i32>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                >,
                            >,
                            crate::UnityEngine::ProBuilder::PickerOptions,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
                            quest_hook::libil2cpp::Gc<i32>,
                        >,
                        5usize,
                    >("PickVerticesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickVerticesInRect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            quest_hook::libil2cpp::Gc<i32>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, rect, selectable, options, pixelsPerPoint))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+SelectionPicker")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPicker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
