#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_HAPIConstants {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_HAPIConstants =>
    "HoudiniEngineUnity"."HEU_HAPIConstants"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_HAPIConstants {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_HAPIConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
impl crate::HoudiniEngineUnity::HEU_HAPIConstants {
    pub const HAPI_ATTRIB_COLOR: &'static str = "Cd";
    pub const HAPI_ATTRIB_INSTANCE: &'static str = "instance";
    pub const HAPI_ATTRIB_NAME: &'static str = "name";
    pub const HAPI_ATTRIB_NORMAL: &'static str = "N";
    pub const HAPI_ATTRIB_POSITION: &'static str = "P";
    pub const HAPI_ATTRIB_TANGENT: &'static str = "tangentu";
    pub const HAPI_ATTRIB_TANGENT2: &'static str = "tangentv";
    pub const HAPI_ATTRIB_UV: &'static str = "uv";
    pub const HAPI_ATTRIB_UV2: &'static str = "uv2";
    pub const HAPI_BMP_FORMAT_NAME: &'static str = "Bitmap";
    pub const HAPI_CACHE_COP_COOK: &'static str = "COP Cook Cache";
    pub const HAPI_CACHE_COP_FLIPBOOK: &'static str = "COP Flipbook Cache";
    pub const HAPI_CACHE_GL_TEXTURE: &'static str = "OpenGL Texture Cache";
    pub const HAPI_CACHE_GL_VERTEX: &'static str = "OpenGL Vertex Cache";
    pub const HAPI_CACHE_IMAGE: &'static str = "Image Cache";
    pub const HAPI_CACHE_OBJ: &'static str = "Object Transform Cache";
    pub const HAPI_CACHE_SOP: &'static str = "SOP Cache";
    pub const HAPI_CACHE_VEX: &'static str = "VEX File Cache";
    pub const HAPI_COLOR_VECTOR_SIZE: i32 = 4i32;
    pub const HAPI_CV_VECTOR_SIZE: i32 = 4i32;
    pub const HAPI_DEFAULT_IMAGE_FORMAT_NAME: &'static str = "PNG";
    pub const HAPI_ENV_CLIENT_NAME: &'static str = "HAPI_CLIENT_NAME";
    pub const HAPI_ENV_HIP: &'static str = "HIP";
    pub const HAPI_ENV_JOB: &'static str = "JOB";
    pub const HAPI_EULER_VECTOR_SIZE: i32 = 3i32;
    pub const HAPI_GLOBAL_NODES_NODE_NAME: &'static str = "GlobalNodes";
    pub const HAPI_INVALID_PARM_ID: i32 = -1i32;
    pub const HAPI_JPEG_FORMAT_NAME: &'static str = "JPEG";
    pub const HAPI_NORMAL_VECTOR_SIZE: i32 = 3i32;
    pub const HAPI_PNG_FORMAT_NAME: &'static str = "PNG";
    pub const HAPI_POSITION_VECTOR_SIZE: i32 = 3i32;
    pub const HAPI_PRIM_MAX_VERTEX_COUNT: i32 = 16i32;
    pub const HAPI_PRIM_MIN_VERTEX_COUNT: i32 = 1i32;
    pub const HAPI_QUATERNION_VECTOR_SIZE: i32 = 4i32;
    pub const HAPI_RAW_FORMAT_NAME: &'static str = "HAPI_RAW";
    pub const HAPI_SCALE_VECTOR_SIZE: i32 = 3i32;
    pub const HAPI_SHEAR_VECTOR_SIZE: i32 = 3i32;
    pub const HAPI_TGA_FORMAT_NAME: &'static str = "Targa";
    pub const HAPI_TIFF_FORMAT_NAME: &'static str = "TIFF";
    pub const HAPI_UNGROUPED_GROUP_NAME: &'static str = "__ungrouped_group";
    pub const HAPI_UV_VECTOR_SIZE: i32 = 2i32;
}
#[cfg(feature = "HoudiniEngineUnity+HEU_HAPIConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_HAPIConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
