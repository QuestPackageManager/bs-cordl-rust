#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputEventListener {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputEventListener";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
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
            crate::System::IObserver_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::IObserver_1<
                        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                1usize,
            >("Subscribe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Subscribe", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked(self, (observer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        _cordl__: crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventListener = unsafe {
            method.invoke_unchecked((), (_cordl__, callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        _cordl__: crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::InputDevice,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::InputSystem::LowLevel::InputEventListener,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventListener = unsafe {
            method.invoke_unchecked((), (_cordl__, callback))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
impl AsRef<
    crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    fn as_ref(
        &self,
    ) -> &crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener")]
impl AsMut<
    crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::InputEventListener {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IObservable_1<
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub observer: quest_hook::libil2cpp::Gc<
        crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputEventListener/DisposableObserver";
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
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
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
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+DisposableObserver"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_DisposableObserver {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventListener_ObserverState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub observers: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        quest_hook::libil2cpp::Gc<
            crate::System::IObserver_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    >,
    pub onEventDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputEventListener/ObserverState";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventListener+ObserverState")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventListener_ObserverState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputDevice,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("<.ctor>b__2_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<.ctor>b__2_0", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventPtr, device))
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
