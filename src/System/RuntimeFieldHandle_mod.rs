#[cfg(feature = "System+RuntimeFieldHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RuntimeFieldHandle {
    pub value: crate::System::IntPtr,
}
#[cfg(feature = "System+RuntimeFieldHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::RuntimeFieldHandle => "System"
    ."RuntimeFieldHandle"
);
#[cfg(feature = "System+RuntimeFieldHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::RuntimeFieldHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+RuntimeFieldHandle")]
impl crate::System::RuntimeFieldHandle {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValue(
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeFieldInfo>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fieldType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        fieldAttr: crate::System::Reflection::FieldAttributes,
        declaringType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        domainInitialized: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetValue",
                (
                    field,
                    obj,
                    value,
                    fieldType,
                    fieldAttr,
                    declaringType,
                    domainInitialized,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueDirect(
        field: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeFieldInfo>,
        fieldType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        pTypedRef: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contextType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetValueDirect",
                (field, fieldType, pTypedRef, value, contextType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueInternal(
        fi: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetValueInternal", (fi, obj, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr0(
        &mut self,
        v: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+RuntimeFieldHandle")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ISerializable>,
> for crate::System::RuntimeFieldHandle {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISerializable,
    > {
        todo!()
    }
}
#[cfg(feature = "System+RuntimeFieldHandle")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::System::Runtime::Serialization::ISerializable>,
> for crate::System::RuntimeFieldHandle {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISerializable,
    > {
        todo!()
    }
}
