#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerLoopSystem {
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub subSystemList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::LowLevel::PlayerLoopSystem,
        >,
    >,
    pub updateDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction,
    >,
    pub updateFunction: crate::System::IntPtr,
    pub loopConditionFunction: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LowLevel::PlayerLoopSystem =>
    "UnityEngine.LowLevel"."PlayerLoopSystem"
);
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::LowLevel::PlayerLoopSystem {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem")]
impl crate::UnityEngine::LowLevel::PlayerLoopSystem {
    #[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
    pub type UpdateFunction = crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction;
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerLoopSystem_UpdateFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction => "UnityEngine.LowLevel"
    ."PlayerLoopSystem/UpdateFunction"
);
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
impl std::ops::Deref for crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
impl std::ops::DerefMut
for crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
impl crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystem+UpdateFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
