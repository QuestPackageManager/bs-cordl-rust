#[cfg(feature = "System+Reflection+Module")]
#[repr(C)]
#[derive(Debug)]
pub struct Module {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Reflection+Module")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Module =>
    "System.Reflection"."Module"
);
#[cfg(feature = "System+Reflection+Module")]
impl std::ops::Deref for crate::System::Reflection::Module {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl std::ops::DerefMut for crate::System::Reflection::Module {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl crate::System::Reflection::Module {
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterTypeNameIgnoreCaseImpl(
        cls: quest_hook::libil2cpp::Gc<crate::System::Type>,
        filterCriteria: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterTypeNameIgnoreCaseImpl", (cls, filterCriteria))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterTypeNameImpl(
        cls: quest_hook::libil2cpp::Gc<crate::System::Type>,
        filterCriteria: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterTypeNameImpl", (cls, filterCriteria))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("GetCustomAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("GetCustomAttributes", (inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModuleVersionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("GetModuleVersionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDefined", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsResource(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsResource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
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
    pub fn get_Assembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("get_Assembly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ModuleVersionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_ModuleVersionId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ScopeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ScopeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
        right: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::Module {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsRef<crate::System::Reflection::ICustomAttributeProvider>
for crate::System::Reflection::Module {
    fn as_ref(&self) -> &crate::System::Reflection::ICustomAttributeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsMut<crate::System::Reflection::ICustomAttributeProvider>
for crate::System::Reflection::Module {
    fn as_mut(&mut self) -> &mut crate::System::Reflection::ICustomAttributeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsRef<crate::System::Runtime::InteropServices::_Module>
for crate::System::Reflection::Module {
    fn as_ref(&self) -> &crate::System::Runtime::InteropServices::_Module {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsMut<crate::System::Runtime::InteropServices::_Module>
for crate::System::Reflection::Module {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::InteropServices::_Module {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::Module {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+Module")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::Module {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
