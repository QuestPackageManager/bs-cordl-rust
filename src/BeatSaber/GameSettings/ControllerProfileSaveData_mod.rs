#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfileSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub alternativeHandling: bool,
    pub leftController: crate::BeatSaber::GameSettings::Controller,
    pub rightController: crate::BeatSaber::GameSettings::Controller,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.GameSettings";
    const CLASS_NAME: &'static str = "ControllerProfileSaveData";
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
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    pub fn New(
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (alternativeHandling, leftController, rightController),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            bool,
                            crate::BeatSaber::GameSettings::Controller,
                            crate::BeatSaber::GameSettings::Controller,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (alternativeHandling, leftController, rightController),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfileSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfileSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
