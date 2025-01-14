#[cfg(feature = "MainThreadDispatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct MainThreadDispatcher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dispatchQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Concurrent::ConcurrentQueue_1<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
    >,
}
#[cfg(feature = "MainThreadDispatcher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MainThreadDispatcher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MainThreadDispatcher";
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
#[cfg(feature = "MainThreadDispatcher")]
impl std::ops::Deref for crate::GlobalNamespace::MainThreadDispatcher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainThreadDispatcher")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainThreadDispatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainThreadDispatcher")]
impl crate::GlobalNamespace::MainThreadDispatcher {
    pub fn DispatchOnMainThread_Action0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DispatchOnMainThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DispatchOnMainThread", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchOnMainThread_Action_1_A1<A>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<A>>,
        firstParameter: A,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_1<A>>, A),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DispatchOnMainThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DispatchOnMainThread", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action, firstParameter))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchOnMainThread_Action_2_A_B2<A, B>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_2<A, B>>,
        firstParameter: A,
        secondParameter: B,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_2<A, B>>, A, B),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DispatchOnMainThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DispatchOnMainThread", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (action, firstParameter, secondParameter))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchOnMainThread_Action_3_A_B_C3<A, B, C>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_3<A, B, C>>,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action_3<A, B, C>>, A, B, C),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DispatchOnMainThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DispatchOnMainThread", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (action, firstParameter, secondParameter, thirdParameter),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchOnMainThread_Action_4_A_B_C_D4<A, B, C, D>(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_4<A, B, C, D>>,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
        fourthParameter: D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        D: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Action_4<A, B, C, D>>,
                    A,
                    B,
                    C,
                    D,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("DispatchOnMainThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DispatchOnMainThread", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        action,
                        firstParameter,
                        secondParameter,
                        thirdParameter,
                        fourthParameter,
                    ),
                )
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
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Tick", 0usize
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
#[cfg(feature = "MainThreadDispatcher")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MainThreadDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MainThreadDispatcher")]
impl AsRef<crate::Zenject::ITickable> for crate::GlobalNamespace::MainThreadDispatcher {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MainThreadDispatcher")]
impl AsMut<crate::Zenject::ITickable> for crate::GlobalNamespace::MainThreadDispatcher {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
