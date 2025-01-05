#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegratedSubsystemDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::IntegratedSubsystemDescriptor =>
    "UnityEngine"."IntegratedSubsystemDescriptor"
);
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl std::ops::Deref for crate::UnityEngine::IntegratedSubsystemDescriptor {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl std::ops::DerefMut for crate::UnityEngine::IntegratedSubsystemDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl crate::UnityEngine::IntegratedSubsystemDescriptor {
    pub fn CreateImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem> = __cordl_object
            .invoke("CreateImpl", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem> = __cordl_object
            .invoke("UnityEngine.ISubsystemDescriptor.Create", ())?;
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
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::IntegratedSubsystemDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystemDescriptor>>
for crate::UnityEngine::IntegratedSubsystemDescriptor {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystemDescriptor> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystemDescriptor>>
for crate::UnityEngine::IntegratedSubsystemDescriptor {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystemDescriptor> {
        unsafe { std::mem::transmute(self) }
    }
}
