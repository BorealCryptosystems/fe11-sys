// Generated from OASIS PKCS#11 3.2; see NOTICE.txt and provenance.json.
use super::*;

#[repr(C, packed)]
pub struct CK_RSA_PKCS_OAEP_PARAMS {
    pub hashAlg: CK_MECHANISM_TYPE,
    pub mgf: CK_RSA_PKCS_MGF_TYPE,
    pub source: CK_RSA_PKCS_OAEP_SOURCE_TYPE,
    pub pSourceData: CK_VOID_PTR,
    pub ulSourceDataLen: CK_ULONG,
}

pub type CK_RSA_PKCS_OAEP_PARAMS_PTR = *mut CK_RSA_PKCS_OAEP_PARAMS;

#[repr(C, packed)]
pub struct CK_RSA_PKCS_PSS_PARAMS {
    pub hashAlg: CK_MECHANISM_TYPE,
    pub mgf: CK_RSA_PKCS_MGF_TYPE,
    pub sLen: CK_ULONG,
}

pub type CK_RSA_PKCS_PSS_PARAMS_PTR = *mut CK_RSA_PKCS_PSS_PARAMS;

#[repr(C, packed)]
pub struct CK_ECDH1_DERIVE_PARAMS {
    pub kdf: CK_EC_KDF_TYPE,
    pub ulSharedDataLen: CK_ULONG,
    pub pSharedData: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
}

pub type CK_ECDH1_DERIVE_PARAMS_PTR = *mut CK_ECDH1_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_ECDH2_DERIVE_PARAMS {
    pub kdf: CK_EC_KDF_TYPE,
    pub ulSharedDataLen: CK_ULONG,
    pub pSharedData: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPrivateDataLen: CK_ULONG,
    pub hPrivateData: CK_OBJECT_HANDLE,
    pub ulPublicDataLen2: CK_ULONG,
    pub pPublicData2: CK_BYTE_PTR,
}

pub type CK_ECDH2_DERIVE_PARAMS_PTR = *mut CK_ECDH2_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_ECMQV_DERIVE_PARAMS {
    pub kdf: CK_EC_KDF_TYPE,
    pub ulSharedDataLen: CK_ULONG,
    pub pSharedData: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPrivateDataLen: CK_ULONG,
    pub hPrivateData: CK_OBJECT_HANDLE,
    pub ulPublicDataLen2: CK_ULONG,
    pub pPublicData2: CK_BYTE_PTR,
    pub publicKey: CK_OBJECT_HANDLE,
}

pub type CK_ECMQV_DERIVE_PARAMS_PTR = *mut CK_ECMQV_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_X9_42_DH1_DERIVE_PARAMS {
    pub kdf: CK_X9_42_DH_KDF_TYPE,
    pub ulOtherInfoLen: CK_ULONG,
    pub pOtherInfo: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
}

pub type CK_X9_42_DH1_DERIVE_PARAMS_PTR = *mut CK_X9_42_DH1_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_X9_42_DH2_DERIVE_PARAMS {
    pub kdf: CK_X9_42_DH_KDF_TYPE,
    pub ulOtherInfoLen: CK_ULONG,
    pub pOtherInfo: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPrivateDataLen: CK_ULONG,
    pub hPrivateData: CK_OBJECT_HANDLE,
    pub ulPublicDataLen2: CK_ULONG,
    pub pPublicData2: CK_BYTE_PTR,
}

pub type CK_X9_42_DH2_DERIVE_PARAMS_PTR = *mut CK_X9_42_DH2_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_X9_42_MQV_DERIVE_PARAMS {
    pub kdf: CK_X9_42_DH_KDF_TYPE,
    pub ulOtherInfoLen: CK_ULONG,
    pub pOtherInfo: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPrivateDataLen: CK_ULONG,
    pub hPrivateData: CK_OBJECT_HANDLE,
    pub ulPublicDataLen2: CK_ULONG,
    pub pPublicData2: CK_BYTE_PTR,
    pub publicKey: CK_OBJECT_HANDLE,
}

pub type CK_X9_42_MQV_DERIVE_PARAMS_PTR = *mut CK_X9_42_MQV_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_KEA_DERIVE_PARAMS {
    pub isSender: CK_BBOOL,
    pub ulRandomLen: CK_ULONG,
    pub pRandomA: CK_BYTE_PTR,
    pub pRandomB: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
}

pub type CK_KEA_DERIVE_PARAMS_PTR = *mut CK_KEA_DERIVE_PARAMS;

pub type CK_RC2_PARAMS = CK_ULONG;

pub type CK_RC2_PARAMS_PTR = *mut CK_RC2_PARAMS;

#[repr(C, packed)]
pub struct CK_RC2_CBC_PARAMS {
    pub ulEffectiveBits: CK_ULONG,
    pub iv: [CK_BYTE; 8usize],
}

