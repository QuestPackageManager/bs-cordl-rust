#[cfg(feature = "OVRGLTFAccessor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRGLTFAccessor {
    __cordl_parent: crate::System::Object,
    pub byteOffset: i32,
    pub byteLength: i32,
    pub byteStride: i32,
    pub bufferId: i32,
    pub bufferLength: i32,
    pub additionalOffset: i32,
    pub dataType: OVRGLTFType,
    pub componentType: OVRGLTFComponentType,
    pub dataCount: i32,
}
#[cfg(feature = "OVRGLTFAccessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRGLTFAccessor => ""."OVRGLTFAccessor"
);
#[cfg(feature = "OVRGLTFAccessor")]
impl std::ops::Deref for OVRGLTFAccessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl std::ops::DerefMut for OVRGLTFAccessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl OVRGLTFAccessor {
    pub fn GetDataCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDataCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxValueForType(
        &mut self,
        _cordl_type: OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetMaxValueForType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetStrideForType(
        &mut self,
        _cordl_type: OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetStrideForType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::OVRSimpleJSON::JSONNode,
        root: *mut crate::OVRSimpleJSON::JSONNode,
        bufferViewOnly: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node, root, bufferViewOnly))?;
        Ok(__cordl_object)
    }
    pub fn ReadAsBoneWeights(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsBoneWeights", (chunk, data, offset))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsColor(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsColor", (chunk, data, offset))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsFloat(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsFloat", (chunk, data, offset))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsInt(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsInt", (chunk, data, offset))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsMatrix4x4(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        offset: i32,
        conversionScale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsMatrix4x4", (chunk, data, offset, conversionScale))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsTexture(
        &mut self,
        chunk: OVRBinaryChunk,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReadAsTexture", (chunk))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsVector2(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsVector2", (chunk, data, offset))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsVector3(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        offset: i32,
        conversionScale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsVector3", (chunk, data, offset, conversionScale))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsVector4(
        &mut self,
        chunk: OVRBinaryChunk,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        offset: i32,
        conversionScale: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAsVector4", (chunk, data, offset, conversionScale))?;
        Ok(__cordl_ret)
    }
    pub fn ReadElementAsFloat(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ReadElementAsFloat", (data, index))?;
        Ok(__cordl_ret)
    }
    pub fn ReadElementAsUint(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        _cordl_type: OVRGLTFComponentType,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("ReadElementAsUint", (data, index, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::OVRSimpleJSON::JSONNode,
        root: *mut crate::OVRSimpleJSON::JSONNode,
        bufferViewOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, root, bufferViewOnly))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRGLTFAccessor")]
impl quest_hook::libil2cpp::ObjectType for OVRGLTFAccessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}