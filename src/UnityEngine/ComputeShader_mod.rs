#[cfg(feature = "UnityEngine+ComputeShader")]
#[repr(C)]
#[derive(Debug)]
pub struct ComputeShader {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+ComputeShader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ComputeShader => "UnityEngine"
    ."ComputeShader"
);
#[cfg(feature = "UnityEngine+ComputeShader")]
impl std::ops::Deref for crate::UnityEngine::ComputeShader {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ComputeShader")]
impl std::ops::DerefMut for crate::UnityEngine::ComputeShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ComputeShader")]
impl crate::UnityEngine::ComputeShader {
    pub fn FindKernel(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindKernel", (name))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ComputeShader")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ComputeShader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
