# OASIS PKCS#11 3.2 header comments

The following comments preserve the original header documentation. Normative
requirements are established by the corresponding published specification.

The Linux profile uses natural alignment; the Windows profile uses one-byte
packing. OASIS comments leave the UNIX packing mechanism to the integrator.
Establish the actual module ABI before calling it; its advertised PKCS#11
version and its structure alignment are separate properties.

## pkcs11.h:1

```text
/* Copyright (c) OASIS Open 2016,2019,2024. All Rights Reserved./
 * /Distributed under the terms of the OASIS IPR Policy,
 * [http://www.oasis-open.org/policies-guidelines/ipr], AS-IS, WITHOUT ANY
 * IMPLIED OR EXPRESS WARRANTY; there is no warranty of MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE or NONINFRINGEMENT of the rights of others.
 */
```

## pkcs11.h:8

```text
/* Latest version of the specification:
 * http://docs.oasis-open.org/pkcs11/pkcs11-base/v3.2/pkcs11-base-v3.2.html
 */
```

## pkcs11.h:19

```text
/* Before including this file (pkcs11.h) (or pkcs11t.h by
 * itself), 5 platform-specific macros must be defined.  These
 * macros are described below, and typical definitions for them
 * are also given.  Be advised that these definitions can depend
 * on both the platform and the compiler used (and possibly also
 * on whether a Cryptoki library is linked statically or
 * dynamically).
 *
 * In addition to defining these 5 macros, the packing convention
 * for Cryptoki structures should be set.  The Cryptoki
 * convention on packing is that structures should be 1-byte
 * aligned.
 *
 * If you're using Windows this might be done by using the following
 * preprocessor directive before including pkcs11.h or pkcs11t.h:
 *
 * #pragma pack(push, cryptoki, 1)
 *
 * and using the following preprocessor directive after including
 * pkcs11.h or pkcs11t.h:
 *
 * #pragma pack(pop, cryptoki)
 *
 * In a UNIX environment, you're on your own for this.  You might
 * not need to do (or be able to do!) anything.
 *
 *
 * Now for the macros:
 *
 *
 * 1. CK_PTR: The indirection string for making a pointer to an
 * object.  It can be used like this:
 *
 * typedef CK_BYTE CK_PTR CK_BYTE_PTR;
 *
 * If you're using Windows, it might be defined by:
 *
 * #define CK_PTR *
 *
 * In a typical UNIX environment, it might be defined by:
 *
 * #define CK_PTR *
 *
 *
 * 2. CK_DECLARE_FUNCTION(returnType, name): A macro which makes
 * an importable Cryptoki library function declaration out of a
 * return type and a function name.  It should be used in the
 * following fashion:
 *
 * extern CK_DECLARE_FUNCTION(CK_RV, C_Initialize)(
 *   CK_VOID_PTR pReserved
 * );
 *
 * If you're using Windows to declare a function in a Win32 Cryptoki .dll,
 * it might be defined by:
 *
 * #define CK_DECLARE_FUNCTION(returnType, name) \
 *   returnType __declspec(dllimport) name
 *
 * In a UNIX environment, it might be defined by:
 *
 * #define CK_DECLARE_FUNCTION(returnType, name) \
 *   returnType name
 *
 *
 * 3. CK_DECLARE_FUNCTION_POINTER(returnType, name): A macro
 * which makes a Cryptoki API function pointer declaration or
 * function pointer type declaration out of a return type and a
 * function name.  It should be used in the following fashion:
 *
 * // Define funcPtr to be a pointer to a Cryptoki API function
 * // taking arguments args and returning CK_RV.
 * CK_DECLARE_FUNCTION_POINTER(CK_RV, funcPtr)(args);
 *
 * or
 *
 * // Define funcPtrType to be the type of a pointer to a
 * // Cryptoki API function taking arguments args and returning
 * // CK_RV, and then define funcPtr to be a variable of type
 * // funcPtrType.
 * typedef CK_DECLARE_FUNCTION_POINTER(CK_RV, funcPtrType)(args);
 * funcPtrType funcPtr;
 *
 * If you're using Windows to access
 * functions in a Win32 Cryptoki .dll, in might be defined by:
 *
 * #define CK_DECLARE_FUNCTION_POINTER(returnType, name) \
 *   returnType __declspec(dllimport) (* name)
 *
 * In a UNIX environment, it might be defined by:
 *
 * #define CK_DECLARE_FUNCTION_POINTER(returnType, name) \
 *   returnType (* name)
 *
 *
 * 4. CK_CALLBACK_FUNCTION(returnType, name): A macro which makes
 * a function pointer type for an application callback out of
 * a return type for the callback and a name for the callback.
 * It should be used in the following fashion:
 *
 * CK_CALLBACK_FUNCTION(CK_RV, myCallback)(args);
 *
 * to declare a function pointer, myCallback, to a callback
 * which takes arguments args and returns a CK_RV.  It can also
 * be used like this:
 *
 * typedef CK_CALLBACK_FUNCTION(CK_RV, myCallbackType)(args);
 * myCallbackType myCallback;
 *
 * If you're using Windows, it might be defined by:
 *
 * #define CK_CALLBACK_FUNCTION(returnType, name) \
 *   returnType (* name)
 *
 * In a UNIX environment, it might be defined by:
 *
 * #define CK_CALLBACK_FUNCTION(returnType, name) \
 *   returnType (* name)
 *
 *
 * 5. NULL_PTR: This macro is the value of a NULL pointer.
 *
 * In any ANSI/ISO C environment (and in many others as well),
 * this should best be defined by
 *
 * #ifndef NULL_PTR
 * #define NULL_PTR 0
 * #endif
 */
```

## pkcs11.h:150

```text
/* All the various Cryptoki types and #define'd values are in the
 * file pkcs11t.h.
 */
```

## pkcs11.h:158

```text
/* ==============================================================
 * Define the "extern" form of all the entry points.
 * ==============================================================
 */
```

## pkcs11.h:167

```text
/* pkcs11f.h has all the information about the Cryptoki
 * function prototypes.
 */
```

## pkcs11.h:176

```text
/* ==============================================================
 * Define the typedef form of all the entry points.  That is, for
 * each Cryptoki function C_XXX, define a type CK_C_XXX which is
 * a pointer to that kind of function.
 * ==============================================================
 */
```

## pkcs11.h:187

```text
/* pkcs11f.h has all the information about the Cryptoki
 * function prototypes.
 */
```

## pkcs11.h:196

```text
/* ==============================================================
 * Define structed vector of entry points.  A CK_FUNCTION_LIST
 * contains a CK_VERSION indicating a library's Cryptoki version
 * and then a whole slew of function pointers to the routines in
 * the library.  This type was declared, but not defined, in
 * pkcs11t.h.
 * ==============================================================
 */
```

## pkcs11.h:208

```text
/* Create the 3.2 Function list */
```

## pkcs11.h:211

```text
/* Cryptoki version */
```

## pkcs11.h:213

```text
/* Pile all the function pointers into the CK_FUNCTION_LIST. */
```

## pkcs11.h:214

```text
/* pkcs11f.h has all the information about the Cryptoki
 * function prototypes.
 */
```

## pkcs11.h:221

```text
/* don't include the 3.2 and later functions */
```

## pkcs11.h:223

```text
/* Create the 3.0 Function list */
```

## pkcs11.h:226

```text
/* Cryptoki version */
```

## pkcs11.h:228

```text
/* Pile all the function pointers into the CK_FUNCTION_LIST. */
```

## pkcs11.h:229

```text
/* pkcs11f.h has all the information about the Cryptoki
 * function prototypes.
 */
```

## pkcs11.h:238

```text
/* don't include the 3.0 and later functions */
```

## pkcs11.h:240

```text
/* Continue to define the old CK_FUNCTION_LIST */
```

## pkcs11.h:243

```text
/* Cryptoki version */
```

## pkcs11.h:245

```text
/* Pile all the function pointers into the CK_FUNCTION_LIST. */
```

## pkcs11.h:246

```text
/* pkcs11f.h has all the information about the Cryptoki
 * function prototypes.
 */
```

## pkcs11.h:263

```text
/* _PKCS11_H_ */
```

## pkcs11t.h:1

```text
/* Copyright (c) OASIS Open 2016, 2019, 2024. All Rights Reserved./
 * /Distributed under the terms of the OASIS IPR Policy,
 * [http://www.oasis-open.org/policies-guidelines/ipr], AS-IS, WITHOUT ANY
 * IMPLIED OR EXPRESS WARRANTY; there is no warranty of MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE or NONINFRINGEMENT of the rights of others.
 */
```

## pkcs11t.h:8

```text
/* Latest version of the specification:
 * http://docs.oasis-open.org/pkcs11/pkcs11-spec/v3.2/pkcs11-spec-v3.2.html
 */
```

## pkcs11t.h:12

```text
/* See top of pkcs11.h for information about the macros that
 * must be defined and the structure-packing conventions that
 * must be set before including this file.
 */
```

## pkcs11t.h:36

```text
/* an unsigned 8-bit value */
```

## pkcs11t.h:39

```text
/* an unsigned 8-bit character */
```

## pkcs11t.h:42

```text
/* an 8-bit UTF-8 character */
```

## pkcs11t.h:45

```text
/* a BYTE-sized Boolean flag */
```

## pkcs11t.h:48

```text
/* an unsigned value, at least 32 bits long */
```

## pkcs11t.h:51

```text
/* a signed value, the same size as a CK_ULONG */
```

## pkcs11t.h:54

```text
/* at least 32 bits; each bit is a Boolean flag */
```

## pkcs11t.h:58

```text
/* some special values for certain CK_ULONG variables */
```

## pkcs11t.h:70

```text
/* Pointer to a CK_VOID_PTR-- i.e., pointer to pointer to void */
```