pub type CK_RC2_CBC_PARAMS_PTR = *mut CK_RC2_CBC_PARAMS;

#[repr(C, packed)]
pub struct CK_RC2_MAC_GENERAL_PARAMS {
    pub ulEffectiveBits: CK_ULONG,
    pub ulMacLength: CK_ULONG,
}

pub type CK_RC2_MAC_GENERAL_PARAMS_PTR = *mut CK_RC2_MAC_GENERAL_PARAMS;

#[repr(C, packed)]
pub struct CK_RC5_PARAMS {
    pub ulWordsize: CK_ULONG,
    pub ulRounds: CK_ULONG,
}

pub type CK_RC5_PARAMS_PTR = *mut CK_RC5_PARAMS;

#[repr(C, packed)]
pub struct CK_RC5_CBC_PARAMS {
    pub ulWordsize: CK_ULONG,
    pub ulRounds: CK_ULONG,
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
}

pub type CK_RC5_CBC_PARAMS_PTR = *mut CK_RC5_CBC_PARAMS;

#[repr(C, packed)]
pub struct CK_RC5_MAC_GENERAL_PARAMS {
    pub ulWordsize: CK_ULONG,
    pub ulRounds: CK_ULONG,
    pub ulMacLength: CK_ULONG,
}

pub type CK_RC5_MAC_GENERAL_PARAMS_PTR = *mut CK_RC5_MAC_GENERAL_PARAMS;

pub type CK_MAC_GENERAL_PARAMS = CK_ULONG;

pub type CK_MAC_GENERAL_PARAMS_PTR = *mut CK_MAC_GENERAL_PARAMS;

#[repr(C, packed)]
pub struct CK_DES_CBC_ENCRYPT_DATA_PARAMS {
    pub iv: [CK_BYTE; 8usize],
    pub pData: CK_BYTE_PTR,
    pub length: CK_ULONG,
}

pub type CK_DES_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut CK_DES_CBC_ENCRYPT_DATA_PARAMS;

#[repr(C, packed)]
pub struct CK_AES_CBC_ENCRYPT_DATA_PARAMS {
    pub iv: [CK_BYTE; 16usize],
    pub pData: CK_BYTE_PTR,
    pub length: CK_ULONG,
}

pub type CK_AES_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut CK_AES_CBC_ENCRYPT_DATA_PARAMS;

#[repr(C, packed)]
pub struct CK_SKIPJACK_PRIVATE_WRAP_PARAMS {
    pub ulPasswordLen: CK_ULONG,
    pub pPassword: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPAndGLen: CK_ULONG,
    pub ulQLen: CK_ULONG,
    pub ulRandomLen: CK_ULONG,
    pub pRandomA: CK_BYTE_PTR,
    pub pPrimeP: CK_BYTE_PTR,
    pub pBaseG: CK_BYTE_PTR,
    pub pSubprimeQ: CK_BYTE_PTR,
}

pub type CK_SKIPJACK_PRIVATE_WRAP_PARAMS_PTR = *mut CK_SKIPJACK_PRIVATE_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_SKIPJACK_RELAYX_PARAMS {
    pub ulOldWrappedXLen: CK_ULONG,
    pub pOldWrappedX: CK_BYTE_PTR,
    pub ulOldPasswordLen: CK_ULONG,
    pub pOldPassword: CK_BYTE_PTR,
    pub ulOldPublicDataLen: CK_ULONG,
    pub pOldPublicData: CK_BYTE_PTR,
    pub ulOldRandomLen: CK_ULONG,
    pub pOldRandomA: CK_BYTE_PTR,
    pub ulNewPasswordLen: CK_ULONG,
    pub pNewPassword: CK_BYTE_PTR,
    pub ulNewPublicDataLen: CK_ULONG,
    pub pNewPublicData: CK_BYTE_PTR,
    pub ulNewRandomLen: CK_ULONG,
    pub pNewRandomA: CK_BYTE_PTR,
}

pub type CK_SKIPJACK_RELAYX_PARAMS_PTR = *mut CK_SKIPJACK_RELAYX_PARAMS;

#[repr(C, packed)]
pub struct CK_PBE_PARAMS {
    pub pInitVector: CK_BYTE_PTR,
    pub pPassword: CK_UTF8CHAR_PTR,
    pub ulPasswordLen: CK_ULONG,
    pub pSalt: CK_BYTE_PTR,
    pub ulSaltLen: CK_ULONG,
    pub ulIteration: CK_ULONG,
}

pub type CK_PBE_PARAMS_PTR = *mut CK_PBE_PARAMS;

#[repr(C, packed)]
pub struct CK_KEY_WRAP_SET_OAEP_PARAMS {
    pub bBC: CK_BYTE,
    pub pX: CK_BYTE_PTR,
    pub ulXLen: CK_ULONG,
}

