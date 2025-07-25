#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaberSwingRatingCounterDidFinishReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ISaberSwingRatingCounterDidFinishReceiver";
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
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl std::ops::Deref
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    pub fn HandleSaberSwingRatingCounterDidFinish(
        &mut self,
        saberSwingRatingCounter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISaberSwingRatingCounter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ISaberSwingRatingCounter,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleSaberSwingRatingCounterDidFinish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleSaberSwingRatingCounterDidFinish", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (saberSwingRatingCounter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ISaberSwingRatingCounterDidFinishReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ISaberSwingRatingCounterDidFinishReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
