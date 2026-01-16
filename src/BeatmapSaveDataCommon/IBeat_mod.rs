#[cfg(feature = "cordl_class_BeatmapSaveDataCommon+IBeat")]
#[derive(Debug)]
#[repr(C)]
pub struct IBeat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataCommon+IBeat")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatmapSaveDataCommon::IBeat {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataCommon";
    const CLASS_NAME: &'static str = "IBeat";
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
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl std::ops::Deref for crate::BeatmapSaveDataCommon::IBeat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl std::ops::DerefMut for crate::BeatmapSaveDataCommon::IBeat {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl crate::BeatmapSaveDataCommon::IBeat {
    pub fn System_IComparable_BeatmapSaveDataCommon_IBeat__CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>),
                        i32,
                        1usize,
                    >("System.IComparable<BeatmapSaveDataCommon.IBeat>.CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IComparable<BeatmapSaveDataCommon.IBeat>.CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_beat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_beat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_beat",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataCommon+IBeat")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataCommon::IBeat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl
    AsRef<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
        >,
    > for crate::BeatmapSaveDataCommon::IBeat
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>>
    {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapSaveDataCommon+IBeat")]
impl
    AsMut<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
        >,
    > for crate::BeatmapSaveDataCommon::IBeat
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataCommon::IBeat>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