## pkcs11t.h:74

```text
/* The following value is always invalid if used as a session
 * handle or object handle
 */
```

## pkcs11t.h:81

```text
/* integer portion of version number */
```

## pkcs11t.h:82

```text
/* 1/100ths portion of version number */
```

## pkcs11t.h:89

```text
/* Cryptoki interface ver */
```

## pkcs11t.h:90

```text
/* blank padded */
```

## pkcs11t.h:91

```text
/* must be zero */
```

## pkcs11t.h:92

```text
/* blank padded */
```

## pkcs11t.h:93

```text
/* version of library */
```

## pkcs11t.h:99

```text
/* CK_NOTIFICATION enumerates the types of notifications that
 * Cryptoki provides to an application
 */
```

## pkcs11t.h:111

```text
/* CK_SLOT_INFO provides information about a slot */
```

## pkcs11t.h:113

```text
/* blank padded */
```

## pkcs11t.h:114

```text
/* blank padded */
```

## pkcs11t.h:117

```text
/* version of hardware */
```

## pkcs11t.h:118

```text
/* version of firmware */
```

## pkcs11t.h:121

```text
/* flags: bit flags that provide capabilities of the slot
 *      Bit Flag              Mask        Meaning
 */
```

## pkcs11t.h:124

```text
/* a token is there */
```

## pkcs11t.h:125

```text
/* removable devices*/
```

## pkcs11t.h:126

```text
/* hardware slot */
```

## pkcs11t.h:131

```text
/* CK_TOKEN_INFO provides information about a token */
```

## pkcs11t.h:133

```text
/* blank padded */
```

## pkcs11t.h:134

```text
/* blank padded */
```

## pkcs11t.h:135

```text
/* blank padded */
```

## pkcs11t.h:136

```text
/* blank padded */
```

## pkcs11t.h:137

```text
/* see below */
```

## pkcs11t.h:139

```text
/* max open sessions */
```

## pkcs11t.h:140

```text
/* sess. now open */
```

## pkcs11t.h:141

```text
/* max R/W sessions */
```

## pkcs11t.h:142

```text
/* R/W sess. now open */
```

## pkcs11t.h:143

```text
/* in bytes */
```

## pkcs11t.h:144

```text
/* in bytes */
```

## pkcs11t.h:145

```text
/* in bytes */
```

## pkcs11t.h:146

```text
/* in bytes */
```

## pkcs11t.h:147

```text
/* in bytes */
```

## pkcs11t.h:148

```text
/* in bytes */
```

## pkcs11t.h:149

```text
/* version of hardware */
```

## pkcs11t.h:150

```text
/* version of firmware */
```

## pkcs11t.h:151

```text
/* time */
```

## pkcs11t.h:154

```text
/* The flags parameter is defined as follows:
 *      Bit Flag                    Mask        Meaning
 */
```

## pkcs11t.h:157

```text
/* has random # generator */
```

## pkcs11t.h:158

```text
/* token is write-protected */
```

## pkcs11t.h:159

```text
/* user must login */
```

## pkcs11t.h:160

```text
/* normal user's PIN is set */
```

## pkcs11t.h:162

```text
/* CKF_RESTORE_KEY_NOT_NEEDED.  If it is set,
 * that means that *every* time the state of cryptographic
 * operations of a session is successfully saved, all keys
 * needed to continue those operations are stored in the state
 */
```

## pkcs11t.h:169

```text
/* CKF_CLOCK_ON_TOKEN.  If it is set, that means
 * that the token has some sort of clock.  The time on that
 * clock is returned in the token info structure
 */
```

## pkcs11t.h:175

```text
/* CKF_PROTECTED_AUTHENTICATION_PATH.  If it is
 * set, that means that there is some way for the user to login
 * without sending a PIN through the Cryptoki library itself
 */
```

## pkcs11t.h:181

```text
/* CKF_DUAL_CRYPTO_OPERATIONS.  If it is true,
 * that means that a single session with the token can perform
 * dual simultaneous cryptographic operations (digest and
 * encrypt; decrypt and digest; sign and encrypt; and decrypt
 * and sign)
 */
```

## pkcs11t.h:189

```text
/* CKF_TOKEN_INITIALIZED. If it is true, the
 * token has been initialized using C_InitializeToken or an
 * equivalent mechanism outside the scope of PKCS #11.
 * Calling C_InitializeToken when this flag is set will cause
 * the token to be reinitialized.
 */
```

## pkcs11t.h:197

```text
/* CKF_SECONDARY_AUTHENTICATION. If it is
 * true, the token supports secondary authentication for
 * private key objects.
 */
```

## pkcs11t.h:203

```text
/* CKF_USER_PIN_COUNT_LOW. If it is true, an
 * incorrect user login PIN has been entered at least once
 * since the last successful authentication.
 */
```

## pkcs11t.h:209

```text
/* CKF_USER_PIN_FINAL_TRY. If it is true,
 * supplying an incorrect user PIN will it to become locked.
 */
```

## pkcs11t.h:214

```text
/* CKF_USER_PIN_LOCKED. If it is true, the
 * user PIN has been locked. User login to the token is not
 * possible.
 */
```

## pkcs11t.h:220

```text
/* CKF_USER_PIN_TO_BE_CHANGED. If it is true,
 * the user PIN value is the default value set by token
 * initialization or manufacturing, or the PIN has been
 * expired by the card.
 */
```

## pkcs11t.h:227

```text
/* CKF_SO_PIN_COUNT_LOW. If it is true, an
 * incorrect SO login PIN has been entered at least once since
 * the last successful authentication.
 */
```

## pkcs11t.h:233

```text
/* CKF_SO_PIN_FINAL_TRY. If it is true,
 * supplying an incorrect SO PIN will it to become locked.
 */
```

## pkcs11t.h:238

```text
/* CKF_SO_PIN_LOCKED. If it is true, the SO
 * PIN has been locked. SO login to the token is not possible.
 */
```

## pkcs11t.h:243

```text
/* CKF_SO_PIN_TO_BE_CHANGED. If it is true,
 * the SO PIN value is the default value set by token
 * initialization or manufacturing, or the PIN has been
 * expired by the card.
 */
```

## pkcs11t.h:250

```text
/* CKF_ERROR_STATE. If it is true, the token failed a FIPS 140
 * self-test and entered an error state. */
```

## pkcs11t.h:254

```text
/*
 * CKF_SEED_RANDOM_REQUIRED. If this is true  the token’s
 * random number generator must be seeded or re-seeded using
 * C_SeedRandom. */
```

## pkcs11t.h:260

```text
/* CKF_ASYNC_SESSION_SUPPORTED. If this is true the token
 * supports asynchronous sessions. */
```

## pkcs11t.h:267

```text
/* CK_SESSION_HANDLE is a Cryptoki-assigned value that
 * identifies a session
 */
```

## pkcs11t.h:275

```text
/* CK_USER_TYPE enumerates the types of Cryptoki users */
```

## pkcs11t.h:277

```text
/* Security Officer */
```

## pkcs11t.h:279

```text
/* Normal user */
```

## pkcs11t.h:281

```text
/* Context specific */
```

## pkcs11t.h:284

```text
/* CK_STATE enumerates the session states */
```

## pkcs11t.h:292

```text
/* CK_SESSION_INFO provides information about a session */
```

## pkcs11t.h:296

```text
/* see below */
```

## pkcs11t.h:297

```text
/* device-dependent error code */
```

## pkcs11t.h:300

```text
/* The flags are defined in the following table:
 *      Bit Flag                    Mask         Meaning
 */
```

## pkcs11t.h:303

```text
/* session is r/w */
```

## pkcs11t.h:304

```text
/* no parallel    */
```

## pkcs11t.h:305

```text
/* session is async */
```

## pkcs11t.h:310

```text
/* CK_OBJECT_HANDLE is a token-specific identifier for an
 * object
 */
```

## pkcs11t.h:318

```text
/* CK_OBJECT_CLASS is a value that identifies the classes (or
 * types) of objects that Cryptoki recognizes.  It is defined
 * as follows:
 */
```

## pkcs11t.h:324

```text
/* The following classes of objects are defined: */
```

## pkcs11t.h:342

```text
/* Profile ID's */
```

## pkcs11t.h:343

```text
/* Profile */
```

## pkcs11t.h:344

```text
/* Profile */
```

## pkcs11t.h:345

```text
/* Profile */
```

## pkcs11t.h:346

```text
/* Profile */
```

## pkcs11t.h:347

```text
/* Profile */
```

## pkcs11t.h:348

```text
/* Profile */
```

## pkcs11t.h:349

```text
/* Profile */
```

## pkcs11t.h:350

```text
/* Profile */
```

## pkcs11t.h:353

```text
/* CK_HW_FEATURE_TYPE is a value that identifies the hardware feature type
 * of an object with CK_OBJECT_CLASS equal to CKO_HW_FEATURE.
 */
```

## pkcs11t.h:358

```text
/* The following hardware feature types are defined */
```

## pkcs11t.h:364

```text
/* CK_KEY_TYPE is a value that identifies a key type */
```

## pkcs11t.h:367

```text
/* the following key types are defined: */
```

## pkcs11t.h:371

```text
/* Deprecated */
```

## pkcs11t.h:374

```text
/* Historical */
```

## pkcs11t.h:376

```text
/* Historical */
```

## pkcs11t.h:377

```text
/* Historical */
```

## pkcs11t.h:378

```text
/* Historical */
```

## pkcs11t.h:381

```text
/* Historical */
```

## pkcs11t.h:382

```text
/* Historical */
```

## pkcs11t.h:383

```text
/* Deprecated */
```

## pkcs11t.h:384

```text
/* Historical */
```

## pkcs11t.h:385

```text
/* Historical */
```

## pkcs11t.h:386

