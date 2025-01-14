#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Defines {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_Defines {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_Defines";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Defines {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Defines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
impl crate::HoudiniEngineUnity::HEU_Defines {
    pub const COOK_LOGS_FILE: &'static str = "cook_logs_file.txt";
    pub const CURVE_CLOSE_PARAM: &'static str = "close";
    pub const CURVE_COORDS_PARAM: &'static str = "coords";
    pub const CURVE_METHOD_PARAM: &'static str = "method";
    pub const CURVE_REVERSE_PARAM: &'static str = "reverse";
    pub const CURVE_TYPE_PARAM: &'static str = "type";
    pub const DEFAULT_COLLISION_GEO: &'static str = "collision_geo";
    pub const DEFAULT_COLLISION_TRIGGER: &'static str = "trigger";
    pub const DEFAULT_CONVEX_COLLISION_GEO: &'static str = "convex";
    pub const DEFAULT_CURVE_SHADER: &'static str = "LineShader";
    pub const DEFAULT_CURVE_SHADER_HDRP: &'static str = "HDRP/Color";
    pub const DEFAULT_CURVE_SHADER_URP: &'static str = "URP/Color";
    pub const DEFAULT_INSTANCE_PREFIX_ATTR: &'static str = "instance_prefix";
    pub const DEFAULT_MATERIAL: &'static str = "HEU_DEFAULT_MATERIAL";
    pub const DEFAULT_RENDERED_COLLISION_GEO: &'static str = "rendered_collision_geo";
    pub const DEFAULT_RENDERED_CONVEX_COLLISION_GEO: &'static str = "rendered_convex_collision_geo";
    pub const DEFAULT_SIMPLE_COLLISION_GEO: &'static str = "collision_geo_simple";
    pub const DEFAULT_SIMPLE_RENDERED_COLLISION_GEO: &'static str = "rendered_collision_geo_simple";
    pub const DEFAULT_SIMPLE_RENDERED_CONVEX_COLLISION_GEO: &'static str = "rendered_convex_collision_geo_simple";
    pub const DEFAULT_STANDARD_SHADER: &'static str = "HoudiniStandard";
    pub const DEFAULT_STANDARD_SHADER_HDRP: &'static str = "HDRP/StandardLit";
    pub const DEFAULT_STANDARD_SHADER_HDRP_SPECULAR: &'static str = "HDRP/SpecularLit";
    pub const DEFAULT_STANDARD_SHADER_SPECULAR: &'static str = "HoudiniSpecular";
    pub const DEFAULT_STANDARD_SHADER_SPECULAR_LEGACY: &'static str = "Legacy/SpecularVertexColor";
    pub const DEFAULT_STANDARD_SHADER_URP: &'static str = "URP/StandardLit";
    pub const DEFAULT_STANDARD_SHADER_URP_SPECULAR: &'static str = "URP/SpecularLit";
    pub const DEFAULT_TERRAIN_MATERIAL_PATH: &'static str = "Resources/unity_builtin_extra::name::Default-Terrain-Standard";
    pub const DEFAULT_TERRAIN_MATERIAL_PATH_HDRP: &'static str = "Packages/com.unity.render-pipelines.high-definition/Runtime/RenderPipelineResources/Material/DefaultHDTerrainMaterial.mat";
    pub const DEFAULT_TERRAIN_MATERIAL_PATH_URP: &'static str = "Packages/com.unity.render-pipelines.universal/Runtime/Materials/TerrainLit.mat";
    pub const DEFAULT_TERRAIN_SHADER: &'static str = "Nature/Terrain/Standard";
    pub const DEFAULT_TERRAIN_SHADER_HDRP: &'static str = "HDRP/TerrainLit";
    pub const DEFAULT_TERRAIN_SHADER_URP: &'static str = "Universal Render Pipeline/Terrain/Lit";
    pub const DEFAULT_TRANSPARENT_SHADER: &'static str = "HoudiniStandardAlpha";
    pub const DEFAULT_TRANSPARENT_SHADER_HDRP: &'static str = "HDRP/StandardLitAlpha";
    pub const DEFAULT_TRANSPARENT_SHADER_HDRP_SPECULAR: &'static str = "HDRP/SpecularLitAlpha";
    pub const DEFAULT_TRANSPARENT_SHADER_SPECULAR: &'static str = "HoudiniSpecularAlpha";
    pub const DEFAULT_TRANSPARENT_SHADER_SPECULAR_LEGACY: &'static str = "Legacy/AlphaSpecularVertexColor";
    pub const DEFAULT_TRANSPARENT_SHADER_URP: &'static str = "URP/StandardLitAlpha";
    pub const DEFAULT_TRANSPARENT_SHADER_URP_SPECULAR: &'static str = "URP/SpecularLitAlpha";
    pub const DEFAULT_UNITY_BUILTIN_RESOURCES: &'static str = "Resources/unity_builtin_extra";
    pub const DEFAULT_UNITY_HEIGHTFIELD_HEIGHT_RANGE: &'static str = "unity_hf_height_range";
    pub const DEFAULT_UNITY_HEIGHTFIELD_METALLIC_ATTR: &'static str = "unity_hf_metallic";
    pub const DEFAULT_UNITY_HEIGHTFIELD_NORMAL_SCALE_ATTR: &'static str = "unity_hf_normal_scale";
    pub const DEFAULT_UNITY_HEIGHTFIELD_SMOOTHNESS_ATTR: &'static str = "unity_hf_smoothness";
    pub const DEFAULT_UNITY_HEIGHTFIELD_SPECULAR_ATTR: &'static str = "unity_hf_specular";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TERRAINDATA_EXPORT_FILE_ATTR: &'static str = "unity_hf_terraindata_export_file";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TERRAINDATA_EXPORT_PATH: &'static str = "unity_hf_terraindata_export_path";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TERRAINDATA_FILE_ATTR: &'static str = "unity_hf_terraindata_file";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TERRAINLAYER_FILE_ATTR: &'static str = "unity_hf_terrainlayer_file";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TEXTURE_DIFFUSE_ATTR: &'static str = "unity_hf_texture_diffuse";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TEXTURE_MASK_ATTR: &'static str = "unity_hf_texture_mask";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TEXTURE_NORMAL_ATTR: &'static str = "unity_hf_texture_normal";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TILE_OFFSET_ATTR: &'static str = "unity_hf_tile_offset";
    pub const DEFAULT_UNITY_HEIGHTFIELD_TILE_SIZE_ATTR: &'static str = "unity_hf_tile_size";
    pub const DEFAULT_UNITY_HEIGHTFIELD_YPOS: &'static str = "unity_hf_ypos";
    pub const DEFAULT_UNITY_INPUT_MESH_ATTR: &'static str = "unity_input_mesh_name";
    pub const DEFAULT_UNITY_INSTANCE_ATTR: &'static str = "unity_instance";
    pub const DEFAULT_UNITY_LAYER_ATTR: &'static str = "unity_layer";
    pub const DEFAULT_UNITY_MATERIAL_ATTR: &'static str = "unity_material";
    pub const DEFAULT_UNITY_MESH_READABLE: &'static str = "unity_mesh_readable";
    pub const DEFAULT_UNITY_SCRIPT_ATTR: &'static str = "unity_script";
    pub const DEFAULT_UNITY_STATIC_ATTR: &'static str = "unity_static";
    pub const DEFAULT_UNITY_SUBMATERIAL_INDEX_ATTR: &'static str = "unity_sub_material_index";
    pub const DEFAULT_UNITY_SUBMATERIAL_NAME_ATTR: &'static str = "unity_sub_material_name";
    pub const DEFAULT_UNITY_TAG_ATTR: &'static str = "unity_tag";
    pub const DEFAULT_VERTEXCOLOR_SHADER: &'static str = "HoudiniStandard";
    pub const DEFAULT_VERTEXCOLOR_SHADER_HDRP: &'static str = "HDRP/StandardLit";
    pub const DEFAULT_VERTEXCOLOR_SHADER_HDRP_SPECULAR: &'static str = "HDRP/SpecularLit";
    pub const DEFAULT_VERTEXCOLOR_SHADER_SPECULAR: &'static str = "HoudiniSpecular";
    pub const DEFAULT_VERTEXCOLOR_SHADER_SPECULAR_LEGACY: &'static str = "Legacy/SpecularVertexColor";
    pub const DEFAULT_VERTEXCOLOR_SHADER_URP: &'static str = "URP/StandardLit";
    pub const DEFAULT_VERTEXCOLOR_SHADER_URP_SPECULAR: &'static str = "URP/SpecularLit";
    pub const EDITABLE_MATERIAL: &'static str = "HEU_EDITABLE_MATERIAL";
    pub const HAPI_ATTRIB_ALPHA: &'static str = "Alpha";
    pub const HAPI_ATTRIB_ORIENT: &'static str = "orient";
    pub const HAPI_ATTRIB_ROTATION: &'static str = "rot";
    pub const HAPI_ATTRIB_SCALE: &'static str = "scale";
    pub const HAPI_CURVE_LOD: f32 = 8f32;
    pub const HAPI_CURVE_REFINE_TO_LINEAR: bool = true;
    pub const HAPI_HANDLE_TRANSFORM: &'static str = "xform";
    pub const HAPI_HEIGHTFIELD_LAYERNAME_HEIGHT: &'static str = "height";
    pub const HAPI_HEIGHTFIELD_LAYERNAME_MASK: &'static str = "mask";
    pub const HAPI_HEIGHTFIELD_TILE_ATTR: &'static str = "tile";
    pub const HAPI_MAX_PAGE_SIZE: i32 = 20000i32;
    pub const HAPI_MAX_UVS: i32 = 8i32;
    pub const HAPI_MAX_VERTICES_PER_FACE: i32 = 3i32;
    pub const HAPI_OBJMERGE_PACK_GEOMETRY: &'static str = "pack";
    pub const HAPI_OBJMERGE_TRANSFORM_PARAM: &'static str = "xformtype";
    pub const HAPI_PATH: &'static str = "HAPI_PATH";
    pub const HAPI_SEC_BEFORE_PROGRESS_BAR_SHOW: i32 = 3i32;
    pub const HAPI_VOLUME_POSITION_MULT: f32 = 2f32;
    pub const HAPI_VOLUME_SURFACE_DELTA_MULT: f32 = 1.2f32;
    pub const HAPI_VOLUME_SURFACE_MAX_PT_PER_C: f32 = 64000f32;
    pub const HAPI_VOLUME_SURFACE_PT_SIZE_MULT: f32 = 1800f32;
    pub const HEIGHTFIELD_DETAIL_DENSITY: &'static str = "unity_hf_detail_density";
    pub const HEIGHTFIELD_DETAIL_DISTANCE: &'static str = "unity_hf_detail_distance";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_BENDFACTOR: &'static str = "unity_hf_detail_prototype_bendfactor";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_DRYCOLOR: &'static str = "unity_hf_detail_prototype_drycolor";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_HEALTHYCOLOR: &'static str = "unity_hf_detail_prototype_healthycolor";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_MAXHEIGHT: &'static str = "unity_hf_detail_prototype_maxheight";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_MAXWIDTH: &'static str = "unity_hf_detail_prototype_maxwidth";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_MINHEIGHT: &'static str = "unity_hf_detail_prototype_minheight";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_MINWIDTH: &'static str = "unity_hf_detail_prototype_minwidth";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_NOISESPREAD: &'static str = "unity_hf_detail_prototype_noisespread";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_PREFAB: &'static str = "unity_hf_detail_prototype_prefab";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_RENDERMODE: &'static str = "unity_hf_detail_prototype_rendermode";
    pub const HEIGHTFIELD_DETAIL_PROTOTYPE_TEXTURE: &'static str = "unity_hf_detail_prototype_texture";
    pub const HEIGHTFIELD_DETAIL_RESOLUTION_PER_PATCH: &'static str = "unity_hf_detail_resolution_patch";
    pub const HEIGHTFIELD_LAYER_ATTR_TYPE: &'static str = "unity_hf_layer_type";
    pub const HEIGHTFIELD_LAYER_TYPE_DETAIL: &'static str = "detail";
    pub const HEIGHTFIELD_TREEINSTANCE_HEIGHTSCALE: &'static str = "unity_hf_treeinstance_heightscale";
    pub const HEIGHTFIELD_TREEINSTANCE_LIGHTMAPCOLOR: &'static str = "unity_hf_treeinstance_lightmapcolor";
    pub const HEIGHTFIELD_TREEINSTANCE_PROTOTYPEINDEX: &'static str = "unity_hf_treeinstance_prototypeindex";
    pub const HEIGHTFIELD_TREEINSTANCE_WIDTHSCALE: &'static str = "unity_hf_treeinstance_widthscale";
    pub const HEIGHTFIELD_TREEPROTOTYPE: &'static str = "unity_hf_tree_prototype";
    pub const HEIGHTFIELD_UNITY_TILE: &'static str = "unity_hf_tile";
    pub const HENGINE_STORE_ATTR: &'static str = "hengine_attr_store";
    pub const HEU_ASSET_CACHE_PATH: &'static str = "HoudiniEngineAssetCache";
    pub const HEU_BAKED_CLONE: &'static str = "_bakedClone";
    pub const HEU_BAKED_HDA: &'static str = "_bakedHDA";
    pub const HEU_BAKED_PATH: &'static str = "Baked";
    pub const HEU_DEFAULT_ASSET_NAME: &'static str = "HoudiniAssetRoot";
    pub const HEU_DEFAULT_GEO_GROUP_NAME: &'static str = "main_geo";
    pub const HEU_DEFAULT_LOD_NAME: &'static str = "lod";
    pub const HEU_ENVPATH_KEY: &'static str = "$";
    pub const HEU_ENVPATH_PREFIX: &'static str = "HEU_ENVPATH_";
    pub const HEU_ERROR_TITLE: &'static str = "Houdini Engine Error";
    pub const HEU_EXT_ASSET: &'static str = ".asset";
    pub const HEU_EXT_MAT: &'static str = ".mat";
    pub const HEU_EXT_TERRAINDATA: &'static str = ".terraindata";
    pub const HEU_EXT_TERRAINLAYER: &'static str = ".terrainlayer";
    pub const HEU_FOLDER_MATERIALS: &'static str = "Materials";
    pub const HEU_FOLDER_MESHES: &'static str = "Meshes";
    pub const HEU_FOLDER_TERRAIN: &'static str = "Terrain";
    pub const HEU_FOLDER_TEXTURES: &'static str = "Textures";
    pub const HEU_FOLDER_TILE: &'static str = "Tile";
    pub const HEU_HENGINE_SHIPPED_SHELF: &'static str = "Default";
    pub const HEU_HENGINE_TOOLS_SHIPPED_FOLDER: &'static str = "<HFS>/engine/tools";
    pub const HEU_INSTALL_INFO: &'static str = "Houdini Engine Installation Info";
    pub const HEU_INSTANCE: &'static str = "_Instance";
    pub const HEU_INSTANCE_PATTERN: &'static str = "_Instance\\d*\\z";
    pub const HEU_INVALID_MATERIAL: i32 = -1i32;
    pub const HEU_INVALID_NODE_ID: i32 = -1i32;
    pub const HEU_KEY_CTRL: &'static str = "Ctrl";
    pub const HEU_NAME: &'static str = "Houdini Engine";
    pub const HEU_PATH_KEY_HFS: &'static str = "<HFS>";
    pub const HEU_PATH_KEY_PLUGIN: &'static str = "<PLUGIN_PATH>";
    pub const HEU_PATH_KEY_PROJECT: &'static str = "<PROJECT_PATH>";
    pub const HEU_PATH_KEY_TOOL: &'static str = "HOUDINI_TOOL_PATH";
    pub const HEU_PRODUCT_NAME: &'static str = "HoudiniEngine";
    pub const HEU_SESSION_AUTOCLOSE: bool = true;
    pub const HEU_SESSION_LOCALHOST: &'static str = "localhost";
    pub const HEU_SESSION_PIPENAME: &'static str = "hapi";
    pub const HEU_SESSION_PORT: i32 = 9090i32;
    pub const HEU_SESSION_TIMEOUT: f32 = 10000f32;
    pub const HEU_SUBASSET: &'static str = "SUBASSET::";
    pub const HEU_TERRAIN_SPLAT_DEFAULT: &'static str = "Textures/heu_terrain_default_splat";
    pub const HEU_UNITY_LOD_TRANSITION_ATTR: &'static str = "lod_screensizes";
    pub const HEU_USERMSG_NONEDITOR_NOT_SUPPORTED: &'static str = "Houdini Engine does not support non-Editor asset creation at this time!";
    pub const HEU_WORKING_PATH: &'static str = "Working";
    pub const HOUDINI_SHADER_PREFIX: &'static str = "Houdini/";
    pub const MAT_ALPHA_ATTR: &'static str = "opac";
    pub const MAT_BASECOLOR_ATTR: &'static str = "basecolor_texture";
    pub const MAT_BASECOLOR_ATTR_ENABLED: &'static str = "basecolor_useTexture";
    pub const MAT_DIFF_ATTR: &'static str = "basecolor";
    pub const MAT_EMISSIVE_ATTR: &'static str = "emitcolor";
    pub const MAT_EMISSIVE_MAP_ATTR: &'static str = "emitcolor_texture";
    pub const MAT_EMISSIVE_MAP_ATTR_ENABLED: &'static str = "emitcolor_useTexture";
    pub const MAT_MAP_ATTR: &'static str = "map";
    pub const MAT_METALLIC_ATTR: &'static str = "metallic";
    pub const MAT_METALLIC_MAP_ATTR: &'static str = "metallic_texture";
    pub const MAT_METALLIC_MAP_ATTR_ENABLED: &'static str = "metallic_useTexture";
    pub const MAT_NORMAL_ATTR: &'static str = "baseNormal_texture";
    pub const MAT_NORMAL_ATTR_ENABLED: &'static str = "baseBumpAndNormal_enable";
    pub const MAT_OGL_ALPHA_ATTR: &'static str = "ogl_alpha";
    pub const MAT_OGL_DIFF_ATTR: &'static str = "ogl_diff";
    pub const MAT_OGL_EMISSIVE_ATTR: &'static str = "ogl_emit";
    pub const MAT_OGL_EMISSIVE_MAP_ATTR: &'static str = "ogl_emissionmap";
    pub const MAT_OGL_EMISSIVE_MAP_ATTR_ENABLED: &'static str = "ogl_use_emissionmap";
    pub const MAT_OGL_METALLIC_ATTR: &'static str = "ogl_metallic";
    pub const MAT_OGL_METALLIC_MAP_ATTR: &'static str = "ogl_metallicmap";
    pub const MAT_OGL_METALLIC_MAP_ATTR_ENABLED: &'static str = "ogl_use_metallicmap";
    pub const MAT_OGL_NORMAL_ATTR: &'static str = "ogl_normalmap";
    pub const MAT_OGL_OCCLUSION_MAP_ATTR: &'static str = "ogl_occlusionmap";
    pub const MAT_OGL_OCCLUSION_MAP_ATTR_ENABLED: &'static str = "ogl_use_occlusionmap";
    pub const MAT_OGL_OPACITY_MAP_ATTR: &'static str = "ogl_opacitymap";
    pub const MAT_OGL_OPACITY_MAP_ATTR_ENABLED: &'static str = "ogl_use_opacitymap";
    pub const MAT_OGL_ROUGH_ATTR: &'static str = "ogl_rough";
    pub const MAT_OGL_ROUGH_MAP_ATTR: &'static str = "ogl_roughmap";
    pub const MAT_OGL_ROUGH_MAP_ATTR_ENABLED: &'static str = "ogl_use_roughmap";
    pub const MAT_OGL_SPEC_ATTR: &'static str = "ogl_spec";
    pub const MAT_OGL_SPEC_MAP_ATTR: &'static str = "ogl_specmap";
    pub const MAT_OGL_SPEC_MAP_ATTR_ENABLED: &'static str = "ogl_use_specmap";
    pub const MAT_OGL_TEX1_ATTR: &'static str = "ogl_tex1";
    pub const MAT_OGL_TEX1_ATTR_ENABLED: &'static str = "ogl_use_tex1";
    pub const MAT_OGL_TRANSPARENCY_ATTR: &'static str = "ogl_transparency";
    pub const MAT_OGL_TRANSPARENCY_ATTR_ENABLED: &'static str = "ogl_use_alpha_transparency";
    pub const MAT_OPACITY_MAP_ATTR: &'static str = "opaccolor_texture";
    pub const MAT_OPACITY_MAP_ATTR_ENABLED: &'static str = "opaccolor_useTexture";
    pub const MAT_ROUGH_ATTR: &'static str = "rough";
    pub const MAT_ROUGH_MAP_ATTR: &'static str = "rough_texture";
    pub const MAT_ROUGH_MAP_ATTR_ENABLED: &'static str = "rough_useTexture";
    pub const MAT_SPEC_ATTR: &'static str = "reflect";
    pub const MAT_SPEC_MAP_ATTR: &'static str = "reflect_texture";
    pub const MAT_SPEC_MAP_ATTR_ENABLED: &'static str = "reflect_useTexture";
    pub const NO_EXISTING_SESSION: &'static str = "No existing session.";
    pub const PLUGIN_SESSION_DATA: &'static str = "HoudiniEngineSession";
    pub const PLUGIN_SESSION_FILE: &'static str = "heu_session.txt";
    pub const PLUGIN_SETTINGS_FILE: &'static str = "heu_settings.ini";
    pub const PLUGIN_STORE_DATA: &'static str = "HoudiniEnginePluginData";
    pub const PLUGIN_STORE_KEYS: &'static str = "HoudiniEnginePluginKeys";
    pub const UNITY_EDITORONLY_TAG: &'static str = "EditorOnly";
    pub const UNITY_HDADATA_NAME: &'static str = "HDA_Data";
    pub const UNITY_SHADER_BUMP_MAP: &'static str = "_BumpMap";
    pub const UNITY_SHADER_COLOR: &'static str = "_Color";
    pub const UNITY_SHADER_EMISSION_COLOR: &'static str = "_EmissionColor";
    pub const UNITY_SHADER_EMISSION_MAP: &'static str = "_EmissionMap";
    pub const UNITY_SHADER_METALLIC: &'static str = "_Metallic";
    pub const UNITY_SHADER_METALLIC_MAP: &'static str = "_MetallicMap";
    pub const UNITY_SHADER_OCCLUSION: &'static str = "_Occlusion";
    pub const UNITY_SHADER_OCCLUSION_MAP: &'static str = "_OcclusionMap";
    pub const UNITY_SHADER_OPACITY: &'static str = "_Opacity";
    pub const UNITY_SHADER_OPACITY_MAP: &'static str = "_OpacityMap";
    pub const UNITY_SHADER_SHININESS: &'static str = "_Shininess";
    pub const UNITY_SHADER_SMOOTHNESS: &'static str = "_Smoothness";
    pub const UNITY_SHADER_SMOOTHNESS_MAP: &'static str = "_SmoothnessMap";
    pub const UNITY_SHADER_SPEC_COLOR: &'static str = "_SpecColor";
    pub const UNITY_SHADER_SPEC_MAP: &'static str = "_SpecMap";
    pub const UNITY_USE_INSTANCE_FLAGS_ATTR: &'static str = "unity_use_instance_flags";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Defines")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Defines {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