pub type CK_KEY_WRAP_SET_OAEP_PARAMS_PTR = *mut CK_KEY_WRAP_SET_OAEP_PARAMS;

#[repr(C, packed)]
pub struct CK_SSL3_RANDOM_DATA {
    pub pClientRandom: CK_BYTE_PTR,
    pub ulClientRandomLen: CK_ULONG,
    pub pServerRandom: CK_BYTE_PTR,
    pub ulServerRandomLen: CK_ULONG,
}

#[repr(C, packed)]
pub struct CK_SSL3_MASTER_KEY_DERIVE_PARAMS {
    pub RandomInfo: CK_SSL3_RANDOM_DATA,
    pub pVersion: CK_VERSION_PTR,
}

pub type CK_SSL3_MASTER_KEY_DERIVE_PARAMS_PTR = *mut CK_SSL3_MASTER_KEY_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_SSL3_KEY_MAT_PARAMS {
    pub ulMacSizeInBits: CK_ULONG,
    pub ulKeySizeInBits: CK_ULONG,
    pub ulIVSizeInBits: CK_ULONG,
    pub bIsExport: CK_BBOOL,
    pub RandomInfo: CK_SSL3_RANDOM_DATA,
    pub pReturnedKeyMaterial: CK_SSL3_KEY_MAT_OUT_PTR,
}

pub type CK_SSL3_KEY_MAT_PARAMS_PTR = *mut CK_SSL3_KEY_MAT_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS_PRF_PARAMS {
    pub pSeed: CK_BYTE_PTR,
    pub ulSeedLen: CK_ULONG,
    pub pLabel: CK_BYTE_PTR,
    pub ulLabelLen: CK_ULONG,
    pub pOutput: CK_BYTE_PTR,
    pub pulOutputLen: CK_ULONG_PTR,
}

pub type CK_TLS_PRF_PARAMS_PTR = *mut CK_TLS_PRF_PARAMS;

#[repr(C, packed)]
pub struct CK_WTLS_RANDOM_DATA {
    pub pClientRandom: CK_BYTE_PTR,
    pub ulClientRandomLen: CK_ULONG,
    pub pServerRandom: CK_BYTE_PTR,
    pub ulServerRandomLen: CK_ULONG,
}

pub type CK_WTLS_RANDOM_DATA_PTR = *mut CK_WTLS_RANDOM_DATA;

#[repr(C, packed)]
pub struct CK_WTLS_MASTER_KEY_DERIVE_PARAMS {
    pub DigestMechanism: CK_MECHANISM_TYPE,
    pub RandomInfo: CK_WTLS_RANDOM_DATA,
    pub pVersion: CK_BYTE_PTR,
}

pub type CK_WTLS_MASTER_KEY_DERIVE_PARAMS_PTR = *mut CK_WTLS_MASTER_KEY_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_WTLS_PRF_PARAMS {
    pub DigestMechanism: CK_MECHANISM_TYPE,
    pub pSeed: CK_BYTE_PTR,
    pub ulSeedLen: CK_ULONG,
    pub pLabel: CK_BYTE_PTR,
    pub ulLabelLen: CK_ULONG,
    pub pOutput: CK_BYTE_PTR,
    pub pulOutputLen: CK_ULONG_PTR,
}

pub type CK_WTLS_PRF_PARAMS_PTR = *mut CK_WTLS_PRF_PARAMS;

#[repr(C, packed)]
pub struct CK_WTLS_KEY_MAT_PARAMS {
    pub DigestMechanism: CK_MECHANISM_TYPE,
    pub ulMacSizeInBits: CK_ULONG,
    pub ulKeySizeInBits: CK_ULONG,
    pub ulIVSizeInBits: CK_ULONG,
    pub ulSequenceNumber: CK_ULONG,
    pub bIsExport: CK_BBOOL,
    pub RandomInfo: CK_WTLS_RANDOM_DATA,
    pub pReturnedKeyMaterial: CK_WTLS_KEY_MAT_OUT_PTR,
}

pub type CK_WTLS_KEY_MAT_PARAMS_PTR = *mut CK_WTLS_KEY_MAT_PARAMS;

#[repr(C, packed)]
pub struct CK_CMS_SIG_PARAMS {
    pub certificateHandle: CK_OBJECT_HANDLE,
    pub pSigningMechanism: CK_MECHANISM_PTR,
    pub pDigestMechanism: CK_MECHANISM_PTR,
    pub pContentType: CK_UTF8CHAR_PTR,
    pub pRequestedAttributes: CK_BYTE_PTR,
    pub ulRequestedAttributesLen: CK_ULONG,
    pub pRequiredAttributes: CK_BYTE_PTR,
    pub ulRequiredAttributesLen: CK_ULONG,
}

