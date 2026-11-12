// Generated from OASIS PKCS#11 3.2; see NOTICE.txt and provenance.json.
use super::*;

#[repr(C)]
pub struct CK_C_INITIALIZE_ARGS {
    pub CreateMutex: CK_CREATEMUTEX,
    pub DestroyMutex: CK_DESTROYMUTEX,
    pub LockMutex: CK_LOCKMUTEX,
    pub UnlockMutex: CK_UNLOCKMUTEX,
    pub flags: CK_FLAGS,
    pub pReserved: CK_VOID_PTR,
}

pub type CK_C_INITIALIZE_ARGS_PTR = *mut CK_C_INITIALIZE_ARGS;

pub type CK_C_Initialize =
    ::core::option::Option<unsafe extern "C" fn(pInitArgs: CK_VOID_PTR) -> CK_RV>;

pub type CK_C_Finalize =
    ::core::option::Option<unsafe extern "C" fn(pReserved: CK_VOID_PTR) -> CK_RV>;

pub type CK_C_GetInfo = ::core::option::Option<unsafe extern "C" fn(pInfo: CK_INFO_PTR) -> CK_RV>;

pub type CK_C_GetFunctionList =
    ::core::option::Option<unsafe extern "C" fn(ppFunctionList: CK_FUNCTION_LIST_PTR_PTR) -> CK_RV>;

pub type CK_C_GetSlotList = ::core::option::Option<
    unsafe extern "C" fn(
        tokenPresent: CK_BBOOL,
        pSlotList: CK_SLOT_ID_PTR,
        pulCount: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_GetSlotInfo = ::core::option::Option<
    unsafe extern "C" fn(slotID: CK_SLOT_ID, pInfo: CK_SLOT_INFO_PTR) -> CK_RV,
>;

pub type CK_C_GetTokenInfo = ::core::option::Option<
    unsafe extern "C" fn(slotID: CK_SLOT_ID, pInfo: CK_TOKEN_INFO_PTR) -> CK_RV,
>;

pub type CK_C_GetMechanismList = ::core::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        pMechanismList: CK_MECHANISM_TYPE_PTR,
        pulCount: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_GetMechanismInfo = ::core::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        type_: CK_MECHANISM_TYPE,
        pInfo: CK_MECHANISM_INFO_PTR,
    ) -> CK_RV,
>;

pub type CK_C_InitToken = ::core::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        pPin: CK_UTF8CHAR_PTR,
        ulPinLen: CK_ULONG,
        pLabel: CK_UTF8CHAR_PTR,
    ) -> CK_RV,
>;

