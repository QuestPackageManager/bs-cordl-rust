#[cfg(feature = "System+Delegate")]
#[repr(C)]
#[derive(Debug)]
pub struct Delegate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub method_ptr: crate::System::IntPtr,
    pub invoke_impl: crate::System::IntPtr,
    pub m_target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub method: crate::System::IntPtr,
    pub delegate_trampoline: crate::System::IntPtr,
    pub extra_arg: crate::System::IntPtr,
    pub method_code: crate::System::IntPtr,
    pub interp_method: crate::System::IntPtr,
    pub interp_invoke_impl: crate::System::IntPtr,
    pub method_info: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub original_method_info: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::MethodInfo,
    >,
    pub data: quest_hook::libil2cpp::Gc<crate::System::DelegateData>,
    pub method_is_virtual: bool,
}
#[cfg(feature = "System+Delegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Delegate => "System"."Delegate"
);
#[cfg(feature = "System+Delegate")]
impl std::ops::Deref for crate::System::Delegate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Delegate")]
impl std::ops::DerefMut for crate::System::Delegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Delegate")]
impl crate::System::Delegate {
    pub fn AllocDelegateLike_internal(
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocDelegateLike_internal", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineImpl(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = __cordl_object
            .invoke("CombineImpl", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_Delegate_Delegate0(
        a: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        b: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_Il2CppArray1(
        delegates: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (delegates))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppObject_Il2CppString4(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, target, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppObject_Il2CppString__cordl_bool8(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, target, method, ignoreCase))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppObject_Il2CppString__cordl_bool__cordl_bool7(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
        throwOnBindFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDelegate",
                (_cordl_type, target, method, ignoreCase, throwOnBindFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppObject_MethodInfo1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        firstArgument: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, firstArgument, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Il2CppObject_MethodInfo__cordl_bool__cordl_bool0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        firstArgument: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        throwOnBindFailure: bool,
        allowClosed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDelegate",
                (_cordl_type, firstArgument, method, throwOnBindFailure, allowClosed),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_MethodInfo3(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_MethodInfo__cordl_bool2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        throwOnBindFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, method, throwOnBindFailure))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Type_Il2CppString6(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDelegate", (_cordl_type, target, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_Type_Il2CppString__cordl_bool__cordl_bool5(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
        throwOnBindFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDelegate",
                (_cordl_type, target, method, ignoreCase, throwOnBindFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDelegate_internal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        info: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        throwOnBindFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateDelegate_internal",
                (_cordl_type, target, info, throwOnBindFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DynamicInvoke(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("DynamicInvoke", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn DynamicInvokeImpl(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("DynamicInvokeImpl", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCandidateMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        target: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bflags: crate::System::Reflection::BindingFlags,
        ignoreCase: bool,
        throwOnBindFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCandidateMethod",
                (_cordl_type, target, method, bflags, ignoreCase, throwOnBindFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvocationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Delegate>,
            >,
        > = __cordl_object.invoke("GetInvocationList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("GetMethodImpl", ())?;
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
    pub fn GetVirtualMethod_internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("GetVirtualMethod_internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeDelegateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeDelegateData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        source: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        value: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Remove", (source, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveImpl(
        &mut self,
        d: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Delegate> = __cordl_object
            .invoke("RemoveImpl", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn arg_type_match(
        delArgType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("arg_type_match", (delArgType, argType))?;
        Ok(__cordl_ret.into())
    }
    pub fn arg_type_match_this(
        delArgType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        argType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        boxedThis: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("arg_type_match_this", (delArgType, argType, boxedThis))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = __cordl_object.invoke("get_Method", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Target(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Target", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        d1: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        d2: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        d1: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        d2: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn return_type_match(
        delReturnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        returnType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("return_type_match", (delReturnType, returnType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Delegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Delegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Delegate")]
impl AsRef<crate::System::ICloneable> for crate::System::Delegate {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Delegate")]
impl AsMut<crate::System::ICloneable> for crate::System::Delegate {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Delegate")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Delegate {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Delegate")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Delegate {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
