#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+FxEventBoxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct FxEventBoxGroup {
    __cordl_parent: crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
    >,
    pub t: crate::BeatmapSaveDataVersion3::FxEventType,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+FxEventBoxGroup")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion3";
    const CLASS_NAME: &'static str = "FxEventBoxGroup";
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
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    type Target = crate::BeatmapSaveDataVersion3::EventBoxGroup_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    pub fn New(
        beat: f32,
        groupId: i32,
        _cordl_type: crate::BeatmapSaveDataVersion3::FxEventType,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beat, groupId, _cordl_type, eventBoxes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beat: f32,
        groupId: i32,
        _cordl_type: crate::BeatmapSaveDataVersion3::FxEventType,
        eventBoxes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion3::FxEventBox>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f32,
                        i32,
                        crate::BeatmapSaveDataVersion3::FxEventType,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion3::FxEventBox,
                                >,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (beat, groupId, _cordl_type, eventBoxes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataVersion3::FxEventType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::BeatmapSaveDataVersion3::FxEventType, 0usize>(
                        "get_type",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_type",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatmapSaveDataVersion3::FxEventType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion3+FxEventBoxGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::FxEventBoxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