pub type CK_C_InitPIN = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPin: CK_UTF8CHAR_PTR,
        ulPinLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_SetPIN = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pOldPin: CK_UTF8CHAR_PTR,
        ulOldLen: CK_ULONG,
        pNewPin: CK_UTF8CHAR_PTR,
        ulNewLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_OpenSession = ::core::option::Option<
    unsafe extern "C" fn(
        slotID: CK_SLOT_ID,
        flags: CK_FLAGS,
        pApplication: CK_VOID_PTR,
        Notify: CK_NOTIFY,
        phSession: CK_SESSION_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_CloseSession =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_CloseAllSessions =
    ::core::option::Option<unsafe extern "C" fn(slotID: CK_SLOT_ID) -> CK_RV>;

pub type CK_C_GetSessionInfo = ::core::option::Option<
    unsafe extern "C" fn(hSession: CK_SESSION_HANDLE, pInfo: CK_SESSION_INFO_PTR) -> CK_RV,
>;

pub type CK_C_GetOperationState = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pOperationState: CK_BYTE_PTR,
        pulOperationStateLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SetOperationState = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pOperationState: CK_BYTE_PTR,
        ulOperationStateLen: CK_ULONG,
        hEncryptionKey: CK_OBJECT_HANDLE,
        hAuthenticationKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_Login = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        userType: CK_USER_TYPE,
        pPin: CK_UTF8CHAR_PTR,
        ulPinLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_Logout =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_CreateObject = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
        phObject: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_CopyObject = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        hObject: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
        phNewObject: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DestroyObject = ::core::option::Option<
    unsafe extern "C" fn(hSession: CK_SESSION_HANDLE, hObject: CK_OBJECT_HANDLE) -> CK_RV,
>;

pub type CK_C_GetObjectSize = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        hObject: CK_OBJECT_HANDLE,
        pulSize: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_GetAttributeValue = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        hObject: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_SetAttributeValue = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        hObject: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_FindObjectsInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_FindObjects = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        phObject: CK_OBJECT_HANDLE_PTR,
        ulMaxObjectCount: CK_ULONG,
        pulObjectCount: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_FindObjectsFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_EncryptInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_Encrypt = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pEncryptedData: CK_BYTE_PTR,
        pulEncryptedDataLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_EncryptUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
        pEncryptedPart: CK_BYTE_PTR,
        pulEncryptedPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_EncryptFinal = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pLastEncryptedPart: CK_BYTE_PTR,
        pulLastEncryptedPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_Decrypt = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pEncryptedData: CK_BYTE_PTR,
        ulEncryptedDataLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        pulDataLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pEncryptedPart: CK_BYTE_PTR,
        ulEncryptedPartLen: CK_ULONG,
        pPart: CK_BYTE_PTR,
        pulPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptFinal = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pLastPart: CK_BYTE_PTR,
        pulLastPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DigestInit = ::core::option::Option<
    unsafe extern "C" fn(hSession: CK_SESSION_HANDLE, pMechanism: CK_MECHANISM_PTR) -> CK_RV,
>;

pub type CK_C_Digest = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pDigest: CK_BYTE_PTR,
        pulDigestLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DigestUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_DigestKey = ::core::option::Option<
    unsafe extern "C" fn(hSession: CK_SESSION_HANDLE, hKey: CK_OBJECT_HANDLE) -> CK_RV,
>;

pub type CK_C_DigestFinal = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pDigest: CK_BYTE_PTR,
        pulDigestLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SignInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_Sign = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        pulSignatureLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SignUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_SignFinal = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pSignature: CK_BYTE_PTR,
        pulSignatureLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SignRecoverInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_SignRecover = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        pulSignatureLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_VerifyInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_Verify = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifyUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifyFinal = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifyRecoverInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_VerifyRecover = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        pulDataLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DigestEncryptUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
        pEncryptedPart: CK_BYTE_PTR,
        pulEncryptedPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptDigestUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pEncryptedPart: CK_BYTE_PTR,
        ulEncryptedPartLen: CK_ULONG,
        pPart: CK_BYTE_PTR,
        pulPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SignEncryptUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
        pEncryptedPart: CK_BYTE_PTR,
        pulEncryptedPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptVerifyUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pEncryptedPart: CK_BYTE_PTR,
        ulEncryptedPartLen: CK_ULONG,
        pPart: CK_BYTE_PTR,
        pulPartLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_GenerateKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulCount: CK_ULONG,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_GenerateKeyPair = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        pPublicKeyTemplate: CK_ATTRIBUTE_PTR,
        ulPublicKeyAttributeCount: CK_ULONG,
        pPrivateKeyTemplate: CK_ATTRIBUTE_PTR,
        ulPrivateKeyAttributeCount: CK_ULONG,
        phPublicKey: CK_OBJECT_HANDLE_PTR,
        phPrivateKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_WrapKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hWrappingKey: CK_OBJECT_HANDLE,
        hKey: CK_OBJECT_HANDLE,
        pWrappedKey: CK_BYTE_PTR,
        pulWrappedKeyLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_UnwrapKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hUnwrappingKey: CK_OBJECT_HANDLE,
        pWrappedKey: CK_BYTE_PTR,
        ulWrappedKeyLen: CK_ULONG,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulAttributeCount: CK_ULONG,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DeriveKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hBaseKey: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulAttributeCount: CK_ULONG,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SeedRandom = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pSeed: CK_BYTE_PTR,
        ulSeedLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_GenerateRandom = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        RandomData: CK_BYTE_PTR,
        ulRandomLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_GetFunctionStatus =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_CancelFunction =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_WaitForSlotEvent = ::core::option::Option<
    unsafe extern "C" fn(flags: CK_FLAGS, pSlot: CK_SLOT_ID_PTR, pRserved: CK_VOID_PTR) -> CK_RV,
>;

pub type CK_C_GetInterfaceList = ::core::option::Option<
    unsafe extern "C" fn(pInterfacesList: CK_INTERFACE_PTR, pulCount: CK_ULONG_PTR) -> CK_RV,
>;

pub type CK_C_GetInterface = ::core::option::Option<
    unsafe extern "C" fn(
        pInterfaceName: CK_UTF8CHAR_PTR,
        pVersion: CK_VERSION_PTR,
        ppInterface: CK_INTERFACE_PTR_PTR,
        flags: CK_FLAGS,
    ) -> CK_RV,
>;

pub type CK_C_LoginUser = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        userType: CK_USER_TYPE,
        pPin: CK_UTF8CHAR_PTR,
        ulPinLen: CK_ULONG,
        pUsername: CK_UTF8CHAR_PTR,
        ulUsernameLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_SessionCancel = ::core::option::Option<
    unsafe extern "C" fn(hSession: CK_SESSION_HANDLE, flags: CK_FLAGS) -> CK_RV,
>;

pub type CK_C_MessageEncryptInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_EncryptMessage = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
        pPlaintext: CK_BYTE_PTR,
        ulPlaintextLen: CK_ULONG,
        pCiphertext: CK_BYTE_PTR,
        pulCiphertextLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_EncryptMessageBegin = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_EncryptMessageNext = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pPlaintextPart: CK_BYTE_PTR,
        ulPlaintextPartLen: CK_ULONG,
        pCiphertextPart: CK_BYTE_PTR,
        pulCiphertextPartLen: CK_ULONG_PTR,
        flags: CK_FLAGS,
    ) -> CK_RV,
