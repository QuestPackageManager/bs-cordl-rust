#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputEventListener {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventListener =>
    "UnityEngine.InputSystem.LowLevel"."InputEventListener"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
    )]
    pub type DisposableObserver = crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
    pub type ObserverState = crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState;
    pub fn Subscribe(
        &mut self,
        observer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Subscribe",
            (observer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        _cordl__: crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventListener = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (_cordl__, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        _cordl__: crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventListener = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (_cordl__, callback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::LowLevel::InputEventPtr>,
> for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::LowLevel::InputEventPtr>,
> for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventListener_DisposableObserver {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub observer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver =>
    "UnityEngine.InputSystem.LowLevel"."InputEventListener/DisposableObserver"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventListener_ObserverState {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub observers: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
    pub onEventDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState =>
    "UnityEngine.InputSystem.LowLevel"."InputEventListener/ObserverState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __ctor_b__2_0(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__2_0", (eventPtr, device))?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
