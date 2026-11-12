# Fe11-sys

Raw Rust declarations translated from the OASIS PKCS#11 3.2 headers.
The crate supplies C layouts, constants, and nullable unsafe function pointers.
Applications use these declarations to implement native calls with explicit
resource ownership and module loading. Rust 1.81 and edition 2021 are required;
the generated declarations are included in the crate.

## Supported ABI profiles

| Target | C data model | Structure layout |
| --- | --- | --- |
| x86_64-unknown-linux-gnu | LP64, 64-bit unsigned long | Natural alignment |
| x86_64-pc-windows-msvc | LLP64, 32-bit unsigned long | One-byte packing |

Only these exact target profiles are supported. Custom target descriptions that
imitate their configuration fields are outside this contract. A Linux module
using packed structures is incompatible with the Linux profile. The OASIS header
comments describe one-byte packing but leave the UNIX mechanism to the integrator;
this crate's natural Linux layout must therefore be checked against the module's
actual ABI before any call. No runtime ABI detection is provided.

## Caller obligations

The crate is `no_std`. Raw pointers convey neither ownership nor a valid lifetime.
The caller must establish buffer validity, lengths, initialization, synchronization,
callback lifetime, and the lifetime of the loaded image. A copied address does not
transfer ownership. Secret buffers remain the caller's responsibility.

Function tables for 2.40, 3.0, and 3.2 are distinct. Negotiate an interface and check
its version before accessing fields; a Rust table type does not authorize reading
a longer native table. A declared function or mechanism does not establish module
support. Check optional pointers before calling them.

Structures do not acquire blanket Copy, Clone, Debug, or Default implementations.
Initialize every field explicitly; a zeroed representation is not assumed to be
valid for an operation. Select diagnostic fields without exposing secrets or
addresses. Auto-traits do not establish native thread safety; honor the module's
initialization and synchronization contract.

On Windows, never form a reference to a potentially unaligned packed field. Use
value reads or appropriately justified raw-pointer and unaligned operations.
Check conversions between usize and CK_ULONG, especially its 32-bit Windows form.
CRYPTOKI_VERSION_* use C int while CK_VERSION fields use CK_BYTE. CK_CHAR and
CK_UTF8CHAR are unsigned octets, not a guarantee of valid UTF-8 or NUL termination.
C fields named `type` are exposed as `r#type`.

The aliases CKG_MGF1_SHA1, CKP_PKCS5_PBKD2_HMAC_SHA1, and CKH_HEDGE_PREFERRED
use CK_RSA_PKCS_MGF_TYPE, CK_PKCS5_PBKD2_PSEUDO_RANDOM_FUNCTION_TYPE, and
CK_HEDGE_TYPE respectively. These semantic typedefs retain their CK_ULONG ABI.
See [original header comments](HEADER-NOTES.md) for their source context.

## Source organization and provenance

Version 0.1.0 groups declarations into metadata, parameters, function-pointer
types, tables, and constant families. The crate root reexports these declarations.
Compile-time assertions compare Rust layouts with recorded native C layouts;
they do not establish compatibility with every module using these headers.

[provenance.json](provenance.json) records the OASIS inputs, generator settings
and constant origins. [source-layout.json](source-layout.json) records declaration
hashes and their locations. Together they document the origin and organization of the declarations.

Build with `cargo +1.81.0 build --release` on a supported target. Generate the public API reference with cargo doc --no-deps and open
target/doc/fe11_sys/index.html. Report security concerns privately to security@borealcryptosystems.com.

See [LICENSE](LICENSE), [COPYRIGHT](COPYRIGHT), and [NOTICE.txt](NOTICE.txt) for
licensing and source attribution, and [CONTRIBUTING.md](CONTRIBUTING.md) for maintenance and contributions.

## API reference

Browse the [Fe11-sys API reference](https://borealcryptosystems.github.io/fe11/0.1.0/api/fe11_sys/index.html)
for PKCS#11 constants, C types and function tables. The
[Fe11 How To guides](https://borealcryptosystems.github.io/fe11/0.1.0/library/docs/how-to/index.html)
cover module loading, sessions, cryptographic operations and PQC.