pub type CK_CMS_SIG_PARAMS_PTR = *mut CK_CMS_SIG_PARAMS;

#[repr(C, packed)]
pub struct CK_KEY_DERIVATION_STRING_DATA {
    pub pData: CK_BYTE_PTR,
    pub ulLen: CK_ULONG,
}

pub type CK_KEY_DERIVATION_STRING_DATA_PTR = *mut CK_KEY_DERIVATION_STRING_DATA;

pub type CK_EXTRACT_PARAMS = CK_ULONG;

pub type CK_EXTRACT_PARAMS_PTR = *mut CK_EXTRACT_PARAMS;

#[repr(C, packed)]
pub struct CK_PKCS5_PBKD2_PARAMS {
    pub saltSource: CK_PKCS5_PBKDF2_SALT_SOURCE_TYPE,
    pub pSaltSourceData: CK_VOID_PTR,
    pub ulSaltSourceDataLen: CK_ULONG,
    pub iterations: CK_ULONG,
    pub prf: CK_PKCS5_PBKD2_PSEUDO_RANDOM_FUNCTION_TYPE,
    pub pPrfData: CK_VOID_PTR,
    pub ulPrfDataLen: CK_ULONG,
    pub pPassword: CK_UTF8CHAR_PTR,
    pub ulPasswordLen: CK_ULONG_PTR,
}

pub type CK_PKCS5_PBKD2_PARAMS_PTR = *mut CK_PKCS5_PBKD2_PARAMS;

#[repr(C, packed)]
pub struct CK_PKCS5_PBKD2_PARAMS2 {
    pub saltSource: CK_PKCS5_PBKDF2_SALT_SOURCE_TYPE,
    pub pSaltSourceData: CK_VOID_PTR,
    pub ulSaltSourceDataLen: CK_ULONG,
    pub iterations: CK_ULONG,
    pub prf: CK_PKCS5_PBKD2_PSEUDO_RANDOM_FUNCTION_TYPE,
    pub pPrfData: CK_VOID_PTR,
    pub ulPrfDataLen: CK_ULONG,
    pub pPassword: CK_UTF8CHAR_PTR,
    pub ulPasswordLen: CK_ULONG,
}

pub type CK_PKCS5_PBKD2_PARAMS2_PTR = *mut CK_PKCS5_PBKD2_PARAMS2;

pub type CK_OTP_PARAM_TYPE = CK_ULONG;

pub type CK_PARAM_TYPE = CK_OTP_PARAM_TYPE;

#[repr(C, packed)]
pub struct CK_OTP_PARAM {
    pub r#type: CK_OTP_PARAM_TYPE,
    pub pValue: CK_VOID_PTR,
    pub ulValueLen: CK_ULONG,
}

pub type CK_OTP_PARAM_PTR = *mut CK_OTP_PARAM;

#[repr(C, packed)]
pub struct CK_OTP_PARAMS {
    pub pParams: CK_OTP_PARAM_PTR,
    pub ulCount: CK_ULONG,
}

pub type CK_OTP_PARAMS_PTR = *mut CK_OTP_PARAMS;

#[repr(C, packed)]
pub struct CK_KIP_PARAMS {
    pub pMechanism: CK_MECHANISM_PTR,
    pub hKey: CK_OBJECT_HANDLE,
    pub pSeed: CK_BYTE_PTR,
    pub ulSeedLen: CK_ULONG,
}

pub type CK_KIP_PARAMS_PTR = *mut CK_KIP_PARAMS;

#[repr(C, packed)]
pub struct CK_AES_CTR_PARAMS {
    pub ulCounterBits: CK_ULONG,
    pub cb: [CK_BYTE; 16usize],
}

pub type CK_AES_CTR_PARAMS_PTR = *mut CK_AES_CTR_PARAMS;

#[repr(C, packed)]
pub struct CK_GCM_PARAMS {
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
    pub ulIvBits: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulTagBits: CK_ULONG,
}

pub type CK_GCM_PARAMS_PTR = *mut CK_GCM_PARAMS;

#[repr(C, packed)]
pub struct CK_GCM_MESSAGE_PARAMS {
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
    pub ulIvFixedBits: CK_ULONG,
    pub ivGenerator: CK_GENERATOR_FUNCTION,
    pub pTag: CK_BYTE_PTR,
    pub ulTagBits: CK_ULONG,
}

pub type CK_GCM_MESSAGE_PARAMS_PTR = *mut CK_GCM_MESSAGE_PARAMS;

#[repr(C, packed)]
pub struct CK_GCM_WRAP_PARAMS {
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
    pub ulIvFixedBits: CK_ULONG,
    pub ivGenerator: CK_GENERATOR_FUNCTION,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulTagBits: CK_ULONG,
}

