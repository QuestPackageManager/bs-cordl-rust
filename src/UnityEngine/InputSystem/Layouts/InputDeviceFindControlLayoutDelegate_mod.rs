#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct InputDeviceFindControlLayoutDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate =>
    "UnityEngine.InputSystem.Layouts"."InputDeviceFindControlLayoutDelegate"
);
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
impl crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate {
    pub fn BeginInvoke(
        &mut self,
        description: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        matchedLayout: *mut crate::System::String,
        executeDeviceCommand: *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceExecuteCommandDelegate,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (description, matchedLayout, executeDeviceCommand, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        description: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("EndInvoke", (description, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        description: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        matchedLayout: *mut crate::System::String,
        executeDeviceCommand: *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceExecuteCommandDelegate,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("Invoke", (description, matchedLayout, executeDeviceCommand))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Layouts+InputDeviceFindControlLayoutDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
