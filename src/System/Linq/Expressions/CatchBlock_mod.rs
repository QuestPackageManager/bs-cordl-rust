#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct CatchBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Variable_k__BackingField: *mut crate::System::Linq::Expressions::ParameterExpression,
    pub _Test_k__BackingField: *mut crate::System::Type,
    pub _Body_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
    pub _Filter_k__BackingField: *mut crate::System::Linq::Expressions::Expression,
}
#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::CatchBlock =>
    "System.Linq.Expressions"."CatchBlock"
);
#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
impl std::ops::Deref for crate::System::Linq::Expressions::CatchBlock {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::CatchBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
impl crate::System::Linq::Expressions::CatchBlock {
    pub fn New(
        test: *mut crate::System::Type,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        body: *mut crate::System::Linq::Expressions::Expression,
        filter: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (test, variable, body, filter))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        filter: *mut crate::System::Linq::Expressions::Expression,
        body: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::CatchBlock,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::CatchBlock = __cordl_object
            .invoke("Update", (variable, filter, body))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        test: *mut crate::System::Type,
        variable: *mut crate::System::Linq::Expressions::ParameterExpression,
        body: *mut crate::System::Linq::Expressions::Expression,
        filter: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (test, variable, body, filter))?;
        Ok(__cordl_ret)
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Body", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Filter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::Expression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::Expression = __cordl_object
            .invoke("get_Filter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Test(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Test", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Variable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Linq::Expressions::ParameterExpression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Linq::Expressions::ParameterExpression = __cordl_object
            .invoke("get_Variable", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Expressions+CatchBlock")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::CatchBlock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
