#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandle {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    >,
    pub allocVerts: crate::UnityEngine::UIElements::UIR::Alloc,
    pub allocIndices: crate::UnityEngine::UIElements::UIR::Alloc,
    pub triangleCount: u32,
    pub allocPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub allocTime: u32,
    pub updateAllocID: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::MeshHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshHandle";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::MeshHandle {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::MeshHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl crate::UnityEngine::UIElements::UIR::MeshHandle {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::MeshHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