```text
/* Historical */
```

## pkcs11t.h:387

```text
/* Historical */
```

## pkcs11t.h:388

```text
/* Historical */
```

## pkcs11t.h:389

```text
/* Historical */
```

## pkcs11t.h:390

```text
/* Historical */
```

## pkcs11t.h:395

```text
/* Historical */
```

## pkcs11t.h:396

```text
/* Historical */
```

## pkcs11t.h:400

```text
/* the following definitions were added in the 2.30 header file,
 * but never defined in the spec. */
```

## pkcs11t.h:402

```text
/* Historical */
```

## pkcs11t.h:404

```text
/* Historical */
```

## pkcs11t.h:405

```text
/* Historical */
```

## pkcs11t.h:447

```text
/* CK_CERTIFICATE_TYPE is a value that identifies a certificate
 * type
 */
```

## pkcs11t.h:463

```text
/* The following certificate types are defined: */
```

## pkcs11t.h:470

```text
/* CK_ATTRIBUTE_TYPE is a value that identifies an attribute
 * type
 */
```

## pkcs11t.h:475

```text
/* The CKF_ARRAY_ATTRIBUTE flag identifies an attribute which
 * consists of an array of values.
 */
```

## pkcs11t.h:480

```text
/* The following OTP-related defines relate to the CKA_OTP_FORMAT attribute */
```

## pkcs11t.h:486

```text
/* The following OTP-related defines relate to the CKA_OTP_..._REQUIREMENT
 * attributes
 */
```

## pkcs11t.h:493

```text
/* The following attribute types are defined: */
```

## pkcs11t.h:563

```text
/* Deprecated */
```

## pkcs11t.h:568

```text
/* Deprecated */
```

## pkcs11t.h:569

```text
/* Deprecated */
```

## pkcs11t.h:635

```text
/* HSS */
```

## pkcs11t.h:642

```text
/* new post-quantum (general) */
```

## pkcs11t.h:644

```text
/* validation objects */
```

## pkcs11t.h:657

```text
/* KEM */
```

## pkcs11t.h:660

```text
/* trust objects */
```

## pkcs11t.h:671

```text
/* linking pubic and private keys */
```

## pkcs11t.h:673

```text
/* new post-quantum (general) */
```

## pkcs11t.h:678

```text
/* CK_ATTRIBUTE is a structure that includes the type, length
 * and value of an attribute
 */
```

## pkcs11t.h:684

```text
/* in bytes */
```

## pkcs11t.h:689

```text
/* CK_DATE is a structure that defines a date */
```

## pkcs11t.h:691

```text
/* the year ("1900" - "9999") */
```

## pkcs11t.h:692

```text
/* the month ("01" - "12") */
```

## pkcs11t.h:693

```text
/* the day   ("01" - "31") */
```

## pkcs11t.h:697

```text
/* CK_MECHANISM_TYPE is a value that identifies a mechanism
 * type
 */
```

## pkcs11t.h:702

```text
/* the following mechanism types are defined: */
```

## pkcs11t.h:775

```text
/* Historical */
```

## pkcs11t.h:776

```text
/* Historical */
```

## pkcs11t.h:777

```text
/* Historical */
```

## pkcs11t.h:778

```text
/* Historical */
```

## pkcs11t.h:780

```text
/* Historical */
```

## pkcs11t.h:781

```text
/* Historical */
```

## pkcs11t.h:783

```text
/* Historical */
```

## pkcs11t.h:784

```text
/* Historical */
```

## pkcs11t.h:785

```text
/* Historical */
```

## pkcs11t.h:786

```text
/* Historical */
```

## pkcs11t.h:787

```text
/* Historical */
```

## pkcs11t.h:788

```text
/* Historical */
```

## pkcs11t.h:790

```text
/* Historical */
```

## pkcs11t.h:791

```text
/* Historical */
```

## pkcs11t.h:803

```text
/* Historical */
```

## pkcs11t.h:804

```text
/* Historical */
```

## pkcs11t.h:805

```text
/* Historical */
```

## pkcs11t.h:806

```text
/* Historical */
```

## pkcs11t.h:807

```text
/* Historical */
```

## pkcs11t.h:808

```text
/* Historical */
```

## pkcs11t.h:815

```text
/* Historical */
```

## pkcs11t.h:817

```text
/* Historical */
```

## pkcs11t.h:818

```text
/* Historical */
```

## pkcs11t.h:820

```text
/* Historical */
```

## pkcs11t.h:822

```text
/* Historical */
```

## pkcs11t.h:823

```text
/* Historical */
```

## pkcs11t.h:830

```text
/* Historical */
```

## pkcs11t.h:831

```text
/* Historical */
```

## pkcs11t.h:832

```text
/* Historical */
```

## pkcs11t.h:833

```text
/* Historical */
```

## pkcs11t.h:834

```text
/* Historical */
```

## pkcs11t.h:835

```text
/* Historical */
```

## pkcs11t.h:874

```text
/* Historical */
```

## pkcs11t.h:875

```text
/* Historical */
```

## pkcs11t.h:876

```text
/* Historical */
```

## pkcs11t.h:877

```text
/* Historical */
```

## pkcs11t.h:878

```text
/* Historical */
```

## pkcs11t.h:879

```text
/* Historical */
```

## pkcs11t.h:880

```text
/* Historical */
```

## pkcs11t.h:881

```text
/* Historical */
```

## pkcs11t.h:882

```text
/* Historical */
```

## pkcs11t.h:883

```text
/* Historical */
```

## pkcs11t.h:884

```text
/* Historical */
```

## pkcs11t.h:885

```text
/* Historical */
```

## pkcs11t.h:886

```text
/* Note that CAST128 and CAST5 are the same algorithm */
```

## pkcs11t.h:887

```text
/* Historical */
```

## pkcs11t.h:888

```text
/* Historical */
```

## pkcs11t.h:889

```text
/* Historical */
```

## pkcs11t.h:890

```text
/* Historical */
```

## pkcs11t.h:891

```text
/* Deprecated */
```

## pkcs11t.h:892

```text
/* Historical */
```

## pkcs11t.h:893

```text
/* Deprecated */
```

## pkcs11t.h:894

```text
/* Historical */
```

## pkcs11t.h:895

```text
/* Deprecated */
```

## pkcs11t.h:896

```text
/* Historical */
```

## pkcs11t.h:897

```text
/* Deprecated */
```

## pkcs11t.h:898

```text
/* Historical */
```

## pkcs11t.h:899

```text
/* Historical */
```

## pkcs11t.h:900

```text
/* Historical */
```

## pkcs11t.h:901

```text
/* Historical */
```

## pkcs11t.h:902

```text
/* Historical */
```

## pkcs11t.h:903

```text
/* Historical */
```

## pkcs11t.h:904

```text
/* Historical */
```

## pkcs11t.h:905

```text
/* Historical */
```

## pkcs11t.h:906

```text
/* Historical */
```

## pkcs11t.h:907

```text
/* Historical */
```

## pkcs11t.h:908

```text
/* Historical */
```

## pkcs11t.h:909

```text
/* Historical */
```

## pkcs11t.h:910

```text
/* Historical */
```

## pkcs11t.h:931

```text
/* Historical */
```

## pkcs11t.h:932

```text
/* Historical */
```

## pkcs11t.h:952

```text
/* Historical */
```

## pkcs11t.h:953

```text
/* Historical */
```

## pkcs11t.h:954

```text
/* Historical */
```

## pkcs11t.h:955

```text
/* Historical */
```

## pkcs11t.h:956

```text
/* Deprecated */
```

## pkcs11t.h:957

```text
/* Historical */
```

## pkcs11t.h:958

```text
/* Deprecated */
```

## pkcs11t.h:959

```text
/* Historical */
```

## pkcs11t.h:960

```text
/* Historical */
```

## pkcs11t.h:961

```text
/* Historical */
```

## pkcs11t.h:1005

```text
/* Historical */
```

## pkcs11t.h:1025

```text
/* Historical */
```

## pkcs11t.h:1026

```text
/* Historical */
```

## pkcs11t.h:1027

```text
/* Historical */
```

## pkcs11t.h:1028

```text
/* Historical */
```

## pkcs11t.h:1029

```text
/* Historical */
```

## pkcs11t.h:1030

```text
/* Historical */
```

## pkcs11t.h:1031

```text
/* Historical */
```

## pkcs11t.h:1032

```text
/* Historical */
```

## pkcs11t.h:1033

```text
/* Historical */
```

## pkcs11t.h:1034

```text
/* Historical */
```

## pkcs11t.h:1035

```text
/* Historical */
```

## pkcs11t.h:1036

```text
/* Historical */
```

## pkcs11t.h:1037

```text
/* Historical */
```

## pkcs11t.h:1038

```text
/* Historical */
```

## pkcs11t.h:1039

```text
/* Historical */
```

## pkcs11t.h:1040

```text
/* Historical */
```

## pkcs11t.h:1041

```text
/* Historical */
```

## pkcs11t.h:1042

```text
/* Historical */
```

## pkcs11t.h:1043

```text
/* Historical */
```

## pkcs11t.h:1044

```text
/* Historical */
```

## pkcs11t.h:1045

```text
/* Historical */
```

## pkcs11t.h:1046

```text
/* Historical */
```

## pkcs11t.h:1048

```text
/* Deprecated */
```

## pkcs11t.h:1066

```text
/* Historical */
```

## pkcs11t.h:1067

```text
/* Historical */
```

## pkcs11t.h:1068

```text
/* Historical */
```

## pkcs11t.h:1069

```text
/* Historical */
```

## pkcs11t.h:1070

```text
/* Historical */
```

## pkcs11t.h:1071

```text
/* Historical */
```

## pkcs11t.h:1127

```text
/* Depricated */
```

## pkcs11t.h:1137

