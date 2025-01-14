#[cfg(feature = "OVRControllerHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_modelOculusTouchQuestAndRiftSLeftController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelOculusTouchQuestAndRiftSRightController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelOculusTouchRiftLeftController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelOculusTouchRiftRightController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelOculusTouchQuest2LeftController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelOculusTouchQuest2RightController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelMetaTouchProLeftController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelMetaTouchProRightController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelMetaTouchPlusLeftController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_modelMetaTouchPlusRightController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub m_controller: crate::GlobalNamespace::OVRInput_Controller,
    pub m_showState: crate::GlobalNamespace::OVRInput_InputDeviceShowState,
    pub showWhenHandsArePoweredByNaturalControllerPoses: bool,
    pub m_animator: quest_hook::libil2cpp::Gc<crate::UnityEngine::Animator>,
    pub m_activeController: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub m_controllerModelsInitialized: bool,
    pub m_hasInputFocus: bool,
    pub m_hasInputFocusPrev: bool,
    pub activeControllerType: crate::GlobalNamespace::OVRControllerHelper_ControllerType,
    pub m_prevControllerConnected: bool,
    pub m_prevControllerConnectedCached: bool,
    pub m_prevControllerInHandState: crate::GlobalNamespace::OVRInput_ControllerInHandState,
}
#[cfg(feature = "OVRControllerHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRControllerHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRControllerHelper";
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
#[cfg(feature = "OVRControllerHelper")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerHelper {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerHelper")]
impl crate::GlobalNamespace::OVRControllerHelper {
    #[cfg(feature = "OVRControllerHelper+ControllerType")]
    pub type ControllerType = crate::GlobalNamespace::OVRControllerHelper_ControllerType;
    pub fn InitializeControllerModels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("InitializeControllerModels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitializeControllerModels", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn InputFocusAquired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InputFocusAquired")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InputFocusAquired", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn InputFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InputFocusLost")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InputFocusLost", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "OVRControllerHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRControllerHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRControllerHelper+ControllerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRControllerHelper_ControllerType {
    #[default]
    Quest2 = 3i32,
    QuestAndRiftS = 1i32,
    Rift = 2i32,
    TouchPlus = 5i32,
    TouchPro = 4i32,
}
#[cfg(feature = "OVRControllerHelper+ControllerType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRControllerHelper_ControllerType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRControllerHelper/ControllerType";
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
#[cfg(feature = "OVRControllerHelper+ControllerType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRControllerHelper_ControllerType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRControllerHelper+ControllerType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRControllerHelper_ControllerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRControllerHelper+ControllerType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRControllerHelper_ControllerType {
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
#[cfg(feature = "OVRControllerHelper+ControllerType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRControllerHelper_ControllerType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
