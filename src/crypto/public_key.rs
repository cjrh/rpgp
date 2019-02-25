use std::{fmt, io};

use num_bigint::BigUint;

use crypto::ecc_curve::ECCCurve;
use crypto::hash::HashAlgorithm;
use crypto::sym::SymmetricKeyAlgorithm;
use errors::Result;
use ser::Serialize;
use util::{write_bignum_mpi, write_mpi};

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum PublicKeyAlgorithm {
    /// RSA (Encrypt and Sign) [HAC]
    RSA = 1,
    /// DEPRECATED: RSA (Encrypt-Only) [HAC]
    RSAEncrypt = 2,
    /// DEPRECATED: RSA (Sign-Only) [HAC]
    RSASign = 3,
    /// Elgamal (Encrypt-Only) [ELGAMAL] [HAC]
    ElgamalSign = 16,
    /// DSA (Digital Signature Algorithm) [FIPS186] [HAC]
    DSA = 17,
    /// Elliptic Curve: RFC-6637
    ECDH = 18,
    /// ECDSA: RFC-6637
    ECDSA = 19,
    /// DEPRECATED: Elgamal (Encrypt and Sign)
    Elgamal = 20,
    /// Reserved for Diffie-Hellman (X9.42, as defined for IETF-S/MIME)
    DiffieHellman = 21,
    /// EdDSA (not yet assigned)
    EdDSA = 22,
    /// Private experimental range (from OpenGPG)
    // TODO: genenric Unknown(u8)
    Private100 = 100,
    Private101 = 101,
    Private102 = 102,
    Private103 = 103,
    Private104 = 104,
    Private105 = 105,
    Private106 = 106,
    Private107 = 107,
    Private108 = 108,
    Private109 = 109,
    Private110 = 110,
}

/// Represent the public paramaters for the different algorithms.
#[derive(PartialEq, Eq, Clone)]
pub enum PublicParams {
    RSA {
        n: BigUint,
        e: BigUint,
    },
    DSA {
        p: BigUint,
        q: BigUint,
        g: BigUint,
        y: BigUint,
    },
    ECDSA {
        curve: ECCCurve,
        p: Vec<u8>,
    },
    ECDH {
        curve: ECCCurve,
        p: Vec<u8>,
        hash: HashAlgorithm,
        alg_sym: SymmetricKeyAlgorithm,
    },
    Elgamal {
        p: BigUint,
        g: BigUint,
        y: BigUint,
    },
    EdDSA {
        curve: ECCCurve,
        q: Vec<u8>,
    },
}

impl Serialize for PublicParams {
    fn to_writer<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            PublicParams::RSA { ref n, ref e } => {
                write_bignum_mpi(n, writer)?;
                write_bignum_mpi(e, writer)?;
            }
            PublicParams::DSA {
                ref p,
                ref q,
                ref g,
                ref y,
            } => {
                write_bignum_mpi(p, writer)?;
                write_bignum_mpi(q, writer)?;
                write_bignum_mpi(g, writer)?;
                write_bignum_mpi(y, writer)?;
            }
            PublicParams::ECDSA { ref curve, ref p } => {
                let oid = curve.oid();
                writer.write_all(&[oid.len() as u8])?;
                writer.write_all(&oid)?;

                write_mpi(p, writer)?;
            }
            PublicParams::ECDH {
                ref curve,
                ref p,
                ref hash,
                ref alg_sym,
            } => {
                let oid = curve.oid();
                writer.write_all(&[oid.len() as u8])?;
                writer.write_all(&oid)?;

                write_mpi(p, writer)?;

                writer.write_all(&[
                    // len of the following fields
                    0x03,
                    // fixed tag
                    0x01,
                    *hash as u8,
                    *alg_sym as u8,
                ])?;
            }
            PublicParams::Elgamal {
                ref p,
                ref g,
                ref y,
            } => {
                write_bignum_mpi(p, writer)?;
                write_bignum_mpi(g, writer)?;
                write_bignum_mpi(y, writer)?;
            }
            PublicParams::EdDSA { ref curve, ref q } => {
                let oid = curve.oid();
                writer.write_all(&[oid.len() as u8])?;
                writer.write_all(&oid)?;

                write_mpi(q, writer)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for PublicParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PublicParams::RSA { ref n, ref e } => f
                .debug_struct("PublicParams::RSA")
                .field("n", &hex::encode(&n.to_bytes_be()))
                .field("e", &hex::encode(&e.to_bytes_be()))
                .finish(),
            PublicParams::DSA {
                ref p,
                ref q,
                ref g,
                ref y,
            } => f
                .debug_struct("PublicParams::DSA")
                .field("p", &hex::encode(&p.to_bytes_be()))
                .field("q", &hex::encode(&q.to_bytes_be()))
                .field("g", &hex::encode(&y.to_bytes_be()))
                .field("y", &hex::encode(&g.to_bytes_be()))
                .finish(),
            PublicParams::ECDSA { ref curve, ref p } => f
                .debug_struct("PublicParams::ECDSA")
                .field("curve", curve)
                .field("p", &hex::encode(p))
                .finish(),
            PublicParams::ECDH {
                ref curve,
                ref p,
                hash,
                alg_sym,
            } => f
                .debug_struct("PublicParams::ECDSA")
                .field("curve", curve)
                .field("hash", hash)
                .field("alg_sym", alg_sym)
                .field("p", &hex::encode(p))
                .finish(),
            PublicParams::Elgamal {
                ref p,
                ref g,
                ref y,
            } => f
                .debug_struct("PublicParams::Elgamal")
                .field("p", &hex::encode(&p.to_bytes_be()))
                .field("g", &hex::encode(&y.to_bytes_be()))
                .field("y", &hex::encode(&g.to_bytes_be()))
                .finish(),

            PublicParams::EdDSA { ref curve, ref q } => f
                .debug_struct("PublicParams::EdDSA")
                .field("curve", curve)
                .field("q", &hex::encode(q))
                .finish(),
        }
    }
}
