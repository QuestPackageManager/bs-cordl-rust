#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorEventBoxGroup {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::LightColorEventBox>,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "LightColorEventBoxGroup";
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
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::LightColorEventBox>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    pub fn CopyWith(
        &mut self,
        newBeat: crate::System::Nullable_1<f32>,
        newGroupId: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
        > = __cordl_object.invoke("CopyWith", (newBeat, newGroupId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beat: f32,
        groupId: i32,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorEventBox,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatmapSaveDataVersion3::LightColorEventBox,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beat, groupId, eventBoxes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+LightColorEventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