```text
/* WAS: 0x00001090 */
```

## pkcs11t.h:1138

```text
/* WAS: 0x00001091 */
```

## pkcs11t.h:1255

```text
/* CK_MECHANISM is a structure that specifies a particular
 * mechanism
 */
```

## pkcs11t.h:1261

```text
/* in bytes */
```

## pkcs11t.h:1267

```text
/* CK_MECHANISM_INFO provides information about a particular
 * mechanism
 */
```

## pkcs11t.h:1276

```text
/* The flags are defined as follows:
 *      Bit Flag               Mask          Meaning */
```

## pkcs11t.h:1278

```text
/* performed by HW */
```

## pkcs11t.h:1280

```text
/* Specify whether or not a mechanism can be used for a particular task */
```

## pkcs11t.h:1302

```text
/* Describe a token's EC capabilities not available in mechanism
 * information.
 */
```

## pkcs11t.h:1309

```text
/* deprecated since PKCS#11 3.00 */
```

## pkcs11t.h:1321

```text
/* CK_RV is a value that identifies the return value of a
 * Cryptoki function
 */
```

## pkcs11t.h:1460

```text
/* CK_NOTIFY is an application callback that processes events */
```

## pkcs11t.h:1462

```text
/* the session's handle */
```

## pkcs11t.h:1464

```text
/* passed to C_OpenSession */
```

## pkcs11t.h:1468

```text
/* CK_FUNCTION_LIST is a structure holding a Cryptoki spec
 * version and pointers of appropriate types to all the
 * Cryptoki functions
 */
```

## pkcs11t.h:1496

```text
/* CK_CREATEMUTEX is an application callback for creating a
 * mutex object
 */
```

## pkcs11t.h:1500

```text
/* location to receive ptr to mutex */
```

## pkcs11t.h:1504

```text
/* CK_DESTROYMUTEX is an application callback for destroying a
 * mutex object
 */
```

## pkcs11t.h:1508

```text
/* pointer to mutex */
```

## pkcs11t.h:1512

```text
/* CK_LOCKMUTEX is an application callback for locking a mutex */
```

## pkcs11t.h:1514

```text
/* pointer to mutex */
```

## pkcs11t.h:1518

```text
/* CK_UNLOCKMUTEX is an application callback for unlocking a
 * mutex
 */
```

## pkcs11t.h:1522

```text
/* pointer to mutex */
```

## pkcs11t.h:1525

```text
/* Get functionlist flags */
```

## pkcs11t.h:1528

```text
/* CK_C_INITIALIZE_ARGS provides the optional arguments to
 * C_Initialize
 */
```

## pkcs11t.h:1540

```text
/* flags: bit flags that provide capabilities of the slot
 *      Bit Flag                           Mask       Meaning
 */
```

## pkcs11t.h:1550

```text
/* additional flags for parameters to functions */
```

## pkcs11t.h:1552

```text
/* CKF_DONT_BLOCK is for the function C_WaitForSlotEvent */
```

## pkcs11t.h:1555

```text
/* CK_RSA_PKCS_MGF_TYPE  is used to indicate the Message
 * Generation Function (MGF) applied to a message block when
 * formatting a message block for the PKCS #1 OAEP encryption
 * scheme.
 */
```

## pkcs11t.h:1564

```text
/* The following MGFs are defined */
```

## pkcs11t.h:1576

```text
/* CK_RSA_PKCS_OAEP_SOURCE_TYPE  is used to indicate the source
 * of the encoding parameter when formatting a message block
 * for the PKCS #1 OAEP encryption scheme.
 */
```

## pkcs11t.h:1584

```text
/* The following encoding parameter sources are defined */
```

## pkcs11t.h:1587

```text
/* CK_RSA_PKCS_OAEP_PARAMS provides the parameters to the
 * CKM_RSA_PKCS_OAEP mechanism.
 */
```

## pkcs11t.h:1600

```text
/* CK_RSA_PKCS_PSS_PARAMS provides the parameters to the
 * CKM_RSA_PKCS_PSS mechanism(s).
 */
```

## pkcs11t.h:1614

```text
/* The following EC Key Derivation Functions are defined */
```

## pkcs11t.h:1618

```text
/* The following X9.42 DH key derivation functions are defined */
```

## pkcs11t.h:1644

```text
/* CK_ECDH1_DERIVE_PARAMS provides the parameters to the
 * CKM_ECDH1_DERIVE and CKM_ECDH1_COFACTOR_DERIVE mechanisms,
 * where each party contributes one key pair.
 */
```

## pkcs11t.h:1658

```text
/*
 * CK_ECDH2_DERIVE_PARAMS provides the parameters to the
 * CKM_ECMQV_DERIVE mechanism, where each party contributes two key pairs.
 */
```

## pkcs11t.h:1691

```text
/* Typedefs and defines for the CKM_X9_42_DH_KEY_PAIR_GEN and the
 * CKM_X9_42_DH_PARAMETER_GEN mechanisms
 */
```

## pkcs11t.h:1697

```text
/* CK_X9_42_DH1_DERIVE_PARAMS provides the parameters to the
 * CKM_X9_42_DH_DERIVE key derivation mechanism, where each party
 * contributes one key pair
 */
```

## pkcs11t.h:1711

```text
/* CK_X9_42_DH2_DERIVE_PARAMS provides the parameters to the
 * CKM_X9_42_DH_HYBRID_DERIVE and CKM_X9_42_MQV_DERIVE key derivation
 * mechanisms, where each party contributes two key pairs
 */
```

## pkcs11t.h:1744

```text
/* CK_KEA_DERIVE_PARAMS provides the parameters to the
 * CKM_KEA_DERIVE mechanism
 */
```

## pkcs11t.h:1759

```text
/* CK_RC2_PARAMS provides the parameters to the CKM_RC2_ECB and
 * CKM_RC2_MAC mechanisms.  An instance of CK_RC2_PARAMS just
 * holds the effective keysize
 */
```

## pkcs11t.h:1768

```text
/* CK_RC2_CBC_PARAMS provides the parameters to the CKM_RC2_CBC
 * mechanism
 */
```

## pkcs11t.h:1772

```text
/* effective bits (1-1024) */
```

## pkcs11t.h:1773

```text
/* IV for CBC mode */
```

## pkcs11t.h:1779

```text
/* CK_RC2_MAC_GENERAL_PARAMS provides the parameters for the
 * CKM_RC2_MAC_GENERAL mechanism
 */
```

## pkcs11t.h:1783

```text
/* effective bits (1-1024) */
```

## pkcs11t.h:1784

```text
/* Length of MAC in bytes */
```

## pkcs11t.h:1791

```text
/* CK_RC5_PARAMS provides the parameters to the CKM_RC5_ECB and
 * CKM_RC5_MAC mechanisms
 */
```

## pkcs11t.h:1795

```text
/* wordsize in bits */
```

## pkcs11t.h:1796

```text
/* number of rounds */
```

## pkcs11t.h:1802

```text
/* CK_RC5_CBC_PARAMS provides the parameters to the CKM_RC5_CBC
 * mechanism
 */
```

## pkcs11t.h:1806

```text
/* wordsize in bits */
```

## pkcs11t.h:1807

```text
/* number of rounds */
```

## pkcs11t.h:1808

```text
/* pointer to IV */
```

## pkcs11t.h:1809

```text
/* length of IV in bytes */
```

## pkcs11t.h:1815

```text
/* CK_RC5_MAC_GENERAL_PARAMS provides the parameters for the
 * CKM_RC5_MAC_GENERAL mechanism
 */
```

## pkcs11t.h:1819

```text
/* wordsize in bits */
```

## pkcs11t.h:1820

```text
/* number of rounds */
```

## pkcs11t.h:1821

```text
/* Length of MAC in bytes */
```

## pkcs11t.h:1827

```text
/* CK_MAC_GENERAL_PARAMS provides the parameters to most block
 * ciphers' MAC_GENERAL mechanisms.  Its value is the length of
 * the MAC
 */
```

## pkcs11t.h:1851

```text
/* CK_SKIPJACK_PRIVATE_WRAP_PARAMS provides the parameters to the
 * CKM_SKIPJACK_PRIVATE_WRAP mechanism
 */
```

## pkcs11t.h:1872

```text
/* CK_SKIPJACK_RELAYX_PARAMS provides the parameters to the
 * CKM_SKIPJACK_RELAYX mechanism
 */
```

## pkcs11t.h:1908

```text
/* CK_KEY_WRAP_SET_OAEP_PARAMS provides the parameters to the
 * CKM_KEY_WRAP_SET_OAEP mechanism
 */
```

## pkcs11t.h:1912

```text
/* block contents byte */
```

## pkcs11t.h:1913

```text
/* extra data */
```

## pkcs11t.h:1914

```text
/* length of extra data in bytes */
```

## pkcs11t.h:2042

```text
/* The CK_EXTRACT_PARAMS is used for the
 * CKM_EXTRACT_KEY_FROM_KEY mechanism.  It specifies which bit
 * of the base key should be used as the first bit of the
 * derived key
 */
```

## pkcs11t.h:2051

```text
/* CK_PKCS5_PBKD2_PSEUDO_RANDOM_FUNCTION_TYPE is used to
 * indicate the Pseudo-Random Function (PRF) used to generate
 * key bits using PKCS #5 PBKDF2.
 */
```

## pkcs11t.h:2069

```text
/* CK_PKCS5_PBKDF2_SALT_SOURCE_TYPE is used to indicate the
 * source of the salt value when deriving a key using PKCS #5
 * PBKDF2.
 */
```

## pkcs11t.h:2078

```text
/* The following salt value sources are defined in PKCS #5 v2.0. */
```

## pkcs11t.h:2081

