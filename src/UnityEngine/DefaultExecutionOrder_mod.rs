#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultExecutionOrder {
    __cordl_parent: crate::System::Attribute,
    pub m_Order: i32,
}
#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DefaultExecutionOrder =>
    "UnityEngine"."DefaultExecutionOrder"
);
#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
impl std::ops::Deref for crate::UnityEngine::DefaultExecutionOrder {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
impl std::ops::DerefMut for crate::UnityEngine::DefaultExecutionOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
impl crate::UnityEngine::DefaultExecutionOrder {
    pub fn New(order: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (order))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (order))?;
        Ok(__cordl_ret)
    }
    pub fn get_order(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_order", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+DefaultExecutionOrder")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::DefaultExecutionOrder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
