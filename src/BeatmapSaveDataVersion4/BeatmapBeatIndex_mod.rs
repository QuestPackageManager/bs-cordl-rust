#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapBeatIndex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub b: f32,
    pub r: i32,
    pub i: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapBeatIndex";
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
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_beat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beat", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl AsRef<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn as_ref(&self) -> &crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl AsMut<crate::BeatmapSaveDataCommon::IBeat>
for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn as_mut(&mut self) -> &mut crate::BeatmapSaveDataCommon::IBeat {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    >,
> for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapBeatIndex")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    >,
> for crate::BeatmapSaveDataVersion4::BeatmapBeatIndex {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