```text
/* CK_PKCS5_PBKD2_PARAMS is a structure that provides the
 * parameters to the CKM_PKCS5_PBKD2 mechanism.
 */
```

## pkcs11t.h:2098

```text
/* CK_PKCS5_PBKD2_PARAMS2 is a corrected version of the CK_PKCS5_PBKD2_PARAMS
 * structure that provides the parameters to the CKM_PKCS5_PBKD2 mechanism
 * noting that the ulPasswordLen field is a CK_ULONG and not a CK_ULONG_PTR.
 */
```

## pkcs11t.h:2117

```text
/* backward compatibility */
```

## pkcs11t.h:2226

```text
/*plaintext or ciphertext*/
```

## pkcs11t.h:2238

```text
/*wrappedkey data*/
```

## pkcs11t.h:2250

```text
/* Deprecated. Use CK_GCM_PARAMS */
```

## pkcs11t.h:2262

```text
/* Deprecated. Use CK_CCM_PARAMS */
```

## pkcs11t.h:2407

```text
/*
 * New PKCS 11 v3.0 data structures.
 */
```

## pkcs11t.h:2414

```text
/* Typedefs for Flexible KDF */
```

## pkcs11t.h:2490

```text
/* EDDSA */
```

## pkcs11t.h:2499

```text
/* Extended ChaCha20/Salsa20 support*/
```

## pkcs11t.h:2538

```text
/* X3dh, ratchet */
```

## pkcs11t.h:2591

```text
/* XEDDSA */
```

## pkcs11t.h:2597

```text
/* HKDF params */
```

## pkcs11t.h:2615

```text
/* HSS */
```

## pkcs11t.h:2626

```text
/* IKE Params */
```

## pkcs11t.h:2670

```text
/* async */
```

## pkcs11t.h:2680

```text
/* validation */
```

## pkcs11t.h:2698

```text
/* XMSS */
```

## pkcs11t.h:2702

```text
/* generic PQ mechanism parameters */
```

## pkcs11t.h:2722

```text
/* ML-DSA values for CKA_PARAMETER_SETS */
```

## pkcs11t.h:2728

```text
/* SLH-DSA values for CKA_PARAMETER_SETS */
```

## pkcs11t.h:2743

```text
/* ML-KEM values for CKA_PARAMETER_SETS */
```

## pkcs11t.h:2749

```text
/* Trust values for CKA_TRUST_* */
```

## pkcs11t.h:2758

```text
/* _PKCS11T_H_ */
```

## pkcs11f.h:1

```text
/* Copyright (c) OASIS Open 2016, 2019, 2024. All Rights Reserved./
 * /Distributed under the terms of the OASIS IPR Policy,
 * [http://www.oasis-open.org/policies-guidelines/ipr], AS-IS, WITHOUT ANY
 * IMPLIED OR EXPRESS WARRANTY; there is no warranty of MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE or NONINFRINGEMENT of the rights of others.
 */
```

## pkcs11f.h:8

```text
/* Latest version of the specification:
 * http://docs.oasis-open.org/pkcs11/pkcs11-spec/v3.2/pkcs11-spec-v3.2.html
 */
```

## pkcs11f.h:12

```text
/* This header file contains pretty much everything about all the
 * Cryptoki function prototypes.  Because this information is
 * used for more than just declaring function prototypes, the
 * order of the functions appearing herein is important, and
 * should not be altered.
 */
```

## pkcs11f.h:19

```text
/* General-purpose */
```

## pkcs11f.h:21

```text
/* C_Initialize initializes the Cryptoki library. */
```

## pkcs11f.h:25

```text
/* if this is not NULL_PTR, it gets
                            * cast to CK_C_INITIALIZE_ARGS_PTR
                            * and dereferenced
                            */
```

## pkcs11f.h:33

```text
/* C_Finalize indicates that an application is done with the
 * Cryptoki library.
 */
```

## pkcs11f.h:39

```text
/* reserved.  Should be NULL_PTR */
```

## pkcs11f.h:44

```text
/* C_GetInfo returns general information about Cryptoki. */
```

## pkcs11f.h:48

```text
/* location that receives information */
```

## pkcs11f.h:53

```text
/* C_GetFunctionList returns the function list. */
```

## pkcs11f.h:57

```text
/* receives pointer to
                                            * function list
                                            */
```

## pkcs11f.h:65

```text
/* Slot and token management */
```

## pkcs11f.h:67

```text
/* C_GetSlotList obtains a list of slots in the system. */
```

## pkcs11f.h:71

```text
/* only slots with tokens */
```

## pkcs11f.h:72

```text
/* receives array of slot IDs */
```

## pkcs11f.h:73

```text
/* receives number of slots */
```

## pkcs11f.h:78

```text
/* C_GetSlotInfo obtains information about a particular slot in
 * the system.
 */
```

## pkcs11f.h:84

```text
/* the ID of the slot */
```

## pkcs11f.h:85

```text
/* receives the slot information */
```

## pkcs11f.h:90

```text
/* C_GetTokenInfo obtains information about a particular token
 * in the system.
 */
```

## pkcs11f.h:96

```text
/* ID of the token's slot */
```

## pkcs11f.h:97

```text
/* receives the token information */
```

## pkcs11f.h:102

```text
/* C_GetMechanismList obtains a list of mechanism types
 * supported by a token.
 */
```

## pkcs11f.h:108

```text
/* ID of token's slot */
```

## pkcs11f.h:109

```text
/* gets mech. array */
```

## pkcs11f.h:110

```text
/* gets # of mechs. */
```

## pkcs11f.h:115

```text
/* C_GetMechanismInfo obtains information about a particular
 * mechanism possibly supported by a token.
 */
```

## pkcs11f.h:121

```text
/* ID of the token's slot */
```

## pkcs11f.h:122

```text
/* type of mechanism */
```

## pkcs11f.h:123

```text
/* receives mechanism info */
```

## pkcs11f.h:128

```text
/* C_InitToken initializes a token. */
```

## pkcs11f.h:132

```text
/* ID of the token's slot */
```

## pkcs11f.h:133

```text
/* the SO's initial PIN */
```

## pkcs11f.h:134

```text
/* length in bytes of the PIN */
```

## pkcs11f.h:135

```text
/* 32-byte token label (blank padded) */
```

## pkcs11f.h:140

```text
/* C_InitPIN initializes the normal user's PIN. */
```

## pkcs11f.h:144

```text
/* the session's handle */
```

## pkcs11f.h:145

```text
/* the normal user's PIN */
```

## pkcs11f.h:146

```text
/* length in bytes of the PIN */
```

## pkcs11f.h:151

```text
/* C_SetPIN modifies the PIN of the user who is logged in. */
```

## pkcs11f.h:155

```text
/* the session's handle */
```

## pkcs11f.h:156

```text
/* the old PIN */
```

## pkcs11f.h:157

```text
/* length of the old PIN */
```

## pkcs11f.h:158

```text
/* the new PIN */
```

## pkcs11f.h:159

```text
/* length of the new PIN */
```

## pkcs11f.h:165

```text
/* Session management */
```

## pkcs11f.h:167

```text
/* C_OpenSession opens a session between an application and a
 * token.
 */
```

## pkcs11f.h:173

```text
/* the slot's ID */
```

## pkcs11f.h:174

```text
/* from CK_SESSION_INFO */
```

## pkcs11f.h:175

```text
/* passed to callback */
```

## pkcs11f.h:176

```text
/* callback function */
```

## pkcs11f.h:177

```text
/* gets session handle */
```

## pkcs11f.h:182

```text
/* C_CloseSession closes a session between an application and a
 * token.
 */
```

## pkcs11f.h:188

```text
/* the session's handle */
```

## pkcs11f.h:193

```text
/* C_CloseAllSessions closes all sessions with a token. */
```

## pkcs11f.h:197

```text
/* the token's slot */
```

## pkcs11f.h:202

```text
/* C_GetSessionInfo obtains information about the session. */
```

## pkcs11f.h:206

```text
/* the session's handle */
```

## pkcs11f.h:207

```text
/* receives session info */
```

## pkcs11f.h:212

```text
/* C_GetOperationState obtains the state of the cryptographic operation
 * in a session.
 */
```

## pkcs11f.h:218

```text
/* session's handle */
```

## pkcs11f.h:219

```text
/* gets state */
```

## pkcs11f.h:220

```text
/* gets state length */
```

## pkcs11f.h:225

```text
/* C_SetOperationState restores the state of the cryptographic
 * operation in a session.
 */
```

## pkcs11f.h:231

```text
/* session's handle */
```

## pkcs11f.h:232

```text
/* holds state */
```

## pkcs11f.h:233

```text
/* holds state length */
```

## pkcs11f.h:234

```text
/* en/decryption key */
```

## pkcs11f.h:235

```text
/* sign/verify key */
```

## pkcs11f.h:240

```text
/* C_Login logs a user into a token. */
```

## pkcs11f.h:244

```text
/* the session's handle */
```

## pkcs11f.h:245

```text
/* the user type */
```

## pkcs11f.h:246

```text
/* the user's PIN */
```

## pkcs11f.h:247

```text
/* the length of the PIN */
```

## pkcs11f.h:252

```text
/* C_Logout logs a user out from a token. */
```

## pkcs11f.h:256

```text
/* the session's handle */
```

## pkcs11f.h:262

```text
/* Object management */
```

## pkcs11f.h:264

```text
/* C_CreateObject creates a new object. */
```

## pkcs11f.h:268

```text
/* the session's handle */
```

## pkcs11f.h:269

```text
/* the object's template */
```

## pkcs11f.h:270

```text
/* attributes in template */
```

## pkcs11f.h:271

```text
/* gets new object's handle. */
```

## pkcs11f.h:276

```text
/* C_CopyObject copies an object, creating a new object for the
 * copy.
 */
```

## pkcs11f.h:282

