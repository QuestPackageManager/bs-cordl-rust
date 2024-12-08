#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumBuilder {
    __cordl_parent: crate::System::Reflection::TypeInfo,
}
#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::EnumBuilder =>
    "System.Reflection.Emit"."EnumBuilder"
);
#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::EnumBuilder {
    type Target = crate::System::Reflection::TypeInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::EnumBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
impl crate::System::Reflection::Emit::EnumBuilder {
    pub fn GetMembers(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MemberInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MemberInfo,
        > = __cordl_object.invoke("GetMembers", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_BaseType", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPrimitiveImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrimitiveImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Module(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Module> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Module = __cordl_object
            .invoke("get_Module", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetElementType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetElementType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCustomAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetCustomAttributes", (inherit))?;
        Ok(__cordl_ret)
    }
    pub fn GetCustomAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: *mut crate::System::Type,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        > = __cordl_object.invoke("GetCustomAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret)
    }
    pub fn GetAttributeFlagsImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::TypeAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::TypeAttributes = __cordl_object
            .invoke("GetAttributeFlagsImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEvent(
        &mut self,
        name: *mut crate::System::String,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::EventInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::EventInfo = __cordl_object
            .invoke("GetEvent", (name, bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn get_Assembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("get_Assembly", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConstructors(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ConstructorInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ConstructorInfo,
        > = __cordl_object.invoke("GetConstructors", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyImpl(
        &mut self,
        name: *mut crate::System::String,
        bindingAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        returnType: *mut crate::System::Type,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::PropertyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::PropertyInfo = __cordl_object
            .invoke(
                "GetPropertyImpl",
                (name, bindingAttr, binder, returnType, types, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Namespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnderlyingSystemType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_UnderlyingSystemType", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPointerImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPointerImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMethodImpl(
        &mut self,
        name: *mut crate::System::String,
        bindingAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        callConvention: crate::System::Reflection::CallingConventions,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke(
                "GetMethodImpl",
                (name, bindingAttr, binder, callConvention, types, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetInterfaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeMember(
        &mut self,
        name: *mut crate::System::String,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        target: *mut crate::System::Object,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
        culture: *mut crate::System::Globalization::CultureInfo,
        namedParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "InvokeMember",
                (
                    name,
                    invokeAttr,
                    binder,
                    target,
                    args,
                    modifiers,
                    culture,
                    namedParameters,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsByRefImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsByRefImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsArrayImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsArrayImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFields(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::FieldInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::FieldInfo,
        > = __cordl_object.invoke("GetFields", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn GetField(
        &mut self,
        name: *mut crate::System::String,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::FieldInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::FieldInfo = __cordl_object
            .invoke("GetField", (name, bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn GetConstructorImpl(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        callConvention: crate::System::Reflection::CallingConventions,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        modifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Reflection::ParameterModifier,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::ConstructorInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::ConstructorInfo = __cordl_object
            .invoke(
                "GetConstructorImpl",
                (bindingAttr, binder, callConvention, types, modifiers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_AssemblyQualifiedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssemblyQualifiedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GUID(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object.invoke("get_GUID", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMethods(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MethodInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("GetMethods", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn GetProperties(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::PropertyInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::PropertyInfo,
        > = __cordl_object.invoke("GetProperties", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn IsCOMObjectImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCOMObjectImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasElementTypeImpl(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasElementTypeImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullName", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEvents(
        &mut self,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::EventInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::EventInfo,
        > = __cordl_object.invoke("GetEvents", (bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn GetNestedType(
        &mut self,
        name: *mut crate::System::String,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetNestedType", (name, bindingAttr))?;
        Ok(__cordl_ret)
    }
    pub fn IsDefined(
        &mut self,
        attributeType: *mut crate::System::Type,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDefined", (attributeType, inherit))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+Emit+EnumBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::Emit::EnumBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