pub type CK_GCM_WRAP_PARAMS_PTR = *mut CK_GCM_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_CCM_PARAMS {
    pub ulDataLen: CK_ULONG,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulMACLen: CK_ULONG,
}

pub type CK_CCM_PARAMS_PTR = *mut CK_CCM_PARAMS;

#[repr(C, packed)]
pub struct CK_CCM_MESSAGE_PARAMS {
    pub ulDataLen: CK_ULONG,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub ulNonceFixedBits: CK_ULONG,
    pub nonceGenerator: CK_GENERATOR_FUNCTION,
    pub pMAC: CK_BYTE_PTR,
    pub ulMACLen: CK_ULONG,
}

pub type CK_CCM_MESSAGE_PARAMS_PTR = *mut CK_CCM_MESSAGE_PARAMS;

#[repr(C, packed)]
pub struct CK_CCM_WRAP_PARAMS {
    pub ulDataLen: CK_ULONG,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub ulNonceFixedBits: CK_ULONG,
    pub nonceGenerator: CK_GENERATOR_FUNCTION,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulMACLen: CK_ULONG,
}

pub type CK_CCM_WRAP_PARAMS_PTR = *mut CK_CCM_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_AES_GCM_PARAMS {
    pub pIv: CK_BYTE_PTR,
    pub ulIvLen: CK_ULONG,
    pub ulIvBits: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulTagBits: CK_ULONG,
}

pub type CK_AES_GCM_PARAMS_PTR = *mut CK_AES_GCM_PARAMS;

#[repr(C, packed)]
pub struct CK_AES_CCM_PARAMS {
    pub ulDataLen: CK_ULONG,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
    pub ulMACLen: CK_ULONG,
}

pub type CK_AES_CCM_PARAMS_PTR = *mut CK_AES_CCM_PARAMS;

#[repr(C, packed)]
pub struct CK_CAMELLIA_CTR_PARAMS {
    pub ulCounterBits: CK_ULONG,
    pub cb: [CK_BYTE; 16usize],
}

pub type CK_CAMELLIA_CTR_PARAMS_PTR = *mut CK_CAMELLIA_CTR_PARAMS;

#[repr(C, packed)]
pub struct CK_CAMELLIA_CBC_ENCRYPT_DATA_PARAMS {
    pub iv: [CK_BYTE; 16usize],
    pub pData: CK_BYTE_PTR,
    pub length: CK_ULONG,
}

pub type CK_CAMELLIA_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut CK_CAMELLIA_CBC_ENCRYPT_DATA_PARAMS;

#[repr(C, packed)]
pub struct CK_ARIA_CBC_ENCRYPT_DATA_PARAMS {
    pub iv: [CK_BYTE; 16usize],
    pub pData: CK_BYTE_PTR,
    pub length: CK_ULONG,
}

pub type CK_ARIA_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut CK_ARIA_CBC_ENCRYPT_DATA_PARAMS;

#[repr(C, packed)]
pub struct CK_DSA_PARAMETER_GEN_PARAM {
    pub hash: CK_MECHANISM_TYPE,
    pub pSeed: CK_BYTE_PTR,
    pub ulSeedLen: CK_ULONG,
    pub ulIndex: CK_ULONG,
}

pub type CK_DSA_PARAMETER_GEN_PARAM_PTR = *mut CK_DSA_PARAMETER_GEN_PARAM;

#[repr(C, packed)]
pub struct CK_ECDH_AES_KEY_WRAP_PARAMS {
    pub ulAESKeyBits: CK_ULONG,
    pub kdf: CK_EC_KDF_TYPE,
    pub ulSharedDataLen: CK_ULONG,
    pub pSharedData: CK_BYTE_PTR,
}

pub type CK_ECDH_AES_KEY_WRAP_PARAMS_PTR = *mut CK_ECDH_AES_KEY_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_RSA_AES_KEY_WRAP_PARAMS {
    pub ulAESKeyBits: CK_ULONG,
    pub pOAEPParams: CK_RSA_PKCS_OAEP_PARAMS_PTR,
}

pub type CK_RSA_AES_KEY_WRAP_PARAMS_PTR = *mut CK_RSA_AES_KEY_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS12_MASTER_KEY_DERIVE_PARAMS {
    pub RandomInfo: CK_SSL3_RANDOM_DATA,
    pub pVersion: CK_VERSION_PTR,
    pub prfHashMechanism: CK_MECHANISM_TYPE,
}

