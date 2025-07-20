#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SubsystemDescriptor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SubsystemDescriptor";
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
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl std::ops::Deref for crate::UnityEngine::SubsystemDescriptor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl std::ops::DerefMut for crate::UnityEngine::SubsystemDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl crate::UnityEngine::SubsystemDescriptor {
    pub fn CreateImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SubsystemDescriptor as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
                0usize,
            >("CreateImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SubsystemDescriptor as
                    quest_hook::libil2cpp::Type > ::class(), "CreateImpl", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem> = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn UnityEngine_ISubsystemDescriptor_Create(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SubsystemDescriptor as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
                0usize,
            >("UnityEngine.ISubsystemDescriptor.Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SubsystemDescriptor as
                    quest_hook::libil2cpp::Type > ::class(),
                    "UnityEngine.ISubsystemDescriptor.Create", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SubsystemDescriptor as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SubsystemDescriptor as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SubsystemDescriptor as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_id")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SubsystemDescriptor as
                    quest_hook::libil2cpp::Type > ::class(), "get_id", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SubsystemDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl AsRef<crate::UnityEngine::ISubsystemDescriptor>
for crate::UnityEngine::SubsystemDescriptor {
    fn as_ref(&self) -> &crate::UnityEngine::ISubsystemDescriptor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+SubsystemDescriptor")]
impl AsMut<crate::UnityEngine::ISubsystemDescriptor>
for crate::UnityEngine::SubsystemDescriptor {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISubsystemDescriptor {
        unsafe { std::mem::transmute(self) }
    }
}
