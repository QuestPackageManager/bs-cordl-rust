#[cfg(feature = "System+Reflection+PropertyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyInfo {
    __cordl_parent: crate::System::Reflection::MemberInfo,
}
#[cfg(feature = "System+Reflection+PropertyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::PropertyInfo =>
    "System.Reflection"."PropertyInfo"
);
#[cfg(feature = "System+Reflection+PropertyInfo")]
impl std::ops::Deref for crate::System::Reflection::PropertyInfo {
    type Target = crate::System::Reflection::MemberInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+PropertyInfo")]
impl std::ops::DerefMut for crate::System::Reflection::PropertyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+PropertyInfo")]
impl crate::System::Reflection::PropertyInfo {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetGetMethod_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("GetGetMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGetMethod__cordl_bool1(
        &mut self,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("GetGetMethod", (nonPublic))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexParameters(
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
        > = __cordl_object.invoke("GetIndexParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSetMethod_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("GetSetMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSetMethod__cordl_bool1(
        &mut self,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("GetSetMethod", (nonPublic))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue_BindingFlags_Binder_Il2CppArray_CultureInfo2(
        &mut self,
        obj: *mut crate::System::Object,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        index: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValue", (obj, invokeAttr, binder, index, culture))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue_Il2CppArray1(
        &mut self,
        obj: *mut crate::System::Object,
        index: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValue", (obj, index))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValue", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetValue_BindingFlags_Binder_Il2CppArray_CultureInfo2(
        &mut self,
        obj: *mut crate::System::Object,
        value: *mut crate::System::Object,
        invokeAttr: crate::System::Reflection::BindingFlags,
        binder: *mut crate::System::Reflection::Binder,
        index: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (obj, value, invokeAttr, binder, index, culture))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_Il2CppArray1(
        &mut self,
        obj: *mut crate::System::Object,
        value: *mut crate::System::Object,
        index: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (obj, value, index))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue_Object_Object0(
        &mut self,
        obj: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (obj, value))?;
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
    pub fn get_CanRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanWrite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("get_GetMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MemberTypes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::MemberTypes = __cordl_object
            .invoke("get_MemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PropertyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_PropertyType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+PropertyInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::PropertyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