>;

pub type CK_C_MessageEncryptFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_MessageDecryptInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_DecryptMessage = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
        pCiphertext: CK_BYTE_PTR,
        ulCiphertextLen: CK_ULONG,
        pPlaintext: CK_BYTE_PTR,
        pulPlaintextLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecryptMessageBegin = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_DecryptMessageNext = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pCiphertextPart: CK_BYTE_PTR,
        ulCiphertextPartLen: CK_ULONG,
        pPlaintextPart: CK_BYTE_PTR,
        pulPlaintextPartLen: CK_ULONG_PTR,
        flags: CK_FLAGS,
    ) -> CK_RV,
>;

pub type CK_C_MessageDecryptFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_MessageSignInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_SignMessage = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        pulSignatureLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_SignMessageBegin = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_SignMessageNext = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        pulSignatureLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_MessageSignFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_MessageVerifyInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
    ) -> CK_RV,
>;

pub type CK_C_VerifyMessage = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifyMessageBegin = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifyMessageNext = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pParameter: CK_VOID_PTR,
        ulParameterLen: CK_ULONG,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_MessageVerifyFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_EncapsulateKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hPublicKey: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulAttributeCount: CK_ULONG,
        pCiphertext: CK_BYTE_PTR,
        pulCiphertextLen: CK_ULONG_PTR,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_DecapsulateKey = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hPrivateKey: CK_OBJECT_HANDLE,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulAttributeCount: CK_ULONG,
        pCiphertext: CK_BYTE_PTR,
        ulCiphertextLen: CK_ULONG,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;

pub type CK_C_VerifySignatureInit = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hKey: CK_OBJECT_HANDLE,
        pSignature: CK_BYTE_PTR,
        ulSignatureLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifySignature = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pData: CK_BYTE_PTR,
        ulDataLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifySignatureUpdate = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pPart: CK_BYTE_PTR,
        ulPartLen: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_VerifySignatureFinal =
    ::core::option::Option<unsafe extern "C" fn(hSession: CK_SESSION_HANDLE) -> CK_RV>;

pub type CK_C_GetSessionValidationFlags = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        type_: CK_SESSION_VALIDATION_FLAGS_TYPE,
        pFlags: CK_FLAGS_PTR,
    ) -> CK_RV,
>;

pub type CK_C_AsyncComplete = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pFunctionName: CK_UTF8CHAR_PTR,
        pResult: CK_ASYNC_DATA_PTR,
    ) -> CK_RV,
>;

pub type CK_C_AsyncGetID = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pFunctionName: CK_UTF8CHAR_PTR,
        pulID: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_AsyncJoin = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pFunctionName: CK_UTF8CHAR_PTR,
        ulID: CK_ULONG,
        pData: CK_BYTE_PTR,
        ulData: CK_ULONG,
    ) -> CK_RV,
>;

pub type CK_C_WrapKeyAuthenticated = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hWrappingKey: CK_OBJECT_HANDLE,
        hKey: CK_OBJECT_HANDLE,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
        pWrappedKey: CK_BYTE_PTR,
        pulWrappedKeyLen: CK_ULONG_PTR,
    ) -> CK_RV,
>;

pub type CK_C_UnwrapKeyAuthenticated = ::core::option::Option<
    unsafe extern "C" fn(
        hSession: CK_SESSION_HANDLE,
        pMechanism: CK_MECHANISM_PTR,
        hUnwrappingKey: CK_OBJECT_HANDLE,
        pWrappedKey: CK_BYTE_PTR,
        ulWrappedKeyLen: CK_ULONG,
        pTemplate: CK_ATTRIBUTE_PTR,
        ulAttributeCount: CK_ULONG,
        pAssociatedData: CK_BYTE_PTR,
        ulAssociatedDataLen: CK_ULONG,
        phKey: CK_OBJECT_HANDLE_PTR,
    ) -> CK_RV,
>;