```text
/* the session's handle */
```

## pkcs11f.h:283

```text
/* the object's handle */
```

## pkcs11f.h:284

```text
/* template for new object */
```

## pkcs11f.h:285

```text
/* attributes in template */
```

## pkcs11f.h:286

```text
/* receives handle of copy */
```

## pkcs11f.h:291

```text
/* C_DestroyObject destroys an object. */
```

## pkcs11f.h:295

```text
/* the session's handle */
```

## pkcs11f.h:296

```text
/* the object's handle */
```

## pkcs11f.h:301

```text
/* C_GetObjectSize gets the size of an object in bytes. */
```

## pkcs11f.h:305

```text
/* the session's handle */
```

## pkcs11f.h:306

```text
/* the object's handle */
```

## pkcs11f.h:307

```text
/* receives size of object */
```

## pkcs11f.h:312

```text
/* C_GetAttributeValue obtains the value of one or more object
 * attributes.
 */
```

## pkcs11f.h:318

```text
/* the session's handle */
```

## pkcs11f.h:319

```text
/* the object's handle */
```

## pkcs11f.h:320

```text
/* specifies attrs; gets vals */
```

## pkcs11f.h:321

```text
/* attributes in template */
```

## pkcs11f.h:326

```text
/* C_SetAttributeValue modifies the value of one or more object
 * attributes.
 */
```

## pkcs11f.h:332

```text
/* the session's handle */
```

## pkcs11f.h:333

```text
/* the object's handle */
```

## pkcs11f.h:334

```text
/* specifies attrs and values */
```

## pkcs11f.h:335

```text
/* attributes in template */
```

## pkcs11f.h:340

```text
/* C_FindObjectsInit initializes a search for token and session
 * objects that match a template.
 */
```

## pkcs11f.h:346

```text
/* the session's handle */
```

## pkcs11f.h:347

```text
/* attribute values to match */
```

## pkcs11f.h:348

```text
/* attrs in search template */
```

## pkcs11f.h:353

```text
/* C_FindObjects continues a search for token and session
 * objects that match a template, obtaining additional object
 * handles.
 */
```

## pkcs11f.h:360

```text
/* session's handle */
```

## pkcs11f.h:361

```text
/* gets obj. handles */
```

## pkcs11f.h:362

```text
/* max handles to get */
```

## pkcs11f.h:363

```text
/* actual # returned */
```

## pkcs11f.h:368

```text
/* C_FindObjectsFinal finishes a search for token and session
 * objects.
 */
```

## pkcs11f.h:374

```text
/* the session's handle */
```

## pkcs11f.h:380

```text
/* Encryption and decryption */
```

## pkcs11f.h:382

```text
/* C_EncryptInit initializes an encryption operation. */
```

## pkcs11f.h:386

```text
/* the session's handle */
```

## pkcs11f.h:387

```text
/* the encryption mechanism */
```

## pkcs11f.h:388

```text
/* handle of encryption key */
```

## pkcs11f.h:393

```text
/* C_Encrypt encrypts single-part data. */
```

## pkcs11f.h:397

```text
/* session's handle */
```

## pkcs11f.h:398

```text
/* the plaintext data */
```

## pkcs11f.h:399

```text
/* bytes of plaintext */
```

## pkcs11f.h:400

```text
/* gets ciphertext */
```

## pkcs11f.h:401

```text
/* gets c-text size */
```

## pkcs11f.h:406

```text
/* C_EncryptUpdate continues a multiple-part encryption
 * operation.
 */
```

## pkcs11f.h:412

```text
/* session's handle */
```

## pkcs11f.h:413

```text
/* the plaintext data */
```

## pkcs11f.h:414

```text
/* plaintext data len */
```

## pkcs11f.h:415

```text
/* gets ciphertext */
```

## pkcs11f.h:416

```text
/* gets c-text size */
```

## pkcs11f.h:421

```text
/* C_EncryptFinal finishes a multiple-part encryption
 * operation.
 */
```

## pkcs11f.h:427

```text
/* session handle */
```

## pkcs11f.h:428

```text
/* last c-text */
```

## pkcs11f.h:429

```text
/* gets last size */
```

## pkcs11f.h:434

```text
/* C_DecryptInit initializes a decryption operation. */
```

## pkcs11f.h:438

```text
/* the session's handle */
```

## pkcs11f.h:439

```text
/* the decryption mechanism */
```

## pkcs11f.h:440

```text
/* handle of decryption key */
```

## pkcs11f.h:445

```text
/* C_Decrypt decrypts encrypted data in a single part. */
```

## pkcs11f.h:449

```text
/* session's handle */
```

## pkcs11f.h:450

```text
/* ciphertext */
```

## pkcs11f.h:451

```text
/* ciphertext length */
```

## pkcs11f.h:452

```text
/* gets plaintext */
```

## pkcs11f.h:453

```text
/* gets p-text size */
```

## pkcs11f.h:458

```text
/* C_DecryptUpdate continues a multiple-part decryption
 * operation.
 */
```

## pkcs11f.h:464

```text
/* session's handle */
```

## pkcs11f.h:465

```text
/* encrypted data */
```

## pkcs11f.h:466

```text
/* input length */
```

## pkcs11f.h:467

```text
/* gets plaintext */
```

## pkcs11f.h:468

```text
/* p-text size */
```

## pkcs11f.h:473

```text
/* C_DecryptFinal finishes a multiple-part decryption
 * operation.
 */
```

## pkcs11f.h:479

```text
/* the session's handle */
```

## pkcs11f.h:480

```text
/* gets plaintext */
```

## pkcs11f.h:481

```text
/* p-text size */
```

## pkcs11f.h:487

```text
/* Message digesting */
```

## pkcs11f.h:489

```text
/* C_DigestInit initializes a message-digesting operation. */
```

## pkcs11f.h:493

```text
/* the session's handle */
```

## pkcs11f.h:494

```text
/* the digesting mechanism */
```

## pkcs11f.h:499

```text
/* C_Digest digests data in a single part. */
```

## pkcs11f.h:503

```text
/* the session's handle */
```

## pkcs11f.h:504

```text
/* data to be digested */
```

## pkcs11f.h:505

```text
/* bytes of data to digest */
```

## pkcs11f.h:506

```text
/* gets the message digest */
```

## pkcs11f.h:507

```text
/* gets digest length */
```

## pkcs11f.h:512

```text
/* C_DigestUpdate continues a multiple-part message-digesting
 * operation.
 */
```

## pkcs11f.h:518

```text
/* the session's handle */
```

## pkcs11f.h:519

```text
/* data to be digested */
```

## pkcs11f.h:520

```text
/* bytes of data to be digested */
```

## pkcs11f.h:525

```text
/* C_DigestKey continues a multi-part message-digesting
 * operation, by digesting the value of a secret key as part of
 * the data already digested.
 */
```

## pkcs11f.h:532

```text
/* the session's handle */
```

## pkcs11f.h:533

```text
/* secret key to digest */
```

## pkcs11f.h:538

```text
/* C_DigestFinal finishes a multiple-part message-digesting
 * operation.
 */
```

## pkcs11f.h:544

```text
/* the session's handle */
```

## pkcs11f.h:545

```text
/* gets the message digest */
```

## pkcs11f.h:546

```text
/* gets byte count of digest */
```

## pkcs11f.h:552

```text
/* Signing and MACing */
```

## pkcs11f.h:554

```text
/* C_SignInit initializes a signature (private key encryption)
 * operation, where the signature is (will be) an appendix to
 * the data, and plaintext cannot be recovered from the
 * signature.
 */
```

## pkcs11f.h:562

```text
/* the session's handle */
```

## pkcs11f.h:563

```text
/* the signature mechanism */
```

## pkcs11f.h:564

```text
/* handle of signature key */
```

## pkcs11f.h:569

```text
/* C_Sign signs (encrypts with private key) data in a single
 * part, where the signature is (will be) an appendix to the
 * data, and plaintext cannot be recovered from the signature.
 */
```

## pkcs11f.h:576

```text
/* the session's handle */
```

## pkcs11f.h:577

```text
/* the data to sign */
```

## pkcs11f.h:578

```text
/* count of bytes to sign */
```

## pkcs11f.h:579

```text
/* gets the signature */
```

## pkcs11f.h:580

```text
/* gets signature length */
```

## pkcs11f.h:585

```text
/* C_SignUpdate continues a multiple-part signature operation,
 * where the signature is (will be) an appendix to the data,
 * and plaintext cannot be recovered from the signature.
 */
```

## pkcs11f.h:592

```text
/* the session's handle */
```

## pkcs11f.h:593

```text
/* the data to sign */
```

## pkcs11f.h:594

```text
/* count of bytes to sign */
```

## pkcs11f.h:599

```text
/* C_SignFinal finishes a multiple-part signature operation,
 * returning the signature.
 */
```

## pkcs11f.h:605

```text
/* the session's handle */
```

## pkcs11f.h:606

```text
/* gets the signature */
```

## pkcs11f.h:607

```text
/* gets signature length */
```

## pkcs11f.h:612

```text
/* C_SignRecoverInit initializes a signature operation, where
 * the data can be recovered from the signature.
 */
```

## pkcs11f.h:618

```text
/* the session's handle */
```

## pkcs11f.h:619

```text
/* the signature mechanism */
```

## pkcs11f.h:620

```text
/* handle of the signature key */
```

## pkcs11f.h:625

```text
/* C_SignRecover signs data in a single operation, where the
 * data can be recovered from the signature.
 */
```

## pkcs11f.h:631

```text
/* the session's handle */
```

## pkcs11f.h:632

```text
/* the data to sign */
```

## pkcs11f.h:633

```text
/* count of bytes to sign */
```

## pkcs11f.h:634

```text
/* gets the signature */
```

## pkcs11f.h:635

