#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimePropertyInfo_GetterAdapter {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::RuntimePropertyInfo_GetterAdapter => "System.Reflection"
    ."RuntimePropertyInfo/GetterAdapter"
);
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
impl std::ops::Deref for crate::System::Reflection::RuntimePropertyInfo_GetterAdapter {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
impl std::ops::DerefMut
for crate::System::Reflection::RuntimePropertyInfo_GetterAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
impl crate::System::Reflection::RuntimePropertyInfo_GetterAdapter {
    pub fn Invoke(
        &mut self,
        _this: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Invoke", (_this))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::RuntimePropertyInfo_GetterAdapter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimePropertyInfo_Getter_2<
    T: quest_hook::libil2cpp::Type,
    R: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_R: std::marker::PhantomData<R>,
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::RuntimePropertyInfo_Getter_2
    < T, R > => "System.Reflection"."RuntimePropertyInfo/Getter`2" < T, R >
);
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
impl<T: quest_hook::libil2cpp::Type, R: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Reflection::RuntimePropertyInfo_Getter_2<T, R> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
impl<T: quest_hook::libil2cpp::Type, R: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Reflection::RuntimePropertyInfo_Getter_2<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    R: quest_hook::libil2cpp::Type,
> crate::System::Reflection::RuntimePropertyInfo_Getter_2<T, R> {
    pub fn Invoke(&mut self, _this: T) -> quest_hook::libil2cpp::Result<R>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: R = __cordl_object.invoke("Invoke", (_this))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    R: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::RuntimePropertyInfo_Getter_2<T, R> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimePropertyInfo {
    __cordl_parent: crate::System::Reflection::PropertyInfo,
    pub klass: crate::System::IntPtr,
    pub prop: crate::System::IntPtr,
    pub info: crate::System::Reflection::MonoPropertyInfo,
    pub cached: crate::System::Reflection::PInfo,
    pub cached_getter: *mut crate::System::Reflection::RuntimePropertyInfo_GetterAdapter,
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::RuntimePropertyInfo =>
    "System.Reflection"."RuntimePropertyInfo"
);
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
impl std::ops::Deref for crate::System::Reflection::RuntimePropertyInfo {
    type Target = crate::System::Reflection::PropertyInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
impl std::ops::DerefMut for crate::System::Reflection::RuntimePropertyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
impl crate::System::Reflection::RuntimePropertyInfo {
    #[cfg(feature = "System+Reflection+RuntimePropertyInfo+Getter_2")]
    pub type Getter_2<T: quest_hook::libil2cpp::Type, R: quest_hook::libil2cpp::Type> = crate::System::Reflection::RuntimePropertyInfo_Getter_2<
        T,
        R,
    >;
    #[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
    pub type StaticGetter_1<R: quest_hook::libil2cpp::Type> = crate::System::Reflection::RuntimePropertyInfo_StaticGetter_1<
        R,
    >;
    #[cfg(feature = "System+Reflection+RuntimePropertyInfo+GetterAdapter")]
    pub type GetterAdapter = crate::System::Reflection::RuntimePropertyInfo_GetterAdapter;
    pub fn CachePropertyInfo(
        &mut self,
        flags: crate::System::Reflection::PInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CachePropertyInfo", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn FormatNameAndSig(
        &mut self,
        serialization: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatNameAndSig", (serialization))?;
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
    pub fn GetDeclaringTypeInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::RuntimeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::RuntimeType = __cordl_object
            .invoke("GetDeclaringTypeInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGetMethod(
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
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn GetRuntimeModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::RuntimeModule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::RuntimeModule = __cordl_object
            .invoke("GetRuntimeModule", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSetMethod(
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
    pub fn GetValue_BindingFlags_Binder_Il2CppArray_CultureInfo1(
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
    pub fn GetValue_Il2CppArray0(
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SerializationToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SerializationToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetValue(
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn get_BindingFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::BindingFlags = __cordl_object
            .invoke("get_BindingFlags", ())?;
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
    pub fn get_MetadataToken(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MetadataToken", ())?;
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
    pub fn get_ReflectedTypeInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::RuntimeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::RuntimeType = __cordl_object
            .invoke("get_ReflectedTypeInternal", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::RuntimePropertyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimePropertyInfo_StaticGetter_1<R: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_R: std::marker::PhantomData<R>,
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::RuntimePropertyInfo_StaticGetter_1 < R > => "System.Reflection"
    ."RuntimePropertyInfo/StaticGetter`1" < R >
);
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
impl<R: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Reflection::RuntimePropertyInfo_StaticGetter_1<R> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
impl<R: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Reflection::RuntimePropertyInfo_StaticGetter_1<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
impl<
    R: quest_hook::libil2cpp::Type,
> crate::System::Reflection::RuntimePropertyInfo_StaticGetter_1<R> {
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<R>
    where
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: R = __cordl_object.invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        R: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+RuntimePropertyInfo+StaticGetter_1")]
impl<R: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::RuntimePropertyInfo_StaticGetter_1<R> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}