pub type CK_TLS12_MASTER_KEY_DERIVE_PARAMS_PTR = *mut CK_TLS12_MASTER_KEY_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS12_KEY_MAT_PARAMS {
    pub ulMacSizeInBits: CK_ULONG,
    pub ulKeySizeInBits: CK_ULONG,
    pub ulIVSizeInBits: CK_ULONG,
    pub bIsExport: CK_BBOOL,
    pub RandomInfo: CK_SSL3_RANDOM_DATA,
    pub pReturnedKeyMaterial: CK_SSL3_KEY_MAT_OUT_PTR,
    pub prfHashMechanism: CK_MECHANISM_TYPE,
}

pub type CK_TLS12_KEY_MAT_PARAMS_PTR = *mut CK_TLS12_KEY_MAT_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS_KDF_PARAMS {
    pub prfMechanism: CK_MECHANISM_TYPE,
    pub pLabel: CK_BYTE_PTR,
    pub ulLabelLength: CK_ULONG,
    pub RandomInfo: CK_SSL3_RANDOM_DATA,
    pub pContextData: CK_BYTE_PTR,
    pub ulContextDataLength: CK_ULONG,
}

pub type CK_TLS_KDF_PARAMS_PTR = *mut CK_TLS_KDF_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS_MAC_PARAMS {
    pub prfHashMechanism: CK_MECHANISM_TYPE,
    pub ulMacLength: CK_ULONG,
    pub ulServerOrClient: CK_ULONG,
}

pub type CK_TLS_MAC_PARAMS_PTR = *mut CK_TLS_MAC_PARAMS;

#[repr(C, packed)]
pub struct CK_TLS12_EXTENDED_MASTER_KEY_DERIVE_PARAMS {
    pub prfHashMechanism: CK_MECHANISM_TYPE,
    pub pSessionHash: CK_BYTE_PTR,
    pub ulSessionHashLen: CK_ULONG,
    pub pVersion: CK_VERSION_PTR,
}

pub type CK_TLS12_EXTENDED_MASTER_KEY_DERIVE_PARAMS_PTR =
    *mut CK_TLS12_EXTENDED_MASTER_KEY_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_GOSTR3410_DERIVE_PARAMS {
    pub kdf: CK_EC_KDF_TYPE,
    pub pPublicData: CK_BYTE_PTR,
    pub ulPublicDataLen: CK_ULONG,
    pub pUKM: CK_BYTE_PTR,
    pub ulUKMLen: CK_ULONG,
}

pub type CK_GOSTR3410_DERIVE_PARAMS_PTR = *mut CK_GOSTR3410_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_GOSTR3410_KEY_WRAP_PARAMS {
    pub pWrapOID: CK_BYTE_PTR,
    pub ulWrapOIDLen: CK_ULONG,
    pub pUKM: CK_BYTE_PTR,
    pub ulUKMLen: CK_ULONG,
    pub hKey: CK_OBJECT_HANDLE,
}

pub type CK_GOSTR3410_KEY_WRAP_PARAMS_PTR = *mut CK_GOSTR3410_KEY_WRAP_PARAMS;

#[repr(C, packed)]
pub struct CK_SEED_CBC_ENCRYPT_DATA_PARAMS {
    pub iv: [CK_BYTE; 16usize],
    pub pData: CK_BYTE_PTR,
    pub length: CK_ULONG,
}

pub type CK_SEED_CBC_ENCRYPT_DATA_PARAMS_PTR = *mut CK_SEED_CBC_ENCRYPT_DATA_PARAMS;

pub type CK_PRF_DATA_TYPE = CK_ULONG;

pub type CK_SP800_108_PRF_TYPE = CK_MECHANISM_TYPE;

#[repr(C, packed)]
pub struct CK_PRF_DATA_PARAM {
    pub r#type: CK_PRF_DATA_TYPE,
    pub pValue: CK_VOID_PTR,
    pub ulValueLen: CK_ULONG,
}

pub type CK_PRF_DATA_PARAM_PTR = *mut CK_PRF_DATA_PARAM;

#[repr(C, packed)]
pub struct CK_DERIVED_KEY {
    pub pTemplate: CK_ATTRIBUTE_PTR,
    pub ulAttributeCount: CK_ULONG,
    pub phKey: CK_OBJECT_HANDLE_PTR,
}

pub type CK_DERIVED_KEY_PTR = *mut CK_DERIVED_KEY;

#[repr(C, packed)]
pub struct CK_SP800_108_KDF_PARAMS {
    pub prfType: CK_SP800_108_PRF_TYPE,
    pub ulNumberOfDataParams: CK_ULONG,
    pub pDataParams: CK_PRF_DATA_PARAM_PTR,
    pub ulAdditionalDerivedKeys: CK_ULONG,
    pub pAdditionalDerivedKeys: CK_DERIVED_KEY_PTR,
}

pub type CK_SP800_108_KDF_PARAMS_PTR = *mut CK_SP800_108_KDF_PARAMS;

