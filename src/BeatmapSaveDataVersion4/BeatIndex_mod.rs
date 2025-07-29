#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatIndex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub b: f32,
    pub i: i32,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatIndex")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataVersion4::BeatIndex {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatIndex";
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
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::BeatIndex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl crate::BeatmapSaveDataVersion4::BeatIndex {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_beat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_beat", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion4+BeatIndex")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl AsRef<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn as_ref(&self) -> &crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl AsMut<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn as_mut(&mut self) -> &mut crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    >,
> for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatIndex")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    >,
> for crate::BeatmapSaveDataVersion4::BeatIndex {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
