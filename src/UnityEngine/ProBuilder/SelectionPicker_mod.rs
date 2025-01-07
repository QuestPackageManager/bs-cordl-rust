#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPicker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectionPicker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl crate::UnityEngine::ProBuilder::SelectionPicker {
    pub fn PickEdgesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickEdgesInRect",
                (cam, rect, selectable, options, pixelsPerPoint),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PickFace", (camera, mousePosition, pickable))?;
        Ok(__cordl_ret.into())
    }
    pub fn PickFacesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickFacesInRect",
                (cam, rect, selectable, options, pixelsPerPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickVerticesInRect(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        rect: crate::UnityEngine::Rect,
        selectable: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        options: crate::UnityEngine::ProBuilder::PickerOptions,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<i32>,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<i32>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickVerticesInRect",
                (cam, rect, selectable, options, pixelsPerPoint),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPicker")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPicker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