#[repr(C, packed)]
pub struct CK_SP800_108_FEEDBACK_KDF_PARAMS {
    pub prfType: CK_SP800_108_PRF_TYPE,
    pub ulNumberOfDataParams: CK_ULONG,
    pub pDataParams: CK_PRF_DATA_PARAM_PTR,
    pub ulIVLen: CK_ULONG,
    pub pIV: CK_BYTE_PTR,
    pub ulAdditionalDerivedKeys: CK_ULONG,
    pub pAdditionalDerivedKeys: CK_DERIVED_KEY_PTR,
}

pub type CK_SP800_108_FEEDBACK_KDF_PARAMS_PTR = *mut CK_SP800_108_FEEDBACK_KDF_PARAMS;

#[repr(C, packed)]
pub struct CK_EDDSA_PARAMS {
    pub phFlag: CK_BBOOL,
    pub ulContextDataLen: CK_ULONG,
    pub pContextData: CK_BYTE_PTR,
}

pub type CK_EDDSA_PARAMS_PTR = *mut CK_EDDSA_PARAMS;

#[repr(C, packed)]
pub struct CK_CHACHA20_PARAMS {
    pub pBlockCounter: CK_BYTE_PTR,
    pub blockCounterBits: CK_ULONG,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceBits: CK_ULONG,
}

pub type CK_CHACHA20_PARAMS_PTR = *mut CK_CHACHA20_PARAMS;

#[repr(C, packed)]
pub struct CK_SALSA20_PARAMS {
    pub pBlockCounter: CK_BYTE_PTR,
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceBits: CK_ULONG,
}

pub type CK_SALSA20_PARAMS_PTR = *mut CK_SALSA20_PARAMS;

#[repr(C, packed)]
pub struct CK_SALSA20_CHACHA20_POLY1305_PARAMS {
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub pAAD: CK_BYTE_PTR,
    pub ulAADLen: CK_ULONG,
}

pub type CK_SALSA20_CHACHA20_POLY1305_PARAMS_PTR = *mut CK_SALSA20_CHACHA20_POLY1305_PARAMS;

#[repr(C, packed)]
pub struct CK_SALSA20_CHACHA20_POLY1305_MSG_PARAMS {
    pub pNonce: CK_BYTE_PTR,
    pub ulNonceLen: CK_ULONG,
    pub pTag: CK_BYTE_PTR,
}

pub type CK_SALSA20_CHACHA20_POLY1305_MSG_PARAMS_PTR = *mut CK_SALSA20_CHACHA20_POLY1305_MSG_PARAMS;

#[repr(C, packed)]
pub struct CK_X3DH_INITIATE_PARAMS {
    pub kdf: CK_X3DH_KDF_TYPE,
    pub pPeer_identity: CK_OBJECT_HANDLE,
    pub pPeer_prekey: CK_OBJECT_HANDLE,
    pub pPrekey_signature: CK_BYTE_PTR,
    pub pOnetime_key: CK_BYTE_PTR,
    pub pOwn_identity: CK_OBJECT_HANDLE,
    pub pOwn_ephemeral: CK_OBJECT_HANDLE,
}

#[repr(C, packed)]
pub struct CK_X3DH_RESPOND_PARAMS {
    pub kdf: CK_X3DH_KDF_TYPE,
    pub pIdentity_id: CK_BYTE_PTR,
    pub pPrekey_id: CK_BYTE_PTR,
    pub pOnetime_id: CK_BYTE_PTR,
    pub pInitiator_identity: CK_OBJECT_HANDLE,
    pub pInitiator_ephemeral: CK_BYTE_PTR,
}

#[repr(C, packed)]
pub struct CK_X2RATCHET_INITIALIZE_PARAMS {
    pub sk: CK_BYTE_PTR,
    pub peer_public_prekey: CK_OBJECT_HANDLE,
    pub peer_public_identity: CK_OBJECT_HANDLE,
    pub own_public_identity: CK_OBJECT_HANDLE,
    pub bEncryptedHeader: CK_BBOOL,
    pub eCurve: CK_ULONG,
    pub aeadMechanism: CK_MECHANISM_TYPE,
    pub kdfMechanism: CK_X2RATCHET_KDF_TYPE,
}

pub type CK_X2RATCHET_INITIALIZE_PARAMS_PTR = *mut CK_X2RATCHET_INITIALIZE_PARAMS;

#[repr(C, packed)]
pub struct CK_X2RATCHET_RESPOND_PARAMS {
    pub sk: CK_BYTE_PTR,
    pub own_prekey: CK_OBJECT_HANDLE,
    pub initiator_identity: CK_OBJECT_HANDLE,
    pub own_public_identity: CK_OBJECT_HANDLE,
    pub bEncryptedHeader: CK_BBOOL,
    pub eCurve: CK_ULONG,
    pub aeadMechanism: CK_MECHANISM_TYPE,
    pub kdfMechanism: CK_X2RATCHET_KDF_TYPE,
}

