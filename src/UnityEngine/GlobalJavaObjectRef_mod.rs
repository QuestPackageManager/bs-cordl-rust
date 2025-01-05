#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
#[repr(C)]
#[derive(Debug)]
pub struct GlobalJavaObjectRef {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_disposed: bool,
    pub m_jobject: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GlobalJavaObjectRef =>
    "UnityEngine"."GlobalJavaObjectRef"
);
#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
impl std::ops::Deref for crate::UnityEngine::GlobalJavaObjectRef {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
impl std::ops::DerefMut for crate::UnityEngine::GlobalJavaObjectRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
impl crate::UnityEngine::GlobalJavaObjectRef {
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
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        jobject: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (jobject))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        jobject: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (jobject))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GlobalJavaObjectRef>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (obj))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GlobalJavaObjectRef")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GlobalJavaObjectRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
