#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ManagerUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "ManagerUtils";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
impl crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils {
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1")]
    pub type RegisterMember_1<T: quest_hook::libil2cpp::Type> = crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<
        T,
    >;
    pub fn RebuildInspectorForType<T>(
        panel: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
        >,
        cache: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberPairs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::System::ValueTuple_2<
                    T,
                    quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::DebugMember,
                    >,
                >,
            >,
        >,
        memberRegistration: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::IDebugUIPanel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::System::ValueTuple_2<
                                        T,
                                        quest_hook::libil2cpp::Gc<
                                            crate::Meta::XR::ImmersiveDebugger::DebugMember,
                                        >,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<
                                    T,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("RebuildInspectorForType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RebuildInspectorForType", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (panel, cache, _cordl_type, memberPairs, memberRegistration),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ManagerUtils_RegisterMember_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.Manager";
    const CLASS_NAME: &'static str = "ManagerUtils/RegisterMember`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Meta.XR.ImmersiveDebugger.Manager",
                        "ManagerUtils/RegisterMember`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T> {
    pub fn BeginInvoke(
        &mut self,
        memberController: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
        >,
        member: T,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::DebugMember,
        >,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::DebugMember,
                            >,
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        6usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        memberController,
                        member,
                        attribute,
                        instanceHandle,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        memberController: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
        >,
        member: T,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::DebugMember,
        >,
        instanceHandle: crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::UserInterface::IMember,
                            >,
                            T,
                            quest_hook::libil2cpp::Gc<
                                crate::Meta::XR::ImmersiveDebugger::DebugMember,
                            >,
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (memberController, member, attribute, instanceHandle),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+Manager+ManagerUtils+RegisterMember_1"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::Manager::ManagerUtils_RegisterMember_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