pub type CK_X2RATCHET_RESPOND_PARAMS_PTR = *mut CK_X2RATCHET_RESPOND_PARAMS;

#[repr(C, packed)]
pub struct CK_XEDDSA_PARAMS {
    pub hash: CK_XEDDSA_HASH_TYPE,
}

pub type CK_XEDDSA_PARAMS_PTR = *mut CK_XEDDSA_PARAMS;

#[repr(C, packed)]
pub struct CK_HKDF_PARAMS {
    pub bExtract: CK_BBOOL,
    pub bExpand: CK_BBOOL,
    pub prfHashMechanism: CK_MECHANISM_TYPE,
    pub ulSaltType: CK_ULONG,
    pub pSalt: CK_BYTE_PTR,
    pub ulSaltLen: CK_ULONG,
    pub hSaltKey: CK_OBJECT_HANDLE,
    pub pInfo: CK_BYTE_PTR,
    pub ulInfoLen: CK_ULONG,
}

pub type CK_HKDF_PARAMS_PTR = *mut CK_HKDF_PARAMS;

#[repr(C, packed)]
pub struct specifiedParams {
    pub levels: CK_HSS_LEVELS,
    pub lm_type: [CK_LMS_TYPE; 8usize],
    pub lm_ots_type: [CK_LMOTS_TYPE; 8usize],
}

#[repr(C, packed)]
pub struct CK_IKE2_PRF_PLUS_DERIVE_PARAMS {
    pub prfMechanism: CK_MECHANISM_TYPE,
    pub bHasSeedKey: CK_BBOOL,
    pub hSeedKey: CK_OBJECT_HANDLE,
    pub pSeedData: CK_BYTE_PTR,
    pub ulSeedDataLen: CK_ULONG,
}

pub type CK_IKE2_PRF_PLUS_DERIVE_PARAMS_PTR = *mut CK_IKE2_PRF_PLUS_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_IKE_PRF_DERIVE_PARAMS {
    pub prfMechanism: CK_MECHANISM_TYPE,
    pub bDataAsKey: CK_BBOOL,
    pub bRekey: CK_BBOOL,
    pub pNi: CK_BYTE_PTR,
    pub ulNiLen: CK_ULONG,
    pub pNr: CK_BYTE_PTR,
    pub ulNrLen: CK_ULONG,
    pub hNewKey: CK_OBJECT_HANDLE,
}

pub type CK_IKE_PRF_DERIVE_PARAMS_PTR = *mut CK_IKE_PRF_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_IKE1_PRF_DERIVE_PARAMS {
    pub prfMechanism: CK_MECHANISM_TYPE,
    pub bHasPrevKey: CK_BBOOL,
    pub hKeygxy: CK_OBJECT_HANDLE,
    pub hPrevKey: CK_OBJECT_HANDLE,
    pub pCKYi: CK_BYTE_PTR,
    pub ulCKYiLen: CK_ULONG,
    pub pCKYr: CK_BYTE_PTR,
    pub ulCKYrLen: CK_ULONG,
    pub keyNumber: CK_BYTE,
}

pub type CK_IKE1_PRF_DERIVE_PARAMS_PTR = *mut CK_IKE1_PRF_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_IKE1_EXTENDED_DERIVE_PARAMS {
    pub prfMechanism: CK_MECHANISM_TYPE,
    pub bHasKeygxy: CK_BBOOL,
    pub hKeygxy: CK_OBJECT_HANDLE,
    pub pExtraData: CK_BYTE_PTR,
    pub ulExtraDataLen: CK_ULONG,
}

pub type CK_IKE1_EXTENDED_DERIVE_PARAMS_PTR = *mut CK_IKE1_EXTENDED_DERIVE_PARAMS;

#[repr(C, packed)]
pub struct CK_ASYNC_DATA {
    pub ulVersion: CK_ULONG,
    pub pValue: CK_BYTE_PTR,
    pub ulValue: CK_ULONG,
    pub hObject: CK_OBJECT_HANDLE,
    pub hAdditionalObject: CK_OBJECT_HANDLE,
}

pub type CK_ASYNC_DATA_PTR = *mut CK_ASYNC_DATA;

pub type CK_XMSSMT_PARAMETER_SET_TYPE = CK_ULONG;

pub type CK_XMSS_PARAMETER_SET_TYPE = CK_ULONG;

pub type CK_ML_DSA_PARAMETER_SET_TYPE = CK_ULONG;

pub type CK_SLH_DSA_PARAMETER_SET_TYPE = CK_ULONG;

pub type CK_ML_KEM_PARAMETER_SET_TYPE = CK_ULONG;
