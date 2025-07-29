#[cfg(feature = "cordl_class_NoteCutHapticEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutHapticEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _normalPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _chainHeadPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _chainLinkPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _bombPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _badCutPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcHeadPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcTailPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _arcHeadAndTailPreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteCutHapticEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutHapticEffect";
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
#[cfg(feature = "cordl_class_NoteCutHapticEffect")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutHapticEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutHapticEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutHapticEffect")]
impl crate::GlobalNamespace::NoteCutHapticEffect {
    #[cfg(feature = "NoteCutHapticEffect+Type")]
    pub type Type = crate::GlobalNamespace::NoteCutHapticEffect_Type;
    pub fn HitNote(
        &mut self,
        saberType: crate::GlobalNamespace::SaberType,
        _cordl_type: crate::GlobalNamespace::NoteCutHapticEffect_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::SaberType,
                            crate::GlobalNamespace::NoteCutHapticEffect_Type,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HitNote")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "HitNote",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (saberType, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
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
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutHapticEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteCutHapticEffect_Type {
    #[default]
    ArcHead = 5i32,
    ArcHeadAndTail = 7i32,
    ArcTail = 6i32,
    BadCut = 4i32,
    Bomb = 3i32,
    ChainHead = 1i32,
    ChainLink = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteCutHapticEffect_Type {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutHapticEffect/Type";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NoteCutHapticEffect_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NoteCutHapticEffect_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NoteCutHapticEffect_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_NoteCutHapticEffect+Type")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NoteCutHapticEffect_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
