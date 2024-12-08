#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleProperty {
    __cordl_parent: crate::System::Object,
    pub m_Name: *mut crate::System::String,
    pub m_Line: i32,
    pub m_Values: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIElements::StyleValueHandle,
    >,
    pub isCustomProperty: bool,
    pub requireVariableResolve: bool,
}
#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleProperty =>
    "UnityEngine.UIElements"."StyleProperty"
);
#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleProperty {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
impl crate::UnityEngine::UIElements::StyleProperty {
    pub fn get_values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleValueHandle,
        > = __cordl_object.invoke("get_values", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
