#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeforeRenderHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::BeforeRenderHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "BeforeRenderHelper";
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
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl std::ops::Deref for crate::UnityEngine::BeforeRenderHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl std::ops::DerefMut for crate::UnityEngine::BeforeRenderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl crate::UnityEngine::BeforeRenderHelper {
    #[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
    pub type OrderBlock = crate::UnityEngine::BeforeRenderHelper_OrderBlock;
    pub fn GetUpdateOrder(
        callback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::BeforeRenderHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>),
                i32,
                1usize,
            >("GetUpdateOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::BeforeRenderHelper as
                    quest_hook::libil2cpp::Type > ::class(), "GetUpdateOrder", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::BeforeRenderHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Invoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::BeforeRenderHelper as
                    quest_hook::libil2cpp::Type > ::class(), "Invoke", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback(
        callback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::BeforeRenderHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::BeforeRenderHelper as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback(
        callback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::BeforeRenderHelper as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::BeforeRenderHelper as
                    quest_hook::libil2cpp::Type > ::class(), "UnregisterCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::BeforeRenderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeforeRenderHelper_OrderBlock {
    pub order: i32,
    pub callback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityAction>,
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "BeforeRenderHelper/OrderBlock";
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
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
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
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
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
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
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
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
impl crate::UnityEngine::BeforeRenderHelper_OrderBlock {}
