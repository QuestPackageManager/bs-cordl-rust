#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
#[repr(C)]
#[derive(Debug)]
pub struct CipherSuite {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::CipherSuite =>
    "Org.BouncyCastle.Crypto.Tls"."CipherSuite"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::CipherSuite {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::CipherSuite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
impl crate::Org::BouncyCastle::Crypto::Tls::CipherSuite {
    pub const DRAFT_TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256: i32 = 52397i32;
    pub const DRAFT_TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256: i32 = 52394i32;
    pub const DRAFT_TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: i32 = 52393i32;
    pub const DRAFT_TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256: i32 = 52396i32;
    pub const DRAFT_TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: i32 = 52392i32;
    pub const DRAFT_TLS_PSK_WITH_CHACHA20_POLY1305_SHA256: i32 = 52395i32;
    pub const DRAFT_TLS_RSA_PSK_WITH_CHACHA20_POLY1305_SHA256: i32 = 52398i32;
    pub const TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA: i32 = 17i32;
    pub const TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA: i32 = 19i32;
    pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA: i32 = 50i32;
    pub const TLS_DHE_DSS_WITH_AES_128_CBC_SHA256: i32 = 64i32;
    pub const TLS_DHE_DSS_WITH_AES_128_GCM_SHA256: i32 = 162i32;
    pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA: i32 = 56i32;
    pub const TLS_DHE_DSS_WITH_AES_256_CBC_SHA256: i32 = 106i32;
    pub const TLS_DHE_DSS_WITH_AES_256_GCM_SHA384: i32 = 163i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA: i32 = 68i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256: i32 = 189i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49280i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA: i32 = 135i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256: i32 = 195i32;
    pub const TLS_DHE_DSS_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49281i32;
    pub const TLS_DHE_DSS_WITH_DES_CBC_SHA: i32 = 18i32;
    pub const TLS_DHE_DSS_WITH_SEED_CBC_SHA: i32 = 153i32;
    pub const TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 143i32;
    pub const TLS_DHE_PSK_WITH_AES_128_CBC_SHA: i32 = 144i32;
    pub const TLS_DHE_PSK_WITH_AES_128_CBC_SHA256: i32 = 178i32;
    pub const TLS_DHE_PSK_WITH_AES_128_CCM: i32 = 49318i32;
    pub const TLS_DHE_PSK_WITH_AES_128_GCM_SHA256: i32 = 170i32;
    pub const TLS_DHE_PSK_WITH_AES_256_CBC_SHA: i32 = 145i32;
    pub const TLS_DHE_PSK_WITH_AES_256_CBC_SHA384: i32 = 179i32;
    pub const TLS_DHE_PSK_WITH_AES_256_CCM: i32 = 49319i32;
    pub const TLS_DHE_PSK_WITH_AES_256_GCM_SHA384: i32 = 171i32;
    pub const TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49302i32;
    pub const TLS_DHE_PSK_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49296i32;
    pub const TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49303i32;
    pub const TLS_DHE_PSK_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49297i32;
    pub const TLS_DHE_PSK_WITH_NULL_SHA: i32 = 45i32;
    pub const TLS_DHE_PSK_WITH_NULL_SHA256: i32 = 180i32;
    pub const TLS_DHE_PSK_WITH_NULL_SHA384: i32 = 181i32;
    pub const TLS_DHE_PSK_WITH_RC4_128_SHA: i32 = 142i32;
    pub const TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA: i32 = 20i32;
    pub const TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 22i32;
    pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA: i32 = 51i32;
    pub const TLS_DHE_RSA_WITH_AES_128_CBC_SHA256: i32 = 103i32;
    pub const TLS_DHE_RSA_WITH_AES_128_CCM: i32 = 49310i32;
    pub const TLS_DHE_RSA_WITH_AES_128_CCM_8: i32 = 49314i32;
    pub const TLS_DHE_RSA_WITH_AES_128_GCM_SHA256: i32 = 158i32;
    pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA: i32 = 57i32;
    pub const TLS_DHE_RSA_WITH_AES_256_CBC_SHA256: i32 = 107i32;
    pub const TLS_DHE_RSA_WITH_AES_256_CCM: i32 = 49311i32;
    pub const TLS_DHE_RSA_WITH_AES_256_CCM_8: i32 = 49315i32;
    pub const TLS_DHE_RSA_WITH_AES_256_GCM_SHA384: i32 = 159i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 69i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 190i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49276i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 136i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 196i32;
    pub const TLS_DHE_RSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49277i32;
    pub const TLS_DHE_RSA_WITH_DES_CBC_SHA: i32 = 21i32;
    pub const TLS_DHE_RSA_WITH_SEED_CBC_SHA: i32 = 154i32;
    pub const TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA: i32 = 11i32;
    pub const TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA: i32 = 13i32;
    pub const TLS_DH_DSS_WITH_AES_128_CBC_SHA: i32 = 48i32;
    pub const TLS_DH_DSS_WITH_AES_128_CBC_SHA256: i32 = 62i32;
    pub const TLS_DH_DSS_WITH_AES_128_GCM_SHA256: i32 = 164i32;
    pub const TLS_DH_DSS_WITH_AES_256_CBC_SHA: i32 = 54i32;
    pub const TLS_DH_DSS_WITH_AES_256_CBC_SHA256: i32 = 104i32;
    pub const TLS_DH_DSS_WITH_AES_256_GCM_SHA384: i32 = 165i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA: i32 = 66i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256: i32 = 187i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49282i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA: i32 = 133i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256: i32 = 193i32;
    pub const TLS_DH_DSS_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49283i32;
    pub const TLS_DH_DSS_WITH_DES_CBC_SHA: i32 = 12i32;
    pub const TLS_DH_DSS_WITH_SEED_CBC_SHA: i32 = 151i32;
    pub const TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA: i32 = 14i32;
    pub const TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 16i32;
    pub const TLS_DH_RSA_WITH_AES_128_CBC_SHA: i32 = 49i32;
    pub const TLS_DH_RSA_WITH_AES_128_CBC_SHA256: i32 = 63i32;
    pub const TLS_DH_RSA_WITH_AES_128_GCM_SHA256: i32 = 160i32;
    pub const TLS_DH_RSA_WITH_AES_256_CBC_SHA: i32 = 55i32;
    pub const TLS_DH_RSA_WITH_AES_256_CBC_SHA256: i32 = 105i32;
    pub const TLS_DH_RSA_WITH_AES_256_GCM_SHA384: i32 = 161i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 67i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 188i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49278i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 134i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 194i32;
    pub const TLS_DH_RSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49279i32;
    pub const TLS_DH_RSA_WITH_DES_CBC_SHA: i32 = 15i32;
    pub const TLS_DH_RSA_WITH_SEED_CBC_SHA: i32 = 152i32;
    pub const TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA: i32 = 25i32;
    pub const TLS_DH_anon_EXPORT_WITH_RC4_40_MD5: i32 = 23i32;
    pub const TLS_DH_anon_WITH_3DES_EDE_CBC_SHA: i32 = 27i32;
    pub const TLS_DH_anon_WITH_AES_128_CBC_SHA: i32 = 52i32;
    pub const TLS_DH_anon_WITH_AES_128_CBC_SHA256: i32 = 108i32;
    pub const TLS_DH_anon_WITH_AES_128_GCM_SHA256: i32 = 166i32;
    pub const TLS_DH_anon_WITH_AES_256_CBC_SHA: i32 = 58i32;
    pub const TLS_DH_anon_WITH_AES_256_CBC_SHA256: i32 = 109i32;
    pub const TLS_DH_anon_WITH_AES_256_GCM_SHA384: i32 = 167i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA: i32 = 70i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256: i32 = 191i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49284i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA: i32 = 137i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256: i32 = 197i32;
    pub const TLS_DH_anon_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49285i32;
    pub const TLS_DH_anon_WITH_DES_CBC_SHA: i32 = 26i32;
    pub const TLS_DH_anon_WITH_RC4_128_MD5: i32 = 24i32;
    pub const TLS_DH_anon_WITH_SEED_CBC_SHA: i32 = 155i32;
    pub const TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA: i32 = 49160i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA: i32 = 49161i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256: i32 = 49187i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_CCM: i32 = 49324i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8: i32 = 49326i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: i32 = 49195i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA: i32 = 49162i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384: i32 = 49188i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_CCM: i32 = 49325i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_CCM_8: i32 = 49327i32;
    pub const TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: i32 = 49196i32;
    pub const TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49266i32;
    pub const TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49286i32;
    pub const TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49267i32;
    pub const TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49287i32;
    pub const TLS_ECDHE_ECDSA_WITH_NULL_SHA: i32 = 49158i32;
    pub const TLS_ECDHE_ECDSA_WITH_RC4_128_SHA: i32 = 49159i32;
    pub const TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 49204i32;
    pub const TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA: i32 = 49205i32;
    pub const TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256: i32 = 49207i32;
    pub const TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA: i32 = 49206i32;
    pub const TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384: i32 = 49208i32;
    pub const TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49306i32;
    pub const TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49307i32;
    pub const TLS_ECDHE_PSK_WITH_NULL_SHA: i32 = 49209i32;
    pub const TLS_ECDHE_PSK_WITH_NULL_SHA256: i32 = 49210i32;
    pub const TLS_ECDHE_PSK_WITH_NULL_SHA384: i32 = 49211i32;
    pub const TLS_ECDHE_PSK_WITH_RC4_128_SHA: i32 = 49203i32;
    pub const TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 49170i32;
    pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA: i32 = 49171i32;
    pub const TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256: i32 = 49191i32;
    pub const TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: i32 = 49199i32;
    pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA: i32 = 49172i32;
    pub const TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384: i32 = 49192i32;
    pub const TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: i32 = 49200i32;
    pub const TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49270i32;
    pub const TLS_ECDHE_RSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49290i32;
    pub const TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49271i32;
    pub const TLS_ECDHE_RSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49291i32;
    pub const TLS_ECDHE_RSA_WITH_NULL_SHA: i32 = 49168i32;
    pub const TLS_ECDHE_RSA_WITH_RC4_128_SHA: i32 = 49169i32;
    pub const TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA: i32 = 49155i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA: i32 = 49156i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256: i32 = 49189i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256: i32 = 49197i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA: i32 = 49157i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384: i32 = 49190i32;
    pub const TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384: i32 = 49198i32;
    pub const TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49268i32;
    pub const TLS_ECDH_ECDSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49288i32;
    pub const TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49269i32;
    pub const TLS_ECDH_ECDSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49289i32;
    pub const TLS_ECDH_ECDSA_WITH_NULL_SHA: i32 = 49153i32;
    pub const TLS_ECDH_ECDSA_WITH_RC4_128_SHA: i32 = 49154i32;
    pub const TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 49165i32;
    pub const TLS_ECDH_RSA_WITH_AES_128_CBC_SHA: i32 = 49166i32;
    pub const TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256: i32 = 49193i32;
    pub const TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256: i32 = 49201i32;
    pub const TLS_ECDH_RSA_WITH_AES_256_CBC_SHA: i32 = 49167i32;
    pub const TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384: i32 = 49194i32;
    pub const TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384: i32 = 49202i32;
    pub const TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49272i32;
    pub const TLS_ECDH_RSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49292i32;
    pub const TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49273i32;
    pub const TLS_ECDH_RSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49293i32;
    pub const TLS_ECDH_RSA_WITH_NULL_SHA: i32 = 49163i32;
    pub const TLS_ECDH_RSA_WITH_RC4_128_SHA: i32 = 49164i32;
    pub const TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA: i32 = 49175i32;
    pub const TLS_ECDH_anon_WITH_AES_128_CBC_SHA: i32 = 49176i32;
    pub const TLS_ECDH_anon_WITH_AES_256_CBC_SHA: i32 = 49177i32;
    pub const TLS_ECDH_anon_WITH_NULL_SHA: i32 = 49173i32;
    pub const TLS_ECDH_anon_WITH_RC4_128_SHA: i32 = 49174i32;
    pub const TLS_EMPTY_RENEGOTIATION_INFO_SCSV: i32 = 255i32;
    pub const TLS_FALLBACK_SCSV: i32 = 22016i32;
    pub const TLS_NULL_WITH_NULL_NULL: i32 = 0i32;
    pub const TLS_PSK_DHE_WITH_AES_128_CCM_8: i32 = 49322i32;
    pub const TLS_PSK_DHE_WITH_AES_256_CCM_8: i32 = 49323i32;
    pub const TLS_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 139i32;
    pub const TLS_PSK_WITH_AES_128_CBC_SHA: i32 = 140i32;
    pub const TLS_PSK_WITH_AES_128_CBC_SHA256: i32 = 174i32;
    pub const TLS_PSK_WITH_AES_128_CCM: i32 = 49316i32;
    pub const TLS_PSK_WITH_AES_128_CCM_8: i32 = 49320i32;
    pub const TLS_PSK_WITH_AES_128_GCM_SHA256: i32 = 168i32;
    pub const TLS_PSK_WITH_AES_256_CBC_SHA: i32 = 141i32;
    pub const TLS_PSK_WITH_AES_256_CBC_SHA384: i32 = 175i32;
    pub const TLS_PSK_WITH_AES_256_CCM: i32 = 49317i32;
    pub const TLS_PSK_WITH_AES_256_CCM_8: i32 = 49321i32;
    pub const TLS_PSK_WITH_AES_256_GCM_SHA384: i32 = 169i32;
    pub const TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49300i32;
    pub const TLS_PSK_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49294i32;
    pub const TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49301i32;
    pub const TLS_PSK_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49295i32;
    pub const TLS_PSK_WITH_NULL_SHA: i32 = 44i32;
    pub const TLS_PSK_WITH_NULL_SHA256: i32 = 176i32;
    pub const TLS_PSK_WITH_NULL_SHA384: i32 = 177i32;
    pub const TLS_PSK_WITH_RC4_128_SHA: i32 = 138i32;
    pub const TLS_RSA_EXPORT_WITH_DES40_CBC_SHA: i32 = 8i32;
    pub const TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5: i32 = 6i32;
    pub const TLS_RSA_EXPORT_WITH_RC4_40_MD5: i32 = 3i32;
    pub const TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA: i32 = 147i32;
    pub const TLS_RSA_PSK_WITH_AES_128_CBC_SHA: i32 = 148i32;
    pub const TLS_RSA_PSK_WITH_AES_128_CBC_SHA256: i32 = 182i32;
    pub const TLS_RSA_PSK_WITH_AES_128_GCM_SHA256: i32 = 172i32;
    pub const TLS_RSA_PSK_WITH_AES_256_CBC_SHA: i32 = 149i32;
    pub const TLS_RSA_PSK_WITH_AES_256_CBC_SHA384: i32 = 183i32;
    pub const TLS_RSA_PSK_WITH_AES_256_GCM_SHA384: i32 = 173i32;
    pub const TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256: i32 = 49304i32;
    pub const TLS_RSA_PSK_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49298i32;
    pub const TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384: i32 = 49305i32;
    pub const TLS_RSA_PSK_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49299i32;
    pub const TLS_RSA_PSK_WITH_NULL_SHA: i32 = 46i32;
    pub const TLS_RSA_PSK_WITH_NULL_SHA256: i32 = 184i32;
    pub const TLS_RSA_PSK_WITH_NULL_SHA384: i32 = 185i32;
    pub const TLS_RSA_PSK_WITH_RC4_128_SHA: i32 = 146i32;
    pub const TLS_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 10i32;
    pub const TLS_RSA_WITH_AES_128_CBC_SHA: i32 = 47i32;
    pub const TLS_RSA_WITH_AES_128_CBC_SHA256: i32 = 60i32;
    pub const TLS_RSA_WITH_AES_128_CCM: i32 = 49308i32;
    pub const TLS_RSA_WITH_AES_128_CCM_8: i32 = 49312i32;
    pub const TLS_RSA_WITH_AES_128_GCM_SHA256: i32 = 156i32;
    pub const TLS_RSA_WITH_AES_256_CBC_SHA: i32 = 53i32;
    pub const TLS_RSA_WITH_AES_256_CBC_SHA256: i32 = 61i32;
    pub const TLS_RSA_WITH_AES_256_CCM: i32 = 49309i32;
    pub const TLS_RSA_WITH_AES_256_CCM_8: i32 = 49313i32;
    pub const TLS_RSA_WITH_AES_256_GCM_SHA384: i32 = 157i32;
    pub const TLS_RSA_WITH_CAMELLIA_128_CBC_SHA: i32 = 65i32;
    pub const TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256: i32 = 186i32;
    pub const TLS_RSA_WITH_CAMELLIA_128_GCM_SHA256: i32 = 49274i32;
    pub const TLS_RSA_WITH_CAMELLIA_256_CBC_SHA: i32 = 132i32;
    pub const TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256: i32 = 192i32;
    pub const TLS_RSA_WITH_CAMELLIA_256_GCM_SHA384: i32 = 49275i32;
    pub const TLS_RSA_WITH_DES_CBC_SHA: i32 = 9i32;
    pub const TLS_RSA_WITH_IDEA_CBC_SHA: i32 = 7i32;
    pub const TLS_RSA_WITH_NULL_MD5: i32 = 1i32;
    pub const TLS_RSA_WITH_NULL_SHA: i32 = 2i32;
    pub const TLS_RSA_WITH_NULL_SHA256: i32 = 59i32;
    pub const TLS_RSA_WITH_RC4_128_MD5: i32 = 4i32;
    pub const TLS_RSA_WITH_RC4_128_SHA: i32 = 5i32;
    pub const TLS_RSA_WITH_SEED_CBC_SHA: i32 = 150i32;
    pub const TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA: i32 = 49180i32;
    pub const TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA: i32 = 49183i32;
    pub const TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA: i32 = 49186i32;
    pub const TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA: i32 = 49179i32;
    pub const TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA: i32 = 49182i32;
    pub const TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA: i32 = 49185i32;
    pub const TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA: i32 = 49178i32;
    pub const TLS_SRP_SHA_WITH_AES_128_CBC_SHA: i32 = 49181i32;
    pub const TLS_SRP_SHA_WITH_AES_256_CBC_SHA: i32 = 49184i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+CipherSuite")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::CipherSuite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
