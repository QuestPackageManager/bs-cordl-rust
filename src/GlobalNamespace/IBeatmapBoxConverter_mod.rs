#[cfg(feature = "cordl_class_IBeatmapBoxConverter")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IBeatmapBoxConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_IBeatmapBoxConverter")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IBeatmapBoxConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBeatmapBoxConverter";
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
#[cfg(feature = "IBeatmapBoxConverter")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatmapBoxConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapBoxConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatmapBoxConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapBoxConverter")]
impl crate::GlobalNamespace::IBeatmapBoxConverter {
    pub fn ConvertBoxEventGroupId(
        &mut self,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::System::ValueTuple_2<i32, i32>>,
        >,
        groupId: i32,
        boxEventType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::System::ValueTuple_2<i32, i32>,
                            >,
                        >,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ConvertBoxEventGroupId"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConvertBoxEventGroupId",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, groupId, boxEventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertRotationEvent(
        &mut self,
        beat: f32,
        lightRotationEvent: crate::BeatmapSaveDataVersion4::LightRotationEvent,
        eventBox: crate::BeatmapSaveDataVersion4::LightRotationEventBox,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightRotationBaseData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            crate::BeatmapSaveDataVersion4::LightRotationEvent,
                            crate::BeatmapSaveDataVersion4::LightRotationEventBox,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LightRotationBaseData,
                        >,
                        3usize,
                    >("ConvertRotationEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertRotationEvent", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightRotationBaseData> = unsafe {
            cordl_method_info.invoke_unchecked(self, (beat, lightRotationEvent, eventBox))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_IBeatmapBoxConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBeatmapBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
