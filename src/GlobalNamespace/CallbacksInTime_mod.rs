#[cfg(feature = "cordl_class_CallbacksInTime")]
#[repr(C)]
#[derive(Debug)]
pub struct CallbacksInTime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lastProcessedNode: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
    pub aheadTime: f32,
    pub beatmapEventDataForCallbacksAfterNodeRemoval:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    pub _callbacksWithSubtypeIdentifier: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<quest_hook::libil2cpp::Gc<crate::System::Type>, i32>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
                >,
            >,
        >,
    >,
    pub _callbacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataCallbackWrapper>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_CallbacksInTime")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CallbacksInTime {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CallbacksInTime";
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
#[cfg(feature = "CallbacksInTime")]
impl std::ops::Deref for crate::GlobalNamespace::CallbacksInTime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl std::ops::DerefMut for crate::GlobalNamespace::CallbacksInTime {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CallbacksInTime")]
impl crate::GlobalNamespace::CallbacksInTime {
    pub fn AddCallback(
        &mut self,
        callbackWrapper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callbackWrapper))? };
        Ok(__cordl_ret.into())
    }
    pub fn CallCallbacks_BeatmapDataItem0(
        &mut self,
        beatmapDataItem: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CallCallbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CallCallbacks", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (beatmapDataItem))? };
        Ok(__cordl_ret.into())
    }
    pub fn CallCallbacks_Type_BeatmapDataItem1(
        &mut self,
        beatmapEventDataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        beatmapDataItem: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                    ), quest_hook::libil2cpp::Void, 2usize>("CallCallbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CallCallbacks",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapEventDataType, beatmapDataItem))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(aheadTime: f32) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aheadTime))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveCallback(
        &mut self,
        callbackWrapper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataCallbackWrapper,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callbackWrapper))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        aheadTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (aheadTime))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isEmpty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_CallbacksInTime")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CallbacksInTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
