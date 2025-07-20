#[cfg(feature = "HMUI+ScreenModeData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenModeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Vector3,
    pub scale: f32,
    pub radius: f32,
    pub offsetHeightByHeadPos: bool,
    pub yOffsetRelativeToHead: f32,
    pub minYPos: f32,
}
#[cfg(feature = "HMUI+ScreenModeData")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::ScreenModeData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ScreenModeData";
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
#[cfg(feature = "HMUI+ScreenModeData")]
impl std::ops::Deref for crate::HMUI::ScreenModeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl std::ops::DerefMut for crate::HMUI::ScreenModeData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl crate::HMUI::ScreenModeData {
    pub fn New(
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Vector3,
        scale: f32,
        radius: f32,
        offsetHeightByHeadPos: bool,
        yOffsetRelativeToHead: f32,
        minYPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    position,
                    rotation,
                    scale,
                    radius,
                    offsetHeightByHeadPos,
                    yOffsetRelativeToHead,
                    minYPos,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Vector3,
        scale: f32,
        radius: f32,
        offsetHeightByHeadPos: bool,
        yOffsetRelativeToHead: f32,
        minYPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            f32,
                            f32,
                            bool,
                            f32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        position,
                        rotation,
                        scale,
                        radius,
                        offsetHeightByHeadPos,
                        yOffsetRelativeToHead,
                        minYPos,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ScreenModeData")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ScreenModeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