```text
/* gets signature length */
```

## pkcs11f.h:641

```text
/* Verifying signatures and MACs */
```

## pkcs11f.h:643

```text
/* C_VerifyInit initializes a verification operation, where the
 * signature is an appendix to the data, and plaintext cannot
 * cannot be recovered from the signature (e.g. DSA).
 */
```

## pkcs11f.h:650

```text
/* the session's handle */
```

## pkcs11f.h:651

```text
/* the verification mechanism */
```

## pkcs11f.h:652

```text
/* verification key */
```

## pkcs11f.h:657

```text
/* C_Verify verifies a signature in a single-part operation,
 * where the signature is an appendix to the data, and plaintext
 * cannot be recovered from the signature.
 */
```

## pkcs11f.h:664

```text
/* the session's handle */
```

## pkcs11f.h:665

```text
/* signed data */
```

## pkcs11f.h:666

```text
/* length of signed data */
```

## pkcs11f.h:667

```text
/* signature */
```

## pkcs11f.h:668

```text
/* signature length*/
```

## pkcs11f.h:673

```text
/* C_VerifyUpdate continues a multiple-part verification
 * operation, where the signature is an appendix to the data,
 * and plaintext cannot be recovered from the signature.
 */
```

## pkcs11f.h:680

```text
/* the session's handle */
```

## pkcs11f.h:681

```text
/* signed data */
```

## pkcs11f.h:682

```text
/* length of signed data */
```

## pkcs11f.h:687

```text
/* C_VerifyFinal finishes a multiple-part verification
 * operation, checking the signature.
 */
```

## pkcs11f.h:693

```text
/* the session's handle */
```

## pkcs11f.h:694

```text
/* signature to verify */
```

## pkcs11f.h:695

```text
/* signature length */
```

## pkcs11f.h:700

```text
/* C_VerifyRecoverInit initializes a signature verification
 * operation, where the data is recovered from the signature.
 */
```

## pkcs11f.h:706

```text
/* the session's handle */
```

## pkcs11f.h:707

```text
/* the verification mechanism */
```

## pkcs11f.h:708

```text
/* verification key */
```

## pkcs11f.h:713

```text
/* C_VerifyRecover verifies a signature in a single-part
 * operation, where the data is recovered from the signature.
 */
```

## pkcs11f.h:719

```text
/* the session's handle */
```

## pkcs11f.h:720

```text
/* signature to verify */
```

## pkcs11f.h:721

```text
/* signature length */
```

## pkcs11f.h:722

```text
/* gets signed data */
```

## pkcs11f.h:723

```text
/* gets signed data len */
```

## pkcs11f.h:729

```text
/* Dual-function cryptographic operations */
```

## pkcs11f.h:731

```text
/* C_DigestEncryptUpdate continues a multiple-part digesting
 * and encryption operation.
 */
```

## pkcs11f.h:737

```text
/* session's handle */
```

## pkcs11f.h:738

```text
/* the plaintext data */
```

## pkcs11f.h:739

```text
/* plaintext length */
```

## pkcs11f.h:740

```text
/* gets ciphertext */
```

## pkcs11f.h:741

```text
/* gets c-text length */
```

## pkcs11f.h:746

```text
/* C_DecryptDigestUpdate continues a multiple-part decryption and
 * digesting operation.
 */
```

## pkcs11f.h:752

```text
/* session's handle */
```

## pkcs11f.h:753

```text
/* ciphertext */
```

## pkcs11f.h:754

```text
/* ciphertext length */
```

## pkcs11f.h:755

```text
/* gets plaintext */
```

## pkcs11f.h:756

```text
/* gets plaintext len */
```

## pkcs11f.h:761

```text
/* C_SignEncryptUpdate continues a multiple-part signing and
 * encryption operation.
 */
```

## pkcs11f.h:767

```text
/* session's handle */
```

## pkcs11f.h:768

```text
/* the plaintext data */
```

## pkcs11f.h:769

```text
/* plaintext length */
```

## pkcs11f.h:770

```text
/* gets ciphertext */
```

## pkcs11f.h:771

```text
/* gets c-text length */
```

## pkcs11f.h:776

```text
/* C_DecryptVerifyUpdate continues a multiple-part decryption and
 * verify operation.
 */
```

## pkcs11f.h:782

```text
/* session's handle */
```

## pkcs11f.h:783

```text
/* ciphertext */
```

## pkcs11f.h:784

```text
/* ciphertext length */
```

## pkcs11f.h:785

```text
/* gets plaintext */
```

## pkcs11f.h:786

```text
/* gets p-text length */
```

## pkcs11f.h:792

```text
/* Key management */
```

## pkcs11f.h:794

```text
/* C_GenerateKey generates a secret key, creating a new key
 * object.
 */
```

## pkcs11f.h:800

```text
/* the session's handle */
```

## pkcs11f.h:801

```text
/* key generation mech. */
```

## pkcs11f.h:802

```text
/* template for new key */
```

## pkcs11f.h:803

```text
/* # of attrs in template */
```

## pkcs11f.h:804

```text
/* gets handle of new key */
```

## pkcs11f.h:809

```text
/* C_GenerateKeyPair generates a public-key/private-key pair,
 * creating new key objects.
 */
```

## pkcs11f.h:815

```text
/* session handle */
```

## pkcs11f.h:816

```text
/* key-gen mech. */
```

## pkcs11f.h:817

```text
/* template for pub. key */
```

## pkcs11f.h:818

```text
/* # pub. attrs. */
```

## pkcs11f.h:819

```text
/* template for priv. key */
```

## pkcs11f.h:820

```text
/* # priv.  attrs. */
```

## pkcs11f.h:821

```text
/* gets pub. key handle */
```

## pkcs11f.h:822

```text
/* gets priv. key handle */
```

## pkcs11f.h:827

```text
/* C_WrapKey wraps (i.e., encrypts) a key. */
```

## pkcs11f.h:831

```text
/* the session's handle */
```

## pkcs11f.h:832

```text
/* the wrapping mechanism */
```

## pkcs11f.h:833

```text
/* wrapping key */
```

## pkcs11f.h:834

```text
/* key to be wrapped */
```

## pkcs11f.h:835

```text
/* gets wrapped key */
```

## pkcs11f.h:836

```text
/* gets wrapped key size */
```

## pkcs11f.h:841

```text
/* C_UnwrapKey unwraps (decrypts) a wrapped key, creating a new
 * key object.
 */
```

## pkcs11f.h:847

```text
/* session's handle */
```

## pkcs11f.h:848

```text
/* unwrapping mech. */
```

## pkcs11f.h:849

```text
/* unwrapping key */
```

## pkcs11f.h:850

```text
/* the wrapped key */
```

## pkcs11f.h:851

```text
/* wrapped key len */
```

## pkcs11f.h:852

```text
/* new key template */
```

## pkcs11f.h:853

```text
/* template length */
```

## pkcs11f.h:854

```text
/* gets new handle */
```

## pkcs11f.h:859

```text
/* C_DeriveKey derives a key from a base key, creating a new key
 * object.
 */
```

## pkcs11f.h:865

```text
/* session's handle */
```

## pkcs11f.h:866

```text
/* key deriv. mech. */
```

## pkcs11f.h:867

```text
/* base key */
```

## pkcs11f.h:868

```text
/* new key template */
```

## pkcs11f.h:869

```text
/* template length */
```

## pkcs11f.h:870

```text
/* gets new handle */
```

## pkcs11f.h:876

```text
/* Random number generation */
```

## pkcs11f.h:878

```text
/* C_SeedRandom mixes additional seed material into the token's
 * random number generator.
 */
```

## pkcs11f.h:884

```text
/* the session's handle */
```

## pkcs11f.h:885

```text
/* the seed material */
```

## pkcs11f.h:886

```text
/* length of seed material */
```

## pkcs11f.h:891

```text
/* C_GenerateRandom generates random data. */
```

## pkcs11f.h:895

```text
/* the session's handle */
```

## pkcs11f.h:896

```text
/* receives the random data */
```

## pkcs11f.h:897

```text
/* # of bytes to generate */
```

## pkcs11f.h:903

```text
/* Parallel function management */
```

## pkcs11f.h:905

```text
/* C_GetFunctionStatus is a legacy function; it obtains an
 * updated status of a function running in parallel with an
 * application.
 */
```

## pkcs11f.h:912

```text
/* the session's handle */
```

## pkcs11f.h:917

```text
/* C_CancelFunction is a legacy function; it cancels a function
 * running in parallel.
 */
```

## pkcs11f.h:923

```text
/* the session's handle */
```

## pkcs11f.h:928

```text
/* C_WaitForSlotEvent waits for a slot event (token insertion,
 * removal, etc.) to occur.
 */
```

## pkcs11f.h:934

```text
/* blocking/nonblocking flag */
```

## pkcs11f.h:935

```text
/* location that receives the slot ID */
```

## pkcs11f.h:936

```text
/* reserved.  Should be NULL_PTR */
```

## pkcs11f.h:941

```text
/* C_GetInterfaceList returns all the interfaces supported by the module*/
```

## pkcs11f.h:945

```text
/* returned interfaces */
```

## pkcs11f.h:946

```text
/* number of interfaces returned */
```

## pkcs11f.h:950

```text
/* C_GetInterface returns a specific interface from the module. */
```

## pkcs11f.h:954

```text
/* name of the interface */
```

## pkcs11f.h:955

```text
/* version of the interface */
```

## pkcs11f.h:956

```text
/* returned interface */
```

## pkcs11f.h:957

```text
/* flags controlling the semantics
                                         * of the interface */
```

## pkcs11f.h:965

```text
/* the session's handle */
```

## pkcs11f.h:966

```text
/* the user type */
```

## pkcs11f.h:967

```text
/* the user's PIN */
```

