#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
#[repr(C)]
#[derive(Debug)]
pub struct OnScreenControl {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_Control: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_NextControlOnDevice: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    >,
    pub m_InputEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenControl =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenControl"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::OnScreen::OnScreenControl {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::OnScreen::OnScreenControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
impl crate::UnityEngine::InputSystem::OnScreen::OnScreenControl {
    #[cfg(
        feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl+OnScreenDeviceInfo"
    )]
    pub type OnScreenDeviceInfo = crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendValueToControl<TValue>(
        &mut self,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValueToControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SentDefaultValueToControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SentDefaultValueToControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupInputControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupInputControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_control", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_controlPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlPathInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_controlPathInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controlPath(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPath", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_controlPathInternal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPathInternal", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::OnScreen::OnScreenControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl+OnScreenDeviceInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OnScreenControl_OnScreenDeviceInfo {
    pub eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    pub buffer: crate::Unity::Collections::NativeArray_1<u8>,
    pub device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    pub firstControl: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl+OnScreenDeviceInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenControl/OnScreenDeviceInfo"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl+OnScreenDeviceInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenControl+OnScreenDeviceInfo")]
impl crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo {
    pub fn AddControl(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddControl",
            (control),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Destroy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveControl(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::OnScreen::OnScreenControl_OnScreenDeviceInfo = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveControl",
            (control),
        )?;
        Ok(__cordl_ret.into())
    }
}
