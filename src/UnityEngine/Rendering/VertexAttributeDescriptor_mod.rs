#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VertexAttributeDescriptor {
    pub _attribute_k__BackingField: crate::UnityEngine::Rendering::VertexAttribute,
    pub _format_k__BackingField: crate::UnityEngine::Rendering::VertexAttributeFormat,
    pub _dimension_k__BackingField: i32,
    pub _stream_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "VertexAttributeDescriptor";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
impl crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_VertexAttributeDescriptor1(
        &mut self,
        other: crate::UnityEngine::Rendering::VertexAttributeDescriptor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        attribute: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dimension: i32,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (attribute, format, dimension, stream),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_attribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::VertexAttribute> {
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttribute = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_attribute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dimension(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dimension",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::VertexAttributeFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttributeFormat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stream(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stream",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_attribute(
        &mut self,
        value: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_attribute",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dimension(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dimension",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_format(
        &mut self,
        value: crate::UnityEngine::Rendering::VertexAttributeFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_format",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stream(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_stream",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::Rendering::VertexAttributeDescriptor>,
> for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::VertexAttributeDescriptor,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::Rendering::VertexAttributeDescriptor>,
> for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::VertexAttributeDescriptor,
    > {
        todo!()
    }
}
