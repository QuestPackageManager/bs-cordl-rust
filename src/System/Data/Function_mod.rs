#[cfg(feature = "System+Data+Function")]
#[repr(C)]
#[derive(Debug)]
pub struct Function {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _id: crate::System::Data::FunctionId,
    pub _result: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _isValidateArguments: bool,
    pub _isVariantArgumentList: bool,
    pub _argumentCount: i32,
    pub _parameters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
}
#[cfg(feature = "System+Data+Function")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Function => "System.Data"
    ."Function"
);
#[cfg(feature = "System+Data+Function")]
impl std::ops::Deref for crate::System::Data::Function {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Function")]
impl std::ops::DerefMut for crate::System::Data::Function {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Function")]
impl crate::System::Data::Function {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        id: crate::System::Data::FunctionId,
        result: quest_hook::libil2cpp::Gc<crate::System::Type>,
        IsValidateArguments: bool,
        IsVariantArgumentList: bool,
        argumentCount: i32,
        a1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a3: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    name,
                    id,
                    result,
                    IsValidateArguments,
                    IsVariantArgumentList,
                    argumentCount,
                    a1,
                    a2,
                    a3,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        id: crate::System::Data::FunctionId,
        result: quest_hook::libil2cpp::Gc<crate::System::Type>,
        IsValidateArguments: bool,
        IsVariantArgumentList: bool,
        argumentCount: i32,
        a1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a3: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    name,
                    id,
                    result,
                    IsValidateArguments,
                    IsVariantArgumentList,
                    argumentCount,
                    a1,
                    a2,
                    a3,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Function")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Function {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