## pkcs11f.h:968

```text
/* the length of the PIN */
```

## pkcs11f.h:969

```text
/* the user's name */
```

## pkcs11f.h:970

```text
/*the length of the user's name */
```

## pkcs11f.h:977

```text
/* the session's handle */
```

## pkcs11f.h:978

```text
/* flags control which sessions are cancelled */
```

## pkcs11f.h:985

```text
/* the session's handle */
```

## pkcs11f.h:986

```text
/* the encryption mechanism */
```

## pkcs11f.h:987

```text
/* handle of encryption key */
```

## pkcs11f.h:994

```text
/* the session's handle */
```

## pkcs11f.h:995

```text
/* message specific parameter */
```

## pkcs11f.h:996

```text
/* length of message specific parameter */
```

## pkcs11f.h:997

```text
/* AEAD Associated data */
```

## pkcs11f.h:998

```text
/* AEAD Associated data length */
```

## pkcs11f.h:999

```text
/* plain text  */
```

## pkcs11f.h:1000

```text
/* plain text length */
```

## pkcs11f.h:1001

```text
/* gets cipher text */
```

## pkcs11f.h:1002

```text
/* gets cipher text length */
```

## pkcs11f.h:1009

```text
/* the session's handle */
```

## pkcs11f.h:1010

```text
/* message specific parameter */
```

## pkcs11f.h:1011

```text
/* length of message specific parameter */
```

## pkcs11f.h:1012

```text
/* AEAD Associated data */
```

## pkcs11f.h:1013

```text
/* AEAD Associated data length */
```

## pkcs11f.h:1020

```text
/* the session's handle */
```

## pkcs11f.h:1021

```text
/* message specific parameter */
```

## pkcs11f.h:1022

```text
/* length of message specific parameter */
```

## pkcs11f.h:1023

```text
/* plain text */
```

## pkcs11f.h:1024

```text
/* plain text length */
```

## pkcs11f.h:1025

```text
/* gets cipher text */
```

## pkcs11f.h:1026

```text
/* gets cipher text length */
```

## pkcs11f.h:1027

```text
/* multi mode flag */
```

## pkcs11f.h:1034

```text
/* the session's handle */
```

## pkcs11f.h:1041

```text
/* the session's handle */
```

## pkcs11f.h:1042

```text
/* the decryption mechanism */
```

## pkcs11f.h:1043

```text
/* handle of decryption key */
```

## pkcs11f.h:1050

```text
/* the session's handle */
```

## pkcs11f.h:1051

```text
/* message specific parameter */
```

## pkcs11f.h:1052

```text
/* length of message specific parameter */
```

## pkcs11f.h:1053

```text
/* AEAD Associated data */
```

## pkcs11f.h:1054

```text
/* AEAD Associated data length */
```

## pkcs11f.h:1055

```text
/* cipher text */
```

## pkcs11f.h:1056

```text
/* cipher text length */
```

## pkcs11f.h:1057

```text
/* gets plain text */
```

## pkcs11f.h:1058

```text
/* gets plain text length */
```

## pkcs11f.h:1065

```text
/* the session's handle */
```

## pkcs11f.h:1066

```text
/* message specific parameter */
```

## pkcs11f.h:1067

```text
/* length of message specific parameter */
```

## pkcs11f.h:1068

```text
/* AEAD Associated data */
```

## pkcs11f.h:1069

```text
/* AEAD Associated data length */
```

## pkcs11f.h:1076

```text
/* the session's handle */
```

## pkcs11f.h:1077

```text
/* message specific parameter */
```

## pkcs11f.h:1078

```text
/* length of message specific parameter */
```

## pkcs11f.h:1079

```text
/* cipher text */
```

## pkcs11f.h:1080

```text
/* cipher text length */
```

## pkcs11f.h:1081

```text
/* gets plain text */
```

## pkcs11f.h:1082

```text
/* gets plain text length */
```

## pkcs11f.h:1083

```text
/* multi mode flag */
```

## pkcs11f.h:1090

```text
/* the session's handle */
```

## pkcs11f.h:1097

```text
/* the session's handle */
```

## pkcs11f.h:1098

```text
/* the signing mechanism */
```

## pkcs11f.h:1099

```text
/* handle of signing key */
```

## pkcs11f.h:1106

```text
/* the session's handle */
```

## pkcs11f.h:1107

```text
/* message specific parameter */
```

## pkcs11f.h:1108

```text
/* length of message specific parameter */
```

## pkcs11f.h:1109

```text
/* data to sign */
```

## pkcs11f.h:1110

```text
/* data to sign length */
```

## pkcs11f.h:1111

```text
/* gets signature */
```

## pkcs11f.h:1112

```text
/* gets signature length */
```

## pkcs11f.h:1119

```text
/* the session's handle */
```

## pkcs11f.h:1120

```text
/* message specific parameter */
```

## pkcs11f.h:1121

```text
/* length of message specific parameter */
```

## pkcs11f.h:1128

```text
/* the session's handle */
```

## pkcs11f.h:1129

```text
/* message specific parameter */
```

## pkcs11f.h:1130

```text
/* length of message specific parameter */
```

## pkcs11f.h:1131

```text
/* data to sign */
```

## pkcs11f.h:1132

```text
/* data to sign length */
```

## pkcs11f.h:1133

```text
/* gets signature */
```

## pkcs11f.h:1134

```text
/* gets signature length */
```

## pkcs11f.h:1141

```text
/* the session's handle */
```

## pkcs11f.h:1148

```text
/* the session's handle */
```

## pkcs11f.h:1149

```text
/* the signing mechanism */
```

## pkcs11f.h:1150

```text
/* handle of signing key */
```

## pkcs11f.h:1157

```text
/* the session's handle */
```

## pkcs11f.h:1158

```text
/* message specific parameter */
```

## pkcs11f.h:1159

```text
/* length of message specific parameter */
```

## pkcs11f.h:1160

```text
/* data to sign */
```

## pkcs11f.h:1161

```text
/* data to sign length */
```

## pkcs11f.h:1162

```text
/* signature */
```

## pkcs11f.h:1163

```text
/* signature length */
```

## pkcs11f.h:1170

```text
/* the session's handle */
```

## pkcs11f.h:1171

```text
/* message specific parameter */
```

## pkcs11f.h:1172

```text
/* length of message specific parameter */
```

## pkcs11f.h:1179

```text
/* the session's handle */
```

## pkcs11f.h:1180

```text
/* message specific parameter */
```

## pkcs11f.h:1181

```text
/* length of message specific parameter */
```

## pkcs11f.h:1182

```text
/* data to sign */
```

## pkcs11f.h:1183

```text
/* data to sign length */
```

## pkcs11f.h:1184

```text
/* signature */
```

## pkcs11f.h:1185

```text
/* signature length */
```

## pkcs11f.h:1192

```text
/* the session's handle */
```

## pkcs11f.h:1200

```text
/* the session's handle */
```

## pkcs11f.h:1201

```text
/* the encapsulation mechanism */
```

## pkcs11f.h:1202

```text
/* the encapsulating key */
```

## pkcs11f.h:1203

```text
/* new key template */
```

## pkcs11f.h:1204

```text
/* template length */
```

## pkcs11f.h:1205

```text
/* the wrapped key */
```

## pkcs11f.h:1206

```text
/* the wrapped key size */
```

## pkcs11f.h:1207

```text
/* the encapsulated key */
```

## pkcs11f.h:1214

```text
/* the session's handle */
```

## pkcs11f.h:1215

```text
/* the decapsulation mechanism */
```

## pkcs11f.h:1216

```text
/* the decapsulating key */
```

## pkcs11f.h:1217

```text
/* new key template */
```

## pkcs11f.h:1218

```text
/* template length */
```

## pkcs11f.h:1219

```text
/* the wrapped key */
```

## pkcs11f.h:1220

```text
/* the wrapped key size */
```

## pkcs11f.h:1221

```text
/* the decapsulated key */
```

## pkcs11f.h:1228

```text
/* the session's handle */
```

## pkcs11f.h:1229

```text
/* the verification mechanism */
```

## pkcs11f.h:1230

```text
/* verification key */
```

## pkcs11f.h:1231

```text
/* signature */
```

## pkcs11f.h:1232

```text
/* signature length */
```

## pkcs11f.h:1239

```text
/* the session's handle */
```

## pkcs11f.h:1240

```text
/* signed data */
```

## pkcs11f.h:1241

```text
/* length of signed data */
```

## pkcs11f.h:1248

```text
/* the session's handle */
```

## pkcs11f.h:1249

```text
/* signed data */
```

## pkcs11f.h:1250

```text
/* length of signed data */
```

## pkcs11f.h:1257

```text
/* the session's handle */
```

## pkcs11f.h:1264

```text
/* the session's handle */
```

## pkcs11f.h:1265

```text
/* which state of flags */
```

## pkcs11f.h:1266

```text
/* validation flags */
```

## pkcs11f.h:1273

```text
/* the session's handle */
```

## pkcs11f.h:1274

```text
/* pkcs11 function name */
```

## pkcs11f.h:1275

```text
/* operation result */
```

## pkcs11f.h:1282

```text
/* the session's handle */
```

## pkcs11f.h:1283

```text
/* pkcs11 function name */
```

## pkcs11f.h:1284

```text
/* persistent operation id */
```

## pkcs11f.h:1291

```text
/* the session's handle */
```

## pkcs11f.h:1292

```text
/* pkcs11 function name */
```

## pkcs11f.h:1293

```text
/* persistent operation id */
```

## pkcs11f.h:1294

```text
/* location for the data */
```

## pkcs11f.h:1329

```text
/* CK_PKCS11_3_0_ONLY */
```

## pkcs11f.h:1330

```text
/* CK_PKCS11_2_0_ONLY */
```
