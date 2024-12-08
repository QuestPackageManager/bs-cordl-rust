#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicNode_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::BasicNode_1<T>,
    >,
    pub next: *mut crate::UnityEngine::UIElements::UIR::BasicNode_1<T>,
    pub data: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::BasicNode_1 < T >
    => "UnityEngine.UIElements.UIR"."BasicNode`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UIR::BasicNode_1<T> {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::BasicNode_1<T>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::BasicNode_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UIR::BasicNode_1<T> {
    pub fn AppendTo(
        &mut self,
        first: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UIElements::UIR::BasicNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendTo", (first))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BasicNode_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::BasicNode_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}