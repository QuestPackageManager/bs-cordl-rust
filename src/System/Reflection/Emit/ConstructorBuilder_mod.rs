#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstructorBuilder {
    __cordl_parent: crate::System::Reflection::ConstructorInfo,
}
#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::Emit::ConstructorBuilder =>
    "System.Reflection.Emit"."ConstructorBuilder"
);
#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
impl std::ops::Deref for crate::System::Reflection::Emit::ConstructorBuilder {
    type Target = crate::System::Reflection::ConstructorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
impl std::ops::DerefMut for crate::System::Reflection::Emit::ConstructorBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
impl crate::System::Reflection::Emit::ConstructorBuilder {
    pub fn get_DeclaringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_DeclaringType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MethodHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::RuntimeMethodHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::RuntimeMethodHandle = __cordl_object
            .invoke("get_MethodHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::ParameterInfo,
        > = __cordl_object.invoke("GetParameters", ())?;
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
    pub fn GetMethodImplementationFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MethodImplAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::MethodImplAttributes = __cordl_object
            .invoke("GetMethodImplementationFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MethodAttributes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::MethodAttributes = __cordl_object
            .invoke("get_Attributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReflectedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ReflectedType", ())?;
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
    pub fn Invoke_BindingFlags_Binder_Il2CppArray_CultureInfo0(
        &mut self,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Invoke", (invokeAttr, binder, parameters, culture))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke_Object_BindingFlags_Binder_Il2CppArray_CultureInfo1(
        &mut self,
        obj: *mut crate::System::Object,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Invoke", (obj, invokeAttr, binder, parameters, culture))?;
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
}
#[cfg(feature = "System+Reflection+Emit+ConstructorBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::Emit::ConstructorBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
